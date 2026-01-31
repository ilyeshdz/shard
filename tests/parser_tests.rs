use shard::ast::{Expression, Literal, Statement};
use shard::{parse, tokenize};

#[test]
fn test_parse_simple_assignment() {
    let tokens = tokenize("x = 10").unwrap();
    let ast = parse(tokens).unwrap();
    assert_eq!(ast.0.len(), 1);

    if let Statement::Assignment { name, value } = &ast.0[0] {
        assert_eq!(name, "x");
        if let Expression::Literal(Literal::Integer(n)) = value {
            assert_eq!(*n, 10);
        } else {
            panic!("Expected integer literal");
        }
    } else {
        panic!("Expected assignment");
    }
}

#[test]
fn test_parse_assignment_with_string() {
    let tokens = tokenize("name = 'Shard'").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Assignment { name, value } = &ast.0[0] {
        assert_eq!(name, "name");
        if let Expression::Literal(Literal::String(s)) = value {
            assert_eq!(s, "Shard");
        } else {
            panic!("Expected string literal");
        }
    } else {
        panic!("Expected assignment");
    }
}

#[test]
fn test_parse_assignment_with_boolean() {
    let tokens = tokenize("flag = true").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Assignment { name, value } = &ast.0[0] {
        assert_eq!(name, "flag");
        if let Expression::Literal(Literal::Boolean(b)) = value {
            assert!(*b);
        } else {
            panic!("Expected boolean literal");
        }
    } else {
        panic!("Expected assignment");
    }
}

#[test]
fn test_parse_assignment_with_null() {
    let tokens = tokenize("val = null").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Assignment { name, value } = &ast.0[0] {
        assert_eq!(name, "val");
        assert!(matches!(value, Expression::Literal(Literal::Null)));
    } else {
        panic!("Expected assignment");
    }
}

#[test]
fn test_parse_simple_command() {
    let tokens = tokenize("echo hello").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Command { name, args } = &ast.0[0] {
        assert_eq!(name, "echo");
        assert_eq!(args.len(), 1);
        if let Expression::Identifier(arg) = &args[0] {
            assert_eq!(arg, "hello");
        } else {
            panic!("Expected identifier argument");
        }
    } else {
        panic!("Expected command");
    }
}

#[test]
fn test_parse_command_with_literal_args() {
    let tokens = tokenize("print 123").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Command { name, args } = &ast.0[0] {
        assert_eq!(name, "print");
        assert_eq!(args.len(), 1);
        if let Expression::Literal(Literal::Integer(n)) = &args[0] {
            assert_eq!(*n, 123);
        } else {
            panic!("Expected integer argument");
        }
    } else {
        panic!("Expected command");
    }
}

#[test]
fn test_parse_command_with_mixed_args() {
    let tokens = tokenize("cmd arg1 'string' 42").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Command { name, args } = &ast.0[0] {
        assert_eq!(name, "cmd");
        assert_eq!(args.len(), 3);
        assert!(matches!(args[0], Expression::Identifier(_)));
        assert!(matches!(args[1], Expression::Literal(Literal::String(_))));
        assert!(matches!(args[2], Expression::Literal(Literal::Integer(42))));
    } else {
        panic!("Expected command");
    }
}

#[test]
fn test_parse_multiple_statements() {
    let tokens = tokenize("x = 1\ny = 2\necho x").unwrap();
    let ast = parse(tokens).unwrap();

    assert_eq!(ast.0.len(), 3);
    assert!(matches!(ast.0[0], Statement::Assignment { .. }));
    assert!(matches!(ast.0[1], Statement::Assignment { .. }));
    assert!(matches!(ast.0[2], Statement::Command { .. }));
}

#[test]
fn test_parse_command_no_args() {
    let tokens = tokenize("clear").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Command { name, args } = &ast.0[0] {
        assert_eq!(name, "clear");
        assert!(args.is_empty());
    } else {
        panic!("Expected command");
    }
}

#[test]
fn test_parse_underscore_variables() {
    let tokens = tokenize("_private = 10").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Assignment { name, .. } = &ast.0[0] {
        assert_eq!(name, "_private");
    } else {
        panic!("Expected assignment");
    }
}

#[test]
fn test_parse_empty_program() {
    let tokens = tokenize("").unwrap();
    let ast = parse(tokens).unwrap();
    assert!(ast.0.is_empty());
}

#[test]
fn test_parse_assignment_with_variable_reference() {
    let tokens = tokenize("x = y").unwrap();
    let ast = parse(tokens).unwrap();

    if let Statement::Assignment { name, value } = &ast.0[0] {
        assert_eq!(name, "x");
        if let Expression::Identifier(id) = value {
            assert_eq!(id, "y");
        } else {
            panic!("Expected identifier");
        }
    } else {
        panic!("Expected assignment");
    }
}
