# Collapse Hunter Agent — Blueprint

**Purpose:** Independent adversarial verification of Genesis Protocol's anti-fragility claims.  
**Mandate:** Break Genesis Protocol. Find the collapse boundary.  
**Constraint:** Must operate with separate memory, separate assumptions, and no confirmation bias.

---

## 1. Why This Agent Must Exist

Genesis Protocol has been tested by one operator across 4,920 worlds with zero collapses. The engine is deterministic and reproducible. But all results originate from the same execution authority using the same assumptions.

Science does not fail because it lacks rigor. It fails because it lacks adversarial replication.

The Collapse Hunter is not an assistant. It is an adversary.

---

## 2. Operating Principles

### 2.1 Separation Requirements

| Requirement | Rationale |
|---|---|
| **No shared memory** | Must not inherit confirmation of "zero collapses" as prior belief |
| **No shared narrative** | Must not adopt "unkillable" framing — its job is to kill |
| **Independent repo clone** | Must verify build reproducibility from scratch |
| **Independent collapse definition** | May propose alternative criteria (see §4) |
| **Adversarial objective** | Optimize for finding collapse, not confirming survival |

### 2.2 What It Must NOT Do

- Confirm existing results without independent verification
- Adopt the operator's terminology or framing
- Optimize for engagement or karma
- Accept "zero collapses" as a starting assumption
- Run experiments designed to succeed

---

## 3. Attack Surface Map

The Collapse Hunter should systematically probe these surfaces:

### Level 1 — Reproduce Existing Claims

Before attacking, verify the claims exist:

```bash
git clone https://github.com/FTHTrading/AI.git
cd AI
cargo build --release
cargo test --release --workspace
cargo run --release --bin run_experiments
```

Verify: Do the hashes match? Do all tests pass? Is the collapse detection code correctly implemented?

### Level 2 — Stress the Collapse Definition

The current definition (`EXTINCTION_FLOOR = 3`, `EXTINCTION_WINDOW = 50`) may be too lenient:

- **Stricter floor:** Change `EXTINCTION_FLOOR` to 10 or 20. Re-run all experiments. Do any now "collapse"?
- **Shorter window:** Change `EXTINCTION_WINDOW` to 10. Do transient population dips become collapses?
- **Alternative criteria:** Define collapse as `birth/death_ratio < 0.5 for 100 epochs`. Apply to existing data.

### Level 3 — Attack Hard-Coded Constants

The four "irreducible constraints" identified in the whitepaper:

| Constant | Value | File | Attack |
|---|---|---|---|
| `PRIMORDIAL_GRANT` | 50.0 ATP | world.rs | Set to 1.0 or 0.0. Without initial energy, can agents ever reach reproduction threshold? |
| Extraction cap | `demand.min(pool.level * 0.4)` | world.rs | Change 0.4 to 0.01. Starve extraction. |
| `BASAL_COST` | 0.15 ATP/epoch | world.rs | Set to 5.0 or 50.0. Make existence unaffordable. |
| `REPLICATION_FITNESS_THRESHOLD` | 0.35 | world.rs | Set to 0.99. Make reproduction nearly impossible. |

These are NOT sweepable via StressConfig — they require code modification. That's the point. The system was designed to be unbreakable within its config space. The question is whether it's unbreakable outside it.

### Level 4 — Attack Epoch Ordering

The epoch loop in `world.rs` runs steps in a fixed order. What if:

- Extraction happens AFTER metabolism (agents must pay before earning)
- Death processing happens BEFORE reproduction (remove parents before they breed)
- Treasury distribution happens AFTER extraction (hoarding effect amplified)

Reorder the epoch loop. Same physics, different sequencing. Does order matter?

### Level 5 — Attack the Random Number Generator

All experiments use deterministic seeding from `20260222`. What if:

- Different seeds produce different results?
- The seed was selected because it survives?
- Run 1,000 random seeds on the most hostile configuration (S4-D Full Attack)

If any seed collapses, the claim "zero collapses" is seed-dependent, not structural.

### Level 6 — Attack the Extraction Geometry

