use shard::lexer::tokenize;
use shard::parser::parse;

#[test]
fn test_parse_if_statement() {
    let tokens = tokenize("if x == 1 { echo one }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_if_else_statement() {
    let tokens = tokenize("if x == 1 { echo one } else { echo other }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_if_else_if_chain() {
    let tokens =
        tokenize("if x == 1 { echo one } else if x == 2 { echo two } else { echo other }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_while_loop() {
    let tokens = tokenize("while x < 10 { x = x + 1 }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_for_loop() {
    let tokens = tokenize("for item in [1, 2, 3] { echo item }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_for_loop_with_array() {
    let tokens = tokenize("for i in arr { echo i }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_break_statement() {
    let tokens = tokenize("while true { break }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_continue_statement() {
    let tokens = tokenize("for i in arr { continue }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_try_catch() {
    let tokens = tokenize("try { echo trying } catch e { echo caught }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_return_statement() {
    let tokens = tokenize("return 42").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_return_empty() {
    let tokens = tokenize("return").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_nested_control_flow() {
    let tokens = tokenize("if x { while y { for z in arr { break } } }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}
