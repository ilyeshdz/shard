use insta::assert_snapshot;
use shard::{generate, parse, tokenize};

#[test]
fn test_snapshot_simple_assignment() {
    let tokens = tokenize("x = 10").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_string_assignment() {
    let tokens = tokenize("name = 'Shard'").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_boolean_assignment() {
    let tokens = tokenize("active = true").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_null_assignment() {
    let tokens = tokenize("value = null").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_simple_command() {
    let tokens = tokenize("echo hello").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_command_with_args() {
    let tokens = tokenize("ls -la /home").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_mixed_program() {
    let input = "name = 'Shard'\nversion = 1\necho name\nprint version";
    let tokens = tokenize(input).unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_complex_command() {
    let input = "cmd arg1 'string literal' 42 true";
    let tokens = tokenize(input).unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_full_example() {
    let input = "name = 'Shard'\necho 'Hello' name";
    let tokens = tokenize(input).unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_multiple_assignments() {
    let input = "x = 1\ny = 2\nz = 'three'";
    let tokens = tokenize(input).unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}

#[test]
fn test_snapshot_empty_program() {
    let tokens = tokenize("").unwrap();
    let ast = parse(tokens).unwrap();
    let output = generate(&ast).unwrap();
    assert_snapshot!(output);
}
