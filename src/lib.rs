//! A small Unix-like shell built test-first.
//!
//! Pipeline: input string -> lexer -> tokens -> parser -> AST -> executor.

pub mod ast;
pub mod builtins;
pub mod env;
pub mod error;
pub mod executor;
pub mod lexer;
pub mod parser;

use std::io::{self, Write};

use crate::env::Shell;
use crate::executor::{execute, Outcome};

/// Run the read-eval-print loop until EOF or an `exit` builtin.
///
/// On a clean shutdown the process exits with the status of the last command.
pub fn run() -> error::Result<()> {
    let mut shell = Shell::new();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        line.clear();
        let bytes = stdin.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        let tokens = lexer::tokenize(&line);
        let Some(command) = parser::parse(tokens) else {
            continue;
        };

        match execute(&command) {
            Outcome::Status(status) => shell.set_last_status(status),
            Outcome::Exit(status) => {
                shell.set_last_status(status);
                break;
            }
        }
    }

    std::process::exit(shell.last_status());
}
