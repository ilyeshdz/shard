pub mod error;
pub mod token;

pub use crate::lexer::token::Lexer;
pub use error::{LexerError, LexerResult};
pub use token::{tokenize, SpannedToken, Token, TokenType};
