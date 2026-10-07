# Branch Strategy & State Audit

**Audit Date:** 2026-10-07
**Repository:** `Sorobo-Gate/soroban-anchor-gate-contract`
**Current HEAD:** `3411dec` on branch `develop`
**Default Branch (Remote):** `develop`
**Release Branch:** `main` (at `4f8b33c`)
**Public Release Tag:** `v0.1.1` (commit `3411dec`, release `v0.1.1`)

---

## 1. Observed Branch Condition

Inspection via Git CLI and GitHub API confirms the following branch topology:

- **Local Branches:**
  - `* develop`: Primary development and collaboration branch (`3411dec`).
  - `  main`: Release branch anchored at `4f8b33c` ("Initial commit").
  - `  backup/pre-history-remediation-20261005`: Historical snapshot branch at `3dba838`.
- **Remote Branches (`origin`):**
  - `origin/develop`: Matches local `develop` (currently 59 commits ahead of `origin/main`).
  - `origin/main`: Production release target.
  - `origin/backup/pre-history-remediation-20261005`: Preserved historical backup.
- **Tags & Releases:**
  - Git tag `v0.1.1` exists at commit `3411dec`.
  - Formal GitHub Release `v0.1.1` ("v0.1.1 - Verified Milestone Escrow with Live Testnet Deployment") published as Latest Release with compiled WASM binary `soroban_anchor_gate_contract.wasm` (hash `ba9eaef2...`).
  - Prior release `v0.1.0` remains documented as historical baseline.
- **Continuous Integration (CI):**
  - Workflow `Rust & Soroban Checks` is green/passing on `develop`.
- **Issues & PR Status:**
  - Open PRs: **0**.
  - Open Issues: **2** (`#1` multi-token support, `#2` arbiter dispute resolution).
- **Branch Protection & Enforcement:**
  - **Status:** **Active & Verified on `develop`**
  - **Required Status Check:** `Rust & Soroban Checks` (strict matching against Contracts CI).
  - **Pull Request Requirement:** Minimum 1 approving review required; stale reviews dismissed on new push.
  - **Force Pushes:** Disabled (`allow_force_pushes: false`).
  - **Branch Deletions:** Disabled (`allow_deletions: false`).
  - **Administrative Override:** Permitted for maintainer emergency maintenance (`enforce_admins: false`).

---

## 2. Onchain Deployment Baseline

- **Authoritative Contract ID:** `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT`
- **WASM Hash:** `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32`
- **Network:** Stellar Testnet (`Test SDF Network ; September 2015`)
- **Initialization:** Admin `GAC6AIE...`, Treasury `GAC6AIE...`, Protocol Fee: 200 bps (2.00%)
- **Verified Lifecycle Evidence:** Documented in [`evidence/testnet-2026-10-06.md`](../evidence/testnet-2026-10-06.md)

---

## 3. Branch Model Governance

The repository strictly implements **Gitflow-Lite**:
- `develop` serves as the primary integration and default development branch where all active features, fixes, tests, and documentation are merged via pull requests.
- `main` serves as the verified production/release branch, receiving updates only via audited release PRs from `develop` upon milestone tagging.
- Pull requests must originate from topic branches (`fix/*`, `feat/*`, `docs/*`, `test/*`, `ci/*`, `chore/*`) targeting `develop`.
- Direct pushes to `develop` without PRs are blocked for standard contributors by the configured branch protection rules.
