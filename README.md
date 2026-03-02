# Genesis Protocol

Deterministic multi-agent economic simulation engine. 13 Rust crates. 396 tests. 6,820 simulated worlds. Zero compiler warnings.

## Abstract

Genesis Protocol is a closed-economy agent-based simulation in which heterogeneous agents extract resources from logistic niche pools, pay metabolic costs, reproduce conditionally based on a four-trait fitness function, and face stochastic catastrophes. A redistributive treasury, homeostatic parameter controller (Cortex), and bounded genetic mutation engine provide adaptive capacity. Every state transition is deterministically seeded via SHA-256 hash chains and Knuth MMIX LCG, producing bit-identical results on the same architecture.

44 experiment configurations across three research phases (Season 1, Season 2, Sensitivity) have produced 6,820 independent world simulations totaling over 3,410,000 computed epochs. Under the default collapse definition ($P_{\text{floor}} = 3$, 50-epoch sustained window), no collapses were observed. Under stricter definitions ($P_{\text{floor}} = 10$), collapse rates exceed 97%. The zero-collapse result is contingent on the permissive default definition, the presence of a world-level extinction floor mechanism, and multi-layer engineered stabilization. The global stability boundary remains an open problem.

## Architecture

| Layer | Crate | Purpose |
|---|---|---|
| Identity | `genesis-dna` | SHA-256 genome derivation, four-trait phenotype, bounded mutation |
| Economics | `metabolism` | ATP energy ledger, treasury, metabolic decay |
| Econometrics | `genesis-econometrics` | Gini coefficient, wealth distribution, inequality indices |
| Evolution | `evolution` | Selection pressure, conditional replication, horizontal gene transfer |
| Population | `ecosystem` | Social mesh, niche pools, carrying capacity, telemetry |
| Regulation | `genesis-homeostasis` | Adaptive Cortex — hand-engineered feedback controller |
| Multiverse | `genesis-multiverse` | Parallel world instantiation, parameter sweep orchestration |
| Experiments | `genesis-experiment` | Experiment engine, configurable runner, statistical reporting |
| Cryptography | `genesis-anchor` | Dual-chain anchoring (SHA-256 state + BLAKE3 genome) |
| Replay | `genesis-replay` | Deterministic replay from any checkpoint |
| Federation | `genesis-federation` | Cross-instance communication protocol |
| Gateway | `gateway` | HTTP API, SSE event stream, stress testing |
| Recruitment | `apostle` | Outbound integration |

### Dual-Chain Integrity

Two independent hash chains advance per epoch:

- **State Chain** (SHA-256): `H(prev_state_hash ‖ epoch ‖ population_snapshot)`
- **Genome Chain** (BLAKE3): `H(prev_genome_hash ‖ mutated_genomes)`

Divergence between chains is detectable. Replay integrity is verifiable to any depth.

## Experiment Summary

### Season 1 — Parameter Sweeps (17 experiments, 3,640 worlds)

| Domain | Experiments | Worlds | Key Finding |
|---|---|---|---|
| Entropy Sweep | 1 | 200 | 10× metabolic cost increase → 4.9% Gini increase. No collapses. |
| Catastrophe Resilience | 1 | 140 | 0–3% catastrophe probability → deaths scale linearly, population declines 4.6% |
| Inequality Threshold | 1 | 160 | Tax threshold 0.20–0.90 → 31.6% Gini increase, population invariant |
| Treasury Stability | 1 | 180 | Reserve threshold sweep → <1% Gini variation across all policies |
| Reserve Stress (4 tiers) | 4 | 540 | Optimal threshold shifts +0.60 under escalating shock rates |
| Resource Depletion (4 tiers) | 4 | 600 | Carrying capacity compression from 200 to 30 |
| Resilience Quadrants | 4 | 880 | Cortex immunity × genetic immunity factorial design |
| Inversion Experiments | 3 | 360 | Basal, dual, and metabolic inversion sweeps |
| Multi-Axis Collapse | 1 | 240 | Combined stressor escalation |
| Evolution Forbidden | 1 | 200 | Mutation disabled: population persists on initial genome |

