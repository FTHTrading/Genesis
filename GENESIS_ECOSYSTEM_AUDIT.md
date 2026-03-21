# Genesis Protocol — Ecosystem Audit

**Date:** 2026-03-17
**Auditor:** Automated deep-dive from codebase, data files, commit history, and published artifacts
**Repo:** https://github.com/FTHTrading/Genesis
**Latest tag:** v0.1.0 (commit `c67de30`)
**HEAD:** `c67de30` — "Add MIT License"

---

## AUDIT UPDATE — 2026-03-21

All P0 and P1 issues identified in this audit have been resolved in subsequent commits. The following critical items were addressed:

| # | Item | Resolution | Status |
|---|---|---|---|
| 1 | `CITATION.cff` version "2.0.0" | Updated to `0.1.0` | ✅ Resolved |
| 2 | `CITATION.cff` abstract — 52 configs / 7,360 worlds | Updated to 44 configs / 6,820 worlds | ✅ Resolved |
| 3 | `.zenodo.json` version "2.0.0" | Updated to `0.1.0` | ✅ Resolved |
| 4 | `.zenodo.json` description — 7,360 worlds / 52 configs | Updated to 6,820 worlds / 44 configs / 403 tests | ✅ Resolved |
| 5 | `IP_RECORD.md` license "Proprietary" | Updated to MIT | ✅ Resolved |
| 6 | `README.md` test count — 396 | README now shows 403 total (396 passing + 7 ignored) at line 3 and line 133. Line 100 correctly shows `396 pass, 7 long-run ignored`. | ✅ Resolved |

**Current state (HEAD `50d4da4`):** All public-facing metadata documents (README, CITATION.cff, .zenodo.json, IP_RECORD.md) are consistent and accurate. The section references below that show the OLD claimed values represent the state at audit-date commit `c67de30` and are retained for historical completeness.

For the current independent technical audit and peer review, see:
- `THIRD_PARTY_AUDIT.md` — Third-person code and security audit (2026-03-21)
- `PEER_REVIEW.md` — Formal peer review of research claims (2026-03-21)

---

## SECTION 1 — CURRENT TRUTH

All values below are derived directly from source code, data files, and experiment artifacts.

### 1.1 Crate Inventory

| # | Crate | Path | Published (crates.io) |
|---|---|---|---|
| 1 | `genesis-dna` | crates/genesis-dna | **No** |
| 2 | `metabolism` | crates/metabolism | **No** |
| 3 | `apostle` | crates/apostle | **No** |
| 4 | `ecosystem` | crates/ecosystem | **No** |
| 5 | `evolution` | crates/evolution | **No** |
| 6 | `gateway` | crates/gateway | **No** |
| 7 | `genesis-anchor` | crates/genesis-anchor | **No** |
| 8 | `genesis-replay` | crates/genesis-replay | **No** |
| 9 | `genesis-federation` | crates/genesis-federation | **No** |
| 10 | `genesis-econometrics` | crates/genesis-econometrics | **No** |
| 11 | `genesis-homeostasis` | crates/genesis-homeostasis | **No** |
| 12 | `genesis-multiverse` | crates/genesis-multiverse | **Yes** — v0.1.0, 12 downloads |
| 13 | `genesis-experiment` | crates/genesis-experiment | **No** |

**Root package:** `genesis-protocol` (not published)
**Publish stub:** `publish/genesis-multiverse/` (namespace claim only, 4 SLoC)

**Verdict:** 1 of 13 crates published on crates.io.

### 1.2 Test Count

| Source | Count |
|---|---|
| `#[test]` annotations | 373 |
| `#[tokio::test]` annotations | 30 |
| **Total test functions in code** | **403** |
| README claim | 396 |
| crates.io claim | 396 |
| IP_RECORD.md claim | 345 (earlier snapshot) |

**Verdict:** Actual test count is **403**. README/crates.io say 396 (stale by 7). IP_RECORD says 345 (frozen at earlier state, acceptable).

