use std::{fs, path::Path, process::Command, str::FromStr};

use compiler::{
    desugar::{desugar_ast, import_resolver::ImportResolver},
    generation::ir_generator::{create_generator, IRGenerator},
    lexer::Token,
    parser::parser::Parser,
    type_system::type_check::TypeChecker,
};

use inkwell::{
    context::Context,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use logos::Logos;

pub struct FileBuilder<'m> {
    pub context: &'m Context,
    pub modules: Vec<(String, IRGenerator<'m>)>,
}

impl<'m> FileBuilder<'m> {
    pub fn new(ctx: &'m Context) -> Self {
        Self {
            context: ctx,
            modules: Vec::new(),
        }
    }

    pub fn generate_module_ir(&mut self, path: &str, print_ir_code: bool) -> bool {
        let content = if let Ok(c) = Self::read_file_content(path) {
            c
        } else {
            return false;
        };

        let lexer = Token::lexer(content.as_str());
        let tokens = lexer.collect();
        let module_dir = Self::extract_module_directory(path);
        let mut parser = Parser::new(tokens, &module_dir);
        let mut import_resolve = ImportResolver::new();

        if let Some(mut stmts) = parser.parse() {
            match import_resolve.resolve_imports(&stmts) {
                Ok(s) => stmts = s,
                Err(msg) => {
                    eprintln!("{}", msg);
                    return false;
                }
            }

            let mut type_checker = TypeChecker::new();

            let t_check = type_checker.check_ast_type(&mut stmts);
            if let Ok(_) = t_check {
                let mut generator =
                    create_generator(self.context, path, &type_checker.get_type_table());
                generator.module = self
                    .context
                    .create_module(Path::new(path).file_name().unwrap().to_str().unwrap());

                desugar_ast(&mut stmts);
                generator.generate_ir(&stmts);

                if print_ir_code {
                    generator.print_code();
                }
                self.modules
                    .push((String::from_str(path).unwrap(), generator));
            } else {
                eprintln!("{}", t_check.err().unwrap());
                return false;
            }
        } else {
            return false;
        }

        true
    }

    pub fn build_objects_files(&self) -> bool {
        for (name, generator) in &self.modules {
            Target::initialize_x86(&InitializationConfig::default());
            let opt = OptimizationLevel::Default;
            let reloc = RelocMode::Default;
            let model = CodeModel::Default;
            let target = Target::from_name("x86-64").unwrap();

            let target_machine = target
                .create_target_machine(
                    &TargetMachine::get_default_triple(),
                    "x86-64",
                    "",
                    opt,
                    reloc,
                    model,
                )
                .unwrap();

            if let Err(msg) = target_machine.write_to_file(
                &generator.module,
                FileType::Object,
                &Path::new(&(name.to_string() + ".o")),
            ) {
                eprintln!("{}", msg);
                return false;
            }
        }

        true
    }

    // TODO: Support for non x64-32 systems
    pub fn link_executable(&self, name: &str) -> bool {
        if let Ok(s) = Command::new("ld")
            .arg("-m")
            .arg("elf_x86_64")
            .args(self.build_objects_list())
            .arg("/lib64/crt1.o") // C runtime
            .arg("-lc") // Link Lib C
            .arg("-dynamic-linker") // Use ld-linux-*
            .arg("/lib/ld-linux-x86-64.so.2")
            .arg("-o")
            .arg(name)
            .status()
        {
            s.success()
        } else {
            false
        }
    }

    fn read_file_content(path: &str) -> Result<String, ()> {
        let read_result = fs::read_to_string(path);

        if let Ok(content) = read_result {
            Ok(content)
        } else {
            eprintln!("{}", read_result.err().unwrap());
            Err(())
        }
    }

    fn build_objects_list(&self) -> Vec<String> {
        self.modules
            .iter()
            .map(|t| t.0.to_string() + ".o")
            .collect()
    }

    fn extract_module_directory(path: &str) -> String {
        let p = Path::new(path);
        let full_path = fs::canonicalize(p).unwrap();

        String::from(full_path.parent().unwrap().to_str().unwrap())
    }
}
