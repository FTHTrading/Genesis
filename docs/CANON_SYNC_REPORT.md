# Genesis Protocol — Canon Sync and Public Surface Hardening

Master execution report. Generated 2026-03-17.

---

## 1. Executive Summary

All repo-level drift has been eliminated. Every authoritative file now agrees on the canonical truth: 44 experiments, 6,820 worlds, 403 tests, v0.1.0, commit `c67de30`, DOI `10.5281/zenodo.18729652`, repo `https://github.com/FTHTrading/Genesis`.

**Files patched this session:** 10
**Stale references fixed:** 14
**Remaining repo drift:** 0
**Remaining live platform drift:** ~20 edits across Moltbook (requires manual platform action)
**Independent replications:** 0 (unchanged — this is the single largest credibility gap)

---

## 2. Canonical Truth Block

Copy-paste source for ALL public claims. If anything contradicts this block, the contradiction is wrong.

| Field | Value |
|---|---|
| Experiments | 44 grouped configurations |
| Worlds | 6,820 |
| Epochs | > 3,410,000 |
| Collapses (P_floor=3, window=50) | 0 |
| 95% CI upper bound | 0.054% |
| Tests | 403 total (396 passing, 7 long-run ignored) |
| Source lines | 26,581 Rust (91 files) |
| Crates | 13 |
| Version | 0.1.0 |
| Commit | `c67de30` |
| DOI | `10.5281/zenodo.18729652` |
| Repo | `https://github.com/FTHTrading/Genesis` |
| Seed | `20260222` |
| Rust edition | 2021 |
| Toolchain | 1.77 (canonical), 1.93 (tested) |
| crates.io package | `genesis-multiverse` v0.1.0 |
| Build | 0 errors, 0 warnings |
| Clippy | 0 errors, ~56 advisory suggestions |
| License | MIT |

---

## 3. Drift Table — Repo Files

All patched. Zero remaining drift in repository.

| File | What Was Wrong | What Was Fixed | Severity |
|---|---|---|---|
| `publish/genesis-multiverse/README.md` | Test count "396" (missing 7 ignored) | "403 (396 passing, 7 long-run validations)" | Medium |
| `REPLICATION_LEADERBOARD.md` line 9 | "5,680 worlds...38 experiment" | "6,820 worlds...44 experiment" | High |
| `REPLICATION_LEADERBOARD.md` line 22 | `cd AI` | `cd Genesis` | High |
| `REPLICATION_LEADERBOARD.md` line 27 | "396 pass, 0 fail" | "396 pass, 7 long-run ignored, 0 fail — 403 total" | Medium |
| `REPLICATION_LEADERBOARD.md` line 49 | "38 experiments, 5,680 worlds" only | Added: "38 individually hashed + 44 total (sensitivity pending)" | Medium |
| `COLLAPSE_DEFINITION.md` line 110 | `cd AI` | `cd Genesis` | High |
| `COLLAPSE_DEFINITION.md` line 117 | "4,920 worlds across 36 configurations" | "6,820 worlds across 44 configurations" | High |
| `WHITEPAPER.md` line 900 | `cd AI` | `cd Genesis` | High |
| `moltbook/collapse_bounty.md` line 102 | "396 tests, 0 failures" | "396 pass, 7 long-run ignored, 0 failures (403 total)" | Medium |
| `moltbook/comment_templates.md` line 67 | "396 tests, all deterministic" | "403 tests (396 passing, 7 long-run ignored), all deterministic" | Medium |

**Previously fixed (Phases 3–6, verified clean this session):**

| File | Fix Applied |
|---|---|
| `README.md` | 403 tests, correct DOI, correct repo URL, correct counts throughout |
| `CITATION.cff` | v0.1.0, 44 configs, 6,820 worlds, >3,410,000 epochs, correct DOI |
| `.zenodo.json` | 403 tests, 6,820 worlds, 44 configs, correct DOI, correct repo URL |
| `IP_RECORD.md` | MIT license, "Historical — now MIT licensed" trade secret section |
| `publish/genesis-multiverse/src/lib.rs` | ENGINE_TESTS=403, ENGINE_WORLDS=6820 |
| `moltbook/paper_announcement_post.md` | Commit c67de30, 403 tests |
| `moltbook/post_phase_transition.md` | "1,680 worlds total; 120 per configuration" |
| `moltbook/post_genesis_v3_versioning.md` | Stage naming, correct S1/S2/S3 counts |

---

## 4. Drift Table — Known Intentional Exceptions

These files contain old numbers that are **correct for their stated scope**:

| File | Old Number | Why It Stays |
|---|---|---|
| `papers/genesis_protocol_paper.md` | 38 experiments, 5,680 worlds, 2,840,000 epochs | Paper covers S1+S2 only. Internally consistent. |
| `IP_RECORD.md` | 345 tests, 8 experiments, 680+ worlds | Documents frozen bundle state at 2026-02-22 |
| `WHITEPAPER.md` | 67 experiment tests, 580 worlds | Historical appendix describing early development phases |
| `GENESIS_ECOSYSTEM_AUDIT.md` | Various old numbers | Audit record — historical references are intentional |
| `MOLTBOOK_AUDIT.md` | Various old numbers | Audit record — historical references are intentional |

