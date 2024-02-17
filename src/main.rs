#![allow(unused)]

mod app;
mod evaluator;
mod lexer;
mod parser;
mod repl;
mod utils;

fn main() {
    repl::Repl::new().start();
}
