use crate::{
    evaluator::object::Object,
    evaluator::{environment::Environment, eval},
    lexer::Lexer,
    parser::Parser,
};
use std::rc::Rc;
pub struct App {
    input: String,
}

impl App {
    pub fn new<T: Into<String>>(input: T) -> Self {
        App {
            input: input.into(),
        }
    }

    pub fn execute(&self) -> Option<Object> {
        let lexer = Lexer::new(self.input.as_str());
        let mut parser = Parser::new(lexer);

        let prog = parser.parse_program();
        let mut env = Environment::new();

        eval(&prog, Rc::clone(&env))
    }
}
