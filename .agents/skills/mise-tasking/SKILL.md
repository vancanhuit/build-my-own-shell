---
name: mise-tasking
description: mise-based project automation guidance for tools, environment variables, namespaced tasks, mise.lock, task-scoped tools, native file tasks under mise/tasks, bash/python task scripts, logging, reusable task libraries, root detection, and local release workflows. Use when adding or changing mise.toml, mise.lock, mise tasks, task scripts, task libraries, or local automation.
---

# Mise Tasking

Use this skill when changing `mise.toml`, `mise.lock`, native `mise` file tasks, task libraries, or local automation.

## Core Rules

1. Use `mise` as the project control plane.
2. Use namespaced task names.
3. Keep `mise.toml` readable.
4. Use inline tasks only for short commands.
5. Use native file tasks under `mise/tasks/` for long or complex logic.
6. Use Bash for simple task scripts.
7. Use Python for complex task scripts.
8. Add traceable logging to task scripts.
9. Use `MISE_PROJECT_ROOT` for repository root detection.
10. Commit and update `mise.lock`.
11. Scope one-off tools to the task that needs them.

## Rationalizations / Do Not Skip

| Rationalization | Required response |
|---|---|
| “This task is easier as a one-off shell command.” | Put reusable behavior in `mise.toml` or a native file task. |
| “This script is small enough to keep inline.” | Keep only short, obvious commands inline; move branching or arguments to `mise/tasks/`. |
| “Complex Bash is fine.” | Use Python for complex logic or non-trivial argument parsing. |
| “This tool might be useful globally.” | Scope one-off tools to the specific task first. |
| “The lockfile probably does not matter.” | Update and commit `mise.lock` when tools change. |
| “I can find the repo root with Git.” | Use `MISE_PROJECT_ROOT`, not `git rev-parse --show-toplevel`. |
| “Logging is noise.” | Add start, major step, command, artifact, and completion logs. |
| “`set -x` is faster.” | Do not use `set -x` by default because it can leak secrets. |

## Checklist

Before completing automation work, verify:

- task names follow `<group>:<action>` or `<group>:<sub-group>:<action>`
- `mise.toml` updated for tools, env vars, or short inline tasks
- native file tasks added under `mise/tasks/` for complex logic
- reusable libraries added under `mise/lib/` only when shared logic exists
- task scripts include traceable logging with module/file, function, and line number context
- task scripts use `MISE_PROJECT_ROOT`
- task-specific tools are scoped to the relevant task instead of global `[tools]`
- `mise.lock` updated when global or task-scoped tools change
- CI calls `mise run <task>` instead of duplicating task logic

## References

Load these only when relevant:

- `references/task-conventions.md` for task naming, lockfile, and task-scoped tools.
- `references/file-tasks.md` for native file task layout and Bash/Python choice.
- `references/script-logging.md` for traceable logging patterns.
- `references/script-libraries.md` for reusable Bash/Python helper conventions.