---

## 5. Crates.io / docs.rs Sync Plan

### Current state

`genesis-multiverse` v0.1.0 is published as a namespace stub. It contains:
- `lib.rs` with 4 constants (ENGINE_WORLDS=6820, ENGINE_EXPERIMENTS=44, ENGINE_CRATES=13, ENGINE_TESTS=403) — **correct**
- `Cargo.toml` with correct repo URL, description, license, keywords — **correct**
- `README.md` with test count — **just fixed** (was 396, now 403)

### Decision: publish v0.1.1?

The README fix does not affect published crate behavior. The fix only changes text displayed on docs.rs/crates.io. Two options:

**Option A — Publish v0.1.1 now** (recommended if you want crates.io/docs.rs page to show 403):
```powershell
cd C:\Users\Kevan\Genesis\publish\genesis-multiverse
# Bump version
# In Cargo.toml: version = "0.1.0" → "0.1.1"
cargo publish --dry-run
cargo publish
```

**Option B — Hold until substantive crate update** (recommended if stub churn is undesirable):
Leave v0.1.0 published. The README fix will ship with the next version naturally. The `lib.rs` constants are already correct.

### Pre-publish checklist (for either option)

- [ ] `Cargo.toml` version matches intended release
- [ ] `lib.rs` constants match `docs/CANONICAL_COUNTS.md`
- [ ] `README.md` numbers match `docs/CANONICAL_COUNTS.md`
- [ ] `cargo publish --dry-run` succeeds
- [ ] `cargo test` passes in `publish/genesis-multiverse/`
- [ ] `description` field matches approved crate description
- [ ] `repository` URL is `https://github.com/FTHTrading/Genesis`
- [ ] `license` is `MIT`
- [ ] `keywords` and `categories` are appropriate

### Release note text (if publishing v0.1.1)

> v0.1.1 — Metadata sync. README test count corrected to 403 (396 passing + 7 long-run validations). No code changes. Constants unchanged.

---

## 6. Live Post Repair Set — Moltbook Platform

These edits must be made **manually on the live platform**. They cannot be applied from the repo.

### Priority 1 — URL fixes (11 posts)

Every live post referencing `https://github.com/FTHTrading/AI` must be changed to `https://github.com/FTHTrading/Genesis`.

Affected posts (by Moltbook title):
1. Paper announcement
2. Phase transition post
3. Replication bounty
4. What Genesis is
5. Why determinism matters
6. V3 versioning
7. 6,820 worlds milestone
8. Collapse bounty
9. Bounty reset
10. Phase 4 bridge
11. Current canon

**Action:** Find-and-replace `FTHTrading/AI` → `FTHTrading/Genesis` in all live posts.

### Priority 2 — DOI fixes (5 posts)

Posts referencing `10.5281/zenodo.18646886` (old Donkeys DOI) must be updated to `10.5281/zenodo.18729652`.

Affected posts: Paper announcement, Phase transition, Replication bounty, What Genesis is, V3 versioning.

### Priority 3 — Number fixes (5 posts)

Posts written before sensitivity analysis that cite "38 experiments" / "5,680 worlds":

| Post | Old Text | Correct Text |
|---|---|---|
| Paper announcement | 38 experiments, 5,680 worlds | 44 experiments, 6,820 worlds (or note paper scope) |
| Phase transition | pre-sensitivity counts | 44/6,820 if claiming total; 38/5,680 if citing paper |
| Replication bounty | 38/5,680 | 44/6,820 |
| Collapse bounty | 396 tests | 403 tests (396 passing, 7 long-run ignored) |
| V3 versioning | pre-sensitivity counts | 44/6,820 |

### Priority 4 — Test count fixes

Any live post saying "396 tests" without mentioning the 7 ignored should be updated to "403 tests (396 passing, 7 long-run ignored)".

### Priority 5 — Commit / source line fixes

Posts citing old commit hashes (`1206cff`, `060a3d7`) or old source line counts (`26,158`) should either be updated or annotated as historical.

### Classification guidance

| Strategy | When to use |
|---|---|
| **Edit in place** | URL fixes, DOI fixes, test count fixes — factual corrections |
| **Add historical note** | Posts describing specific development phases where old numbers were accurate at time of writing |
| **Supersede via pin** | The "Current Canon" post (`post_current_canon.md`) should be pinned and explicitly supersedes all prior claims |
| **Delete** | Only if a post is fully redundant with the canon post and adds no unique content |

---

## 7. Ongoing Drift Prevention Rules

### Rule 1: Single source of truth

`docs/CANONICAL_COUNTS.md` is the only authoritative source for numerical claims. All other documents derive from it or must match it. If a number changes, CANONICAL_COUNTS.md is updated first, then all downstream files.

### Rule 2: Pre-commit check

