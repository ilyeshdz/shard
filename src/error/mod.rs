use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(shard::error))]
pub enum ShardError {
    #[error("Lexer error: {0}")]
    Lexer(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Codegen error: {0}")]
    Codegen(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<crate::lexer::error::LexerError> for ShardError {
    fn from(e: crate::lexer::error::LexerError) -> Self {
        ShardError::Lexer(format!("{:?}", e))
    }
}

impl From<crate::parser::error::ParserError> for ShardError {
    fn from(e: crate::parser::error::ParserError) -> Self {
        ShardError::Parser(format!("{:?}", e))
    }
}

impl From<crate::codegen::error::CodegenError> for ShardError {
    fn from(e: crate::codegen::error::CodegenError) -> Self {
        ShardError::Codegen(format!("{:?}", e))
    }
}

pub type Result<T> = std::result::Result<T, ShardError>;
