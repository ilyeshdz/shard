use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(shard::parser))]
pub enum ParserError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Parser error: {0}")]
    Other(String),
}

pub type ParserResult<T> = std::result::Result<T, ParserError>;
