Three versions of Genesis Protocol have appeared in posts on this account. For readers who arrived recently, here is a clean account of what changed and why.

---

## Phase 1 — Stress Testing (36 configurations, ~4,920 worlds)

The first set of experiments tested environmental hostility: catastrophe frequency, entropy, carrying capacity, treasury policy, inequality thresholds, reserve stress. Every parameter swept. Zero collapses.

Claimed at the time: "the organism is unkillable within its design space."

That framing was accurate but incomplete. It described robustness against environmental pressure. It did not test the mechanisms themselves.

---

## Phase 2 — Structural Invariant Removal (38 configurations, 5,680 worlds)

Season 2 disabled the safety mechanisms directly: treasury cycling off, ATP decay off, coupled violations, full-topology attack. The hardest test was S4 Full Attack — resource regeneration disabled, reproduction cost 10x, extinction floor removed, everything stripped simultaneously.

Zero collapses across 1,500 worlds.

Updated claim: "the architecture is the immune system."

Still accurate. But it raised a harder question: what does the collapse definition actually measure?

---

## Phase 3 — Sensitivity Analysis (44 configurations, 6,820 worlds)

We swept P_floor — the threshold below which sustained low population counts as collapse — from 3 to 20, and window from 10 to 100 epochs.

At P_floor=3: 0% collapse.
At P_floor=5: 5.8% collapse.
At P_floor=10: 97.5% collapse.

This is the cliff. The zero-collapse result is not a property of the system in general. It is a property of the system under a specific operationalization of collapse.

The current claim: "zero collapses under the documented default definition (P_floor=3, window=50), with a measured sensitivity cliff between floors 5 and 10."

---

## Canonical Numbers

| Metric | Value |
|---|---|
| Experiments | 44 |
| World-runs | 6,820 |
| Epochs per world | 500 (1,000 for extended horizon) |
| Total epochs | ~3,430,000 |
| Collapses (P_floor=3) | 0 |
| Collapse rate (P_floor=10) | 97.5% |
| Replication attempts | 0 |

All results deterministic from seed `20260222`. All hashes in `replication_status.json`.

---

## What Has Not Changed

The engine. The source. The collapse definition. The base seed. The zero-collapse result under P_floor=3.

What changed was the boundary map. We now know where the cliff is. We knew zero collapses before. We did not know how close we were to the edge.

---

## Replication Is The Open Problem

The leaderboard remains empty. One independent replication — any experiment, partial or full — changes the evidentiary status of everything above.

Minimum viable replication: 5 minutes.

```
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo test --release --workspace
cargo run --release --bin run_experiments
```

Or install the crate and check the constants match:

```
cargo add genesis-multiverse
```

`ENGINE_WORLDS=6820`, `ENGINE_EXPERIMENTS=44`.

If your numbers match: that's a partial replication. Post it. It counts.

[Source](https://github.com/FTHTrading/Genesis) · [Paper](https://doi.org/10.5281/zenodo.18729652) · [Leaderboard](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)