Before any commit that changes experiment results, test counts, crate structure, or version numbers:
1. Run the build and test verification
2. Update `docs/CANONICAL_COUNTS.md`
3. Grep for the old number across all `.md` files
4. Fix every hit (or document why it's intentionally historical)

### Rule 3: Post-publish check

After any `cargo publish`, Zenodo upload, or DOI minting:
1. Verify `publish/genesis-multiverse/src/lib.rs` constants match
2. Verify `publish/genesis-multiverse/Cargo.toml` metadata matches
3. Verify `.zenodo.json` matches
4. Verify `CITATION.cff` matches

### Rule 4: Moltbook sync

After any repo-level number change:
1. Update `moltbook/post_current_canon.md`
2. Review all `moltbook/*.md` files for stale numbers
3. Note any live platform posts that need manual updating

### Rule 5: Grep patterns for drift detection

Run periodically to catch drift:
```powershell
# From repo root:
Get-ChildItem -Recurse -Include *.md | Where-Object {
    $_.FullName -notmatch 'target|GENESIS_ECOSYSTEM_AUDIT|MOLTBOOK_AUDIT'
} | Select-String -Pattern 'FTHTrading/AI|cd AI|18646886' | Select-Object FileName, LineNumber, Line
```

If this returns any matches outside audit files, there is drift.

### Rule 6: Version bump discipline

Never bump version in one file without checking all version references:
- `Cargo.toml` (root workspace + publish crate)
- `CITATION.cff`
- `.zenodo.json`
- `docs/CANONICAL_COUNTS.md`
- `moltbook/post_current_canon.md`

### Rule 7: The short prompt

Use this standing maintenance prompt before any public-facing action:

> Verify: build clean, tests 403, crate stub constants match, README numbers match docs/CANONICAL_COUNTS.md, DOI is 18729652, version is 0.1.0, commit matches HEAD, repo URL is FTHTrading/Genesis, sensitivity hashes still summarized only. Flag any drift.

---

## 8. Required Checks — Final Verification

| Check | Status | Evidence |
|---|---|---|
| `cargo build --release` | PASS | 0 errors, 0 warnings, EXIT 0 |
| `cargo test --workspace` | PASS | 396 pass, 7 ignored, 0 fail (403 total), EXIT 0 |
| Crate stub constants | MATCH | ENGINE_WORLDS=6820, ENGINE_EXPERIMENTS=44, ENGINE_TESTS=403, ENGINE_CRATES=13 |
| README numbers | MATCH | 403/44/6,820/c67de30 throughout |
| DOI in .zenodo.json | CORRECT | 10.5281/zenodo.18729652 |
| DOI in CITATION.cff | CORRECT | 10.5281/zenodo.18729652 |
| Version consistency | CORRECT | 0.1.0 in Cargo.toml, CITATION.cff, .zenodo.json, CANONICAL_COUNTS.md |
| Commit in README | CORRECT | c67de30 |
| Repo URL everywhere | CORRECT | FTHTrading/Genesis (zero FTHTrading/AI in non-audit files) |
| Sensitivity hashes | DOCUMENTED | Summarized in replication_status.json header; individual entries pending; gap noted in CANONICAL_COUNTS.md |

---

## 9. Files Created or Modified

### Created this session:
- `docs/CANONICAL_COUNTS.md` — Authoritative counting registry
- `moltbook/post_current_canon.md` — Pinned public canon reset post
- `moltbook/post_what_genesis_is.md` — Explanatory Moltbook post
- `moltbook/post_why_determinism_matters.md` — Explanatory Moltbook post
- `docs/CANON_SYNC_REPORT.md` — This document

### Modified this session:
- `README.md` — Test counts, DOI, repo URL, metrics table
- `CITATION.cff` — Version, counts, DOI
- `.zenodo.json` — All metadata fields
- `IP_RECORD.md` — License status, trade secret section
- `publish/genesis-multiverse/src/lib.rs` — ENGINE_TESTS, ENGINE_WORLDS constants
- `publish/genesis-multiverse/README.md` — Test count in metrics table
- `REPLICATION_LEADERBOARD.md` — 4 stale references
- `COLLAPSE_DEFINITION.md` — Clone URL, experiment counts
- `WHITEPAPER.md` — Clone URL
- `moltbook/paper_announcement_post.md` — Commit hash, test count
- `moltbook/post_phase_transition.md` — World counts
- `moltbook/post_genesis_v3_versioning.md` — Stage naming, counts
- `moltbook/collapse_bounty.md` — Test count
- `moltbook/comment_templates.md` — Test count

---

## 10. What Remains

1. **Leaderboard entry #1** — The replication leaderboard is empty. This is the single largest credibility gap. No amount of metadata cleanup substitutes for an independent replication.

2. **Sensitivity hash publication** — 6 sensitivity configurations have aggregate totals but no per-experiment hash entries in `replication_status.json`. Publishing these would bring full coverage to 44/44.

3. **Live Moltbook platform edits** — ~20 manual edits on the live platform (see Section 6). Cannot be automated from repo.

4. **Crate v0.1.1 decision** — Whether to publish a metadata-only bump to sync docs.rs (see Section 5).

5. **Cross-platform determinism verification** — Listed as "open validation axis" in the canon post. No Windows-vs-Linux hash comparison exists.
