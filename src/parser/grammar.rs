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
        TokenType::RBrace => {
            // Block delimiter - return None to signal end of block
            Ok(None)
        }
        TokenType::Identifier => {
            let name = token.value.clone().unwrap_or_default();
            let keyword = name.as_str();

            // Check for keywords BEFORE incrementing position
            match keyword {
                "if" => return parse_if(tokens, pos),
                "while" => return parse_while(tokens, pos),
                "for" => return parse_for(tokens, pos),
                "fn" => return parse_function_def(tokens, pos),
                "return" => return parse_return(tokens, pos),
                "try" => return parse_try(tokens, pos),
                "break" => {
                    *pos += 1;
                    consume_newline(tokens, pos);
                    return Ok(Some(Statement::Break));
                }
                "continue" => {
                    *pos += 1;
                    consume_newline(tokens, pos);
                    return Ok(Some(Statement::Continue));
                }
                _ => {}
            }

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

            // Check if this is a function call like "len(arr)"
            if *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::LParen {
                    let name_value = name.clone();
                    *pos += 1;
                    let func_args = parse_function_args(tokens, pos)?;
                    args.push(Expression::FunctionCall {
                        name: name_value,
                        args: func_args,
                    });
                }
            }

            // Parse remaining command arguments
            while *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::Newline
                    || next_token.token_type == TokenType::EOF
                    || next_token.token_type == TokenType::RBrace
                {
                    break;
                }
                args.push(parse_command_argument(tokens, pos)?);
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

fn consume_newline(tokens: &[SpannedToken], pos: &mut usize) {
    if *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::Newline {
            *pos += 1;
        }
    }
}

fn get_token_type(tokens: &[SpannedToken], pos: usize) -> TokenType {
    if pos < tokens.len() {
        tokens[pos].1.token_type.clone()
    } else {
        TokenType::EOF
    }
}

fn parse_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    parse_or_expression(tokens, pos)
}

