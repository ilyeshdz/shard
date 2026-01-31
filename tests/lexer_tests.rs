use shard::lexer::{tokenize, TokenType};

#[test]
fn test_tokenize_identifier() {
    let tokens = tokenize("hello").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Identifier);
    assert_eq!(tokens[0].1.value, Some("hello".to_string()));
}

#[test]
fn test_tokenize_integer() {
    let tokens = tokenize("42").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Integer);
    assert_eq!(tokens[0].1.value, Some("42".to_string()));
}

#[test]
fn test_tokenize_boolean_true() {
    let tokens = tokenize("true").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Boolean);
    assert_eq!(tokens[0].1.value, Some("true".to_string()));
}

#[test]
fn test_tokenize_boolean_false() {
    let tokens = tokenize("false").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Boolean);
    assert_eq!(tokens[0].1.value, Some("false".to_string()));
}

#[test]
fn test_tokenize_null() {
    let tokens = tokenize("null").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Null);
    assert_eq!(tokens[0].1.value, Some("null".to_string()));
}

#[test]
fn test_tokenize_string_single_quotes() {
    let tokens = tokenize("'hello world'").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::String);
    assert_eq!(tokens[0].1.value, Some("hello world".to_string()));
}

#[test]
fn test_tokenize_string_with_spaces() {
    let tokens = tokenize("'  spaced out  '").unwrap();
    assert_eq!(tokens[0].1.value, Some("  spaced out  ".to_string()));
}

#[test]
fn test_tokenize_equals() {
    let tokens = tokenize("=").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Equals);
}

#[test]
fn test_tokenize_assignment() {
    let tokens = tokenize("x = 10").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Identifier);
    assert_eq!(tokens[1].1.token_type, TokenType::Equals);
    assert_eq!(tokens[2].1.token_type, TokenType::Integer);
}

#[test]
fn test_tokenize_command() {
    let tokens = tokenize("echo hello").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Identifier);
    assert_eq!(tokens[0].1.value, Some("echo".to_string()));
    assert_eq!(tokens[1].1.token_type, TokenType::Identifier);
    assert_eq!(tokens[1].1.value, Some("hello".to_string()));
}

#[test]
fn test_tokenize_multiple_statements() {
    let tokens = tokenize("x = 10\ny = 20").unwrap();
    // x = 10 \n y = 20 \n EOF
    assert_eq!(tokens[0].1.value, Some("x".to_string()));
    // Find the identifier token for y
    let y_token = tokens.iter().find(|t| t.1.value == Some("y".to_string()));
    assert!(y_token.is_some());
}

#[test]
fn test_tokenize_underscore_identifier() {
    let tokens = tokenize("_var_name").unwrap();
    assert_eq!(tokens[0].1.value, Some("_var_name".to_string()));
}

#[test]
fn test_tokenize_mixed_alphanumeric() {
    let tokens = tokenize("var123").unwrap();
    assert_eq!(tokens[0].1.value, Some("var123".to_string()));
}

#[test]
fn test_tokenize_empty_input() {
    let tokens = tokenize("").unwrap();
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(tokens[0].1.token_type, TokenType::EOF);
}

#[test]
fn test_tokenize_whitespace_only() {
    let tokens = tokenize("   \n\t  ").unwrap();
    // With newlines as tokens, we get: Newline, EOF
    assert!(tokens.len() >= 1);
    assert_eq!(tokens[tokens.len() - 1].1.token_type, TokenType::EOF);
}

#[test]
fn test_tokenize_complex_expression() {
    let tokens = tokenize("name = 'Shard'\necho name").unwrap();
    // Should have: name, =, 'Shard', Newline, echo, name, EOF
    assert!(tokens.len() >= 6);
}

#[test]
fn test_tokenize_string_with_escape() {
    let tokens = tokenize("'hello\\'world'").unwrap();
    assert_eq!(tokens[0].1.value, Some("hello'world".to_string()));
}

#[test]
fn test_tokenize_large_number() {
    let tokens = tokenize("999999999").unwrap();
    assert_eq!(tokens[0].1.value, Some("999999999".to_string()));
}

#[test]
fn test_tokenize_leading_zeros() {
    let tokens = tokenize("007").unwrap();
    assert_eq!(tokens[0].1.value, Some("007".to_string()));
}

#[test]
fn test_unterminated_string_error() {
    let result = tokenize("'unterminated");
    assert!(result.is_err());
}

#[test]
fn test_unexpected_character_error() {
    let result = tokenize("@#$");
    assert!(result.is_err());
}

#[test]
fn test_tokenize_reserved_words_as_identifiers() {
    // true123 should be identifier, not boolean
    let tokens = tokenize("true123").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Identifier);

    // nullify should be identifier
    let tokens = tokenize("nullify").unwrap();
    assert_eq!(tokens[0].1.token_type, TokenType::Identifier);
}

#[test]
fn test_tokenize_command_with_arguments() {
    let tokens = tokenize("ls -la /home").unwrap();
    assert_eq!(tokens[0].1.value, Some("ls".to_string()));
    assert_eq!(tokens[1].1.value, Some("-la".to_string()));
    assert_eq!(tokens[2].1.value, Some("/home".to_string()));
}

#[test]
fn test_tokenize_newline_separator() {
    let tokens = tokenize("a = 1\nb = 2").unwrap();
    let newline_count = tokens
        .iter()
        .filter(|t| t.1.token_type == TokenType::Newline)
        .count();
    assert_eq!(newline_count, 1);
}
