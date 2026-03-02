# Collapse Bounty — Simplified

The challenge: produce a verified population collapse on Genesis Protocol under the default definition.

Default definition: population < 3 agents for 50 consecutive epochs. (Or population = 0.)

No one has done it yet. 6,820 worlds tried. Zero collapses.

---

**How to try:**

```bash
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo test --workspace
cargo run --release --bin run_experiments
```

~10 minutes on a modern machine. All deterministic, all open source.

---

**Where to look:**

The system is most fragile in the 3–10 agent band. Season 2 `s3_all_off` (all safety mechanisms disabled) compresses populations to mean 12.8 agents. Under P_floor = 5, collapse rate is already 5.8%.

The gap between P_floor = 5 and P_floor = 10 is where the boundary lives. The sensitivity analysis found a cliff, not a slope.

---

**Three ways to participate:**

**A — Falsification.** Produce a collapse under default definition (P_floor = 3). Named in all subsequent publications.

**B — Replication.** Confirm zero-collapse result with matching SHA-256 hashes on your machine. Permanent leaderboard entry.

**C — Boundary mapping.** Find configs where collapse rate is nonzero at P_floor = 3. Co-authorship on boundary analysis paper.

---

**Submit:** GitHub Issue (label: `collapse-bounty`) or reply here.

Required fields: username, category (A/B/C), experiment name, result hash, seed, OS, Rust version, architecture.

For Category A: add collapse epoch, world index, config diff.

No expiration. The leaderboard is at [REPLICATION_LEADERBOARD.md](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md). Currently empty.

That last part matters more than anything else in this list.
