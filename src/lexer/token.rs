use crate::lexer::error::LexerError;
use crate::lexer::error::LexerResult;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier,
    Integer,
    Boolean,
    Null,
    String,
    InterpolatedString,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    And,
    Or,
    Not,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Arrow,
    Newline,
    Whitespace,
    Comment,
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
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
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

    fn read_double_string(&mut self) -> String {
        self.advance();

        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    value.push(escaped);
                    self.advance();
                }
            } else if c == '\n' {
                break;
            } else {
                value.push(c);
                self.advance();
            }
        }
        value
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<SpannedToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.current_char.is_none() {
            return None;
        }

        let start = self.pos;

        let token = match self.current_char {
            Some('#') => {
                self.skip_comment();
                Token::new(TokenType::Comment, start, self.pos, None)
            }
            Some('\n') => {
                self.advance();
                Token::new(TokenType::Newline, start, self.pos, None)
            }
            Some('"') => {
                let content = self.read_double_string();
                Token::new(
                    TokenType::InterpolatedString,
                    start,
                    self.pos,
                    Some(content),
                )
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let value = self.read_identifier();
                match value.as_str() {
                    "true" => Token::new(TokenType::Boolean, start, self.pos, Some(value)),
                    "false" => Token::new(TokenType::Boolean, start, self.pos, Some(value)),
                    "null" => Token::new(TokenType::Null, start, self.pos, Some(value)),
                    "if" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "else" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "while" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "for" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "in" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "fn" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "return" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "try" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "catch" => Token::new(TokenType::Identifier, start, self.pos, Some(value)),
                    "and" => Token::new(TokenType::And, start, self.pos, Some(value)),
                    "or" => Token::new(TokenType::Or, start, self.pos, Some(value)),
                    "not" => Token::new(TokenType::Not, start, self.pos, Some(value)),
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
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::EqEq, start, self.pos, None)
                } else {
                    Token::new(TokenType::Equals, start, self.pos, None)
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::NotEq, start, self.pos, None)
                } else {
                    Token::new(TokenType::Not, start, self.pos, None)
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::LessEq, start, self.pos, None)
                } else {
                    Token::new(TokenType::Less, start, self.pos, None)
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::new(TokenType::GreaterEq, start, self.pos, None)
                } else {
                    Token::new(TokenType::Greater, start, self.pos, None)
                }
            }
            Some('+') => {
                self.advance();
                Token::new(TokenType::Plus, start, self.pos, None)
            }
            Some('-') => {
                self.advance();
                if self.current_char == Some('>') {
                    self.advance();
                    Token::new(TokenType::Arrow, start, self.pos, None)
                } else {
                    Token::new(TokenType::Minus, start, self.pos, None)
                }
            }
            Some('*') => {
                self.advance();
                Token::new(TokenType::Star, start, self.pos, None)
            }
            Some('/') => {
                self.advance();
                Token::new(TokenType::Slash, start, self.pos, None)
            }
            Some('%') => {
                self.advance();
                Token::new(TokenType::Percent, start, self.pos, None)
            }
            Some('(') => {
                self.advance();
                Token::new(TokenType::LParen, start, self.pos, None)
            }
            Some(')') => {
                self.advance();
                Token::new(TokenType::RParen, start, self.pos, None)
            }
            Some('[') => {
                self.advance();
                Token::new(TokenType::LBracket, start, self.pos, None)
            }
            Some(']') => {
                self.advance();
                Token::new(TokenType::RBracket, start, self.pos, None)
            }
            Some('{') => {
                self.advance();
                Token::new(TokenType::LBrace, start, self.pos, None)
            }
            Some('}') => {
                self.advance();
                Token::new(TokenType::RBrace, start, self.pos, None)
            }
            Some(',') => {
                self.advance();
                Token::new(TokenType::Comma, start, self.pos, None)
            }
            Some(':') => {
                self.advance();
                Token::new(TokenType::Colon, start, self.pos, None)
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