### 1.3 Experiment Counts

#### By experiment directory:

| Season | Directories | Worlds (from hash_registry + calculation) | Epochs |
|---|---|---|---|
| Season 1 | 25 | 4,180 | 2,090,000 |
| Season 2 | 13 | 1,500 | 1,020,000 |
| Sensitivity | 13 | 1,560 (13 × 120) | 780,000 |
| **Total** | **51** | **7,240** | **3,890,000** |

#### Declared in `replication_status.json`:

| Field | Value |
|---|---|
| total_experiments | 44 |
| total_worlds | 6,820 |
| season_1.experiments | 17 |
| season_1.worlds | 3,640 |
| season_2.experiments | 13 |
| season_2.worlds | 1,500 |
| sensitivity.experiments | 14 |
| sensitivity.worlds | 1,680 |

#### Declared in README:

| Field | Value |
|---|---|
| S1 header | "17 experiments, 3,640 worlds" |
| S2 header | "13 experiments, 1,500 worlds" |
| Sensitivity header | "14 configurations, 1,680 worlds" |
| Total | 44 configs, 6,820 worlds, >3,410,000 epochs |

#### Declared in paper (`genesis_protocol_paper.md`):

| Field | Value |
|---|---|
| S1 | "25 experiments, 4,180 worlds" |
| S2 | "13 experiments, 1,500 worlds" |
| Sensitivity | Appendix only |
| Total | "38 experiments, 5,680 worlds, 2,840,000 epochs" |

#### Declared in WHITEPAPER:

| Field | Value |
|---|---|
| Total | "6,820 worlds, 44 configs, ~3,410,000 epochs" |

#### Declared in CITATION.cff:

| Field | Value |
|---|---|
| Experiments | "52 experiment configurations" |
| Worlds | "7,360 independent world simulations" |
| Epochs | "over 3,680,000 computed epochs" |
| Version | "2.0.0" |

#### Declared in .zenodo.json:

| Field | Value |
|---|---|
| Worlds | "7,360 simulated worlds" |
| Epochs | "3,680,000+" |
| Experiments | "52 experiment configurations" |
| Version | "2.0.0" |

### 1.4 Counting Convention Analysis

The experiment count ambiguity arises from two legitimate conventions:

1. **Raw count** (paper convention): Each experiment directory = 1 experiment → 25 S1 + 13 S2 + 13 sensitivity = 51
2. **Grouped count** (README convention): Logical groups → 17 S1 groups + 13 S2 + 14 sensitivity = 44

The README "17 experiments" groups multi-tier experiments (e.g., 4 reserve_* dirs = 1 "Reserve Stress" group). The README header "3,640 worlds" excludes the 4 `fth_reserve_*` experiments (540 worlds), which ARE in the hash_registry.

**Sensitivity discrepancy:** README/replication_status say 14 configs, 1,680 worlds. Actual directories: 13 (5 floor + 8 weight). At 120 worlds each = 1,560 worlds, not 1,680. The 14th config may be the baseline s4_full_attack (already in S2). If counted separately for sensitivity: 14 × 120 = 1,680.

**Grand total discrepancy:**
- replication_status.json math: 3,640 + 1,500 + 1,680 = 6,820 ✓ (internal consistency)
- Actual dirs: 4,180 + 1,500 + 1,560 = 7,240 (raw count is higher due to fth_reserve_* and sensitivity baseline)
- The "6,820" number is computed using the grouped convention and is internally consistent within that convention.

### 1.5 DOI and Publication

| Item | Value | Status |
|---|---|---|
| Canonical DOI | `10.5281/zenodo.18729652` | Resolves ✓ |
| Old DOI (2500-donkeys) | `10.5281/zenodo.18646886` | Referenced in CITATION.cff `references` section (correct as a separate project reference) |
| CITATION.cff DOI | `18729652` | ✓ |
| DOI in README | `18729652` | ✓ |
| DOI in WHITEPAPER | `18729652` | ✓ |
| DOI in paper | `18729652` | ✓ |

