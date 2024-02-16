#![allow(unused)]

mod lexer;
mod parser;
mod evaluator;
mod app;

use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use app::App;
use lexer::Lexer;
use parser::Parser;
use evaluator::{environment::Environment, object::Object, *};


fn main() {
    
    dbg!(&App::new("
        let global = 100;
        let a = fn(x) { 
            if (false) {
                return 0;
            } else {
                let a = 10; 
                return a + x + global; 
            }
        };
        a(1) + 4
    ").execute());


    // let input = "let a = if (true) { 10 + 2; 2; } else { 5 }";
    // let input = "let a = if (false) { 10 + 2; return 99; } else { return 55; }; a;";
    // let input = "
    // let global = 100;
    // let a = fn(x) { 
    //     if (false) {
    //         return 0;
    //     } else {
    //         let a = 10; 
    //         return a + x + global; 
    //     }
    // };
    // a(1) + 4
    // ";
    // let lexer = Lexer::new(input);
    // let mut parser = Parser::new(lexer);

    // let prog = parser.parse_program();
    // let mut env = Environment::new();

    // dbg!(&prog);
    // dbg!(&env);
    // dbg!(eval(&prog, Rc::clone(&env)));
}

