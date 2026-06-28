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

/// Create a regular, non-executable file named `name` inside `dir`.
fn make_non_executable(dir: &Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, "not a program").unwrap();
    let mut perms = std::fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o644);
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

// Stage: Locate executable files
// A PATH entry that exists but is not executable is not a command.
#[test]
fn type_ignores_a_non_executable_path_entry() {
    let dir = tempfile::tempdir().unwrap();
    make_non_executable(dir.path(), "not_runnable");

    shell()
        .env("PATH", path_with(dir.path()))
        .write_stdin("type not_runnable\n")
        .assert()
        .stdout(contains("not_runnable: not found"));
}

// Stage: Locate executable files
// Resolution must skip a non-executable match and keep scanning later dirs.
#[test]
fn type_skips_non_executable_match_for_a_later_executable() {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    make_non_executable(first.path(), "tool");
    let prog = make_executable(second.path(), "tool", "#!/bin/sh\n");

    let path = format!("{}:{}", first.path().display(), second.path().display());
    shell()
        .env("PATH", path)
        .write_stdin("type tool\n")
        .assert()
        .stdout(contains(format!("tool is {}", prog.display())));
}

// Stage: Implement `type`
// `type` accepts several names and reports each one.
#[test]
fn type_reports_each_of_multiple_names() {
    shell()
        .write_stdin("type echo nonexistent_xyz exit\n")
        .assert()
        .code(1)
        .stdout(
            contains("echo is a shell builtin")
                .and(contains("nonexistent_xyz: not found"))
                .and(contains("exit is a shell builtin")),
        );
}

// Stage: Implement `type`
// `type` with no operands succeeds and prints nothing of its own.
#[test]
fn type_without_arguments_succeeds_silently() {
    shell()
        .write_stdin("type\n")
        .assert()
        .success()
        .stdout("$ $ ");
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
