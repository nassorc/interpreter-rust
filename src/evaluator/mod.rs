pub mod environment;
pub mod object;

use crate::parser::ast::*;
use environment::*;
use object::*;
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

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
        Node::Int(v) => Some(Object::Integer(IntegerObject { value: v.0 })),
        Node::Ident(v) => eval_identifier(v, env),
        Node::Boolean(v) => Some(Object::Boolean(to_native_bool(v.0))),
        _ => None,
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
            if v.value {
                true
            } else {
                false
            }
        }
        Object::Null => false,
        _ => true,
    }
}

fn eval_call(call: &CallExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let func_local_env = Environment::new_extended(Rc::clone(&env));
    match eval(call.function.borrow(), Rc::clone(&env)).unwrap() {
        Object::Function(f) => {
            // set arguments to parameters
            for (idx, param) in f.parameters.iter().enumerate() {
                match param {
                    Node::Ident(i) => {
                        let value = eval(&call.arguments[idx], Rc::clone(&env))?;
                        // func_local_env.borrow_mut().store.insert(i.0.clone(), e);
                        func_local_env.borrow_mut().insert(i.0.clone(), value);
                    }
                    _ => {
                        return None;
                    }
                }
            }

            // evaluate function body
            return eval(&f.body, Rc::clone(&func_local_env));
            // None
        }
        _ => {
            return None;
        }
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
                }
                _ => {
                    return None;
                }
            }
            if let Object::Integer(mut right) = right {
                right.value *= -1;
                Some(Object::Integer(right))
            } else {
                None
            }
        }
        "!" => {
            let right = eval(&prefix.right, env)?;
            match right {
                Object::Boolean(b) => {
                    return match b.value {
                        true => Some(Object::Boolean(FALSE.clone())),
                        false => Some(Object::Boolean(TRUE.clone())),
                    };
                }
                _ => return Some(Object::Boolean(FALSE)),
            }
        }
        _ => None,
    }
}

fn eval_infix(infix: &InfixExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let left = eval(&infix.left, Rc::clone(&env))?;
    let right = eval(&infix.right, Rc::clone(&env))?;

    //

    match (left, right, infix.op.as_str()) {
        (Object::Integer(left), Object::Integer(right), _) => {
            eval_integer_infix_opr(&infix.op, left, right)
        }
        (_, _, "==") => None,
        _ => None,
    }
}

fn eval_integer_infix_opr(op: &str, left: IntegerObject, right: IntegerObject) -> Option<Object> {
    let result = match op {
        "+" => Object::Integer(IntegerObject {
            value: left.value + right.value,
        }),
        "-" => Object::Integer(IntegerObject {
            value: left.value - right.value,
        }),
        "*" => Object::Integer(IntegerObject {
            value: left.value * right.value,
        }),
        "/" => Object::Integer(IntegerObject {
            value: left.value / right.value,
        }),
        "<" => Object::Boolean(to_native_bool(left.value < right.value)),
        ">" => Object::Boolean(to_native_bool(left.value > right.value)),
        "==" => Object::Boolean(to_native_bool(left.value == right.value)),
        "!=" => Object::Boolean(to_native_bool(left.value != right.value)),
        _ => Object::Null,
    };

    Some(result)
}

fn eval_identifier(ident: &Identifier, env: Rc<RefCell<Environment>>) -> Option<Object> {
    env.as_ref()
        .borrow()
        .get(ident.0.clone())
        .map_or(None, |v| Some(v.as_ref().borrow().clone()))
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
    Some(Object::Return(ReturnObject {
        value: Rc::new(value),
    }))
}

fn eval_function_literal(stmt: &FunctionLiteral, env: Rc<RefCell<Environment>>) -> Option<Object> {
    match stmt.body.as_ref() {
        Node::BlockStatement(v) => {
            return Some(Object::Function(FunctionLiteralObject {
                body: stmt.body.as_ref().clone(),
                parameters: stmt.parameters.clone(), // TODO!: cloning a vec. Use references.
            }));
        }
        _ => {
            return None;
        }
    }
    None
}

