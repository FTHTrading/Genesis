# Formal Specification: Genesis Protocol Simulation Engine v1.0

The full technical paper is now available.

---

## What it covers

**Title:** Deterministic Multi-Agent Economic Simulation Under Structural Invariant Violations: Collapse Boundary Analysis

**Scope:** 44 experiments, 6,820 world-runs, 3,410,000 computed epochs. Three phases: Season 1 parameter sweeps, Season 2 structural invariant violations, and Sensitivity Analysis.

**Key results documented:**

- Under the default collapse definition (P_floor = 3, window = 50 epochs), zero collapses observed across all configurations
- 95% Clopper-Pearson confidence interval: [0, 0.065%]
- Under P_floor = 10, collapse rates exceed 97% — a discontinuous phase transition in the definition parameter
- Under maximal structural violation (all safety mechanisms disabled), populations contracted to mean 12.8 agents but did not reach extinction
- Fitness weight perturbation of ±20% produces ≤0.8pp collapse rate change

**What the paper does NOT claim:**

- Does not claim the system is immune to collapse
- Does not claim the tested parameter space is exhaustive
- Explicitly documents known failure modes and the sparse coverage of the configuration space
- States that independent replication has not occurred

---

## Where to find it

- **PDF:** [papers/arxiv/main.tex](https://github.com/FTHTrading/Genesis/tree/main/papers/arxiv) — 16 pages, compiled from source
- **Source code:** [github.com/FTHTrading/Genesis](https://github.com/FTHTrading/Genesis)
- **Collapse definition:** [COLLAPSE_DEFINITION.md](https://github.com/FTHTrading/Genesis/blob/main/COLLAPSE_DEFINITION.md)
- **Replication protocol:** Section 7 of the paper, or [REPLICATION_LEADERBOARD.md](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)

---

## Version

- Engine: 13 crates, 396 tests, 26,158 source lines
- Commit: `1206cff`
- Base seed: `20260222`
- All result hashes published in `replication_status.json`

---

This is documentation, not argument. The data is reproducible. The source is open. The collapse bounty is active.
