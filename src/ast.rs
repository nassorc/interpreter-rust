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
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Nil
}

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(pub i32);

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixExpression {
    pub op: String,
    pub right: Box<Expression>
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixExpression {
    pub op: String,
    pub right: Box<Expression>,
    pub left: Box<Expression
    >
}