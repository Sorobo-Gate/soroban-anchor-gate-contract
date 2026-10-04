# Drips Wave Contribution Guide

SorobanAnchor Gate participates in the **Drips Wave** program to reward open-source contributions.

---

## Working on Issues

All approved tasks are tracked in our repository issue boards:
- [Contracts Repository Issues](https://github.com/Sorobo-Gate/soroban-anchor-gate-contract/issues)
- [Application Repository Issues](https://github.com/Sorobo-Gate/soroban-anchor-gate-app/issues)

Issues are weighted according to the Drips Wave point system:
- `wave:trivial` (100 Points) — Documentation fixes, test additions, minor refactoring.
- `wave:medium` (150 Points) — Client SDK methods, UI form validations, error handler expansions.
- `wave:high` (200 Points) — New contract modules, SEP protocol integrations, multi-sig dispute resolution.

---

## Quality & Pull Request Checklist

Before submitting a pull request, ensure your branch passes all checks:
- **Contracts:** Must pass `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` with zero warnings.
- **Relay Daemon:** Must pass `go vet ./...` and `go test ./...`.
- **Frontend:** Must pass `npm run build` and `npm run lint`.