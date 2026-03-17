# genesis-multiverse

Public crate namespace for the **Genesis Protocol** simulation ecosystem.

This crate is a canonical entry point — it publishes authoritative metadata constants and links to the full 13-crate engine. It is not the engine itself.

## What This Crate Is For

- **Namespace ownership** for the `genesis-multiverse` package on crates.io
- **Canonical constants** — experiment totals, world counts, test counts, crate count
- **On-ramp** into the full repository, paper, and replication protocol

## What This Crate Does NOT Contain

The simulation engine, experiment runner, deterministic replay, econometric analysis, and all runtime code live in the full workspace:

**Full engine → <https://github.com/FTHTrading/Genesis>**

## Current Scale (full engine)

| Metric | Value |
|---|---|
| World simulations | 6,820 |
| Experiment configurations | 44 |
| Engine crates | 13 |
| Tests | 403 (396 passing, 7 long-run validations) |
| Source lines | 26,581 Rust |
| Collapses at P_floor=3 | 0 |
| Phase transition zone | floors 5–10 |
| DOI | `10.5281/zenodo.18729652` |

## Constants Exported

```rust
pub const ENGINE_WORLDS: u32 = 6820;
pub const ENGINE_EXPERIMENTS: u32 = 44;
pub const ENGINE_CRATES: u32 = 13;
pub const ENGINE_TESTS: u32 = 403;
```

## Replication Challenge

The replication challenge is open. Clone the repo, run the experiments, compare hashes.

```bash
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo test --release --workspace
cargo run --release --bin run_experiments
```

Full protocol: [REPLICATION_LEADERBOARD.md](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)

## Links

- [Repository](https://github.com/FTHTrading/Genesis)
- [Paper](https://github.com/FTHTrading/Genesis/blob/main/papers/genesis_protocol_paper.md)
- [DOI](https://doi.org/10.5281/zenodo.18729652)
- [Canonical Counts](https://github.com/FTHTrading/Genesis/blob/main/docs/CANONICAL_COUNTS.md)

## License

MIT
