//! Turns an input line into tokens.

/// Split an input line into whitespace-separated tokens.
///
/// Quoting and escaping are intentionally not handled yet; they are added in
/// later CodeCrafters stages.
pub fn tokenize(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::to_string).collect()
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
}