### Season 2 — Structural Invariant Violations (13 experiments, 1,500 worlds)


Systematically disabled treasury redistribution, ATP decay, resource regeneration, and reproduction grants — individually and in combination.

| Configuration | Key Result |
|---|---|
| Treasury disabled (baseline + hostile) | Population contracts, persists |
| ATP decay disabled (baseline + hostile) | Immortality → resource exhaustion pressure |
| All stabilizers disabled | Population ≈ 12.8, reproductive inequality 0.95 |
| Death-sink economy | Resources drain on death, population compressed |
| Extended horizon (1000 epochs) | Late-stage dynamics observable |

### Sensitivity Analysis (14 configurations, 1,680 worlds)

| Test | Result |
|---|---|
| Collapse floor = 3 (default) | 0% collapse |
| Collapse floor = 5 | 5.8% collapse |
| Collapse floor = 10 | 97.5% collapse |
| Collapse floor = 15 | 100% collapse |
| Collapse floor = 20 | 100% collapse |
| Fitness weights ±20% (8 variants) | Max 0.8 pp collapse rate change |

Sharp phase transition between floor = 5 and floor = 10. The zero-collapse headline is definition-dependent.

## Known Limitations

1. Collapse definition uses a permissive default ($P_{\text{floor}} = 3$)
2. World-level extinction floor mechanism prevents populations below 3 from reaching zero
3. Cortex is a hand-engineered feedback controller, not a learned or emergent system
4. Fitness weights (0.25, 0.30, 0.20, 0.25) are fixed a priori, not optimized
5. Cross-platform determinism depends on floating-point architecture (verified on x86_64 Windows only)
6. No Lyapunov stability proof exists — stability is empirical within tested parameter ranges
7. Statistical reporting uses aggregate per-parameter-value summaries, not per-world time series
8. The global stability boundary in full parameter space is not characterized
9. Independent replication has not occurred
10. Multi-layer engineered redundancy makes it difficult to isolate which mechanisms are necessary vs. sufficient

## Reproduce

```bash
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo build --release
cargo test --workspace                            # 396 tests, 0 failures
cargo run --release --bin run_experiments         # Season 1: 17 experiments
cargo run --release --bin s1_treasury_disabled    # Season 2-S1: treasury
cargo run --release --bin s2_atp_decay_disabled   # Season 2-S2: ATP decay
cargo run --release --bin s3_coupled_violations   # Season 2-S3: coupled
cargo run --release --bin s4_topology_violations  # Season 2-S4: topology
cargo run --release --bin sensitivity_analysis    # Sensitivity: 14 configs
```

Each experiment outputs:
- **CSV** — per-parameter-value aggregate metrics (mean, stddev, min, max, p10, p90)
- **JSON manifest** — configuration, seed, SHA-256 result hash
- **Text report** — human-readable summary

All results are seeded from `20260222`. Deterministic on the same architecture.

## Metrics

| Metric | Value |
|---|---|
| Crates | 13 |
| Tests | 396 passing, 0 failed |
| Experiment configurations | 44 |
| World simulations | 6,820 |
| Computed epochs | > 3,410,000 |
| Collapses (default definition) | 0 |
| Collapses (floor ≥ 10) | > 97% |
| Rust edition | 2021 |
| Deterministic seed | `20260222` |

## Research Paper

[Deterministic Multi-Agent Economic Simulation Under Structural Invariant Violations: Collapse Boundary Analysis](papers/genesis_protocol_paper.md) — formal specification, experiment methodology, sensitivity analysis, and limitations.

## DOI

`10.5281/zenodo.18729652`

## Citation

```bibtex
@software{burns2026genesis,
  author  = {Burns, Kevan},
  title   = {Genesis Protocol: Deterministic Multi-Agent Economic Simulation},
  year    = {2026},
  doi     = {10.5281/zenodo.18729652},
  url     = {https://github.com/FTHTrading/Genesis}
}
```

## License

MIT

---

*Kevan Burns — [FTH Trading](https://github.com/FTHTrading)*
