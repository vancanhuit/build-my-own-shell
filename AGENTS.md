# AGENTS.md

## Project Summary

This repository implements a small Unix-like shell in Rust using a test-first, incremental “Build Your Own X” approach.

The project follows the CodeCrafters Shell course stages as the external acceptance-test track. The detailed engineering rules are kept in project-local agent skills under `.agents/skills/`.

## Core Operating Rules

Agents must:

1. Work test-first.
2. Keep changes small and incremental.
3. Prefer simple, boring Rust over clever Rust.
4. Use `mise` for local tasks and CI-aligned commands.
5. Commit `mise.lock` and update it when tools change.
6. Scope one-off tools to specific `mise` tasks.
7. Use Cocogitto for Conventional Commit validation.
8. Use GitHub CLI for PR, CI, and release workflows.
9. Keep GitHub Actions thin and call `mise run` tasks.
10. Avoid custom GitHub Actions unless necessary.
11. Prefer squash merge and linear history.
12. Avoid implementing unrequested Bash compatibility.

## Relevant Agent Skills

Use these project-local skills by scope:

- `.agents/skills/rust-tdd-project/` for Rust implementation, tests, parser/executor changes, CodeCrafters stage alignment, and dependency updates.
- `.agents/skills/mise-tasking/` for `mise.toml`, `mise.lock`, native file tasks, task libraries, Bash/Python automation, and release scripts.
- `.agents/skills/github-governance/` for GitHub Actions, GitHub CLI, CI/CD, release workflows, and branch protection.
- `.agents/skills/conventional-commits/` for Cocogitto, Conventional Commits, PR title validation, and Git hooks.

## Standard Commands

Use these commands unless a task requires something more specific:

```bash
mise run rust:test
mise run rust:lint
mise run rust:build
mise run rust:ci
```

For GitHub workflows:

```bash
mise run gh:pr:status
mise run gh:pr:checks
mise run gh:ci:list
mise run gh:ci:watch
```

For release workflows:

```bash
mise run release:dry-run -- --tag v0.1.0
mise run release:create -- --tag v0.1.0
```

## Definition of Done

Before considering work complete, agents must verify:

```bash
mise run rust:ci
```

A completed change should include:

- tests added or updated
- no unrelated formatting churn
- no unnecessary dependencies
- no machine-specific paths
- no debug output in normal shell usage
- updated `Cargo.lock` when dependencies change
- updated `mise.toml`, `mise.lock`, or `mise/tasks/` when automation changes
- updated workflows when CI/CD behavior changes
- Conventional Commit compliant PR title
