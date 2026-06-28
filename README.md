# Build Your Own Shell in Rust

This repository implements a small Unix-like shell in Rust using a test-first, incremental “Build Your Own X” approach.

The project follows the CodeCrafters **Build your own Shell** course as the external milestone and acceptance-test track.

Course link:

<https://app.codecrafters.io/courses/shell/overview>

## Project Goals

The goal is not to clone Bash.

The goal is to build a small, well-tested shell that teaches:

- command parsing
- process execution
- built-in shell commands
- environment handling
- redirection
- pipelines
- exit codes
- background jobs
- signal handling
- Rust project structure and idioms

## Development Principles

This repository uses:

- Rust
- test-first development
- `mise` for tool and task management
- `mise.lock` for reproducible tool resolution
- Cocogitto for Conventional Commit validation
- GitHub CLI for pull request, CI, and release workflows
- GitHub Actions for CI/CD
- project-local agent skills under `.agents/skills/`

Before considering work complete, run:

```bash
mise run rust:ci
```

## CodeCrafters Course Alignment

Use CodeCrafters stages as external acceptance criteria, not as architecture.

For each stage:

1. Read the current CodeCrafters stage requirement.
2. Add or update local tests where practical.
3. Implement the smallest passing behavior.
4. Run local tests.
5. Run the full project check.
6. Submit to CodeCrafters.
7. Refactor only after tests are green.

```bash
mise run rust:test
mise run rust:ci
```

Do not overfit implementation to remote tests if it harms clear parser, executor, or shell-state design.

## CodeCrafters Stage Checklist

The following checklist is the implementation track for this repository.

### Core Shell

- [x] Print a prompt
- [x] Handle invalid commands
- [x] Implement a REPL
- [x] Implement `exit`
- [x] Implement `echo`
- [x] Implement `type`
- [x] Locate executable files
- [x] Run a program

### Navigation

- [x] The `pwd` builtin
- [x] The `cd` builtin: absolute paths
- [x] The `cd` builtin: relative paths
- [x] The `cd` builtin: home directory

### Quoting

- [x] Single quotes
- [x] Double quotes
- [x] Backslash outside quotes
- [x] Backslash within single quotes
- [x] Backslash within double quotes
- [x] Executing a quoted executable

### Redirection

- [x] Redirect stdout
- [x] Redirect stderr
- [x] Append stdout
- [x] Append stderr

### Command Completion

- [ ] Builtin completion
- [ ] Completion with arguments
- [ ] Missing completions
- [ ] Executable completion
- [ ] Multiple completions
- [ ] Partial completions

### Filename Completion

- [ ] File completion
- [ ] Nested file completion
- [ ] Directory completion
- [ ] Missing completions
- [ ] Multiple matches
- [ ] Partial completions
- [ ] Multi-argument completions

### Programmable Completion

- [ ] Register `complete` builtin
- [ ] Printing missing specifications
- [ ] Displaying registered specifications
- [ ] Single completion
- [ ] Handling no completions
- [ ] Passing command-line arguments
- [ ] Passing environment variables
- [ ] Multiple completer candidates
- [ ] Longest common prefix
- [ ] Unregister a completion

### Background Jobs

- [ ] The `jobs` builtin
- [ ] Starting background jobs
- [ ] Printing background job output
- [ ] List a single job
- [ ] List multiple jobs
- [ ] Reap one job
- [ ] Reap multiple jobs
- [ ] Reap before the next prompt
- [ ] Recycle job numbers

### Pipelines

- [ ] Dual-command pipeline
- [ ] Pipelines with built-ins
- [ ] Multi-command pipelines

### History

- [ ] The `history` builtin
- [ ] Listing history
- [ ] Limiting history entries
- [ ] Up-arrow navigation
- [ ] Down-arrow navigation
- [ ] Executing commands from history

### History Persistence

- [ ] Read history from file
- [ ] Write history to file
- [ ] Append history to file
- [ ] Read history on startup
- [ ] Write history on exit
- [ ] Append history on exit

### Parameter Expansion

- [ ] The `declare` builtin
- [ ] Printing missing variables
- [ ] Storing shell variables
- [ ] Validating variable names
- [ ] Expanding variables
- [ ] Expansion with braces
- [ ] Expanding empty variables

## Suggested Internal Architecture

The CodeCrafters stage order is the external track. Internally, keep implementation clean by organizing around this flow:

```text
input string
  -> lexer
  -> tokens
  -> parser
  -> AST
  -> expander
  -> executor
```

Suggested Rust modules:

```text
src/
  main.rs
  lib.rs
  ast.rs
  lexer.rs
  parser.rs
  executor.rs
  builtins.rs
  env.rs
  error.rs
tests/
  cli.rs
```

## Local Development

Install tools:

```bash
mise install
mise lock
```

Run the shell:

```bash
mise run dev:shell
```

Run tests:

```bash
mise run rust:test
```

Run linting:

```bash
mise run rust:lint
```

Run the full local CI suite:

```bash
mise run rust:ci
```

## Git Hooks

Install local Git hooks:

```bash
mise run git:hooks:install
```

This project uses Cocogitto for Conventional Commit validation.

## Pull Requests

Pull request titles must follow Conventional Commits because squash merge uses the PR title as the final commit message.

Examples:

```text
feat(parser): support quoted strings
fix(executor): return 127 for missing commands
test(redirection): cover append stderr
ci(github): add release workflow
docs(readme): document codecrafters alignment
```

## GitHub CLI

Common GitHub workflows:

```bash
mise run gh:auth:status
mise run gh:pr:create
mise run gh:pr:status
mise run gh:pr:checks
mise run gh:ci:list
mise run gh:ci:watch
```

## Releases

Dry-run a release:

```bash
mise run release:dry-run -- --tag v0.1.0
```

Create a release:

```bash
mise run release:create -- --tag v0.1.0
```

## Agent Skills

Project-local reusable agent skills live under:

```text
.agents/skills/
```

Use them by scope:

- `.agents/skills/rust-tdd-project/` for Rust implementation, tests, parser/executor changes, CodeCrafters stage alignment, and dependency updates.
- `.agents/skills/mise-tasking/` for `mise.toml`, `mise.lock`, native file tasks, task libraries, Bash/Python automation, and release scripts.
- `.agents/skills/github-governance/` for GitHub Actions, GitHub CLI, CI/CD, release workflows, and branch protection.
- `.agents/skills/conventional-commits/` for Cocogitto, Conventional Commits, PR title validation, and Git hooks.
