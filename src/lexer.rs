//! Turns an input line into tokens.

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
/// The supported rules mirror the CodeCrafters "Quoting" stages:
///
/// - Single quotes preserve every character literally.
/// - Double quotes preserve characters except that `\` escapes one of
///   `\`, `"`, `$`, or a backtick (other backslashes stay literal).
/// - Outside quotes, `\` escapes the next character.
/// - Adjacent quoted and unquoted runs concatenate into a single token.
///
/// Variable expansion is intentionally left for a later stage.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut has_token = false;
    let mut state = State::Normal;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match state {
            State::Normal => match c {
                '\'' => {
                    has_token = true;
                    state = State::Single;
                }
                '"' => {
                    has_token = true;
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
                c if c.is_whitespace() => {
                    if has_token {
                        tokens.push(std::mem::take(&mut current));
                        has_token = false;
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
        tokens.push(current);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_whitespace() {
        assert_eq!(tokenize("echo hello world"), ["echo", "hello", "world"]);
    }

    #[test]
    fn collapses_repeated_whitespace() {
        assert_eq!(tokenize("  echo   hi  "), ["echo", "hi"]);
    }

    #[test]
    fn empty_input_yields_no_tokens() {
        assert!(tokenize("   ").is_empty());
    }

    #[test]
    fn single_quotes_preserve_inner_whitespace() {
        assert_eq!(tokenize("echo 'hello   world'"), ["echo", "hello   world"]);
    }

    #[test]
    fn single_quotes_keep_backslashes_literal() {
        assert_eq!(tokenize("echo 'a\\nb'"), ["echo", "a\\nb"]);
    }

    #[test]
    fn empty_single_quotes_produce_an_empty_token() {
        assert_eq!(tokenize("echo ''"), ["echo", ""]);
    }

    #[test]
    fn double_quotes_preserve_inner_whitespace() {
        assert_eq!(
            tokenize("echo \"hello   world\""),
            ["echo", "hello   world"]
        );
    }

    #[test]
    fn double_quotes_escape_special_characters() {
        assert_eq!(tokenize("echo \"a\\\"b\""), ["echo", "a\"b"]);
    }

    #[test]
    fn double_quotes_keep_other_backslashes_literal() {
        assert_eq!(tokenize("echo \"a\\nb\""), ["echo", "a\\nb"]);
    }

    #[test]
    fn backslash_escapes_the_next_character() {
        assert_eq!(tokenize("echo a\\ b"), ["echo", "a b"]);
        assert_eq!(tokenize("echo \\'"), ["echo", "'"]);
    }

    #[test]
    fn adjacent_quoted_runs_concatenate() {
        assert_eq!(tokenize("echo \"foo\"'bar'baz"), ["echo", "foobarbaz"]);
    }
}
