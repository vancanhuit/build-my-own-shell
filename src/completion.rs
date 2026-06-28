//! Tab-completion of command names.
//!
//! Completion is only meaningful for an interactive terminal session, so the
//! interactive REPL wires [`ShellCompleter`] into `rustyline`. The matching
//! logic lives in [`command_candidates`], a pure function that is unit-tested
//! without a terminal.

use std::collections::BTreeSet;
use std::path::Path;

use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::{Context, Helper, Highlighter, Hinter, Validator};

use crate::builtins;

/// Command names that start with `prefix`, for completing the first word.
///
/// The candidates are the shell builtins plus every executable found in a
/// `PATH` directory, returned sorted and de-duplicated. An empty `prefix`
/// matches everything, mirroring how a shell offers all commands on a bare
/// `Tab`.
pub fn command_candidates(prefix: &str) -> Vec<String> {
    let mut names = BTreeSet::new();

    for &name in builtins::names() {
        if name.starts_with(prefix) {
            names.insert(name.to_string());
        }
    }

    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            collect_executables(Path::new(dir), prefix, &mut names);
        }
    }

    names.into_iter().collect()
}

/// Insert the name of every executable in `dir` that starts with `prefix`.
///
/// Unreadable directories and non-UTF-8 names are skipped, matching the
/// best-effort scanning a shell performs over `PATH`.
fn collect_executables(dir: &Path, prefix: &str, names: &mut BTreeSet<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            continue;
        };
        if name.starts_with(prefix) && builtins::is_executable(&entry.path()) {
            names.insert(name.to_string());
        }
    }
}

/// A `rustyline` helper that completes command names at the start of a line.
///
/// Only the first word is completed; once the cursor sits after a space the
/// position is treated as an argument, which a later stage will handle as
/// filename completion.
#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let prefix = &line[..pos];
        if prefix.contains(char::is_whitespace) {
            return Ok((pos, Vec::new()));
        }

        let candidates = command_candidates(prefix)
            .into_iter()
            .map(|name| Pair {
                // The replacement appends a space so a single match leaves the
                // cursor ready for arguments, just like an interactive shell.
                replacement: format!("{name} "),
                display: name,
            })
            .collect();

        Ok((0, candidates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustyline::history::DefaultHistory;
    use std::os::unix::fs::PermissionsExt;

    /// Create an executable file named `name` inside `dir`.
    fn make_executable(dir: &Path, name: &str) {
        let path = dir.join(name);
        std::fs::write(&path, "#!/bin/sh\n").unwrap();
        let mut perms = std::fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).unwrap();
    }

    /// Names found by scanning `dir` for executables matching `prefix`.
    fn executables_in(dir: &Path, prefix: &str) -> Vec<String> {
        let mut names = BTreeSet::new();
        collect_executables(dir, prefix, &mut names);
        names.into_iter().collect()
    }

    #[test]
    fn matches_a_single_builtin_by_prefix() {
        assert!(command_candidates("ech").contains(&"echo".to_string()));
    }

    #[test]
    fn exact_builtin_name_still_matches_itself() {
        assert!(command_candidates("echo").contains(&"echo".to_string()));
    }

    #[test]
    fn unmatched_prefix_yields_no_candidates() {
        assert!(command_candidates("zzz_no_such_command").is_empty());
    }

    #[test]
    fn candidates_are_sorted() {
        let candidates = command_candidates("");
        let mut sorted = candidates.clone();
        sorted.sort();
        assert_eq!(candidates, sorted);
    }

    #[test]
    fn finds_executables_in_a_directory() {
        let dir = tempfile::tempdir().unwrap();
        make_executable(dir.path(), "custom_tool");

        assert_eq!(
            executables_in(dir.path(), "custom_"),
            vec!["custom_tool".to_string()]
        );
    }

    #[test]
    fn ignores_non_executable_directory_entries() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("custom_data"), "not a program").unwrap();

        assert!(executables_in(dir.path(), "custom_").is_empty());
    }

    #[test]
    fn lists_multiple_matching_executables_sorted() {
        let dir = tempfile::tempdir().unwrap();
        make_executable(dir.path(), "custom_beta");
        make_executable(dir.path(), "custom_alpha");

        assert_eq!(
            executables_in(dir.path(), "custom_"),
            vec!["custom_alpha".to_string(), "custom_beta".to_string()]
        );
    }

    #[test]
    fn deduplicates_a_name_shared_by_a_builtin_and_an_executable() {
        let dir = tempfile::tempdir().unwrap();
        // `type` is a builtin; an identically named executable must not appear
        // twice once both sources are merged into the sorted set.
        make_executable(dir.path(), "type");
        let mut names: BTreeSet<String> = builtins::names().iter().map(|n| n.to_string()).collect();
        collect_executables(dir.path(), "type", &mut names);

        let typed: Vec<_> = names.iter().filter(|n| n.as_str() == "type").collect();
        assert_eq!(typed.len(), 1);
    }

    #[test]
    fn missing_directory_contributes_no_candidates() {
        let mut names = BTreeSet::new();
        collect_executables(Path::new("/nonexistent_dir_xyz"), "", &mut names);
        assert!(names.is_empty());
    }

    #[test]
    fn completer_returns_word_start_and_trailing_space() {
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);
        let completer = ShellCompleter;

        let (start, pairs) = completer.complete("ech", 3, &ctx).unwrap();

        assert_eq!(start, 0);
        let echo = pairs.iter().find(|p| p.display == "echo").unwrap();
        assert_eq!(echo.replacement, "echo ");
    }

    #[test]
    fn completer_offers_nothing_for_arguments() {
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);
        let completer = ShellCompleter;

        let (start, pairs) = completer.complete("echo ar", 7, &ctx).unwrap();

        assert_eq!(start, 7);
        assert!(pairs.is_empty());
    }
}
