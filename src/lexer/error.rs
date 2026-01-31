use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[diagnostic(code(shard::lexer))]
pub enum LexerError {
    #[error("Unexpected character: {found:?}")]
    #[diagnostic(help("Expected a valid token character"))]
    UnexpectedChar {
        found: char,
        #[source_code]
        src: String,
        #[label("here")]
        span: (usize, usize),
    },

    #[error("Unterminated string literal")]
    #[diagnostic(help("Strings must be closed with a matching quote"))]
    UnterminatedString {
        #[source_code]
        src: String,
        #[label("here")]
        span: (usize, usize),
    },
}

pub type LexerResult<T> = std::result::Result<T, LexerError>;
