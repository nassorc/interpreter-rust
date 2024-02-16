use crate::{lexer::Lexer, parser::Parser, Node};
pub fn setup(input: &str) -> (Parser, Node) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let prog = parser.parse_program();
    (parser, prog)
}
