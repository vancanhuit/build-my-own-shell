//! Turns an input line into tokens.

/// A single lexical token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A word: a command name, argument, or redirection target.
    Word(String),
    /// A redirection operator, e.g. `>`, `1>`, `2>`, `>>`, or `2>>`.
    Redirect {
        /// The file descriptor to redirect (defaults to `1` when unspecified).
        fd: u32,
        /// Whether the operator appends (`>>`) rather than truncates (`>`).
        append: bool,
    },
}

/// Lexer state, tracking which quoting context we are inside.
enum State {
    /// Outside any quotes.
    Normal,
    /// Inside a single-quoted string, where every character is literal.
    Single,
    /// Inside a double-quoted string, where `\` escapes a few characters.
    Double,
}

/// Split an input line into tokens, honoring shell quoting and escaping.
///
/// The supported rules mirror the CodeCrafters "Quoting" and "Redirection"
/// stages:
///
/// - Single quotes preserve every character literally.
/// - Double quotes preserve characters except that `\` escapes one of
///   `\`, `"`, `$`, or a backtick (other backslashes stay literal).
/// - Outside quotes, `\` escapes the next character.
/// - Adjacent quoted and unquoted runs concatenate into a single token.
/// - An unquoted `>` or `>>`, optionally prefixed by a bare file descriptor
///   digit, becomes a redirection operator.
///
/// Variable expansion is intentionally left for a later stage.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut has_token = false;
    let mut quoted = false;
    let mut state = State::Normal;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match state {
            State::Normal => match c {
                '\'' => {
                    has_token = true;
                    quoted = true;
                    state = State::Single;
                }
                '"' => {
                    has_token = true;
                    quoted = true;
                    state = State::Double;
                }
                '\\' => match chars.next() {
                    // A backslash-newline is a line continuation: drop both.
                    Some('\n') | None => {}
                    Some(escaped) => {
                        current.push(escaped);
                        has_token = true;
                    }
                },
                '>' => {
                    // A bare, unquoted digit run immediately before `>` names
                    // the file descriptor; otherwise the word is flushed and
                    // the descriptor defaults to stdout.
                    let fd = if has_token
                        && !quoted
                        && !current.is_empty()
                        && current.chars().all(|c| c.is_ascii_digit())
                    {
                        let fd = current.parse().unwrap_or(1);
                        current.clear();
                        has_token = false;
                        quoted = false;
                        fd
                    } else {
                        if has_token {
                            tokens.push(Token::Word(std::mem::take(&mut current)));
                            has_token = false;
                            quoted = false;
                        }
                        1
                    };

                    let append = chars.peek() == Some(&'>');
                    if append {
                        chars.next();
                    }
                    tokens.push(Token::Redirect { fd, append });
                }
                c if c.is_whitespace() => {
                    if has_token {
                        tokens.push(Token::Word(std::mem::take(&mut current)));
                        has_token = false;
                        quoted = false;
                    }
                }
                c => {
                    current.push(c);
                    has_token = true;
                }
            },
            State::Single => match c {
                '\'' => state = State::Normal,
                c => current.push(c),
            },
            State::Double => match c {
                '"' => state = State::Normal,
                '\\' => match chars.next() {
                    Some(escaped @ ('\\' | '"' | '$' | '`')) => current.push(escaped),
                    // A backslash-newline is a line continuation: drop both.
                    Some('\n') => {}
                    // Before any other character the backslash stays literal.
                    Some(other) => {
                        current.push('\\');
                        current.push(other);
                    }
                    None => current.push('\\'),
                },
                c => current.push(c),
            },
        }
    }

    if has_token {
        tokens.push(Token::Word(current));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Convenience: a `Word` token from a string literal.
    fn word(s: &str) -> Token {
        Token::Word(s.to_string())
    }

    #[test]
    fn splits_on_whitespace() {
        assert_eq!(
            tokenize("echo hello world"),
            [word("echo"), word("hello"), word("world")]
        );
    }

    #[test]
    fn collapses_repeated_whitespace() {
        assert_eq!(tokenize("  echo   hi  "), [word("echo"), word("hi")]);
    }

    #[test]
    fn empty_input_yields_no_tokens() {
        assert!(tokenize("   ").is_empty());
    }

    #[test]
    fn single_quotes_preserve_inner_whitespace() {
        assert_eq!(
            tokenize("echo 'hello   world'"),
            [word("echo"), word("hello   world")]
        );
    }

    #[test]
    fn single_quotes_keep_backslashes_literal() {
        assert_eq!(tokenize("echo 'a\\nb'"), [word("echo"), word("a\\nb")]);
    }

    #[test]
    fn empty_single_quotes_produce_an_empty_token() {
        assert_eq!(tokenize("echo ''"), [word("echo"), word("")]);
    }

    #[test]
    fn double_quotes_preserve_inner_whitespace() {
        assert_eq!(
            tokenize("echo \"hello   world\""),
            [word("echo"), word("hello   world")]
        );
    }

    #[test]
    fn double_quotes_escape_special_characters() {
        assert_eq!(tokenize("echo \"a\\\"b\""), [word("echo"), word("a\"b")]);
    }

    #[test]
    fn double_quotes_keep_other_backslashes_literal() {
        assert_eq!(tokenize("echo \"a\\nb\""), [word("echo"), word("a\\nb")]);
    }

    #[test]
    fn backslash_escapes_the_next_character() {
        assert_eq!(tokenize("echo a\\ b"), [word("echo"), word("a b")]);
        assert_eq!(tokenize("echo \\'"), [word("echo"), word("'")]);
    }

    #[test]
    fn adjacent_quoted_runs_concatenate() {
        assert_eq!(
            tokenize("echo \"foo\"'bar'baz"),
            [word("echo"), word("foobarbaz")]
        );
    }

    #[test]
    fn bare_redirect_defaults_to_stdout() {
        assert_eq!(
            tokenize("echo hi > out"),
            [
                word("echo"),
                word("hi"),
                Token::Redirect {
                    fd: 1,
                    append: false
                },
                word("out"),
            ]
        );
    }

    #[test]
    fn numbered_redirects_capture_the_descriptor() {
        assert_eq!(
            tokenize("cmd 1> out 2> err"),
            [
                word("cmd"),
                Token::Redirect {
                    fd: 1,
                    append: false
                },
                word("out"),
                Token::Redirect {
                    fd: 2,
                    append: false
                },
                word("err"),
            ]
        );
    }

    #[test]
    fn double_arrow_appends() {
        assert_eq!(
            tokenize("echo hi >> out"),
            [
                word("echo"),
                word("hi"),
                Token::Redirect {
                    fd: 1,
                    append: true
                },
                word("out"),
            ]
        );
        assert_eq!(
            tokenize("cmd 2>> err"),
            [
                word("cmd"),
                Token::Redirect {
                    fd: 2,
                    append: true
                },
                word("err"),
            ]
        );
    }

    #[test]
    fn redirect_without_surrounding_spaces_splits() {
        assert_eq!(
            tokenize("echo hi>out"),
            [
                word("echo"),
                word("hi"),
                Token::Redirect {
                    fd: 1,
                    append: false
                },
                word("out"),
            ]
        );
    }

    #[test]
    fn quoted_redirect_operator_is_literal() {
        assert_eq!(
            tokenize("echo '>' \">\""),
            [word("echo"), word(">"), word(">")]
        );
    }

    #[test]
    fn quoted_digit_before_redirect_is_not_a_descriptor() {
        assert_eq!(
            tokenize("echo '2'> out"),
            [
                word("echo"),
                word("2"),
                Token::Redirect {
                    fd: 1,
                    append: false
                },
                word("out"),
            ]
        );
    }
}
