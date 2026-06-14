---
name: github-governance
description: github repository governance and automation guidance for GitHub Actions, GitHub CLI, pull requests, CI status checks, release workflows, squash merge, linear history, and branch protection. Use when adding or changing workflows, release automation, PR process, branch rules, GitHub CLI tasks, or repository governance.
---

# GitHub Governance

Use this skill when changing GitHub Actions, GitHub CLI workflows, release automation, pull request rules, branch protection, or repository governance.

## Core Rules

1. Use GitHub CLI for GitHub workflows where practical.
2. Keep GitHub Actions thin and call `mise run` tasks.
3. Avoid custom marketplace actions unless necessary.
4. Use latest stable major versions of actions.
5. Use least-privilege permissions.
6. Use concurrency in workflows.
7. Assume squash merge and linear history.
8. Require CI and PR title validation before merge.

## Rationalizations / Do Not Skip

| Rationalization | Required response |
|---|---|
| “This old workflow example is probably fine.” | Verify current stable major versions before editing workflows. |
| “A marketplace action is easier.” | Prefer `mise`, shell commands, Cargo, Cocogitto, or `gh` unless necessary. |
| “The release action is standard.” | Use GitHub CLI for release creation unless `gh` cannot satisfy the requirement. |
| “Permissions are harmless.” | Use least-privilege `permissions` for every workflow. |
| “Concurrency is optional.” | Add `concurrency` to CI, PR title, and release workflows. |
| “The workflow can just run raw commands.” | Call `mise run <task>` so local and CI behavior stay aligned. |
| “We can fix the PR title when merging.” | Validate PR titles in CI. |
| “Merge commits are fine.” | Preserve linear history unless policy changes explicitly. |

## References

Load these only when relevant:

- `references/workflows.md` for CI and PR title workflow templates.
- `references/releases.md` for GitHub CLI release rules.
- `references/branch-protection.md` for squash merge and required checks.