All DOI references unified. No stale DOI references remain.

### 1.6 Version Numbers

| Source | Version |
|---|---|
| Git tag | `v0.1.0` |
| Cargo workspace | `0.1.0` |
| CITATION.cff | `2.0.0` ← **MISMATCH** |
| .zenodo.json | `2.0.0` ← **MISMATCH** |
| IP_RECORD.md | `v1.0.0` (frozen bundle, historical) |
| crates.io | `0.1.0` |

### 1.7 Moltbook Post Inventory

| File | Title | Purpose | Status |
|---|---|---|---|
| `collapse_bounty.json` | Bounty spec (JSON) | Machine-readable bounty config | Current |
| `collapse_bounty.md` | Bounty specification | Full bounty rules | Current |
| `collapse_bounty_post.md` | Collapse Bounty — Open Challenge | Public bounty announcement | Current |
| `comment_templates.md` | Comment templates | Engagement reply templates | Current, utility |
| `paper_announcement_post.md` | Formal Specification v1.0 | Paper release announcement | **Stale** — commit hash `1206cff` is old |
| `post_6820_worlds_milestone.md` | 6,820 Worlds | Milestone update | Current |
| `post_bounty_reset.md` | Collapse Bounty — Replication First | Reframed bounty | Current |
| `post_genesis_v3_versioning.md` | Three versions of Genesis Protocol | Version history narrative | **Needs review** — phase names/counts |
| `post_phase4_bridge.md` | The Cliff Is Not a Bug | Phase 4 bridge post | **HOLD** — not yet published |
| `post_phase_transition.md` | The Cliff | Phase transition analysis | **Stale** — "120 worlds" may confuse |

### 1.8 Paper and Research Files

| File | Description | Status |
|---|---|---|
| `papers/genesis_protocol_paper.md` | Formal research paper | Uses 38/5,680/2,840,000 (S1+S2 only). Sensitivity in appendix. Internally consistent. |
| `papers/arxiv/` | LaTeX arXiv submission | 16 pages, compiled |
| `papers/figures/` | Paper figures | Present |
| `papers/known_failure_modes.md` | Known failure modes doc | Current |
| `papers/statistical_validation_report.md` | Statistical validation | Current |
| `papers/technical-disclosure.md` | IP technical disclosure | Current |

---

## SECTION 2 — DRIFT / MISMATCHES

### CRITICAL (publicly visible, factually wrong)

| # | File | Claim | Truth | Impact |
|---|---|---|---|---|
| 1 | `CITATION.cff` L33-34 | "52 experiment configurations...7,360 independent world simulations...3,680,000 computed epochs" | 44 configs, 6,820 worlds, ~3,410,000-3,890,000 epochs | Anyone citing via CFF gets wrong numbers |
| 2 | `.zenodo.json` L3 | "7,360 simulated worlds, 3,680,000+ epochs" | 6,820 worlds | Zenodo metadata wrong if re-deposited |
| 3 | `.zenodo.json` L46 | "52 experiment configurations" | 44 (or 51 raw) | Same |
| 4 | `CITATION.cff` L14 | version: "2.0.0" | `v0.1.0` (git tag + Cargo) | Version mismatch |
| 5 | `.zenodo.json` L45 | version: "2.0.0" | `v0.1.0` | Same |

### HIGH (README-level, user-visible)

| # | File | Claim | Truth | Impact |
|---|---|---|---|---|
| 6 | `README.md` L3 | "396 tests" | 403 test functions | Minor but verifiable |
| 7 | `README.md` L103 | `cargo test --workspace # 396 tests, 0 failures` | 403 tests | Same |
| 8 | `README.md` L131 | "Tests \| 396 passing" | 403 | Same |
| 9 | `README.md` S1 table | World counts don't match hash_registry for several experiments (Inversion=360 vs registry 580, Multi-Axis=240 vs 220, Evolution Forbidden=200 vs 140) | Hash registry is source of truth | Table-level data error |
| 10 | `README.md` S1 header | "17 experiments" but table experiments column sums to 21 | Count convention inconsistency | Confusing |

