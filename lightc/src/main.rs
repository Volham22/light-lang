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

    /// Output name
    #[clap(short, long, default_value = "program")]
    pub output: String,
}

fn build_objects(filenames: &Vec<String>, builder: &mut FileBuilder) {
    let mut failure = false;
    for file in filenames {
        if !builder.generate_module_ir(file.as_str()) {
            failure = true;
        }
    }

    if failure {
        return;
    }

    builder.build_objects_files();
}

fn main() {
    let args = Args::parse();
    let ctx = Context::create();
    let mut builder = FileBuilder::new(&ctx);

    println!("Building files: {:?}", &args.files);
    build_objects(&args.files, &mut builder);
    println!("Output: {}", &args.output);
}
