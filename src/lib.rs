//! A small Unix-like shell built test-first.
//!
//! Pipeline: input string -> lexer -> tokens -> parser -> AST -> executor.

pub mod ast;
pub mod builtins;
pub mod completion;
pub mod env;
pub mod error;
pub mod executor;
pub mod lexer;
pub mod parser;

use std::io::{self, IsTerminal, Write};

use crate::completion::ShellCompleter;
use crate::env::Shell;
use crate::executor::{execute, Outcome};

/// Run the read-eval-print loop until EOF or an `exit` builtin.
///
/// An interactive terminal gets line editing and tab completion via
/// `rustyline`; a piped or redirected stdin uses a plain prompt-and-read loop.
/// On a clean shutdown the process exits with the status of the last command.
pub fn run() -> error::Result<()> {
    if io::stdin().is_terminal() {
        run_interactive()
    } else {
        run_batch()
    }
}

/// Evaluate a single input line, updating `shell` state.
///
/// Returns `true` when the shell should exit (an `exit` builtin ran).
fn evaluate(shell: &mut Shell, line: &str) -> bool {
    let tokens = lexer::tokenize(line);
    let Some(command) = parser::parse(tokens) else {
        return false;
    };

    match execute(&command) {
        Outcome::Status(status) => {
            shell.set_last_status(status);
            false
        }
        Outcome::Exit(status) => {
            shell.set_last_status(status);
            true
        }
    }
}

/// The non-interactive loop: print a prompt and read whole lines from stdin.
fn run_batch() -> error::Result<()> {
    let mut shell = Shell::new();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        if evaluate(&mut shell, &line) {
            break;
        }
    }

    std::process::exit(shell.last_status());
}

/// The interactive loop: read lines through `rustyline` with tab completion.
fn run_interactive() -> error::Result<()> {
    let mut shell = Shell::new();
    let config = rustyline::Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();
    let mut editor = rustyline::Editor::with_config(config).map_err(io::Error::other)?;
    editor.set_helper(Some(ShellCompleter));

    loop {
        match editor.readline("$ ") {
            Ok(line) => {
                if evaluate(&mut shell, &line) {
                    break;
                }
            }
            // Ctrl-C abandons the current line; Ctrl-D on an empty line exits.
            Err(rustyline::error::ReadlineError::Interrupted) => continue,
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(error) => return Err(io::Error::other(error).into()),
        }
    }

    std::process::exit(shell.last_status());
}