fn parse_command_argument(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    if *pos >= tokens.len() {
        return Err(ParserError::Other("Unexpected end of input".to_string()));
    }

    let (_, token, _) = &tokens[*pos];

    match &token.token_type {
        TokenType::Integer => {
            let val = token.value.clone().unwrap_or_default().parse().unwrap_or(0);
            *pos += 1;
            Ok(Expression::Literal(Literal::Integer(val)))
        }
        TokenType::Boolean => {
            let val = token.value.clone().unwrap_or_default() == "true";
            *pos += 1;
            Ok(Expression::Literal(Literal::Boolean(val)))
        }
        TokenType::Null => {
            *pos += 1;
            Ok(Expression::Literal(Literal::Null))
        }
        TokenType::String => {
            let val = token.value.clone().unwrap_or_default();
            *pos += 1;
            Ok(Expression::Literal(Literal::String(val)))
        }
        TokenType::InterpolatedString => {
            let content = token.value.clone().unwrap_or_default();
            *pos += 1;
            Ok(Expression::Literal(Literal::String(content)))
        }
        TokenType::Identifier => {
            let name = token.value.clone().unwrap_or_default();
            *pos += 1;

            if *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::LParen {
                    *pos += 1;
                    let args = parse_function_args(tokens, pos)?;
                    return Ok(Expression::FunctionCall { name, args });
                }
            }

            Ok(Expression::Identifier(name))
        }
        TokenType::Minus => {
            // Check if this is a flag like "-la"
            let mut value = String::from("-");
            *pos += 1;

            // Consume following alphanumeric characters
            while *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if let TokenType::Identifier = &next_token.token_type {
                    if let Some(ident_value) = &next_token.value {
                        if ident_value
                            .chars()
                            .next()
                            .map_or(false, |c| c.is_alphanumeric())
                        {
                            value.push_str(ident_value);
                            *pos += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            Ok(Expression::Literal(Literal::String(value)))
        }
        TokenType::Slash => {
            // Handle paths like "/home/user/projects"
            let mut value = String::from("/");
            *pos += 1;

            // Consume identifier after first slash, then continue with slash + identifier pairs
            while *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if let TokenType::Identifier = &next_token.token_type {
                    if let Some(ident_value) = &next_token.value {
                        value.push_str(ident_value);
                        *pos += 1;

                        // Check if there's another slash followed by identifier
                        if *pos + 1 < tokens.len() {
                            let (_, slash_token, _) = &tokens[*pos];
                            let (_, next_ident_token, _) = &tokens[*pos + 1];
                            if slash_token.token_type == TokenType::Slash
                                && next_ident_token.token_type == TokenType::Identifier
                            {
                                value.push('/');
                                *pos += 1;
                                continue;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            Ok(Expression::Literal(Literal::String(value)))
        }
        _ => Err(ParserError::Other(format!(
            "Unexpected token in command argument: {:?}",
            token.token_type
        ))),
    }
}

fn parse_or_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut left = parse_and_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::Or {
            *pos += 1;
            let right = parse_and_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_and_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut left = parse_equality_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::And {
            *pos += 1;
            let right = parse_equality_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_if(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;
    let condition = parse_expression(tokens, pos)?;

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other(
            "Expected '{' after if condition".to_string(),
        ));
    }
    *pos += 1;

    let then_branch = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after if block".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    let else_branch = if *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if let TokenType::Identifier = token.token_type {
            if token.value.as_ref().map_or(false, |v| v == "else") {
                *pos += 1;
                consume_newline(tokens, pos);

                if *pos < tokens.len() {
                    let (_, next_token, _) = &tokens[*pos];
                    if let TokenType::Identifier = next_token.token_type {
                        if next_token.value.as_ref().map_or(false, |v| v == "if") {
                            return parse_if(tokens, pos);
                        }
                    }
                }

                if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
                    return Err(ParserError::Other("Expected '{' after else".to_string()));
                }
                *pos += 1;

                let else_body = parse_block(tokens, pos)?;

                if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
                    return Err(ParserError::Other(
                        "Expected '}' after else block".to_string(),
                    ));
                }
                *pos += 1;

                consume_newline(tokens, pos);
                Some(else_body)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok(Some(Statement::If {
        condition,
        then_branch,
        else_branch,
    }))
}

fn parse_while(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;
    let condition = parse_expression(tokens, pos)?;

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other(
            "Expected '{' after while condition".to_string(),
        ));
    }
    *pos += 1;

    let body = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after while block".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    Ok(Some(Statement::While { condition, body }))
}

fn parse_for(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;

    if *pos >= tokens.len() {
        return Err(ParserError::Other(
            "Expected variable name after for".to_string(),
        ));
    }

    let (_, token, _) = &tokens[*pos];
    let variable = if let TokenType::Identifier = token.token_type {
        token.value.clone().unwrap_or_default()
    } else {
        return Err(ParserError::Other(
            "Expected variable name after for".to_string(),
        ));
    };
    *pos += 1;

    if *pos >= tokens.len() {
        return Err(ParserError::Other(
            "Expected 'in' after variable".to_string(),
        ));
    }

    let (_, next_token, _) = &tokens[*pos];
    if let TokenType::Identifier = next_token.token_type {
        if next_token.value.as_ref().map_or(false, |v| v == "in") {
            *pos += 1;
        } else {
            return Err(ParserError::Other(
                "Expected 'in' after variable".to_string(),
            ));
        }
    } else {
        return Err(ParserError::Other(
            "Expected 'in' after variable".to_string(),
        ));
    }

    let iterable = parse_expression(tokens, pos)?;

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other(
            "Expected '{' after for iterable".to_string(),
        ));
    }
    *pos += 1;

    let body = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after for block".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    Ok(Some(Statement::For {
        variable,
        iterable,
        body,
    }))
}

fn parse_function_def(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;

    if *pos >= tokens.len() {
        return Err(ParserError::Other(
            "Expected function name after fn".to_string(),
        ));
    }

    let (_, token, _) = &tokens[*pos];
    let name = if let TokenType::Identifier = token.token_type {
        token.value.clone().unwrap_or_default()
    } else {
        return Err(ParserError::Other(
            "Expected function name after fn".to_string(),
        ));
    };
    *pos += 1;

    let mut params = Vec::new();
    if *pos < tokens.len() && get_token_type(tokens, *pos) == TokenType::LParen {
        *pos += 1;

        while *pos < tokens.len() {
            let (_, token, _) = &tokens[*pos];
            if token.token_type == TokenType::RParen {
                *pos += 1;
                break;
            }
            if let TokenType::Identifier = &token.token_type {
                if let Some(param) = &token.value {
                    params.push(param.clone());
                }
                *pos += 1;

                if *pos < tokens.len() {
                    let (_, next_token, _) = &tokens[*pos];
                    if next_token.token_type == TokenType::Comma {
                        *pos += 1;
                        continue;
                    }
                }
            } else {
                break;
            }
        }
    }

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other(
            "Expected '{' after function signature".to_string(),
        ));
    }
    *pos += 1;

    let body = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after function body".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    Ok(Some(Statement::FunctionDef {
        name,
        params,
        body,
        return_value: None,
    }))
}

