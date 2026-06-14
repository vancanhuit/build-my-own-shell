# Task Conventions

Use `mise` for tool versions, centralized environment variables, local tasks, CI-aligned commands, release preparation, and GitHub CLI wrappers.

## `mise.lock`

Commit `mise.lock`. Update it whenever `mise.toml` tool versions change.

Recommended workflow:

```bash
mise install
mise lock
mise run rust:ci
```

Do not add `mise.lock` to `.gitignore`.

## Task-Scoped Tools

Use top-level `[tools]` only for broadly required tools:

```toml
[tools]
rust = "stable"
cocogitto = "latest"
github-cli = "latest"
```

If a tool is only needed by one task, scope it to that task:

```toml
[tasks."docs:lint"]
description = "Lint Markdown documentation"
tools.markdownlint-cli2 = "latest"
run = "markdownlint-cli2 '**/*.md'"
```

Update `mise.lock` when global or task-scoped tools change.

## Namespaced Tasks

Use `<group>:<action>` or `<group>:<sub-group>:<action>`.

Good examples:

```text
rust:fmt
rust:test
rust:ci
git:hooks:install
pr:title:validate
gh:pr:checks
release:create
skills:package
```
