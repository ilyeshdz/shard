#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    String(String),
    Array(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    ArrayIndex {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    MapIndex {
        map: Box<Expression>,
        key: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    InterpolatedString {
        parts: Vec<Expression>,
    },
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    Length {
        expr: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expression,
    },
    Command {
        name: String,
        args: Vec<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        return_value: Option<Expression>,
    },
    Return {
        value: Option<Expression>,
    },
    Try {
        body: Vec<Statement>,
        catch_var: String,
        catch_body: Vec<Statement>,
    },
    Break,
    Continue,
    ExpressionStatement(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Statement>);