### MEDIUM (paper vs README divergence)

| # | File | Claim | Truth | Impact |
|---|---|---|---|---|
| 11 | `papers/genesis_protocol_paper.md` L11 | "38 experiments, 5,680 worlds, 2,840,000 epochs" | Correct for S1+S2. Missing sensitivity. | Paper is internally consistent but doesn't cover full scope |
| 12 | `papers/genesis_protocol_paper.md` L323 | "Season 1: 25 experiments, 4,180 worlds" | Correct per registry | Differs from README grouping |
| 13 | `moltbook/paper_announcement_post.md` | Commit ref `1206cff` | HEAD is now `c67de30` | Stale commit hash |
| 14 | `moltbook/post_phase_transition.md` | "120 worlds" baseline context | Ambiguous — 120 per-config or total? | Could confuse readers |
| 15 | `moltbook/post_genesis_v3_versioning.md` | "Phase 2 — 38 configurations, 5,680 worlds" | Earlier total, not a "phase" | Naming confusion with research paper phases |

### LOW (internal docs, IP record)

| # | File | Claim | Truth | Impact |
|---|---|---|---|---|
| 16 | `IP_RECORD.md` L56 | "345 passing tests" "8 completed experiments" "680+ worlds" | Frozen at earlier state | Acceptable — it's a point-in-time snapshot |
| 17 | `IP_RECORD.md` L107 | "License: Proprietary. All rights reserved." | Now MIT (LICENSE file added) | Legal inconsistency |

---

## SECTION 3 — UPDATE TARGETS (Priority Order)

### P0 — Fix immediately (published metadata, legally/scientifically visible)

1. **`CITATION.cff`** — Fix version to `0.1.0`, fix abstract (52→44 configs, 7,360→6,820 worlds, 3,680,000→3,410,000+ epochs)
2. **`.zenodo.json`** — Same fixes: description, notes, version
3. **`IP_RECORD.md` L107** — Update license status from "Proprietary" to "MIT"

### P1 — Fix soon (README accuracy)

4. **`README.md`** — Update test count from 396 to 403 (3 locations)
5. **`README.md`** — Reconcile S1 table world counts with hash_registry, or add note about grouped vs raw counting
6. **`README.md`** — Clarify sensitivity: 13 directories vs 14 claimed configs

### P2 — Fix for consistency (Moltbook, narrative)

7. **`moltbook/paper_announcement_post.md`** — Update commit hash
8. **`moltbook/post_phase_transition.md`** — Clarify "120 worlds" as per-config
9. **`moltbook/post_genesis_v3_versioning.md`** — Review phase/version naming

### P3 — Strategic (paper, scope expansion)

10. **`papers/genesis_protocol_paper.md`** — Consider updating to include sensitivity in the headline numbers (38→44 experiments, 5,680→6,820 worlds) or explicitly state the paper covers S1+S2 only
11. **`replication_status.json`** — Add sensitivity experiments to hash_registry

### P4 — Publish remaining crates

12. Publish remaining 12 crates to crates.io in dependency order:
    1. `genesis-dna` (no internal deps)
    2. `metabolism` (depends on genesis-dna)
    3. `genesis-econometrics` (depends on genesis-dna, metabolism)
    4. `evolution` (depends on genesis-dna, metabolism)
    5. `ecosystem` (depends on genesis-dna, metabolism)
    6. `apostle` (depends on genesis-dna, metabolism)
    7. `genesis-anchor` (depends on genesis-dna, metabolism)
    8. `genesis-homeostasis` (depends on genesis-dna, metabolism, genesis-econometrics)
    9. `genesis-replay` (depends on genesis-dna, metabolism, genesis-anchor)
    10. `genesis-federation` (depends on genesis-dna, metabolism)
    11. `gateway` (depends on genesis-dna, metabolism, apostle, ecosystem)
    12. `genesis-experiment` (depends on gateway, genesis-multiverse, genesis-anchor)

