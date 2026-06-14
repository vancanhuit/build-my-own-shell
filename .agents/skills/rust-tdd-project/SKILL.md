---
name: rust-tdd-project
description: rust project development guidance for test-first implementation, small incremental changes, CodeCrafters shell course alignment, shell feature work, parser/lexer/executor/builtin changes, Rust refactoring, dependency updates, and code quality. Use when modifying Rust source code, adding tests, implementing shell behavior, updating Rust dependencies, or reviewing Rust project structure.
---

# Rust TDD Project

Use this skill when working on Rust code in the shell project.

The project is a small Unix-like shell built with a test-first, incremental “Build Your Own X” approach. The CodeCrafters Shell course stages are the external acceptance-test track; they are acceptance criteria, not architecture.

## Core Rules

1. Work test-first.
2. Keep changes small and incremental.
3. Prefer simple, boring Rust over clever Rust.
4. Avoid implementing unrequested Bash compatibility.
5. Run project quality checks through `mise`.
6. Avoid unnecessary dependencies.
7. Keep the shell intentionally small.
8. Follow the CodeCrafters stage checklist in `README.md` when implementing course-aligned features.

## Rationalizations / Do Not Skip

| Rationalization | Required response |
|---|---|
| “This is a tiny code change, no test needed.” | Add or update a focused test anyway. |
| “The parser change is obvious.” | Add parser or lexer unit tests for the exact behavior. |
| “The integration test is enough.” | Add unit tests too when parser, lexer, or expansion logic changed. |
| “The unit tests are enough.” | Add an integration test when user-visible shell behavior changed. |
| “This is just a refactor.” | Run `mise run rust:ci` and keep behavior covered by existing tests. |
| “This crate is convenient.” | Prefer the standard library unless the dependency clearly improves correctness. |
| “Bash does it this way.” | Implement only the project’s documented shell subset unless Bash compatibility was requested. |
| “CI will catch it.” | Run the relevant local `mise` task first. |
| “CodeCrafters tests are enough.” | Add local tests where practical before submitting. |

## Standard Commands

```bash
mise run rust:test
mise run rust:lint
mise run rust:build
mise run rust:ci
```

Before considering work complete:

```bash
mise run rust:ci
```

## References

Load these only when relevant:

- `references/codecrafters-shell.md` for CodeCrafters stage alignment workflow.
- `references/shell-architecture.md` for parser, executor, builtins, and shell module structure.
- `references/testing.md` for unit/integration testing patterns.
- `references/dependencies.md` for Rust dependency rules.
