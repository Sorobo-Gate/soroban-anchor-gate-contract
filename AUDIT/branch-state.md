# Branch Strategy & State Audit

**Audit Date:** 2026-10-09
**Repository:** `Sorobo-Gate/soroban-anchor-gate-contract`
**Current HEAD:** `a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1` on branch `develop`
**Default Branch (Remote):** `develop`
**Release Branch:** `main` (at `a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`)
**Public Release Tag:** `v0.1.1` (commit `3411dec2321a3f566f357ace3b1ced6d5bf04061`, release `v0.1.1`)

---

## 1. Observed Branch Condition

Inspection via Git CLI and GitHub API confirms the following branch topology:

- **Local Branches:**
  - `* develop`: Primary development and collaboration branch (`a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`).
  - `  main`: Release branch aligned with `develop` (`a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`).
  - `  backup/pre-history-remediation-20261005`: Historical snapshot branch at `3dba838`.
- **Remote Branches (`origin`):**
  - `origin/develop`: Matches local `develop` (`a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`).
  - `origin/main`: Production release target (`a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`).
  - **Branch Relationship**: `main` and `develop` are identical (0 commits ahead, 0 commits behind).
- **Tags & Releases:**
  - Git tag `v0.1.1` exists at commit `3411dec2321a3f566f357ace3b1ced6d5bf04061`.
  - Formal GitHub Release `v0.1.1` ("v0.1.1 - Verified Milestone Escrow with Live Testnet Deployment") published as Latest Release with compiled WASM binary `soroban_anchor_gate_contract.wasm` / `soroban_anchor_escrow.wasm` (hash `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32`).
  - Prior release `v0.1.0` remains documented as historical baseline (commit `bdd568de938a3cb4f3a9ead2808d48de91cbc392`).
- **Continuous Integration (CI):**
  - Workflow `Contracts CI` (`Rust & Soroban Checks`) passes on `main` and `develop`.
- **Issues & PR Status:**
  - Open PRs: **0**.
  - Open Issues: **2**:
    - `#1` `feat(escrow): implement multi-token whitelist storage`
    - `#2` `feat(escrow): implement arbiter dispute resolution branch`
- **Branch Protection & Enforcement:**
  - **Status:** **Active & Verified on both `main` and `develop`**
  - **develop Rules:** Required Status Check `Rust & Soroban Checks` (strict matching against Contracts CI); minimum 1 approving review required; stale reviews dismissed on new push; force pushes disabled; branch deletions disabled; administrative override permitted for maintainer emergency maintenance (`enforce_admins: false`).
  - **main Rules:** Protected release branch; force pushes disabled; branch deletions disabled (`enforce_admins: false`).

---

## 2. Onchain Deployment Baseline

- **Authoritative Contract ID:** `CBIHLECKLMXYK6FPHSGGVYF6AHNFRHR3T5EIFIS3PFQDFKDQWNR25PHT`
- **WASM Hash:** `ba9eaef277a2943c5a48f38aea849c78acf8cd9306d71646225a15ea41965e32`
- **Release:** `v0.1.1`
- **Network:** Stellar Testnet (`Test SDF Network ; September 2015`)
- **Initialization:** Admin `GAC6AIE...`, Treasury `GAC6AIE...`, Protocol Fee: 200 bps (2.00%)
- **Verified Lifecycle Evidence:** Documented in [`evidence/testnet-2026-10-06.md`](../evidence/testnet-2026-10-06.md)

---

## 3. Branch Model Governance

The repository strictly implements **Gitflow-Lite**:
- `develop` serves as the active integration and default development branch where all active features, fixes, tests, and documentation are merged via pull requests.
- `main` serves as the verified production/release branch, receiving updates only via audited release PRs from `develop` upon milestone tagging. Both branches currently point to the same verified release-ready state (`a44c5c1617e4473c1dae52c74ddd17dc8f6ec2e1`).
- Pull requests must originate from topic branches (`fix/*`, `feat/*`, `docs/*`, `test/*`, `ci/*`, `chore/*`) targeting `develop`.
- Direct pushes to `develop` without PRs are blocked for standard contributors by the configured branch protection rules.

---

## 4. Historical State Reference

For auditing and traceability purposes, previous milestone states are catalogued:
- **Pre-history remediation snapshot:** Branch `backup/pre-history-remediation-20261005` preserved at `3dba838`.
- **v0.1.1 Release Tag Commit:** Tagged at `3411dec` on `develop` prior to documentation and presentation alignment passes.
- **Initial Setup:** Historical baseline commit `4f8b33c` ("Initial commit") previously anchored `main` prior to full Gitflow promotion alignment.
