mod object_builder;

use clap::Parser;
use inkwell::context::Context;
use object_builder::FileBuilder;

/// Compiler for light programming language
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Light modules files
    pub files: Vec<String>,

    /// Build only objects files
    #[clap(short = 'c', long = "only-objects")]
    pub only_objects: bool,

    /// Print LLVM IR code for each module to stdout
    #[clap(short = 'p', long = "print-llvm-ir")]
    pub print_ir_code: bool,

    /// Output name
    #[clap(short, long, default_value = "program")]
    pub output: String,
}

fn build_objects(filenames: &Vec<String>, builder: &mut FileBuilder, print_ir: bool) -> bool {
    let mut failure = false;
    for file in filenames {
        if !builder.generate_module_ir(file.as_str(), print_ir) {
            failure = true;
        }
    }

    if failure {
        return false;
    }

    builder.build_objects_files()
}

fn main() {
    let args = Args::parse();
    let ctx = Context::create();
    let mut builder = FileBuilder::new(&ctx);

    if args.files.len() == 0 {
        std::process::exit(0);
    }

    if !build_objects(&args.files, &mut builder, args.print_ir_code) {
        std::process::exit(1);
    }

    if !args.only_objects {
        if !builder.link_executable(&args.output) {
            std::process::exit(2);
        }
    }
}
