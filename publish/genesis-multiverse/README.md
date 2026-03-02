# genesis-multiverse

Multiverse-scale systemic experiment engine.

Deterministic simulation across thousands of parallel worlds with phase transition detection and collapse boundary mapping.

## What This Is

Genesis Multiverse is a research-grade simulation ecosystem built in Rust. It runs deterministic macroeconomic experiments across large parameter spaces, identifies systemic phase transitions, and maps collapse boundaries with statistical precision.

## Current Scale

| Metric | Value |
|---|---|
| Parallel worlds | 6,820 |
| Configurations | 44 |
| Engine crates | 13 |
| Test count | 396 |
| P_floor=3 collapse rate | 0% |
| Phase transition zone | floors 5–10 |

## Architecture

The engine is a multi-crate Rust workspace:

- `genesis-dna` — core agent representation
- `genesis-experiment` — experiment orchestration
- `genesis-homeostasis` — equilibrium modeling
- `genesis-multiverse` — world-sweep engine
- `genesis-econometrics` — statistical analysis
- `genesis-anchor` — deterministic hash anchoring
- `genesis-replay` — deterministic replay engine
- `genesis-federation` — multi-chain coordination

## The Cliff

At P_floor=3, zero collapses across 6,820 worlds.  
At P_floor=5–10, a phase transition emerges — systemic instability crosses threshold.  
This is "The Cliff."

## Replication Challenge

The replication challenge is open. Canonical data and methodology are publicly available.

Full engine: <https://github.com/FTHTrading/Genesis>

## License

MIT
