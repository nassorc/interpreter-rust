use crate::{parser::Parser, Node, lexer::Lexer};
pub fn setup(input: &str) -> (Parser, Node) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let prog = parser.parse_program();
    (parser, prog)
}