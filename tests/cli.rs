//! Integration tests for the CodeCrafters "Core Shell" section.
//!
//! Each test is labelled with the stage from the README checklist it covers.
//! Tests drive the implementation: write code until they pass.

use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// A fresh invocation of the `shell` binary.
fn shell() -> Command {
    Command::cargo_bin("shell").unwrap()
}

/// Build a `PATH` value that searches `dir` first, then the inherited `PATH`.
fn path_with(dir: &Path) -> String {
    match std::env::var("PATH") {
        Ok(existing) => format!("{}:{}", dir.display(), existing),
        Err(_) => dir.display().to_string(),
    }
}

/// Create an executable script named `name` inside `dir` and return its path.
fn make_executable(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, body).unwrap();
    let mut perms = std::fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms).unwrap();
    path
}

// Stage: Print a prompt
#[test]
fn prints_a_prompt() {
    shell().write_stdin("").assert().success().stdout("$ ");
}

// Stage: Implement a REPL
#[test]
fn reads_and_evaluates_multiple_commands() {
    shell()
        .write_stdin("echo first\necho second\n")
        .assert()
        .success()
        .stdout(contains("first").and(contains("second")));
}

// Stage: Handle invalid commands
#[test]
fn reports_invalid_commands_and_exits_127() {
    shell()
        .write_stdin("nonexistent_command_xyz\n")
        .assert()
        .code(127)
        .stderr(contains("nonexistent_command_xyz: command not found"));
}

// Stage: Implement `exit`
#[test]
fn exit_sets_the_process_status() {
    shell().write_stdin("exit 3\n").assert().code(3);
}

// Stage: Implement `exit`
#[test]
fn exit_stops_evaluating_further_commands() {
    shell()
        .write_stdin("exit 0\necho should-not-run\n")
        .assert()
        .code(0)
        .stdout(contains("should-not-run").not());
}

// Stage: Implement `echo`
#[test]
fn echo_prints_its_arguments() {
    shell()
        .write_stdin("echo hello world\n")
        .assert()
        .success()
        .stdout(contains("hello world"));
}

// Stage: Implement `echo`
#[test]
fn echo_without_arguments_prints_an_empty_line() {
    shell()
        .write_stdin("echo\n")
        .assert()
        .success()
        .stdout("$ \n$ ");
}

// Stage: Implement `type`
#[test]
fn type_recognises_the_echo_builtin() {
    shell()
        .write_stdin("type echo\n")
        .assert()
        .stdout(contains("echo is a shell builtin"));
}

// Stage: Implement `type`
#[test]
fn type_recognises_the_exit_builtin() {
    shell()
        .write_stdin("type exit\n")
        .assert()
        .stdout(contains("exit is a shell builtin"));
}

// Stage: Implement `type`
#[test]
fn type_recognises_itself_as_a_builtin() {
    shell()
        .write_stdin("type type\n")
        .assert()
        .stdout(contains("type is a shell builtin"));
}

// Stage: Implement `type`
#[test]
fn type_reports_unknown_commands_as_not_found() {
    shell()
        .write_stdin("type nonexistent_xyz\n")
        .assert()
        .stdout(contains("nonexistent_xyz: not found"));
}

// Stage: Locate executable files
#[test]
fn type_reports_an_executable_with_its_path() {
    let dir = tempfile::tempdir().unwrap();
    let prog = make_executable(dir.path(), "my_prog", "#!/bin/sh\n");

    shell()
        .env("PATH", path_with(dir.path()))
        .write_stdin("type my_prog\n")
        .assert()
        .stdout(contains(format!("my_prog is {}", prog.display())));
}

// Stage: Run a program
#[test]
fn runs_an_external_program_with_arguments() {
    let dir = tempfile::tempdir().unwrap();
    make_executable(dir.path(), "greet", "#!/bin/sh\necho \"hello $1\"\n");

    shell()
        .env("PATH", path_with(dir.path()))
        .write_stdin("greet world\n")
        .assert()
        .success()
        .stdout(contains("hello world"));
}
