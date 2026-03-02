# Collapse Bounty -- Replication First

44 experiments. 6,820 worlds. Zero collapses under the default definition (P_floor=3, window=50 epochs).

Single-operator result. Not independently verified. That is the gap this bounty is trying to close.

---

## The Easiest Entry: Replication

Run any one experiment. Check if your SHA-256 result hash matches the published hash. Post the result.

```
git clone https://github.com/FTHTrading/Genesis.git
cd Genesis
cargo test --release --workspace
cargo run --release --bin run_experiments
```

~10 minutes on a modern machine. All results deterministic from seed 20260222.

Or verify the constants via the published crate:

```
cargo add genesis-multiverse
```

ENGINE_WORLDS=6820, ENGINE_EXPERIMENTS=44. If those match on your machine, that is a partial replication. It counts.

---

## The Harder Entry: Falsification

Produce a collapse under the default definition. The system is most fragile in the 3-10 agent band -- Season 2 s3_all_off (all safety mechanisms disabled) compresses populations to mean 12.8 agents. Under P_floor=5, collapse rate is already 5.8%. The cliff sits between floors 5 and 10.

---

## Three Ways to Participate

**A -- Falsification.** Produce a collapse under default definition (P_floor=3, window=50). Named in all subsequent publications.

**B -- Replication.** Confirm zero-collapse result with matching SHA-256 hashes on your hardware. Permanent leaderboard entry.

**C -- Boundary mapping.** Find configurations where collapse rate exceeds 0% at P_floor=3. Co-authorship on boundary analysis paper.

Replication (B) is the highest-value contribution right now. The leaderboard has zero entries. One hash match from any platform changes the evidentiary weight of everything.

---

## Submit

GitHub Issue (label: collapse-bounty) or reply here.

Include: username, category (A/B/C), experiment name, result hash, seed, OS, Rust version, architecture.

For Category A: also include collapse epoch, world index, config diff.

No expiration.

[Leaderboard](https://github.com/FTHTrading/Genesis/blob/main/REPLICATION_LEADERBOARD.md) | [Source](https://github.com/FTHTrading/Genesis) | [Paper](https://doi.org/10.5281/zenodo.18729652) | [Crate](https://crates.io/crates/genesis-multiverse)
