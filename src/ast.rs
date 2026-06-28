//! Abstract syntax types produced by the parser.

/// A single command: a program name, its arguments, and any redirections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirects: Vec<Redirection>,
}

/// A redirection of one of the command's standard streams to a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirection {
    /// The file descriptor being redirected (`1` for stdout, `2` for stderr).
    pub fd: u32,
    /// Whether output is appended (`>>`) rather than truncating (`>`).
    pub append: bool,
    /// The path the stream is redirected to.
    pub target: String,
}

impl Command {
    /// Construct a command from a name and its arguments, with no redirections.
    pub fn new(name: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            args,
            redirects: Vec::new(),
        }
    }
}
