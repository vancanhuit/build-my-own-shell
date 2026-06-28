//! Shell builtins that run inside the shell process.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::ast::Command;

/// The result of dispatching a command to a builtin.
pub enum Builtin {
    /// The builtin ran and produced this exit status.
    Handled(i32),
    /// The shell should exit with this status.
    Exit(i32),
}

/// Names of the commands implemented as shell builtins.
const BUILTINS: &[&str] = &["cd", "echo", "exit", "pwd", "type"];

/// Whether `name` refers to a shell builtin.
fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

/// Search each `PATH` directory in order for an executable named `name`.
///
/// A directory entry that exists but is not an executable file is skipped, so
/// resolution keeps scanning later directories just like a real shell.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").ok()?;
    path.split(':').find_map(|dir| {
        let candidate = Path::new(dir).join(name);
        is_executable(&candidate).then_some(candidate)
    })
}

/// Whether `path` is a regular file with any execute bit set.
fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

/// Implement the `type` builtin over its operands.
///
/// Each name is reported on its own line. The status is `1` if any name could
/// not be resolved, mirroring the shell's `type` exit code.
fn run_type(names: &[String]) -> Builtin {
    let mut status = 0;
    for name in names {
        if !report_type(name) {
            status = 1;
        }
    }
    Builtin::Handled(status)
}

/// Report how a single `name` would be resolved. Returns whether it was found.
fn report_type(name: &str) -> bool {
    if is_builtin(name) {
        println!("{name} is a shell builtin");
        return true;
    }

    match find_in_path(name) {
        Some(path) => {
            println!("{name} is {}", path.display());
            true
        }
        None => {
            println!("{name}: not found");
            false
        }
    }
}

/// Resolve the directory `cd` should switch to.
///
/// No argument and a bare `~` consult `home`; a `~/` prefix is expanded
/// against it. Every other path is taken verbatim, letting the OS resolve
/// relative components against the current directory. Returns `None` when a
/// home directory is required but unavailable.
fn cd_target(arg: Option<&str>, home: Option<PathBuf>) -> Option<PathBuf> {
    match arg {
        None | Some("~") => home,
        Some(path) => match path.strip_prefix("~/") {
            Some(rest) => home.map(|home| home.join(rest)),
            None => Some(PathBuf::from(path)),
        },
    }
}

/// Implement the `cd` builtin for an optional target directory.
fn run_cd(arg: Option<&str>) -> Builtin {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let Some(target) = cd_target(arg, home) else {
        eprintln!("cd: HOME not set");
        return Builtin::Handled(1);
    };

    match std::env::set_current_dir(&target) {
        Ok(()) => Builtin::Handled(0),
        Err(_) => {
            eprintln!("cd: {}: No such file or directory", target.display());
            Builtin::Handled(1)
        }
    }
}

/// Implement the `pwd` builtin: print the current working directory.
fn run_pwd() -> Builtin {
    match std::env::current_dir() {
        Ok(dir) => {
            println!("{}", dir.display());
            Builtin::Handled(0)
        }
        Err(err) => {
            eprintln!("pwd: {err}");
            Builtin::Handled(1)
        }
    }
}

/// Try to run `command` as a builtin.
///
/// Returns `None` if the command name is not a known builtin.
pub fn dispatch(command: &Command) -> Option<Builtin> {
    match command.name.as_str() {
        "exit" => {
            let status = command
                .args
                .first()
                .and_then(|arg| arg.parse().ok())
                .unwrap_or(0);
            Some(Builtin::Exit(status))
        }
        "echo" => {
            println!("{}", command.args.join(" "));
            Some(Builtin::Handled(0))
        }
        "type" => Some(run_type(&command.args)),
        "pwd" => Some(run_pwd()),
        "cd" => Some(run_cd(command.args.first().map(String::as_str))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_command_is_not_a_builtin() {
        assert!(dispatch(&Command::new("ls", Vec::new())).is_none());
    }

    #[test]
    fn exit_without_args_defaults_to_zero() {
        match dispatch(&Command::new("exit", Vec::new())) {
            Some(Builtin::Exit(0)) => {}
            _ => panic!("expected Exit(0)"),
        }
    }

    #[test]
    fn exit_parses_status_argument() {
        match dispatch(&Command::new("exit", vec!["7".into()])) {
            Some(Builtin::Exit(7)) => {}
            _ => panic!("expected Exit(7)"),
        }
    }

    #[test]
    fn type_reports_known_builtins_as_handled() {
        for name in ["cd", "echo", "exit", "pwd", "type"] {
            match dispatch(&Command::new("type", vec![name.to_string()])) {
                Some(Builtin::Handled(0)) => {}
                _ => panic!("expected Handled(0) for `type {name}`"),
            }
        }
    }

    #[test]
    fn type_reports_unknown_names_as_handled_failure() {
        match dispatch(&Command::new(
            "type",
            vec!["definitely_not_a_real_command_xyz".to_string()],
        )) {
            Some(Builtin::Handled(1)) => {}
            _ => panic!("expected Handled(1) for an unknown name"),
        }
    }

    #[test]
    fn type_without_arguments_is_handled_success() {
        match dispatch(&Command::new("type", Vec::new())) {
            Some(Builtin::Handled(0)) => {}
            _ => panic!("expected Handled(0) for `type` with no operands"),
        }
    }

    #[test]
    fn type_fails_when_any_name_is_unknown() {
        match dispatch(&Command::new(
            "type",
            vec!["echo".into(), "definitely_not_a_real_command_xyz".into()],
        )) {
            Some(Builtin::Handled(1)) => {}
            _ => panic!("expected Handled(1) when one of several names is unknown"),
        }
    }

    #[test]
    fn cd_target_without_argument_uses_home() {
        let home = PathBuf::from("/home/alice");
        assert_eq!(cd_target(None, Some(home.clone())), Some(home));
    }

    #[test]
    fn cd_target_bare_tilde_uses_home() {
        let home = PathBuf::from("/home/alice");
        assert_eq!(cd_target(Some("~"), Some(home.clone())), Some(home));
    }

    #[test]
    fn cd_target_expands_tilde_prefix_against_home() {
        let home = PathBuf::from("/home/alice");
        assert_eq!(
            cd_target(Some("~/projects/shell"), Some(home)),
            Some(PathBuf::from("/home/alice/projects/shell"))
        );
    }

    #[test]
    fn cd_target_keeps_absolute_and_relative_paths_verbatim() {
        let home = Some(PathBuf::from("/home/alice"));
        assert_eq!(
            cd_target(Some("/usr/local"), home.clone()),
            Some(PathBuf::from("/usr/local"))
        );
        assert_eq!(
            cd_target(Some("../sibling"), home),
            Some(PathBuf::from("../sibling"))
        );
    }

    #[test]
    fn cd_target_needs_home_for_tilde() {
        assert_eq!(cd_target(Some("~"), None), None);
        assert_eq!(cd_target(Some("~/docs"), None), None);
        assert_eq!(cd_target(None, None), None);
    }
}
