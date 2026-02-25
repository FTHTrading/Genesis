# Collapse Definition — Genesis Protocol

**Version:** 1.0  
**Date:** February 23, 2026  
**Reference Implementation:** [`runner.rs`](crates/genesis-experiment/src/runner.rs) lines 227–253  
**Whitepaper Section:** §8.6.1

---

## Formal Definition

A civilization **collapses** if either condition is met during a simulation run:

### Condition 1 — Extinction

$$P(t) = 0$$

Population reaches zero at any epoch $t$. The simulation terminates immediately.

### Condition 2 — Functional Extinction

$$P(t) < P_{\text{floor}} \quad \text{for} \quad N_{\text{consec}} \text{ consecutive epochs}$$

Population falls below a survival floor and fails to recover within a defined window. The simulation terminates when the streak threshold is reached.

---

## Parameters

| Parameter | Symbol | Value | Justification |
|---|---|---|---|
| Survival floor | $P_{\text{floor}}$ | 3 agents | Minimum for demographic replacement: 1 parent per birth, births capped at 3/epoch. Population of 2 cannot sustain itself if one agent dies before reproducing. |
| Recovery window | $N_{\text{consec}}$ | 50 epochs | Provides ample time for treasury-assisted recovery if structurally possible. At 1 birth/epoch, 50 epochs would produce 50 births from a floor of 3 — if recovery doesn't happen in 50 epochs, it is not structurally possible under current conditions. |

---

## Implementation (Rust)

From `crates/genesis-experiment/src/runner.rs`:

```rust
const EXTINCTION_FLOOR: usize = 3;
const EXTINCTION_WINDOW: u64 = 50;
let mut below_floor_streak: u64 = 0;

for epoch_num in 0..config.epochs_per_run {
    let stats = world.run_epoch();

    // Condition 1: Hard extinction
    if world.agents.is_empty() {
        collapse_epoch = Some(epoch_num + 1);
        break;
    }

    // Condition 2: Functional extinction
    if world.agents.len() < EXTINCTION_FLOOR {
        below_floor_streak += 1;
        if below_floor_streak >= EXTINCTION_WINDOW {
            collapse_epoch = Some(epoch_num + 1 - EXTINCTION_WINDOW + 1);
            break;
        }
    } else {
        below_floor_streak = 0;
    }
}
```

---

## What Collapse Is NOT

The following conditions are **not** collapse under this definition:

| Condition | Why Not Collapse |
|---|---|
| Population decline (e.g., 50 → 17 agents) | Population stabilized above floor |
| High Gini coefficient (e.g., 0.95) | Inequality is a quality metric, not a survival metric |
| Role extinction (e.g., no Strategists) | Remaining roles sustain population |
| Low birth rate | If deaths also decrease, equilibrium persists |
| Treasury accumulation without redistribution | Agents survive on extraction alone |
| Mean fitness < 0.35 | Selection still operates; low average does not prevent reproduction by fit individuals |

These may indicate **economic degeneracy** — a pathological but survivable state — which is a separate classification from collapse.

---

## Alternative Definitions (Open Questions)

This definition is a design choice, not a natural law. Alternative collapse criteria that an independent replicator might reasonably propose:

| Alternative | Definition | Effect on Results |
|---|---|---|
| Stricter floor | $P_{\text{floor}} = 10$ | Would flag more experiments as collapsed |
| Shorter window | $N_{\text{consec}} = 10$ | Would catch brief population dips |
| Demographic collapse | Birth/death ratio < 0.5 for 100 epochs | Would catch aging populations with no renewal |
| Economic collapse | Mean ATP < basal cost for 50 epochs | Would catch energy starvation before population loss |
| Role collapse | Fewer than 3 of 5 roles present for 100 epochs | Would catch monoculture failure modes |
| Gini collapse | Gini > 0.95 for 100 epochs | Would catch extreme inequality even if population persists |

If you disagree with the current definition, propose your alternative and re-run the experiments. The seeds are deterministic. The results will differ only where your definition draws a different line.

---

## Verification

To verify collapse detection:

```bash
git clone https://github.com/FTHTrading/Genesis.git
cd AI
cargo test --release -p genesis-experiment
```

The test suite includes experiments specifically designed to approach (but not cross) the collapse boundary. All 4,920 worlds across 36 configurations produced `collapse_epoch: None`.

---

## Citation

If you use or contest this definition:

```
Burns, K. (2026). Genesis Protocol: Autonomous Metabolic Organism
with Survival Economics. doi:10.5281/zenodo.18646886
```
