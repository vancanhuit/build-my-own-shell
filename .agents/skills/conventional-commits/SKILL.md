---
name: conventional-commits
description: conventional commit and cocogitto guidance for commit messages, pull request titles, git hooks, changelog policy, and release metadata. Use when creating PR titles, validating commits, editing cog.toml, installing hooks, configuring Conventional Commit validation, or documenting commit conventions.
---

# Conventional Commits

Use this skill when working with Conventional Commits, PR titles, Cocogitto, git hooks, changelog policy, or commit validation.

## Core Rules

1. Use Cocogitto for Conventional Commit validation.
2. Do not use Node-based Commitlint.
3. Keep PR titles Conventional Commit compliant.
4. Remember that squash merge makes the PR title the final commit message.
5. Install hooks through `mise run git:hooks:install`.
6. Avoid adding npm, npx, package.json, Husky, or Commitlint solely for commit validation.

## Rationalizations / Do Not Skip

| Rationalization | Required response |
|---|---|
| “The commit message is internal.” | Use Conventional Commits anyway. |
| “The PR title can be informal.” | Make it Conventional Commit compliant. |
| “The individual commits are good enough.” | Ensure the PR title is good; commits may be squashed away. |
| “This PR has multiple unrelated changes.” | Split the PR or choose one accurate title only if coherent. |
| “Commitlint examples are common.” | Use Cocogitto, not Node-based Commitlint. |
| “Hooks are enough.” | Keep CI validation; hooks are local convenience. |
| “This is a breaking change but obvious.” | Mark it with `!` or `BREAKING CHANGE:`. |

## Commit Format

```text
<type>(optional-scope): <description>
```

Allowed types:

```text
feat
fix
docs
style
refactor
perf
test
build
ci
chore
revert
```

Examples:

```text
feat(parser): support quoted strings
fix(executor): preserve pipeline exit status
test(builtins): cover cd to previous directory
ci(github): validate pull request titles
chore(deps): update Rust dependencies
```

## References

Load these only when relevant:

- `references/cocogitto.md` for `cog.toml`, hooks, and validation.
- `references/pr-title-validation.md` for PR title workflow and title selection.
