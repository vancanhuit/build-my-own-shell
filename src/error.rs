//! Shell error types.

use thiserror::Error;

/// Errors that can abort the shell's main loop.
#[derive(Debug, Error)]
pub enum ShellError {
    /// An I/O error, typically while reading input or writing the prompt.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// The input could not be parsed into a command.
    #[error("parse error: {0}")]
    Parse(String),
}

/// Convenience alias for results returned by shell operations.
pub type Result<T> = std::result::Result<T, ShellError>;
