pub mod environment;
pub mod object;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use crate::{
    parser::ast::*, object::*, environment::*
};
use environment::*;
use object::*;


// TODO: create function to handle block statments: IfStatements, FunctionLiterals, 

pub fn eval(node: &Node, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let env = Rc::clone(&env);
    match node {
        Node::Program(p) => eval_statements(&p.statements, env),
        Node::BlockStatement(p) => eval_statements(&p.statements, env),
        Node::CallExpression(v) => eval_call(v, env),
        Node::LetStatement(v) => eval_let_statement(v, env),
        Node::ReturnStatement(v) => eval_return_statement(v, env),
        Node::IfExpression(v) => eval_if_expression(v, env),
        Node::Prefix(v) => eval_prefix(v, env),
        Node::Infix(v) => eval_infix(v, env),
        Node::Function(v) => eval_function_literal(v, env),
        Node::Int(v) => Some(Object::Integer(IntegerObject{value: v.0})),
        Node::Boolean(v) => Some(Object::Boolean(BooleanObject{value: v.0})),
        Node::Ident(v) => eval_identifier(v, env),
        _ => None
    }
}

fn eval_statements(statements: &[Node], env: Rc<RefCell<Environment>>) -> Option<Object> {
    let mut result = None;
    for stmt in statements {
        result = eval(stmt, Rc::clone(&env));

        if let Some(Object::Return(v)) = result {
            return Some(v.value.as_ref().clone());
        }
    }
    result
}

fn eval_block_statements(statements: &[Node], env: Rc<RefCell<Environment>>) -> Option<Object> {
    let mut result = None;
    for stmt in statements {
        result = eval(stmt, Rc::clone(&env));

        if let Some(Object::Return(v)) = result {
            return Some(Object::Return(v));
        }
    }
    result
}

fn eval_if_expression(stmt: &IfExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let condition = eval(stmt.condition.as_ref(), Rc::clone(&env))?;

    if is_truthy(&condition) {
        return eval_block_statements(&stmt.consequence, Rc::clone(&env));
    } else {
        return eval_block_statements(&stmt.alternative, Rc::clone(&env));
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(v) => {
            if v.value { true } else { false }
        }
        Object::Null => false,
        _ => true
    }
}

fn eval_call(call: &CallExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let func_local_env = Environment::new_extended(Rc::clone(&env));
    // TODO!: handle other evaluated cases 
    match eval(call.function.borrow(), Rc::clone(&env)).unwrap() {
        Object::Function(f) => {
            // set arguments to parameters
            for (idx, param) in f.parameters.iter().enumerate() {
                match param {
                    Node::Ident(i) => {
                        let value = eval(&call.arguments[idx], Rc::clone(&env))?;
                        // func_local_env.borrow_mut().store.insert(i.0.clone(), e);
                        func_local_env.borrow_mut().insert(i.0.clone(), value);
                    },
                    _ => {return None;}
                }
            }

            // evaluate function body
            return eval(&f.body, Rc::clone(&func_local_env));
            // None
        },
        _ => {return None;}
    }

    None
}

fn eval_prefix(prefix: &PrefixExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    match prefix.op.as_str() {
        "-" => {
            let right = eval(&prefix.right, env)?;
            match right {
                Object::Integer(mut right) => {
                    right.value *= -1;
                    return Some(Object::Integer(right));
                },
                _ => { return None; }
            }
            if let Object::Integer(mut right) = right {
                right.value *= -1;
                Some(Object::Integer(right))
            } else {
                None
            }
        },
        "!" => {
            let right = eval(&prefix.right, env)?;
            match right {
                Object::Boolean(b) => {
                    return match b.value {
                        true => Some(Object::Boolean(FALSE)),
                        false => Some(Object::Boolean(TRUE)),
                    };
                },
                _ => return Some(Object::Boolean(TRUE))
            }
        },
        _ => None
    }
}

fn eval_infix(infix: &InfixExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {

    let left = eval(&infix.left, Rc::clone(&env))?;
    let right = eval(&infix.right, Rc::clone(&env))?;

    // most of the time, we only need a copy. 

    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => eval_integer_infix_opr(&infix.op, left, right),
        _ => None
    }
}

fn eval_integer_infix_opr(op: &str, left: IntegerObject, right: IntegerObject) -> Option<Object>{
    match op {
        "+" => Some(Object::Integer(IntegerObject {
            value: left.value + right.value
        })),
        _ => None
    }
}

fn eval_identifier(ident: &Identifier, env: Rc<RefCell<Environment>>) -> Option<Object> {
    env.as_ref().borrow().get(ident.0.clone()).map_or(None, |v| Some(v.as_ref().borrow().clone()))
}

fn eval_let_statement(stmt: &LetStatement, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let value = eval(&stmt.value, Rc::clone(&env))?;
    if let Some(_) = env.borrow_mut().insert(stmt.name.0.clone(), value) {
        return Some(Object::Null);
    }
    None
}

fn eval_return_statement(stmt: &ReturnStatement, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let value = eval(stmt.value.as_ref(), Rc::clone(&env)).unwrap();
    Some(Object::Return(ReturnObject{ value: Rc::new(value) }))
}

fn eval_function_literal(stmt: &FunctionLiteral, env: Rc<RefCell<Environment>>) -> Option<Object> {
    match stmt.body.as_ref() {
        Node::BlockStatement(v) => {
            return Some(Object::Function(FunctionLiteralObject {
                // body: v.clone(),            // TODO!: CLONING A VEC
                body: stmt.body.as_ref().clone(),
                parameters: stmt.parameters.clone() // TODO!: CLONING A VEC
            }));
        },
        _ => { return None; }
    }
    None
}