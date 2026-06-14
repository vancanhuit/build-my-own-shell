# Branch Protection

Repository policy:

```text
Allow squash merging: enabled
Allow merge commits: disabled
Allow rebase merging: optional, preferably disabled
Require pull request before merging: enabled
Require status checks before merging: enabled
Require branches up to date before merging: enabled
Require linear history: enabled
Automatically delete head branches: enabled
```

Required status checks:

```text
CI / Rust CI
PR Title / Validate PR title
```
