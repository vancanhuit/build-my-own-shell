# Cocogitto

Use Cocogitto, not Node-based Commitlint.

Do not add `commitlint`, `@commitlint/cli`, `commitlint.config.cjs`, `package.json`, `npm`, `npx`, or Husky solely for commit validation.

Recommended `cog.toml`:

```toml
from_latest_tag = false
ignore_merge_commits = true
branch_whitelist = ["main"]
skip_ci = "[skip ci]"
pre_bump_hooks = ["mise run rust:ci"]
post_bump_hooks = []

[changelog]
template = "remote"
remote = "github.com"
repository = "rsh"
owner = "<github-owner-or-org>"
authors = []

[git_hooks.commit-msg]
script = "cog verify --file $1"

[git_hooks.pre-push]
script = "mise run rust:ci"
```

Install hooks:

```bash
mise run git:hooks:install
```

Do not commit a `CHANGELOG.md`. Generate the changelog at release time and put it into the GitHub release notes (see `release:create`).
