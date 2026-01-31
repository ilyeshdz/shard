use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::process::Command;

fn run_shard(args: &[&str]) -> (bool, String, String) {
    let result = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .current_dir("/Users/hdzilyes/projects/shard")
        .output()
        .expect("Failed to run cargo");

    (
        result.status.success(),
        String::from_utf8_lossy(&result.stdout).to_string(),
        String::from_utf8_lossy(&result.stderr).to_string(),
    )
}

#[test]
fn test_cli_version() {
    let (success, stdout, _) = run_shard(&["--version"]);
    assert!(success);
    assert!(stdout.contains("shard"));
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_cli_help() {
    let (success, stdout, _) = run_shard(&["--help"]);
    assert!(success);
    assert!(stdout.contains("Usage"));
    assert!(stdout.contains("check"));
    assert!(stdout.contains("build"));
    assert!(stdout.contains("transpile"));
    assert!(stdout.contains("init"));
}

#[test]
fn test_cli_check_basic() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("x = 10").unwrap();

    let (success, stdout, stderr) =
        run_shard(&["check", "-i", input_file.path().to_str().unwrap()]);
    assert!(success, "stderr: {}", stderr);
    assert!(stdout.contains("✓ Check passed"));
}

#[test]
fn test_cli_check_missing_input() {
    let (success, _, stderr) = run_shard(&["check"]);
    assert!(!success);
    assert!(stderr.contains("required"));
}

#[test]
fn test_cli_build_basic() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("name = 'Shard'").unwrap();

    let output_file = temp.child("output.sh");

    let (success, stdout, stderr) = run_shard(&[
        "build",
        "-i",
        input_file.path().to_str().unwrap(),
        "-o",
        output_file.path().to_str().unwrap(),
    ]);
    assert!(success, "stderr: {}", stderr);
    assert!(stdout.contains("✓ Built"));
    assert!(output_file.exists());
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    assert!(content.contains("__shard_name='Shard'"));
}

#[test]
fn test_cli_build_executable() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("echo hello").unwrap();

    let output_file = temp.child("output.sh");

    let (success, stdout, _) = run_shard(&[
        "build",
        "-i",
        input_file.path().to_str().unwrap(),
        "-o",
        output_file.path().to_str().unwrap(),
        "--executable",
    ]);
    assert!(success);
    assert!(stdout.contains("(executable)"));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(output_file.path()).unwrap();
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        // Check if executable bit is set (mode & 0o111 != 0)
        assert!(
            mode & 0o111 != 0,
            "Output should be executable, got mode {:o}",
            mode
        );
    }
}

#[test]
fn test_cli_transpile_basic() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("x = 10").unwrap();

    let (success, stdout, _) = run_shard(&["transpile", "-i", input_file.path().to_str().unwrap()]);
    assert!(success);
    assert!(stdout.contains("#!/bin/sh"));
    assert!(stdout.contains("__shard_x=10"));
}

#[test]
fn test_cli_init_basic() {
    let temp = TempDir::new().unwrap();
    let project_dir = temp.child("myproject");

    let (success, stdout, _) = run_shard(&["init", project_dir.path().to_str().unwrap()]);
    assert!(success);
    assert!(stdout.contains("✓ Created"));

    let main_file = project_dir.child("main.shard");
    assert!(main_file.exists());
}

#[test]
fn test_cli_check_verbose() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("name = 'Shard'").unwrap();

    let (success, stdout, stderr) =
        run_shard(&["-v", "check", "-i", input_file.path().to_str().unwrap()]);
    assert!(success);
    assert!(stderr.contains("Tokenized"));
    assert!(stderr.contains("Parsed"));
}

#[test]
fn test_cli_build_verbose() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("test.shard");
    input_file.write_str("x = 1").unwrap();

    let output_file = temp.child("output.sh");

    let (success, stdout, stderr) = run_shard(&[
        "-v",
        "build",
        "-i",
        input_file.path().to_str().unwrap(),
        "-o",
        output_file.path().to_str().unwrap(),
    ]);
    assert!(success);
    assert!(stderr.contains("Building"));
}

#[test]
fn test_cli_full_example() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("example.shard");
    input_file
        .write_str("name = 'Shard'\necho 'Hello' name")
        .unwrap();

    let (success, stdout, _) = run_shard(&["transpile", "-i", input_file.path().to_str().unwrap()]);
    assert!(success);
    assert!(stdout.contains("#!/bin/sh"));
    assert!(stdout.contains("__shard_name='Shard'"));
    assert!(stdout.contains("__shard_status=$?"));
}
