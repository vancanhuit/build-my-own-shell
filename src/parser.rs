//! Builds an AST from a token stream.

use crate::ast::Command;

/// Build a [`Command`] from tokens.
///
/// Returns `None` when there are no tokens (an empty line).
pub fn parse(tokens: Vec<String>) -> Option<Command> {
    let mut iter = tokens.into_iter();
    let name = iter.next()?;
    let args = iter.collect();
    Some(Command::new(name, args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tokens_parse_to_none() {
        assert_eq!(parse(Vec::new()), None);
    }

    #[test]
    fn first_token_is_the_command_name() {
        let command = parse(vec!["echo".into(), "hi".into(), "there".into()]).unwrap();
        assert_eq!(
            command,
            Command::new("echo", vec!["hi".into(), "there".into()])
        );
    }
}
