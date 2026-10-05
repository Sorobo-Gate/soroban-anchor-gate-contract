# Branch Strategy & State Audit

**Audit Date:** 2026-10-05  
**Repository:** `Sorobo-Gate/soroban-anchor-gate-contract`  
**Current HEAD:** `8487fcf` on branch `develop`  
**Default Branch (Remote):** `develop`  
**Release Branch:** `main` (at `4f8b33c`)

---

## 1. Observed Branch Condition

Inspection via Git CLI and GitHub API revealed the following state:

- **Local Branches:**
  - `* develop`: Commit `8487fcf` (ahead of `main` by 10 commits).
  - `  main`: Commit `4f8b33c` ("Initial commit").
- **Remote Branches (`origin`):**
  - `origin/develop`: Commit `8487fcf` (matches local `develop`).
  - `origin/main`: Commit `4f8b33c`.
  - `origin/HEAD`: Points to `origin/main` in git remote ref, but GitHub repository metadata (`defaultBranchRef`) designates `develop` as the primary default branch.
- **Commit Delta:**
  - `develop` is 10 commits ahead of `main` (`4f8b33c..8487fcf`).
  - `main` has 0 commits ahead of `develop`.
- **Tags & Releases:**
  - Git tag `v0.1.0` exists at commit `bdd568d` on `develop`.
  - GitHub Releases API returns `no releases found` (git tag has not been published as a formal GitHub release).
- **Pull Requests:**
  - No open or closed pull requests exist on `Sorobo-Gate/soroban-anchor-gate-contract`.
- **Branch Protection & Rulesets:**
  - No branch protection rules or rulesets are currently active on `develop` or `main`.

---

## 2. Intended Branch Model Evaluation

Two potential strategies were evaluated:

### Option A: `develop` as Primary Integration Branch, `main` as Release Branch (Selected)
- **Rationale:**
  1. The GitHub repository default branch is explicitly configured to `develop`.
  2. All feature, test, CI, and documentation commits (`c7c40ba` through `8487fcf`) were committed directly to `develop`.
  3. Contributor issue guidelines in `docs/05-contributing.md` instruct contributors to target development branches.
  4. Tag `v0.1.0` was minted directly off `develop` at commit `bdd568d`.
  5. The companion repository `Sorobo-Gate/soroban-anchor-gate-app` follows an identical pattern with active work on `develop`.

### Option B: `main` as Primary Branch
- **Evaluation:**
  - Would require rewriting or fast-forwarding `main` prematurely before external audit and release gating.
  - Conflicts with existing remote default branch configuration.

### Conclusion:
The repository adheres to **Strategy A**: `develop` serves as the primary integration and default development branch. `main` is reserved as the production/release branch, to be updated only upon verified milestone releases.

---

## 3. Discrepancy & Required Actions

1. **CI Branch Strategy Mismatch:**
   - Existing `.github/workflows/ci.yml` strictly specified `push: [main]` and `pull_request: [main]`.
   - As a result, CI checks **never ran** against commits pushed to `develop`.
   - **Fix:** Update `.github/workflows/ci.yml` to trigger on both `develop` and `main`.

2. **Documentation Parity:**
   - Contributing guides and README must explicitly document `develop` as the integration target for pull requests, and `main` as the tagged release target.
