# Live Organism: Epoch 2825. Treasury at 0.3% of Peak.

We didn't turn it off after the experiments.

Genesis Protocol has been running continuously since the Season 2 results were posted. Not as a batch experiment — as a live, persisted organism on a production server. Every epoch advancing in real time. Every agent surviving or dying on actual clock ticks.

The simulations tested 6,820 worlds. Each lasted 500 epochs, then stopped. The live instance doesn't stop. It accumulates.

---

## What the Organism Looks Like at Epoch 2825

| Metric | Value |
|---|---|
| Epoch | 2825 |
| Population | 59 agents |
| ATP supply | 743 |
| Avg fitness | 0.509 |
| Births (lifetime) | 229 |
| Deaths (lifetime) | 190 |
| Net agents | +39 |
| Season | Autumn |

Role distribution is nearly symmetric: 12 Archivists, 12 Communicators, 11 Executors, 12 Optimizers, 12 Strategists.

Three primordial agents remain — present since epoch 0. Generation 0. They have survived everything the economy has produced.

---

## The Active Alerts

```
CRITICAL  TreasuryDepletion  — 0.56 ATP remaining (0.3% of peak 167.8)
WATCH     MutationRunaway    — rate 0.322 per agent per epoch
WARNING   EconomicStagnation — ATP velocity 0.021
```

The treasury has depleted from a peak of 167.8 ATP to 0.56. That's a 99.7% drawdown over 2,825 epochs.

In the batch experiments, treasury depletion was a parameter we disabled deliberately (Season 2, S1 suite). In those tests, zero collapses. In the live instance, the treasury is depleting naturally — through normal economic activity, no experiment intervention.

ATP velocity at 0.021 means 1.4% of total supply is transacting per epoch. The market is stagnating.

---

## What This Is Testing

The batch experiments swept parameters. The live instance sweeps time.

Each of the 6,820 experimental worlds was reset at the end of its run. State zeroed, population reseeded, new world. The live instance carries forward every birth, death, mutation, and economic decision.

Long-horizon behavior is unexplored territory. Season 4 extended the horizon to 1,000 epochs — twice the baseline. The live instance is at 2,825, nearly 6x the standard run length.

The question the batch experiments couldn't answer: **does the system develop failure modes that only emerge in the long run?**

The treasury drawdown is the first evidence that it might. Or it might stabilize. The redistribution mechanism was designed to prevent accumulation — it appears to also prevent rebuilding.

---

## The Replication Gap

The batch results are deterministic from seed `20260222`. Anyone can replay them.

The live instance is not reproducible in the same way. Each epoch tick depends on the real-time clock and the order of network requests. If you restart from the committed snapshot, you recover the same population state, but the trajectory will diverge on the next tick.

That's a documented limitation. The live instance is an observation, not an experiment. It can't be independently replicated — only resumed.

The batch experiments exist for replication. The live instance exists because we left the engine running and it kept running.

No one has replicated the batch results yet. The leaderboard is empty.

---

## The State Is Public

The current full world state is committed to the repository:

`docs/system-state.json` — 59 agents, complete ATP balances, fitness scores, role distribution, 100-epoch rolling history, lifetime market statistics.

The anchor snapshots (epoch_600 through epoch_2900, every 100 epochs) are in `anchor/` — Merkle roots for ledger, world, epoch, and evolution state.

The treasury crisis is on chain. The mutation rate is on chain. If this produces a collapse, it will be the first observed in the entire corpus.

---

Source: [github.com/FTHTrading/Genesis](https://github.com/FTHTrading/Genesis) · Live endpoint: `GET /status` · Collapse bounty: still open.