---

## SECTION 4 — MOLTBOOK STRATEGY

### Current State
10 files in `moltbook/`. No explicit ordering, no table of contents, no narrative thread.

### Proposed Narrative Sequence

#### Layer 1 — Foundation
| Order | Post | Action | Current File |
|---|---|---|---|
| 1 | What Genesis Protocol Is | **NEW** — Write a clean explainer post | (does not exist) |
| 2 | Why Deterministic Simulation Matters | **NEW** — From WHITEPAPER abstract, distilled | (does not exist) |

#### Layer 2 — Milestones & Results
| Order | Post | Action | Current File |
|---|---|---|---|
| 3 | Three Phases of Genesis | **REVISE** | `post_genesis_v3_versioning.md` |
| 4 | 6,820 Worlds | **KEEP** | `post_6820_worlds_milestone.md` |

#### Layer 3 — Validation & Failure Modes
| Order | Post | Action | Current File |
|---|---|---|---|
| 5 | The Cliff (Phase Transition) | **REVISE** — clarify 120 per-config | `post_phase_transition.md` |
| 6 | Formal Specification Published | **REVISE** — update commit hash | `paper_announcement_post.md` |

#### Layer 4 — Replication & Proof
| Order | Post | Action | Current File |
|---|---|---|---|
| 7 | Collapse Bounty | **KEEP** | `collapse_bounty_post.md` |
| 8 | Bounty Reset: Replication First | **KEEP** | `post_bounty_reset.md` |

#### Layer 5 — Bridge to Infrastructure
| Order | Post | Action | Current File |
|---|---|---|---|
| 9 | The Cliff Is Not a Bug | **HOLD** — publish when ready | `post_phase4_bridge.md` |

#### Utility (not in narrative)
| File | Action |
|---|---|
| `collapse_bounty.json` | **KEEP** — machine-readable spec |
| `collapse_bounty.md` | **KEEP** — full spec |
| `comment_templates.md` | **KEEP** — engagement tool |

### Missing Posts to Complete the Story
1. **"What Genesis Protocol Is"** — 200-word clean explainer. No jargon. What it does, why it exists, what it proved.
2. **"Why Deterministic Simulation Matters"** — The case for bit-exact replay, hash-chain integrity, independent verification.
3. **"Genesis in the Stack"** — Where this sits in Burns Infrastructure. How simulation connects to policy, anchoring, capital systems.

---

## SECTION 5 — DRAFTED CHANGES

### 5.1 CITATION.cff (Priority P0)

**Replace abstract (lines 28-37):**
```yaml
abstract: >-
  Deterministic multi-agent macroeconomic simulation engine implemented as a
  thirteen-crate Rust workspace. Heterogeneous agents with SHA-256 derived
  genomes extract resources from logistic niche pools, pay metabolic costs,
  reproduce conditionally based on a four-trait fitness function, and face
  stochastic catastrophes. A redistributive treasury, homeostatic parameter
  controller, and bounded genetic mutation engine provide adaptive capacity.
  44 experiment configurations across three research phases produced 6,820
  independent world simulations totaling over 3,410,000 computed epochs.
  Under the default collapse definition, no collapses were observed. Under
  stricter definitions, collapse rates exceed 97%. Sensitivity analyses
  characterize the collapse boundary as a sharp phase transition dependent
  on the floor definition. All seeds, configurations, and result hashes
  are published for independent replication.
```

**Replace version (line 14):**
```yaml
version: "0.1.0"
```

### 5.2 .zenodo.json (Priority P0)

**Replace description (line 3):**
```json
"description": "Deterministic multi-agent macroeconomic simulation engine. 13 Rust crates, 403 tests, 6,820 simulated worlds, 3,410,000+ epochs. Heterogeneous agents with SHA-256 derived genomes extract resources from logistic niche pools, reproduce conditionally, and face stochastic catastrophes. Sensitivity analyses characterize a sharp collapse boundary dependent on the floor definition."
```

