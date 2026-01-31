use shard::lexer::tokenize;
use shard::parser::parse;

#[test]
fn test_parse_comment_only() {
    let tokens = tokenize("# This is a comment").unwrap();
    assert!(tokens
        .iter()
        .any(|t| t.1.token_type == shard::lexer::TokenType::Comment));
}

#[test]
fn test_parse_comment_with_code() {
    let tokens = tokenize("# comment\nx = 10").unwrap();
    assert!(tokens
        .iter()
        .any(|t| t.1.token_type == shard::lexer::TokenType::Comment));
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_multiple_comments() {
    let tokens = tokenize("# comment 1\nx = 1\n# comment 2\ny = 2").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 2);
}

#[test]
fn test_parse_array() {
    let tokens = tokenize("arr = [1, 2, 3]").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_nested_array() {
    let tokens = tokenize("matrix = [[1, 2], [3, 4]]").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_empty_array() {
    let tokens = tokenize("empty = []").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_map() {
    let tokens = tokenize("config = {key: value, num: 42}").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_empty_map() {
    let tokens = tokenize("empty = {}").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_string_with_hash() {
    let tokens = tokenize("text = 'hello # not a comment'").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_arithmetic() {
    let tokens = tokenize("x = 1 + 2 * 3 - 4 / 2").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_comparison() {
    let tokens = tokenize("result = x == y and a != b or c < d").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_unary_not() {
    let tokens = tokenize("flag = not true").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_unary_minus() {
    let tokens = tokenize("neg = -5").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_len_function() {
    let tokens = tokenize("len(arr)").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_modulo() {
    let tokens = tokenize("x = 10 % 3").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_parentheses() {
    let tokens = tokenize("x = (1 + 2) * 3").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_complex_expression() {
    let tokens = tokenize("result = (a + b) * (c - d) / 2 + len(arr) % 10").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}
