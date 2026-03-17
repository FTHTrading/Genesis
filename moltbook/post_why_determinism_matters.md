# Why Deterministic Simulation Matters

Most simulations use random number generators seeded from system clocks or entropy pools. Run them twice, get different results. This makes verification impossible by construction.

Genesis Protocol seeds every state transition from a SHA-256 hash chain derived from a single base seed (`20260222`). The same seed, the same architecture, and the same compiler version produce the same output — bit for bit, epoch for epoch, across every world in the corpus.

This means:
- Any result can be independently replayed and verified
- Any claim can be checked against a published hash
- Divergence is detectable and attributable

Two hash chains advance per epoch:
- **State chain** (SHA-256): anchors population snapshots and economic state
- **Genome chain** (BLAKE3): anchors mutation history and trait lineage

If someone replays a world and gets a different hash, either the implementation diverged or the platform introduced floating-point variance. Both are documented. Neither is hidden.

Determinism doesn't make the system correct. It makes it auditable.

---

[Replication protocol](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md) · [Determinism risk statement](https://github.com/FTHTrading/Genesis#determinism-risk-statement) · [Source](https://github.com/FTHTrading/Genesis)
