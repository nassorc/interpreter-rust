#![allow(unused)]

mod token;
mod lexer;
mod parser;
mod ast;
mod object;
mod eval;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use lexer::Lexer;
use parser::Parser;
use object::Environment;
use eval::eval;

use crate::object::{IntegerObject, Object};

#[derive(Debug)]
struct TEnv {
    store: HashMap<String, String>,
    outer: Option<Rc<RefCell<TEnv>>>
}

fn main() {
    let input = "let a = if (true) { 10 + 2; 2; } else { 5 }";
    let input = "
    let a = fn(x) { let b = 10; return b + x; 1000; };
    a(1) + 4";
    let input = "if (2 + 4) { 10 + 2; 2; }; 10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let prog = parser.parse_program();

    dbg!(&prog);

    let mut env = Environment::new();
    {
        env.borrow_mut().store.insert(
            "myVar".to_string(), 
            Rc::new(RefCell::new(
                Object::Integer(IntegerObject{ value: 101 })
            ))
        );
    }
    // dbg!(&env);

    dbg!(eval(&prog, Rc::clone(&env)));

    // let mut global_env = Rc::new(RefCell::new(Environment { 
    //     store: HashMap::new(),
    //     outer: None
    // }));

    // let t = Rc::new(RefCell::new(TEnv {
    //     store: HashMap::new(),
    //     outer: None
    // }));

    // let t_clone = Rc::clone(&t);

    // t.borrow_mut().store.insert(String::from("newnew"), String::from("blahblah"));

    // dbg!(&t);

    // let mut t2 = TEnv{
    //     store: HashMap::new(),
    //     outer: Some(Rc::clone(&t))
    // };

    // t2.store.insert(String::from("local to function"), String::from("locallocal"));

    // dbg!(&t2);
    // dbg!((*(t2.outer.unwrap().borrow_mut())).store.insert("lala".to_string(), "rara".to_string()));
    // dbg!(&t);
    
}

