# Branch Strategy & State Audit

**Audit Date:** 2026-10-05  
**Repository:** `Sorobo-Gate/soroban-anchor-gate-contract`  
**Current HEAD:** `1b878dc` on branch `develop`
**Default Branch (Remote):** `develop`
**Release Branch:** `main` (at `4f8b33c`)
**Public Release Tag:** `v0.1.0` (commit `bdd568d`)

---

## 1. Observed Branch Condition

Inspection via Git CLI and GitHub API confirms the following branch topology:

- **Local Branches:**
  - `* develop`: Primary development and collaboration branch.
  - `  main`: Release branch anchored at `4f8b33c` ("Initial commit").
  - `  backup/pre-history-remediation-20261005`: Historical snapshot branch at `3dba838`.
- **Remote Branches (`origin`):**
  - `origin/develop`: Matches local `develop`.
  - `origin/main`: Production release target.
  - `origin/backup/pre-history-remediation-20261005`: Preserved historical backup.
- **Tags & Releases:**
  - Git tag `v0.1.0` exists at commit `bdd568d`.
  - Formal GitHub Release `v0.1.0` published with release notes and compiled WASM binary.
- **Branch Protection & Enforcement:**
  - **Status:** **Active & Verified on `develop`**
  - **Required Status Check:** `Rust & Soroban Checks` (strict matching against Contracts CI).
  - **Pull Request Requirement:** Minimum 1 approving review required; stale reviews dismissed on new push.
  - **Force Pushes:** Disabled (`allow_force_pushes: false`).
  - **Branch Deletions:** Disabled (`allow_deletions: false`).
  - **Administrative Override:** Permitted for maintainer emergency maintenance (`enforce_admins: false`).

---

## 2. Branch Model Governance

The repository strictly implements **Gitflow-Lite**:
- `develop` serves as the primary integration and default development branch where all active features, fixes, tests, and documentation are merged via pull requests.
- `main` serves as the verified production/release branch, receiving updates only via audited release PRs from `develop` upon milestone tagging.
- Pull requests must originate from topic branches (`fix/*`, `feat/*`, `docs/*`, `test/*`, `ci/*`, `chore/*`) targeting `develop`.
- Direct pushes to `develop` without PRs are blocked for standard contributors by the configured branch protection rules.
