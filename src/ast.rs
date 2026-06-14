//! Abstract syntax types produced by the parser.

/// A single command: a program name and its arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    /// Construct a command from a name and its arguments.
    pub fn new(name: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            args,
        }
    }
}