fn to_native_bool(v: bool) -> BooleanObject {
    // TODO!: reference the TRUE and FALSE object instead of cloning.
    if v {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::setup;

    #[test]
    fn test_eval_bool_expressions() {
        let tests = vec![
            ("true;", true),
            ("false;", false),
            ("!true;", false),
            ("!false;", true),
            ("!!true;", true),
            ("!!false;", false),
            ("!10;", false),
            ("!!10;", true),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Boolean(actual) = result else {
                assert!(false, "expected Object::Boolean, got=.");
                return;
            };
            assert_eq!(
                actual.value, expected,
                "{} should be {}, got {}",
                input, expected, actual.value
            );
        }
    }

    #[test]
    fn test_eval_interger_expressions() {
        let tests = vec![
            ("5;", 5),
            ("10;", 10),
            ("-5;", -5),
            ("-10;", -10),
            ("10 + 5;", 15),
            ("10 - 5;", 5),
            ("10 * 5;", 50),
            ("10 / 5;", 2),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Integer(actual) = result else {
                assert!(false, "expected Object::Integer, got=.");
                return;
            };
            assert_eq!(actual.value, expected);
        }
    }

    #[test]
    fn test_eval_let_statement() {
        let tests = vec![
            ("let a = 10; a;", 10),
            ("let b = 5; 5 + 2", 7),
            ("let x = 2; let y = 4; x + y;", 6),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Integer(actual) = result else {
                assert!(false, "expected Object::Integer, got=.");
                return;
            };
            assert_eq!(actual.value, expected);
        }
    }

    #[test]
    fn test_variable_environment_bindings() {
        let tests = vec![
            (
                "
            let global = 100;
            let a = 5;

            let myFunc = fn (a) { return a + global };
            myFunc(10);
            ",
                110,
            ),
            (
                "
            let global = 100;
            let a = 5;

            let myFunc = fn (a) { return a + global };
            myFunc(a);
            ",
                105,
            ),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Integer(actual) = result else {
                assert!(false, "expected Object::Integer, got=.");
                return;
            };
            assert_eq!(actual.value, expected);
        }
    }

    #[test]
    fn test_eval_return_statement() {
        let tests = vec![("return 5;", 5), ("return 5 * 2;", 10)];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Integer(actual) = result else {
                assert!(false, "expected Object::Integer, got=.");
                return;
            };
            assert_eq!(actual.value, expected);
        }
    }

    #[test]
    fn test_function_call() {
        let tests = vec![
            ("let sum = fn (x, y) { return x + y }; sum(10, 5)", 15),
            (
                "
            let sum = fn (x, y) { return x + y };
            let product = fn (x, y) { return x * y }; 
            sum(10, 5) + product(2, 5);
            ",
                25,
            ),
            ("fn (x) { return x; }(1)", 1),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env).expect(
                format!("expected evaluation to result in the value Some({expected}), got None",)
                    .as_str(),
            );
            let Object::Integer(actual) = result else {
                assert!(false, "expected Object::Integer, got=.");
                return;
            };
            assert_eq!(actual.value, expected);
        }
    }

    #[test]
    fn test_if_else_evaluation() {
        let tests = vec![
            ("if (5 == 5) { 5 } else { 10 }", Some(5)),
            ("if (5 != 5) { 5 } else { 10 }", Some(10)),
            ("if (false) { 5 }", None),
        ];

        for (input, expected) in tests {
            let (parser, prog) = setup(input);
            let env = Environment::new();
            let result = eval(&prog, env);

            match expected {
                Some(test) => {
                    let Object::Integer(actual) = result.expect("expected Some(Object), got=None") else {
                        assert!(false, "expected Object::Integer, got=.");
                        return;
                    };
                    assert_eq!(actual.value, expected.unwrap());
                }
                None => {
                    assert!(
                        result.is_none(),
                        "expected evaluation of {} to be {:?}",
                        input,
                        expected
                    );
                }
            }
        }
    }
}
