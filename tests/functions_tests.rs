use shard::codegen::generate;
use shard::lexer::tokenize;
use shard::parser::parse;

#[test]
fn test_parse_function_definition() {
    let tokens = tokenize("fn greet(name) { echo name }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_function_no_params() {
    let tokens = tokenize("fn hello() { echo hello }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_function_multiple_params() {
    let tokens = tokenize("fn add(a, b) { return a + b }").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_function_call() {
    let tokens = tokenize("greet world").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_function_call_with_args() {
    let tokens = tokenize("add 1 2").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);
}

#[test]
fn test_parse_function_definition_and_call() {
    let tokens = tokenize("fn greet(name) { echo name }\ngreet world").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 2);
}

#[test]
fn test_codegen_function_definition() {
    let tokens = tokenize("fn greet(name) { echo name }").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert!(output.contains("greet() {"));
}

#[test]
fn test_codegen_function_call() {
    let tokens = tokenize("greet world").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert!(output.contains("greet"));
}

#[test]
fn test_codegen_function_with_return() {
    let tokens = tokenize("fn add(a, b) { return a + b }").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert!(output.contains("return"));
}
