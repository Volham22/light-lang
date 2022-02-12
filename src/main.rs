mod generation;
mod lexer;
mod parser;
mod repl;
mod type_system;

use crate::repl::repl_loop;

fn main() {
    repl_loop();
}
