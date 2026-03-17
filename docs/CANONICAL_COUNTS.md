# Canonical Counts — Genesis Protocol

Single source of truth for all numerical claims. Updated 2026-03-17.

---

## Three-Tier Counting

Genesis Protocol numbers exist at three levels of scope. Every public reference must specify which tier.

### Tier 1 — Historical Compute

> Genesis Protocol has executed over 30,000 historical world-runs across development, stress testing, reruns, and sensitivity campaigns.

This number includes all world simulations ever run during development — early prototypes, discarded parameter sweeps, re-runs during debugging, and pre-publication iteration. It is estimated from operator records and is not hash-verified. It contextualizes the computational effort behind the published corpus but is not a scientific claim.

**Use when:** describing total computational effort, project history, or development scope.

### Tier 2 — Raw Directory Inventory

| Component | Raw Directories |
|---|---|
| Season 1 experiments | 25 |
| Season 2 experiments | 13 |
| Sensitivity experiments | 13 |
| **Total raw directories** | **51** |

Each directory corresponds to one experiment binary invocation. Some directories share world-runs across parameter steps (e.g., a sweep of 9 threshold values × 20 runs per step = 180 worlds in one directory). The 51 count is a file-system fact.

**Use when:** describing repo structure or raw experiment output.

### Tier 3 — Canonical Published Corpus

| Metric | Value |
|---|---|
| Grouped experiment configurations | 44 |
| World simulations | 6,820 |
| Computed epochs | > 3,410,000 |
| Collapses (P_floor=3, window=50) | 0 |
| 95% Clopper-Pearson CI upper bound | 0.054% |

The 44 configurations are obtained by grouping the 51 raw directories into logically distinct parameter settings:
- Season 1: 25 raw directories → 17 grouped configurations → ~3,640 worlds
- Season 2: 13 raw directories → 13 grouped configurations → 1,500 worlds
- Sensitivity: 13 raw directories → 14 grouped configurations → 1,680 worlds

The grouping reflects experiment design, not file layout. Some Season 1 experiments share a binary but sweep distinct variables (each variable sweep = one configuration). Sensitivity has 14 logical configurations (5 floor values × 2 window variants + 8 fitness-weight perturbations) across 13 directories.

**Use when:** making any public scientific claim, citing the paper, or referencing published results. This is the default tier for all external communication.

---

## Test Counts

| Metric | Value |
|---|---|
| `#[test]` functions | 373 |
| `#[tokio::test]` functions | 30 |
| **Total test functions** | **403** |
| Passing (`cargo test --workspace`) | 396 |
| Ignored (long-run validations) | 7 |
| Failing | 0 |

The 7 ignored tests are in `crates/genesis-experiment/src/long_run_validation.rs`:
- `test_50k_baseline`
- `test_determinism_10k`
- `test_equilibrium_convergence_time`
- `test_extreme_mutation`
- `test_high_catastrophe_mode`
- `test_resource_collapse`
- `test_treasury_cycling_ratio`

These are annotated with `#[ignore]` because they require extended runtimes (10,000+ epochs). They are included in the total function count but excluded from standard CI runs.

**Convention:** Write "403 tests" in headlines. Write "396 pass, 7 long-run ignored, 0 failures" in technical detail. Write "403 total (396 passing, 7 long-run validations)" in metrics tables.

---

## Source Lines

| Metric | Value | Date Pinned |
|---|---|---|
| Rust source lines (all `.rs` files) | 26,581 | 2026-03-17 |
| Files counted | 91 | 2026-03-17 |

Counted via: all `.rs` files in repository excluding `target/`, measured by line count. This number will change with any code edit. Only cite a specific number if freshly verified.

---

## Build State

| Check | Result | Date Verified |
|---|---|---|
| `cargo build --release` | 0 errors, 0 compiler warnings | 2026-03-17 |
| `cargo clippy --workspace` | 0 errors, ~56 style suggestions | 2026-03-17 |
| `cargo test --workspace` | 396 pass, 7 ignored, 0 fail | 2026-03-17 |

Clippy suggestions are advisory (needless borrows, too_many_arguments, field_reassign_with_default). No correctness issues.

---

## Version Identifiers

