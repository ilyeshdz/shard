use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::process::Command;

#[test]
fn test_cli_input_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("x = 10").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("__shard_x=10"));
}

#[test]
fn test_cli_output_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("name = 'Shard'").unwrap();

    let output_file = temp.child("output.sh");

    let result = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--output",
            output_file.path().to_str().unwrap(),
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(result.status.success());
    assert!(output_file.exists());
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    assert!(content.contains("__shard_name='Shard'"));
}

#[test]
fn test_cli_full_example() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("example.shard");
    input_file
        .write_str("name = 'Shard'\necho 'Hello' name")
        .unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#!/bin/sh"));
    assert!(stdout.contains("__shard_name='Shard'"));
    assert!(stdout.contains("__shard_status=$?"));
}

#[test]
fn test_cli_missing_input() {
    let result = Command::new("cargo")
        .args(["run", "--", "--check"])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    // Should fail with error message about missing input
    assert!(!result.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("input"));
}

#[test]
fn test_cli_multiple_assignments() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("multi.shard");
    input_file.write_str("x = 1\ny = 2\nz = 3").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("__shard_x=1"));
    assert!(stdout.contains("__shard_y=2"));
    assert!(stdout.contains("__shard_z=3"));
}

#[test]
fn test_cli_command_with_args() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("cmd.shard");
    input_file.write_str("ls -la /home").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ls"));
    assert!(stdout.contains("-la"));
    assert!(stdout.contains("/home"));
    assert!(stdout.contains("__shard_status=$?"));
}

#[test]
fn test_cli_empty_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("empty.shard");
    input_file.write_str("").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#!/bin/sh"));
}

#[test]
fn test_cli_has_shebang() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("echo hello").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("#!/bin/sh"));
}

#[test]
fn test_cli_generated_comment() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("x = 10").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--check",
        ])
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Generated by Shard"));
}
