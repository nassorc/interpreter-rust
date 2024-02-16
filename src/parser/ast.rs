use std::rc::Rc;
#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    BlockStatement(BlockStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    IfExpression(IfExpression),
    Function(FunctionLiteral),
    CallExpression(CallExpression),
    Ident(Identifier),
    Int(Integer),
    Boolean(Boolean),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Nil,
}

impl Node {
    pub fn is_nil(&self) -> bool {
        match self {
            Node::Nil => true,
            _ => false,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Node::Program(v) => Self::to_string_statements(&v.statements, "\n"),
            Node::BlockStatement(v) => Self::to_string_statements(&v.statements, "\n"),
            Node::LetStatement(v) => format!("let {} = {};", v.name.0, v.value.to_string()),
            Node::ReturnStatement(v) => format!("return {};", v.value.to_string()),
            Node::Int(v) => v.0.to_string(),
            Node::Boolean(v) => v.0.to_string(),
            Node::Ident(v) => v.0.to_string(),
            Node::Prefix(v) => format!("({}{})", v.op, v.right.to_string()),
            Node::Infix(v) => format!("({} {} {})", v.left.to_string(), v.op, v.right.to_string()),
            Node::IfExpression(v) => {
                let condition = v.condition.to_string();
                let consequence = Self::to_string_statements(&v.consequence, " ");
                let alternative = Self::to_string_statements(&v.alternative, " ");

                let alternative = if v.alternative.len() > 0 {
                    format!("else {{ {alternative} }}")
                } else {
                    String::from("")
                };
                format!("if ({condition}) {{ {consequence} }} {alternative}")
            }
            Node::Function(v) => {
                let parameters = Self::to_string_statements(&v.parameters, ", ");
                let Node::BlockStatement(body) = v.body.as_ref() else {
                    return String::from("");
                };
                let body = Self::to_string_statements(&body.statements, " ");
                format!("fn ({parameters}) {{ {body} }}")
            }
            Node::CallExpression(v) => {
                let args = Self::to_string_statements(&v.arguments, ", ");
                format!("{}({})", v.function.to_string(), args)
            }
            _ => String::from(""),
        }
    }
    fn tab_width(w: usize, s: &str) -> String {
        format!("{}{}", "\t".repeat(w), s)
    }
    fn to_string_statements(stmts: &[Node], sep: &str) -> String {
        stmts
            .iter()
            .map(|elm| elm.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub parameters: Vec<Node>,
    pub body: Rc<Node>,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Rc<Node>,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Rc<Node>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Rc<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(pub i32);

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub op: String,
    pub right: Rc<Node>,
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub op: String,
    pub right: Rc<Node>,
    pub left: Rc<Node>,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Rc<Node>,
    pub consequence: Vec<Node>, // block statement
    pub alternative: Vec<Node>,
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};
    #[test]
    fn test_node_to_string() {
        let tests = vec![
            ("10;", "10"),
            ("myVar;", "myVar"),
            ("false;", "false"),
            ("-10;", "(-10)"),
            ("5 + 10;", "(5 + 10)"),
            ("a + 10;", "(a + 10)"),
            ("let a = 100;", "let a = 100;"),
            ("return true;", "return true;"),
            (
                "if (10) { return 10; } else { return 5; }",
                "if (10) { return 10; } else { return 5; }",
            ),
            (
                "let myFunc = fn(a, b) { return -10 + 2; };",
                "let myFunc = fn (a, b) { return ((-10) + 2); };",
            ),
            ("myFunc(-10, false);", "myFunc((-10), false)"),
        ];

        for (idx, (input, test)) in tests.iter().enumerate() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let prog = parser.parse_program();

            let actual = prog.to_string();
            assert_eq!(actual, test.to_owned());
        }
    }
}
