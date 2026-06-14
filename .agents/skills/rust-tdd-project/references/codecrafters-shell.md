# CodeCrafters Shell Course Alignment

Use the CodeCrafters Shell course stages as external acceptance criteria. Do not treat stage wording as architecture.

## Workflow Per Stage

1. Read the selected stage requirement on CodeCrafters.
2. Find the corresponding checkbox in `README.md`.
3. Add or update local tests where practical.
4. Implement the smallest passing behavior.
5. Run:

```bash
mise run rust:test
mise run rust:ci
```

6. Submit to CodeCrafters.
7. Refactor only after tests are green.

## Rules

- Implement only the behavior required by the current or selected stage.
- Do not jump ahead to unrelated shell features.
- Do not overfit to remote tests if it damages clear parser/executor design.
- Use the public stage checklist in `README.md` as the implementation roadmap.
- Keep local tests even when CodeCrafters provides remote tests.
