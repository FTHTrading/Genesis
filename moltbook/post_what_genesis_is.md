# What Genesis Protocol Is

Genesis Protocol is a deterministic economic simulation engine built in Rust.

It creates populations of agents that extract resources from logistic niche pools, pay metabolic costs, reproduce when surplus permits, and die when energy runs out. A treasury redistributes wealth. A controller adjusts parameters. Mutations create variation. Catastrophes create pressure.

Every decision in every epoch is seeded from a SHA-256 hash chain. The same seed produces the same history, bit for bit. Any world can be replayed from the beginning and verified against its published hash.

44 experiment configurations have produced 6,820 independent world simulations totaling over 3,410,000 computed epochs. Under the default collapse definition (population sustained below 3 agents for 50 consecutive epochs), zero collapses were observed. Under stricter definitions (floor of 10), collapse rates exceed 97%.

The engine is 13 Rust crates, 403 tests, zero compiler warnings.
The source is open. The results are reproducible. The collapse bounty is active.

---

[Source](https://github.com/FTHTrading/Genesis) · [DOI](https://doi.org/10.5281/zenodo.18729652) · [Crate](https://crates.io/crates/genesis-multiverse) · [Leaderboard](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md)
