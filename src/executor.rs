//! Executes parsed commands.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::Command as ProcessCommand;

use crate::ast::{Command, Redirection};
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
/// Redirections are applied to both builtins and external programs. Builtins
/// run in-process; everything else is spawned via [`std::process::Command`].
/// A missing program yields exit status `127`, and a redirection target that
/// cannot be opened yields exit status `1` without running the command.
pub fn execute(command: &Command) -> Outcome {
    let mut stdout = match open_redirect(last_redirect(command, 1)) {
        Ok(file) => file,
        Err(error) => return redirect_error(command, 1, &error),
    };
    let mut stderr = match open_redirect(last_redirect(command, 2)) {
        Ok(file) => file,
        Err(error) => return redirect_error(command, 2, &error),
    };

    if builtins::is_builtin(&command.name) {
        let mut inherited_out = io::stdout();
        let mut inherited_err = io::stderr();
        let out: &mut dyn Write = match stdout.as_mut() {
            Some(file) => file,
            None => &mut inherited_out,
        };
        let err: &mut dyn Write = match stderr.as_mut() {
            Some(file) => file,
            None => &mut inherited_err,
        };

        let builtin = builtins::dispatch(command, out, err)
            .expect("a known builtin name should always be handled");
        return match builtin {
            Builtin::Handled(status) => Outcome::Status(status),
            Builtin::Exit(status) => Outcome::Exit(status),
        };
    }

    let mut process = ProcessCommand::new(&command.name);
    process.args(&command.args);
    if let Some(file) = stdout {
        process.stdout(file);
    }
    if let Some(file) = stderr {
        process.stderr(file);
    }

    match process.status() {
        Ok(status) => Outcome::Status(status.code().unwrap_or(1)),
        Err(_) => {
            eprintln!("{}: command not found", command.name);
            Outcome::Status(127)
        }
    }
}

/// The last redirection targeting file descriptor `fd`, if any.
///
/// The last one wins, matching the shell's left-to-right application order.
fn last_redirect(command: &Command, fd: u32) -> Option<&Redirection> {
    command.redirects.iter().rfind(|r| r.fd == fd)
}

/// Open a redirection's target file, truncating or appending as requested.
fn open_redirect(redirect: Option<&Redirection>) -> io::Result<Option<File>> {
    let Some(redirect) = redirect else {
        return Ok(None);
    };

    OpenOptions::new()
        .write(true)
        .create(true)
        .append(redirect.append)
        .truncate(!redirect.append)
        .open(&redirect.target)
        .map(Some)
}

/// Report a redirection target that could not be opened.
fn redirect_error(command: &Command, fd: u32, error: &io::Error) -> Outcome {
    if let Some(redirect) = last_redirect(command, fd) {
        eprintln!("{}: {error}", redirect.target);
    }
    Outcome::Status(1)
}
