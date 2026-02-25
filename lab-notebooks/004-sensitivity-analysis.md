# Lab Notebook 004: Sensitivity Analysis Design

**Date**: 2026-02
**Author**: Kevan Burns

## Motivation

Two adversarial critiques prompted this work:

1. **"Your collapse definition is too permissive."** — Response: vary the definition and document what happens.
2. **"Your fitness weights are tuned to produce zero collapses."** — Response: perturb the weights and show results are stable.

## Floor Sensitivity Design

### Choice of Floor Values: 3, 5, 10, 15, 20

- **3**: Default definition. Included for completeness.
- **5**: Slightly stricter. Tests whether any populations dip transiently below 5 for sustained periods.
- **10**: The critical threshold. Most s4_full_attack populations stabilize in the 3–8 range, so floor = 10 should capture most or all of them.
- **15, 20**: Confirmation that larger populations are not sustained under full attack conditions.

### Why s4_full_attack as the test configuration

This is the most extreme configuration — if floor sensitivity exists, it will be most pronounced here. Testing floor sensitivity under benign conditions (where populations stabilize at 48–49) would show no sensitivity and would mislead.

### Result: Phase Transition Discovery

The jump from 5.8% (floor = 5) to 97.5% (floor = 10) was not anticipated. I expected a gradual increase. Instead, the data shows a sharp phase transition, indicating that the population equilibrium under full attack is concentrated in a narrow band (3–8 agents) rather than being broadly distributed.

**This was the single most important finding of the sensitivity analysis.** It fundamentally recharacterizes the system: not "stable everywhere" but "poised at a narrow equilibrium that falls entirely within or entirely outside the collapse definition depending on where you draw the line."

## Weight Perturbation Design

### Choice of ±20% Perturbation

20% was chosen as a "meaningful but not structural" perturbation. It changes the relative ordering of low-weighted traits slightly but preserves the qualitative structure (SQ remains highest for all perturbations).

**Acknowledged limitation**: This is a local sensitivity analysis. Qualitatively different weight structures (e.g., uniform weights, single-trait dominance) are untested. A reviewer could legitimately ask for broader coverage.

### Why Only One Weight Perturbed at a Time

Combinatorial explosion: 4 weights × {+20%, -20%} = 8 variants. Joint perturbation (all pairs, triples, quadruples) would require 2^8 - 1 = 255 combinations × 120 worlds = 30,600 additional worlds. This was deemed too expensive for the marginal information gain.

**Acknowledged limitation**: Interaction effects between weights are uncharacterized.

### Result: RF Sensitivity

The only collapses occurred in CE +20% and RF -20% — both of which reduce the effective weight of Resource Foraging. This identifies RF as the critical survival-related trait under zero-regeneration conditions.

**Interpretation**: When resources don't regenerate, the ability to efficiently extract remaining resources becomes the bottleneck. Reducing selection pressure for foraging (by deprioritizing it in fitness) allows low-foraging agents to survive and reproduce, collectively depleting resources faster.

This is a mechanistically sensible result, not an artifact.