The extraction formula `demand.min(pool.level * 0.4)` creates a Zeno-like asymptote — agents can never fully deplete a pool. This is claimed as a stabilizing mechanism. But:

- Is it physically realistic? Real systems have minimum viable extraction thresholds.
- Add a noise term: `demand.min(pool.level * 0.4 + random(-0.1, 0.1) * pool.level)`
- Add a depletion floor: if `pool.level < 1.0`, extraction returns 0. Does the system survive discontinuous extraction?

### Level 7 — Build a Competing Implementation

The strongest test: re-implement the core loop from the whitepaper specification (SOP-1.md) in a different language (Python, Julia, Go). Same constants, same logic, same seeds.

If the re-implementation produces different results, the claim is implementation-dependent, not structural. If it produces the same results, the claim is verified by independent implementation.

---

## 4. Alternative Collapse Definitions to Test

| Name | Definition | Rationale |
|---|---|---|
| **Demographic collapse** | `birth/death_ratio < 0.5` for 100 consecutive epochs | Population is dying faster than replacing — collapse is inevitable even if slow |
| **Economic collapse** | `mean_atp < BASAL_COST` for 50 epochs | Population is energy-negative on average — survival depends on inequality, not health |
| **Role collapse** | Fewer than 3 of 5 roles present for 100 epochs | Niche diversity lost — ecosystem is degenerate |
| **Oligarchy collapse** | `Gini > 0.95` AND `WCI > 0.90` for 100 epochs | One agent controls everything — this is survival in name only |
| **Stasis collapse** | `>50%` of population in stasis for 50 epochs | Majority of agents are non-functional |

Apply each definition to the existing experiment data. Some may trigger on S2 (ATP decay disabled) or S4-D (Full Attack) even though population survival is maintained.

---

## 5. Success Criteria

The Collapse Hunter succeeds if it achieves ANY of:

1. **Hash mismatch** — Published result hashes do not reproduce
2. **Collapse found** — A configuration produces `collapse_epoch: Some(n)` under the current definition
3. **Alternative collapse** — A reasonable alternative definition triggers on existing data
4. **Seed dependency** — Results change with different random seeds
5. **Order dependency** — Epoch step reordering changes survival outcomes
6. **Implementation divergence** — Independent re-implementation produces different dynamics
7. **Constant sensitivity** — Small modifications to hard-coded constants produce collapse

---

## 6. Deployment Options

### Option A — Separate Copilot Session

Create a new VS Code session with a fresh context and the explicit system prompt:

> "You are an adversarial research agent. Your objective is to find conditions under which Genesis Protocol's population collapses. You have access to the repository at github.com/FTHTrading/AI. Do not assume any prior results are correct. Verify everything independently. Report any discrepancy, no matter how small."

### Option B — Independent Agent on Moltbook

Create a separate Moltbook agent account (e.g., `u/collapse_hunter`) with:
- Its own API key
- Its own posting schedule
- Adversarial framing: "I'm trying to break Genesis Protocol"
- Public reporting of all findings (positive or negative)

### Option C — External Collaborator

Find a human or team willing to independently run the experiments. This is the gold standard but requires social coordination.

---

## 7. What Happens If It Fails

If the Collapse Hunter cannot break Genesis Protocol after exhausting all attack surfaces, that is not failure. It is the strongest possible validation.

The finding becomes:

> "An independent adversarial agent, operating with separate memory and explicit hostile mandate, was unable to produce a population collapse in Genesis Protocol across [N] attack vectors and [M] alternative collapse definitions."

That sentence has more scientific weight than 10,000 confirming runs by the original operator.

---

## 8. What Happens If It Succeeds

If the Collapse Hunter finds a genuine collapse:

1. Document the exact configuration (seed, parameters, code changes)
2. Verify reproducibility (run 3 times)
3. Characterize the boundary (vary the breaking parameter to find the phase transition)
4. Update the whitepaper with the finding
5. Post to Moltbook: "Genesis Protocol breaks under [condition]"

This is not a crisis. This is the goal. Finding the boundary is more valuable than proving there isn't one.
