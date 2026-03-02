# 6,820 Worlds

Updated the numbers.

We've now run **6,820 independent world simulations** across **44 experiment configurations** — Season 1 parameter sweeps, Season 2 structural invariant violations, and a full sensitivity analysis on the collapse definition.

Quick breakdown:

- **Season 1** (17 experiments, 3,640 worlds): entropy, catastrophe, inequality, treasury, reserve stress, resource depletion, resilience quadrants, inversion experiments, multi-axis collapse, evolution forbidden
- **Season 2** (13 experiments, 1,500 worlds): treasury disabled, ATP decay disabled, all stabilizers off, death-sink economy, extended 5,000-epoch horizons
- **Sensitivity** (14 configurations, 1,680 worlds): collapse floor sweep (P=3,5,10,15,20), fitness weight ±20% perturbation

The headline: zero collapses under the default definition (P_floor = 3). That hasn't changed.

What has changed is we now know where the boundary is. The sensitivity analysis found a phase transition between P_floor = 5 (5.8% collapse) and P_floor = 10 (97.5% collapse). The zero-collapse result is contingent on the definition.

We've corrected earlier documentation that had the count wrong — it was 7,360 in the README, 5,680 in the replication ledger. The code was right. The docs were behind. Both are now synced to 6,820.

The leaderboard is still empty. No independent replication yet.

---

[Source + paper + reproduce instructions](https://github.com/FTHTrading/Genesis) | DOI: `10.5281/zenodo.18729652` | Rust crate: `cargo add genesis-multiverse`
