use crate::ast::{BinaryOperator, Expression, Literal, Program, Statement, UnaryOperator};
use crate::lexer::{SpannedToken, TokenType};
use crate::parser::error::ParserError;
use crate::parser::error::ParserResult;

pub fn parse(tokens: Vec<SpannedToken>) -> ParserResult<Program> {
    let mut statements = Vec::new();
    let mut pos = 0;

    while pos < tokens.len() {
        if let Some(stmt) = parse_statement(&tokens, &mut pos)? {
            statements.push(stmt);
        } else {
            break;
        }
    }

    Ok(Program(statements))
}

fn parse_statement(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    if *pos >= tokens.len() {
        return Ok(None);
    }

    let (_, token, _) = &tokens[*pos];

    match token.token_type {
        TokenType::Comment => {
            *pos += 1;
            parse_statement(tokens, pos)
        }
        TokenType::Newline => {
            *pos += 1;
            parse_statement(tokens, pos)
        }
        TokenType::Identifier => {
            let name = token.value.clone().unwrap_or_default();
            *pos += 1;

            if *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::Equals {
                    *pos += 1;
                    let expr = parse_expression(tokens, pos)?;
                    return Ok(Some(Statement::Assignment { name, value: expr }));
                }
            }

            let mut args = Vec::new();
            while *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::Newline
                    || next_token.token_type == TokenType::EOF
                {
                    break;
                }
                args.push(parse_expression(tokens, pos)?);
            }
            Ok(Some(Statement::Command { name, args }))
        }
        TokenType::EOF => Ok(None),
        _ => Err(ParserError::Other(format!(
            "Unexpected token at position {}: {:?}",
            token.span.0, token.token_type
        ))),
    }
}

fn parse_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    if *pos >= tokens.len() {
        return Err(ParserError::Other("Unexpected end of input".to_string()));
    }

    let (_, token, _) = &tokens[*pos];
    *pos += 1;

    match token.token_type {
        TokenType::Integer => {
            let val = token.value.clone().unwrap_or_default().parse().unwrap_or(0);
            Ok(Expression::Literal(Literal::Integer(val)))
        }
        TokenType::Boolean => {
            let val = token.value.clone().unwrap_or_default() == "true";
            Ok(Expression::Literal(Literal::Boolean(val)))
        }
        TokenType::Null => Ok(Expression::Literal(Literal::Null)),
        TokenType::String => Ok(Expression::Literal(Literal::String(
            token.value.clone().unwrap_or_default(),
        ))),
        TokenType::Identifier => Ok(Expression::Identifier(
            token.value.clone().unwrap_or_default(),
        )),
        _ => Err(ParserError::Other(format!(
            "Unexpected token: {:?}",
            token.token_type
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse_assignment() {
        let tokens = tokenize("x = 10").unwrap();
        let result = parse(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_command() {
        let tokens = tokenize("echo hello").unwrap();
        let result = parse(tokens);
        assert!(result.is_ok());
    }
}
