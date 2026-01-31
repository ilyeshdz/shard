#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Boolean(bool),
    Null,
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment { name: String, value: Expression },
    Command { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Statement>);
