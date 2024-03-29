use crate::{
    lexer::Lexer,
    parser::{ast::Node, Parser},
};
pub fn setup(input: &str) -> (Parser, Node) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let prog = parser.parse_program();
    (parser, prog)
}
