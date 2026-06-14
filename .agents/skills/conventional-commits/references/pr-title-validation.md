# PR Title Validation

Because squash merge uses the PR title as the final commit message, PR titles must follow Conventional Commits.

Good titles:

```text
feat(parser): add quoted string support
fix(executor): handle missing commands as status 127
test(redirection): cover append output
ci(release): create release with github cli
docs(agents): slim project guidance
```

The validation task:

```toml
[tasks."pr:title:validate"]
description = "Validate pull request title as a Conventional Commit"
run = "echo \"$PR_TITLE\" | cog verify --file -"
```

Workflow step:

```yaml
env:
  PR_TITLE: ${{ github.event.pull_request.title }}
run: mise run pr:title:validate
```
