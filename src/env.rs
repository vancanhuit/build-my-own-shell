//! Mutable shell state shared across the REPL.

/// Holds state that persists between commands, such as the last exit status.
#[derive(Debug, Default)]
pub struct Shell {
    last_status: i32,
}

impl Shell {
    /// Create a shell with default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// The exit status of the most recently executed command.
    pub fn last_status(&self) -> i32 {
        self.last_status
    }

    /// Record the exit status of the most recently executed command.
    pub fn set_last_status(&mut self, status: i32) {
        self.last_status = status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_success_status() {
        assert_eq!(Shell::new().last_status(), 0);
    }

    #[test]
    fn remembers_last_status() {
        let mut shell = Shell::new();
        shell.set_last_status(42);
        assert_eq!(shell.last_status(), 42);
    }
}