fn parse_return(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;

    let value = if *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::Newline || token.token_type == TokenType::EOF {
            None
        } else {
            Some(parse_expression(tokens, pos)?)
        }
    } else {
        None
    };

    consume_newline(tokens, pos);
    Ok(Some(Statement::Return { value }))
}

fn parse_try(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Option<Statement>> {
    *pos += 1;

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other("Expected '{' after try".to_string()));
    }
    *pos += 1;

    let body = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after try block".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    if *pos >= tokens.len() {
        return Err(ParserError::Other(
            "Expected 'catch' after try block".to_string(),
        ));
    }

    let (_, token, _) = &tokens[*pos];
    if let TokenType::Identifier = token.token_type {
        if token.value.as_ref().map_or(false, |v| v == "catch") {
            *pos += 1;
        } else {
            return Err(ParserError::Other("Expected 'catch'".to_string()));
        }
    } else {
        return Err(ParserError::Other("Expected 'catch'".to_string()));
    }

    let catch_var = if *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if let TokenType::Identifier = token.token_type {
            let var = token.value.clone().unwrap_or_default();
            *pos += 1;
            var
        } else {
            "e".to_string()
        }
    } else {
        "e".to_string()
    };

    consume_newline(tokens, pos);

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::LBrace {
        return Err(ParserError::Other("Expected '{' after catch".to_string()));
    }
    *pos += 1;

    let catch_body = parse_block(tokens, pos)?;

    if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RBrace {
        return Err(ParserError::Other(
            "Expected '}' after catch block".to_string(),
        ));
    }
    *pos += 1;

    consume_newline(tokens, pos);

    Ok(Some(Statement::Try {
        body,
        catch_var,
        catch_body,
    }))
}

fn parse_block(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Vec<Statement>> {
    let mut statements = Vec::new();

    // Skip initial newlines after opening brace
    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::Newline {
            *pos += 1;
        } else {
            break;
        }
    }

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::RBrace || token.token_type == TokenType::EOF {
            break;
        }
        if let Some(stmt) = parse_statement(tokens, pos)? {
            statements.push(stmt);
        }
    }

    Ok(statements)
}

