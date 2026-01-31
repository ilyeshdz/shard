use crate::lexer::error::LexerError;
use crate::lexer::error::LexerResult;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Integer,
    Boolean,
    Null,
    String,
    Equals,
    Whitespace,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: (usize, usize),
    pub value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, end: usize, value: Option<String>) -> Self {
        Token {
            token_type,
            span: (start, end),
            value,
        }
    }
}

pub type SpannedToken = (usize, Token, usize);

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    pos: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input,
            chars,
            pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
        let start = self.pos;
        self.advance();

        let mut value = String::new();
        while let Some(c) = self.current_char {
            if c == '\'' {
                self.advance();
                return Ok(value);
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    value.push(escaped);
                    self.advance();
                }
            } else {
                value.push(c);
                self.advance();
            }
        }
        Err(LexerError::UnterminatedString {
            src: self.input.to_string(),
            span: (start, self.pos),
        })
    }

    fn read_number(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<SpannedToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.current_char?;

        let start = self.pos;

        let token = match self.current_char {
            Some(c) if c.is_alphabetic() || c == '_' => {
                let value = self.read_identifier();
                match value.as_str() {
                    "true" => Token::new(TokenType::Boolean, start, self.pos, Some(value)),
                    "false" => Token::new(TokenType::Boolean, start, self.pos, Some(value)),
                    "null" => Token::new(TokenType::Null, start, self.pos, Some(value)),
                    _ => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let value = self.read_number();
                Token::new(TokenType::Integer, start, self.pos, Some(value))
            }
            Some('\'') => match self.read_string() {
                Ok(value) => Token::new(TokenType::String, start, self.pos, Some(value)),
                Err(e) => return Some(Err(e)),
            },
            Some('=') => {
                self.advance();
                Token::new(TokenType::Equals, start, self.pos, None)
            }
            Some(c) => {
                self.advance();
                return Some(Err(LexerError::UnexpectedChar {
                    src: self.input.to_string(),
                    span: (start, self.pos),
                    found: c,
                }));
            }
            None => unreachable!(),
        };

        Some(Ok((start, token, self.pos)))
    }
}

pub fn tokenize(input: &str) -> LexerResult<Vec<SpannedToken>> {
    let lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    for result in lexer {
        tokens.push(result?);
    }
    tokens.push((
        tokens.len(),
        Token::new(TokenType::EOF, tokens.len(), tokens.len(), None),
        tokens.len(),
    ));
    Ok(tokens)
}
