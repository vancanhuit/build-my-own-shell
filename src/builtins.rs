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
const BUILTINS: &[&str] = &["echo", "exit", "type"];

/// Whether `name` refers to a shell builtin.
fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

/// Search each `PATH` directory in order for a file named `name`.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").ok()?;
    path.split(':').find_map(|dir| {
        let candidate = Path::new(dir).join(name);
        candidate.exists().then_some(candidate)
    })
}

/// Whether `path` has any execute bit set.
fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

/// Implement the `type` builtin for a single `name`.
fn run_type(name: &str) -> Builtin {
    if is_builtin(name) {
        println!("{name} is a shell builtin");
        return Builtin::Handled(0);
    }

    match find_in_path(name) {
        Some(path) if is_executable(&path) => {
            println!("{name} is {}", path.display());
            Builtin::Handled(0)
        }
        Some(_) => {
            println!("{name} is not executable");
            Builtin::Handled(1)
        }
        None => {
            println!("{name}: not found");
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
        "type" => {
            let name = command.args.first().map(|s| s.as_str()).unwrap_or("");
            Some(run_type(name))
        }
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
        for name in ["echo", "exit", "type"] {
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
}
