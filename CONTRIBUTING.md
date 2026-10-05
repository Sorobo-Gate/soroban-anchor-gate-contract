# Contributing to SorobanAnchor Gate Contract

Thank you for contributing to the `soroban-anchor-gate-contract` repository. We welcome contributions that maintain code quality, security boundaries, and protocol correctness.

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
  # Linux x86_64
  curl -sSL https://github.com/stellar/stellar-cli/releases/download/v28.1.0/stellar-cli-28.1.0-x86_64-unknown-linux-gnu.tar.gz | sudo tar -xz -C /usr/local/bin
  stellar --version
  ```

---

## 2. Branch Model & Contribution Workflow

- **Default / Integration Branch:** `develop`
- **Release Branch:** `main` (reserved strictly for verified tagged milestone releases)

### Branching Rules:
1. Always base feature and bugfix branches off the latest `develop`:
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feat/your-feature-name
   ```
2. Pull requests must target `develop`, never `main`.

---

## 3. Supported Build and Verification Commands

Every pull request must pass all local checks before submission:

```bash
# 1. Code formatting check
cargo fmt --check

# 2. Compiler linting with warnings denied
cargo clippy --all-targets -- -D warnings

# 3. Unit and integration tests (20 test cases)
cargo test

# 4. Canonical Stellar WASM contract compilation
stellar contract build
```

---

## 4. Git Commit Hygiene & Selective Staging

To maintain a clean, auditable Git history, contributors must adhere strictly to these rules:

1. **Conventional Commits Format:**
   ```text
   type(scope): concise description
   ```
   *Allowed types:* `feat`, `fix`, `test`, `ci`, `docs`, `refactor`, `chore`.  
   *Examples:*
   - `feat(escrow): implement multi-token whitelist storage`
   - `test(refund): verify timelock boundary condition`
   - `fix(events): correct payload encoding for disbursed event`

2. **One Logical Unit Per Commit:**
   Never bundle unrelated changes (e.g. Mixing contract logic edits with documentation or dependency upgrades).

3. **Selective File Staging:**
   - **Never run:** `git add .` or `git commit -a`.
   - **Always run:** `git add exact/path/to/file`.
   - Inspect staged diff before committing: `git diff --cached`.

4. **Continuous Integration Verification:**
   Push changes after each verified logical commit.

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
