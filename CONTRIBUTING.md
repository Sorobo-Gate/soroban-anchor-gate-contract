# Contributing to SorobanAnchor Gate Contract

Thank you for contributing to the `soroban-anchor-gate-contract` repository! We welcome contributions that maintain code quality, security boundaries, and protocol correctness.

---

## 1. Prerequisites & Toolchain Setup

Ensure the following tools are installed before developing:

- **Rust Toolchain:** Stable Rust 1.85+ (tested on `1.97.1`)
- **WebAssembly Target:** `wasm32v1-none`
  ```bash
  rustup target add wasm32v1-none
  ```
- **Stellar CLI:** Version `28.1.0` or higher
  ```bash
  stellar --version
  ```

---

## 2. Branch Model & Contribution Workflow

- **Default / Integration Branch:** `develop` (all pull requests must target `develop`)
- **Release Branch:** `main` (reserved strictly for verified tagged milestone releases)

### Branching Rules:
1. Always base feature and bugfix branches off the latest `develop`:
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feat/your-feature-name
   ```
2. Pull requests must target `develop`, never `main`.
3. Branch protection is active on `develop`:
   - All changes must arrive via pull request with review approval.
   - Required status checks (`Contracts CI`) must pass cleanly.
   - Force pushes and branch deletions are disabled.

---

## 3. Supported Build and Verification Commands

Every pull request must pass all local checks before submission:

```bash
# 1. Code formatting check
cargo fmt --check

# 2. Typecheck with compiler
cargo check

# 3. Compiler linting with warnings denied
cargo clippy --all-targets -- -D warnings

# 4. Unit and integration tests (21 test cases)
cargo test

# 5. Canonical Stellar WASM contract compilation
stellar contract build
```

---

## 4. Engineering Standards & Commit Hygiene

To maintain an auditable and coherent project history, please follow these guidelines:

### 1. One Logical Unit per Commit
Each commit should represent a coherent, self-contained change. Related code, tests, and documentation that belong to the same logical task should be committed together. Avoid bundling unrelated tasks.

> **Note**: One logical unit per commit does not mean one file per commit. Stage all files that comprise the logical unit together.

### 2. Conventional Commits Format
Use standard conventional commit prefixes with an appropriate scope:
```text
type(scope): concise description in imperative mood
```
- `feat(escrow): implement multi-token whitelist storage`
- `fix(events): correct payload encoding for disbursed event`
- `test(refund): verify timelock boundary condition`
- `docs(spec): update storage key mapping table`
- `chore(deps): update soroban sdk dependencies`

### 3. Selective File Staging
- Stage specific files that belong to the logical unit: `git add <file1> <file2>`.
- Avoid blanket staging commands such as `git add .` or `git commit -a` to prevent unintentionally committing untracked files, local configuration, or secrets.
- Review staged changes with `git diff --staged` before committing.

### 4. Verification Before Submitting
Run the full test suite and linters locally before submitting your changes to ensure CI passes on the first run.

---

## 5. Security-Sensitive Protocol Areas

Pay rigorous attention when modifying the following components:
- **Authorization Enforcement:** Any public state mutation must enforce caller auth (`admin.require_auth()` or `payer.require_auth()`).
- **Token Transfers:** Direct SAC transfers must balance exactly; ensure contract balance returns to zero on terminal release or refund.
- **Arithmetic Safety:** Never use unchecked operations on token amounts or fees. Use checked arithmetic (`checked_mul`, `checked_sub`, `checked_add`) to prevent panic or overflow.
- **Timelocks:** Ensure timestamp comparisons evaluate against `env.ledger().timestamp()`.
- **Storage Expiry:** Always extend instance and persistent TTL on state mutations.

---

## 6. Documentation Expectations

Any pull request that alters contract interfaces, errors, storage keys, or event schemas must simultaneously update:
- [`docs/contract-spec.md`](docs/contract-spec.md)
- [`docs/app-parity.md`](docs/app-parity.md)
- [`README.md`](README.md)

---

## 7. License

Distributed under the Apache 2.0 License. See [`LICENSE`](LICENSE) for details.
