# Current Canon — Genesis Protocol (2026-03-29)

This post supersedes all previous numerical claims. If a number in an older post conflicts with this one, this post is correct.

---

## Engine

| Component | Value |
|---|---|
| Crates | 15 |
| Tests | 403 total (396 passing, 7 long-run validations) |
| Source lines | 26,581+ Rust (91+ files) |
| Build | Zero compiler errors, zero compiler warnings |
| Clippy | ~56 style suggestions (advisory, no correctness issues) |
| Version | 0.1.0 |
| Commit | `366134d` |
| Rust edition | 2021 |
| Verified toolchain | 1.77 (canonical), 1.93 (tested compatible) |

## New Since 2026-03-17

| Addition | Detail |
|---|---|
| `genesis-x402` crate | x402 payment layer (EIP-3009 USDC micropayments for agent actions) |
| `genesis-ledger` crate | In-house triple-write engine: pricing, accounts, journal, lineage, batch |
| x402PaymentAdapter on Polygon | `0xe25d0C100a98D2004e3CC81b081492Bb3D102a91` |
| GenesisToken (WORLD) on Polygon | `0x14E64b91B96f11D12ef6bDaDc21e2f25a2f45a99` |
| Treasury.sol on Polygon | `0x17A2d219A1C5b7aF2890aFAf6E7045669Dc96952` |
| Encoding fix | All 5 corrupted markdown files corrected (commit `366134d`) |

## Live Organism State (last confirmed: 2026-03-23)

| Metric | Value |
|---|---|
| Epoch | 52,375 |
| Population | 58 agents |
| Season | Winter |
| Avg fitness | 0.552 |
| Total ATP | 667 |
| Treasury reserve | 0.55 ATP |
| Risk status | Stable |
| Primordial agents | 0 — all founding agents have died |
| Total births (lifetime) | 1,831 |
| Total deaths (lifetime) | 1,793 |
| Net agents | +38 |
| Market solutions | 122,209 |
| Oldest living agent | born epoch ~1,118 (survived 51,257 epochs) |

## Experiments

| Phase | Configurations | Worlds | Key Result |
|---|---|---|---|
| Season 1 — Parameter Sweeps | 17 | ~3,640 | Zero collapses across all environmental stress vectors |
| Season 2 — Structural Violations | 13 | 1,500 | Zero collapses with all safety mechanisms disabled |
| Sensitivity — Collapse Definition | 14 | 1,680 | Phase transition between P_floor=5 (5.8%) and P_floor=10 (97.5%) |
| **Total** | **44** | **6,820** | |

## Headline Result

Under the default collapse definition (P_floor = 3, sustained window = 50 epochs): **zero collapses** across 6,820 worlds.

95% Clopper-Pearson confidence interval: [0, 0.054%].

Under P_floor = 10: **97.5% collapse rate**. The boundary is sharp, measured, and sits at the definition layer.

## Determinism

- Base seed: `20260222`
- Hash derivation: SHA-256 chain (state) + BLAKE3 chain (genomes)
- Pseudorandom: Knuth MMIX LCG
- Canonical platform: Windows x86_64, Rust 1.77
- Cross-platform determinism is an open validation axis

All result hashes are published in `replication_status.json`. The hash registry covers all 38 Season 1 + Season 2 experiments (5,680 worlds). Sensitivity experiment hashes are declared in `replication_status.json` totals but individual entries are pending.

## Replication Status

| Metric | Value |
|---|---|
| Independent replications | 0 |
| Leaderboard entries | 0 |
| Cross-platform verifications | 0 |

The leaderboard is empty. This is the single largest credibility gap.

Minimum viable replication:

```
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo test --release --workspace
cargo run --release --bin run_experiments
```

~10 minutes. Any experiment. Any platform. If your hash matches, post it. If it doesn't, that's more valuable — post the diff.

## Published Artifacts

| Artifact | Location |
|---|---|
| Source code | [github.com/FTHTrading/Genesis](https://github.com/FTHTrading/Genesis) |
| DOI | `10.5281/zenodo.18729652` |
| crates.io | [`genesis-multiverse`](https://crates.io/crates/genesis-multiverse) v0.1.1 (namespace stub) |
| Paper | [papers/genesis_protocol_paper.md](https://github.com/FTHTrading/Genesis/blob/main/papers/genesis_protocol_paper.md) |
| Collapse definition | [COLLAPSE_DEFINITION.md](https://github.com/FTHTrading/Genesis/blob/main/COLLAPSE_DEFINITION.md) |
| Replication protocol | [REPLICATION_LEADERBOARD.md](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md) |
| Counting conventions | [docs/CANONICAL_COUNTS.md](https://github.com/FTHTrading/Genesis/blob/main/docs/CANONICAL_COUNTS.md) |

## Historical Context

Genesis Protocol has executed over 30,000 historical world-runs across development, stress testing, reruns, and sensitivity campaigns. The current canonical public replication corpus contains 6,820 worlds across 44 grouped experiment configurations.

## What This Post Is

A snapshot. Not an argument. Every number here is derivable from the source code and experiment output. If any number drifts after this date, `docs/CANONICAL_COUNTS.md` in the repository is the authoritative reference.

---

[Source](https://github.com/FTHTrading/Genesis) · [DOI](https://doi.org/10.5281/zenodo.18729652) · [Leaderboard](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)
