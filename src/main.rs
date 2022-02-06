mod lexer;
mod parser;
mod repl;

use crate::repl::repl_loop;

fn main() {
    repl_loop();
}
