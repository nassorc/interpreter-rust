use std::rc::Rc;
#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    BlockStatement(BlockStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    IfExpression(IfExpression),
    ExpressionStatement(Expression),
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
            _ => false
        }
    }
}

pub trait ExpressionNode {}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(Expression),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(Identifier),
    Int(Integer),
    // Prefix(PrefixExpression),
    // Infix(InfixExpression),
    Nil
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Node>
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Node>
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub parameters: Vec<Node>,
    pub body: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Rc<Node>,
    pub arguments: Vec<Node>
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Rc<Node>
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
    pub right: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub op: String,
    pub right: Rc<Node>,
    pub left: Rc<Node>
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Rc<Node>,
    pub consequence: Vec<Node>, // block statement
    pub alternative: Vec<Node>,
}