fn parse_equality_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut left = parse_comparison_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        let op = match &token.token_type {
            TokenType::EqEq => Some(BinaryOperator::Equals),
            TokenType::NotEq => Some(BinaryOperator::NotEquals),
            _ => None,
        };

        if let Some(op) = op {
            *pos += 1;
            let right = parse_comparison_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_comparison_expression(
    tokens: &[SpannedToken],
    pos: &mut usize,
) -> ParserResult<Expression> {
    let mut left = parse_additive_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        let op = match &token.token_type {
            TokenType::Less => Some(BinaryOperator::Less),
            TokenType::Greater => Some(BinaryOperator::Greater),
            TokenType::LessEq => Some(BinaryOperator::LessEquals),
            TokenType::GreaterEq => Some(BinaryOperator::GreaterEquals),
            _ => None,
        };

        if let Some(op) = op {
            *pos += 1;
            let right = parse_additive_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_additive_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut left = parse_multiplicative_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        let op = match &token.token_type {
            TokenType::Plus => Some(BinaryOperator::Add),
            TokenType::Minus => Some(BinaryOperator::Subtract),
            _ => None,
        };

        if let Some(op) = op {
            *pos += 1;
            let right = parse_multiplicative_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_multiplicative_expression(
    tokens: &[SpannedToken],
    pos: &mut usize,
) -> ParserResult<Expression> {
    let mut left = parse_unary_expression(tokens, pos)?;

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        let op = match &token.token_type {
            TokenType::Star => Some(BinaryOperator::Multiply),
            TokenType::Slash => Some(BinaryOperator::Divide),
            TokenType::Percent => Some(BinaryOperator::Modulo),
            _ => None,
        };

        if let Some(op) = op {
            *pos += 1;
            let right = parse_unary_expression(tokens, pos)?;
            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_unary_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    if *pos >= tokens.len() {
        return Err(ParserError::Other("Unexpected end of input".to_string()));
    }

    let (_, token, _) = &tokens[*pos];

    match &token.token_type {
        TokenType::Minus => {
            *pos += 1;
            let expr = parse_unary_expression(tokens, pos)?;
            Ok(Expression::UnaryOp {
                op: UnaryOperator::Negate,
                expr: Box::new(expr),
            })
        }
        TokenType::Not => {
            *pos += 1;
            let expr = parse_unary_expression(tokens, pos)?;
            Ok(Expression::UnaryOp {
                op: UnaryOperator::Not,
                expr: Box::new(expr),
            })
        }
        _ => parse_primary_expression(tokens, pos),
    }
}

fn parse_primary_expression(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    if *pos >= tokens.len() {
        return Err(ParserError::Other("Unexpected end of input".to_string()));
    }

    let (_, token, _) = &tokens[*pos];

    match &token.token_type {
        TokenType::Integer => {
            let val = token.value.clone().unwrap_or_default().parse().unwrap_or(0);
            *pos += 1;
            Ok(Expression::Literal(Literal::Integer(val)))
        }
        TokenType::Boolean => {
            let val = token.value.clone().unwrap_or_default() == "true";
            *pos += 1;
            Ok(Expression::Literal(Literal::Boolean(val)))
        }
        TokenType::Null => {
            *pos += 1;
            Ok(Expression::Literal(Literal::Null))
        }
        TokenType::String => {
            let val = token.value.clone().unwrap_or_default();
            *pos += 1;
            Ok(Expression::Literal(Literal::String(val)))
        }
        TokenType::InterpolatedString => {
            let content = token.value.clone().unwrap_or_default();
            *pos += 1;
            parse_interpolated_string(tokens, pos, content)
        }
        TokenType::Identifier => {
            let name = token.value.clone().unwrap_or_default();
            *pos += 1;

            if *pos < tokens.len() {
                let (_, next_token, _) = &tokens[*pos];
                if next_token.token_type == TokenType::LParen {
                    *pos += 1;
                    let args = parse_function_args(tokens, pos)?;
                    return Ok(Expression::FunctionCall { name, args });
                }
            }

            Ok(Expression::Identifier(name))
        }
        TokenType::LBracket => {
            *pos += 1;
            parse_array(tokens, pos)
        }
        TokenType::LBrace => {
            *pos += 1;
            parse_map(tokens, pos)
        }
        TokenType::LParen => {
            *pos += 1;
            let expr = parse_expression(tokens, pos)?;
            if *pos >= tokens.len() || get_token_type(tokens, *pos) != TokenType::RParen {
                return Err(ParserError::Other("Expected ')'".to_string()));
            }
            *pos += 1;
            Ok(expr)
        }
        _ => Err(ParserError::Other(format!(
            "Unexpected token in expression: {:?}",
            token.token_type
        ))),
    }
}

fn parse_array(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut elements = Vec::new();

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::RBracket {
            *pos += 1;
            break;
        }
        if token.token_type == TokenType::Newline || token.token_type == TokenType::EOF {
            return Err(ParserError::Other("Expected ']' or element".to_string()));
        }

        elements.push(parse_expression(tokens, pos)?);

        if *pos < tokens.len() {
            let (_, token, _) = &tokens[*pos];
            if token.token_type == TokenType::Comma {
                *pos += 1;
                continue;
            }
        }
    }

    Ok(Expression::Literal(Literal::Array(elements)))
}

fn parse_map(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Expression> {
    let mut pairs = Vec::new();

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::RBrace {
            *pos += 1;
            break;
        }
        if token.token_type == TokenType::Newline || token.token_type == TokenType::EOF {
            return Err(ParserError::Other("Expected '}' or key".to_string()));
        }

        let key = parse_expression(tokens, pos)?;

        if *pos >= tokens.len() {
            return Err(ParserError::Other("Expected ':' after key".to_string()));
        }

        let (_, colon_token, _) = &tokens[*pos];
        if colon_token.token_type != TokenType::Colon {
            return Err(ParserError::Other("Expected ':' after key".to_string()));
        }
        *pos += 1;

        let value = parse_expression(tokens, pos)?;
        pairs.push((key, value));

        if *pos < tokens.len() {
            let (_, token, _) = &tokens[*pos];
            if token.token_type == TokenType::Comma {
                *pos += 1;
                continue;
            }
        }
    }

    Ok(Expression::Literal(Literal::Map(pairs)))
}

fn parse_function_args(tokens: &[SpannedToken], pos: &mut usize) -> ParserResult<Vec<Expression>> {
    let mut args = Vec::new();

    while *pos < tokens.len() {
        let (_, token, _) = &tokens[*pos];
        if token.token_type == TokenType::RParen {
            *pos += 1;
            break;
        }
        if token.token_type == TokenType::Newline || token.token_type == TokenType::EOF {
            return Err(ParserError::Other("Expected ')' or argument".to_string()));
        }

        args.push(parse_expression(tokens, pos)?);

        if *pos < tokens.len() {
            let (_, token, _) = &tokens[*pos];
            if token.token_type == TokenType::Comma {
                *pos += 1;
                continue;
            }
        }
    }

    Ok(args)
}

fn parse_interpolated_string(
    _tokens: &[SpannedToken],
    _pos: &mut usize,
    content: String,
) -> ParserResult<Expression> {
    Ok(Expression::Literal(Literal::String(content)))
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
