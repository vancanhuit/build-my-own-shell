//! Shell builtins that run inside the shell process.

use crate::ast::Command;

/// The result of dispatching a command to a builtin.
pub enum Builtin {
    /// The builtin ran and produced this exit status.
    Handled(i32),
    /// The shell should exit with this status.
    Exit(i32),
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
}