**Replace notes (line 46):**
```json
"notes": "44 experiment configurations across three research phases (parameter sweeps, structural invariant violations, sensitivity analysis). All seeds, configurations, and SHA-256 result hashes published for independent replication."
```

**Replace version (line 45):**
```json
"version": "0.1.0"
```

### 5.3 README.md test count (Priority P1)

Change all three occurrences of "396" to "403":
- Line 3: "13 Rust crates. 403 tests."
- Line 103: `cargo test --workspace  # 403 tests, 0 failures`
- Line 131: "Tests | 403 passing, 0 failed"

### 5.4 IP_RECORD.md license (Priority P0)

**Replace line 107:**
```
- **License:** MIT (open source, see LICENSE file)
```

### 5.5 moltbook/paper_announcement_post.md (Priority P2)

**Replace commit reference:**
```
- Commit: `c67de30`
```

---

## SECTION 6 — ECOSYSTEM POSITIONING

### Genesis Protocol in the Burns Infrastructure Stack

```
┌─────────────────────────────────────────────────┐
│              BURNS INFRASTRUCTURE                │
│                                                  │
│  ┌───────────────────────────────────────────┐  │
│  │            Policy & Capital Layer          │  │
│  │  FTHUSD · Reserve Management · Compliance │  │
│  └────────────────────┬──────────────────────┘  │
│                       │                          │
│  ┌────────────────────▼──────────────────────┐  │
│  │         Simulation & Validation Layer      │  │
│  │  ┌─────────────────────────────────────┐  │  │
│  │  │        GENESIS PROTOCOL             │  │  │
│  │  │  Deterministic ABM engine           │  │  │
│  │  │  6,820 world simulations            │  │  │
│  │  │  Dual-chain integrity               │  │  │
│  │  │  Bit-exact replay                   │  │  │
│  │  │  Policy stress testing              │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  └────────────────────┬──────────────────────┘  │
│                       │                          │
│  ┌────────────────────▼──────────────────────┐  │
│  │          Anchoring & Proof Layer           │  │
│  │  SHA-256 state chains · BLAKE3 genome     │  │
│  │  Zenodo DOI · IPFS CID · Hash registry    │  │
│  └────────────────────┬──────────────────────┘  │
│                       │                          │
│  ┌────────────────────▼──────────────────────┐  │
│  │          Infrastructure Layer              │  │
│  │  Docker · PostgreSQL · Redis · API        │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### Canonical Description (for website, README preamble, or any summary)

> **Genesis Protocol** is a deterministic multi-agent economic simulation engine built in Rust. It models heterogeneous agents that extract resources, pay metabolic costs, reproduce conditionally, and face stochastic catastrophes within a closed economy. Every state transition is cryptographically seeded, producing bit-identical results on the same architecture.
>
> The engine has produced 6,820 independent world simulations across 44 experiment configurations in three research phases. Under the default collapse definition, zero collapses were observed. Under stricter definitions, a sharp phase transition produces near-universal collapse — the "cliff" that defines the system's stability boundary.
>
> Genesis Protocol serves as the simulation and validation layer for Burns Infrastructure: a platform for testing economic policies, reserve stress scenarios, and systemic resilience under controlled, reproducible conditions. All results are anchored via dual SHA-256/BLAKE3 hash chains and published with open replication protocols.

### What Genesis Does (functional capabilities)

1. **Deterministic simulation** — Hash-chain seeded, bit-exact replay from any checkpoint
2. **Policy stress testing** — Parameter sweeps across economic variables, structural invariant removal
3. **Collapse boundary mapping** — Sensitivity analysis across definition parameters
4. **Cryptographic anchoring** — Dual-chain integrity (SHA-256 state + BLAKE3 genome)
5. **Replay verification** — Any world can be replayed from seed, verified against published hashes
6. **Open replication** — All configs, seeds, and result hashes published. Bounty active.

---

*Generated 2026-03-17 from Genesis Protocol repository at commit `c67de30`.*
