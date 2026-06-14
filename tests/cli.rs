use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn echo_prints_its_arguments() {
    Command::cargo_bin("shell")
        .unwrap()
        .write_stdin("echo hello world\n")
        .assert()
        .success()
        .stdout(contains("hello world"));
}

#[test]
fn exit_sets_the_process_status() {
    Command::cargo_bin("shell")
        .unwrap()
        .write_stdin("exit 3\n")
        .assert()
        .code(3);
}

#[test]
fn unknown_command_reports_not_found() {
    Command::cargo_bin("shell")
        .unwrap()
        .write_stdin("definitely-not-a-real-command\n")
        .assert()
        .stderr(contains("command not found"));
}