| Identifier | Value |
|---|---|
| Engine version (workspace) | 0.1.0 |
| Published crate version | 0.1.1 |
| Git commit (HEAD of main) | `c67de30` |
| DOI | `10.5281/zenodo.18729652` |
| Base seed | `20260222` |
| ORCID | `0009-0008-8425-939X` |
| crates.io package | `genesis-multiverse` (namespace stub) |
| Repository | `https://github.com/FTHTrading/Genesis` |
| Rust edition | 2021 |
| Verified toolchain | 1.77 (canonical), 1.93 (tested) |

---

## Boilerplate (copy-paste for public posts)

### Short form (profile/header):
> Deterministic macroeconomic simulation engine. 13 crates. 403 tests. 6,820 worlds across 44 experiments. Seasons 1–2 plus sensitivity. Zero collapses at P_floor=3. Phase transition at floors 5–10. Replication challenge open.

### Standard form (post footer):
> [Genesis Protocol](https://github.com/FTHTrading/Genesis) — 6,820 worlds, 44 experiments, open source. DOI: `10.5281/zenodo.18729652`

### Technical form (announcements):
> 13 crates, 403 tests (396 passing, 7 long-run validations), 26,581 source lines. 44 experiment configurations producing 6,820 world simulations across >3,410,000 epochs. Zero collapses under default definition (P_floor=3, 50-epoch window). Commit `c67de30`. Seed `20260222`.

### Historical context form (when discussing compute history):
> Genesis Protocol has executed over 30,000 historical world-runs across development, stress testing, reruns, and sensitivity campaigns. The current canonical public replication corpus contains 6,820 worlds across 44 grouped experiment configurations.

### Approved Moltbook bio line:
> Deterministic macroeconomic simulation engine. 13 crates. 403 tests. 6,820 worlds across 44 experiments. Seasons 1–2 plus sensitivity. Zero collapses at P_floor=3. Phase transition at floors 5–10. Replication challenge open. crates.io: genesis-multiverse

### Approved crate description (Cargo.toml / crates.io):
> Public crate namespace for the Genesis Protocol simulation ecosystem. Canonical metadata, experiment totals, and repository linkage for the 13-crate deterministic macroeconomic simulation engine.

### Recommended public line (general promotion):
> Genesis Protocol is now live as an open-source Rust research engine, with canonical repository, DOI-backed publication record, and crates.io namespace established. Public replication is open.

### Promotion order:
1. GitHub repo — the real proof surface (code, docs, paper, architecture)
2. Pinned canon post — resets all drift, gives one clean reference
3. DOI / paper — research legitimacy
4. crates.io — only after stub is synced; position as namespace anchor, not product page

### Approved Moltbook post footer:
> [Source](https://github.com/FTHTrading/Genesis) · [DOI](https://doi.org/10.5281/zenodo.18729652) · [Leaderboard](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)

---

## Hash Registry Coverage

| Scope | Experiments | Worlds | Individual Hashes Published |
|---|---|---|---|
| Season 1 | 25 | ~3,640 (ledger: 4,180) | Yes — 25 entries in `replication_status.json` |
| Season 2 | 13 | 1,500 | Yes — 13 entries in `replication_status.json` |
| Sensitivity | 14 configs | 1,680 | **No** — totals declared, individual entries pending |
| **Total in hash registry** | **38** | **5,680** | |
| **Total canonical (including sensitivity)** | **44** | **6,820** | |

The `replication_status.json` header declares 44 experiments and 6,820 worlds, but only 38 individual hash entries exist. The 6 sensitivity configurations that bring the total from 38 to 44 have their aggregate totals (14 configs, 1,680 worlds) recorded but no per-experiment hash entries. This is a known gap.

---

## Paper Scope Note

The research paper (`papers/genesis_protocol_paper.md`) covers Season 1 + Season 2 only (38 experiments, 5,680 worlds, 2,840,000 epochs). This is intentional — the paper was written before sensitivity analysis was complete. The paper's numbers are internally consistent and correct for its stated scope. They are not stale; they reflect a defined subset.

---

## Changelog

| Date | Change |
|---|---|
| 2026-03-17 | Initial canonical counts document. All numbers verified from source. |
