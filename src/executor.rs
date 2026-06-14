//! Executes parsed commands.

use std::process::Command as ProcessCommand;

use crate::ast::Command;
use crate::builtins::{self, Builtin};

/// The outcome of executing a single command.
pub enum Outcome {
    /// Continue the REPL; carries the command's exit status.
    Status(i32),
    /// Exit the shell with the given status.
    Exit(i32),
}

/// Execute a parsed command.
///
/// Builtins run in-process; everything else is spawned via
/// [`std::process::Command`]. A missing program yields exit status `127`.
pub fn execute(command: &Command) -> Outcome {
    if let Some(builtin) = builtins::dispatch(command) {
        return match builtin {
            Builtin::Handled(status) => Outcome::Status(status),
            Builtin::Exit(status) => Outcome::Exit(status),
        };
    }

    match ProcessCommand::new(&command.name)
        .args(&command.args)
        .status()
    {
        Ok(status) => Outcome::Status(status.code().unwrap_or(1)),
        Err(_) => {
            eprintln!("{}: command not found", command.name);
            Outcome::Status(127)
        }
    }
}
