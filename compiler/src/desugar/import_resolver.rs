use std::{fs, path::Path};

use crate::parser::visitors::{FunctionStatement, ImportStatement, Statement, StructStatement};
use crate::{lexer::Token, parser::parser::Parser};

pub struct ImportResolver {
    declared_functions: Vec<FunctionStatement>,
    declared_structs: Vec<StructStatement>,
}

type ImportResolverReturn = Result<(), String>;

impl ImportResolver {
    pub fn new() -> Self {
        ImportResolver {
            declared_functions: Vec::new(),
            declared_structs: Vec::new(),
        }
    }

    pub fn resolve_imports(
        &mut self,
        stmts: &Vec<Statement>,
        file_name: &str,
    ) -> Result<Vec<Statement>, String> {
        for stmt in stmts {
            if let Statement::Import(is) = stmt {
                self.resolve_statement(is, file_name)?;
            }
        }

        let mut result: Vec<Statement> = Vec::new();

        result.extend(
            self.declared_functions
                .iter()
                .map(|f| Statement::Function(f.clone())),
        );

        result.extend(
            self.declared_structs
                .iter()
                .map(|s| Statement::Struct(s.clone())),
        );

        // Add everything else except Import statements since they're resolved now.
        result.extend(
            stmts
                .iter()
                .filter(|s| match s {
                    Statement::Import(_) => false,
                    _ => true,
                })
                .map(|s| s.clone()),
        );

        Ok(result)
    }

    fn parse_file(path: &str) -> Result<Vec<Statement>, String> {
        let file_path = format!("{}.lht", path);
        let file_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(msg) => {
                return Err(format!(
                    "Error while reading imported file {}: {}",
                    path, msg
                ))
            }
        };

        let tokens = Token::lex_string(file_content.as_str());
        let p = Path::new(path).parent().unwrap().to_str().unwrap();
        let mut parser = Parser::new(tokens, p, p);

        match parser.parse() {
            Some(stmts) => Ok(stmts),
            None => Err(format!(
                "Error while parsing imported file '{}' check errors above.",
                path
            )),
        }
    }

    fn resolve_statement(
        &mut self,
        import_stmt: &ImportStatement,
        file_name: &str,
    ) -> ImportResolverReturn {
        // the file_path is relative to the module path so we need
        // to concat the path.
        let path = Path::new(&import_stmt.file_path).join(import_stmt.module_path.as_str());
        let stmts = Self::parse_file(path.to_str().unwrap())?;

        for stmt in stmts {
            match stmt {
                Statement::Function(f) => {
                    let declaration = FunctionStatement {
                        callee: f.callee,
                        args: f.args,
                        block: None, // forward declaration
                        return_type: f.return_type,
                        is_exported: false,
                        line: f.line,
                        column: f.column,
                        // Path is the actual module path
                        filename: file_name.to_string(),
                    };

                    if f.is_exported {
                        self.declared_functions.push(declaration)
                    }
                }
                Statement::Struct(s) => {
                    if s.exported {
                        self.declared_structs.push(s)
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }
}
