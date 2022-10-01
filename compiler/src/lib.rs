mod debug;
pub mod desugar;
pub mod generation;
pub mod lexer;
pub mod parser;
pub mod type_system;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
