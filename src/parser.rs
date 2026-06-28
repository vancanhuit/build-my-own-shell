//! Builds an AST from a token stream.

use crate::ast::{Command, Redirection};
use crate::lexer::Token;

/// Build a [`Command`] from tokens.
///
/// The first word is the command name and the rest are arguments. A
/// [`Token::Redirect`] consumes the following word as its target; a redirect
/// with no target is ignored.
///
/// Returns `None` when there is no command name (an empty or redirect-only
/// line).
pub fn parse(tokens: Vec<Token>) -> Option<Command> {
    let mut name: Option<String> = None;
    let mut args = Vec::new();
    let mut redirects = Vec::new();
    let mut tokens = tokens.into_iter();

    while let Some(token) = tokens.next() {
        match token {
            Token::Word(word) => match name {
                None => name = Some(word),
                Some(_) => args.push(word),
            },
            Token::Redirect { fd, append } => {
                if let Some(Token::Word(target)) = tokens.next() {
                    redirects.push(Redirection { fd, append, target });
                }
            }
        }
    }

    Some(Command {
        name: name?,
        args,
        redirects,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(s: &str) -> Token {
        Token::Word(s.to_string())
    }

    #[test]
    fn empty_tokens_parse_to_none() {
        assert_eq!(parse(Vec::new()), None);
    }

    #[test]
    fn first_token_is_the_command_name() {
        let command = parse(vec![word("echo"), word("hi"), word("there")]).unwrap();
        assert_eq!(
            command,
            Command::new("echo", vec!["hi".into(), "there".into()])
        );
    }

    #[test]
    fn redirect_collects_its_target() {
        let command = parse(vec![
            word("echo"),
            word("hi"),
            Token::Redirect {
                fd: 1,
                append: false,
            },
            word("out.txt"),
        ])
        .unwrap();

        assert_eq!(command.name, "echo");
        assert_eq!(command.args, vec!["hi".to_string()]);
        assert_eq!(
            command.redirects,
            vec![Redirection {
                fd: 1,
                append: false,
                target: "out.txt".into(),
            }]
        );
    }

    #[test]
    fn collects_multiple_redirects() {
        let command = parse(vec![
            word("cmd"),
            Token::Redirect {
                fd: 1,
                append: false,
            },
            word("out"),
            Token::Redirect {
                fd: 2,
                append: true,
            },
            word("err"),
        ])
        .unwrap();

        assert_eq!(
            command.redirects,
            vec![
                Redirection {
                    fd: 1,
                    append: false,
                    target: "out".into(),
                },
                Redirection {
                    fd: 2,
                    append: true,
                    target: "err".into(),
                },
            ]
        );
    }

    #[test]
    fn redirect_without_target_is_ignored() {
        let command = parse(vec![
            word("echo"),
            Token::Redirect {
                fd: 1,
                append: false,
            },
        ])
        .unwrap();

        assert_eq!(command.name, "echo");
        assert!(command.redirects.is_empty());
    }
}
