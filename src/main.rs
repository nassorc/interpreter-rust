#![allow(unused)]

mod app;
mod evaluator;
mod lexer;
mod parser;
mod utils;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use app::App;
use evaluator::{environment::Environment, object::Object, *};
use lexer::Lexer;
use parser::{
    ast::{Identifier, Integer, LetStatement, Node},
    Parser,
};

fn main() {
    let input = "let a = if (true) { 10 + 2; 2; } else { 5 }";
    let input = "let a = if (false) { 10 + 2; return 99; } else { return 55; }; a;";
    let input = "
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
    ";

    // dbg!(&App::new(
    //     "
    //     let global = 100;
    //     let a = fn(x) {
    //         if (false) {
    //             return 0;
    //         } else {
    //             let a = 10;
    //             return a + x + global;
    //         }
    //     };
    //     a(1) + 4
    // "
    // )
    // .execute());

    let input = "
    x + y + myVar;
    10
    let a = 111;
    let b = 222;
    100; 200;
    return true;
    if (10) { 10; 20; let a = 200; } else { 2; }
    !false;
    let myFunc = fn (a, b) { return -10 + 2; };
    myFunc(-10, false);
    a();
    ";
    let input = "if (false) { 5 }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let prog = parser.parse_program();
    println!("{}", prog.to_string());
    let mut env = Environment::new();

    // dbg!(&prog);
    // dbg!(&env);
    dbg!(eval(&prog, Rc::clone(&env)));
}
