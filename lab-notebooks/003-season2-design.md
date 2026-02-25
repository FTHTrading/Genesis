# Lab Notebook 003: Season 2 Design Rationale

**Date**: 2026-02
**Author**: Kevan Burns

## Motivation

Season 1 produced zero collapses across 4,180 worlds. The obvious critique: "The system has too many safety mechanisms. Of course it doesn't collapse."

Season 2 was designed to directly address this by asking: **what happens when we remove the safety mechanisms?**

## Stage Design

The stages follow a **progressive stripping** pattern:

1. **S1**: Remove treasury redistribution only. Does the economy function without redistribution?
2. **S2**: Remove ATP decay only. What happens when currency doesn't decay?
3. **S3**: Remove combinations of decay, treasury, grants, extinction floor. Factorial coverage.
4. **S4**: Remove everything AND add environmental hostility (zero resource regeneration, death drains pools, 10× reproduction cost).

Each stage also includes **hostile variants** that additionally disable mutation and cortex and maximize catastrophe frequency.

## Key Design Choice: The S4 Full Attack Configuration

This is the most extreme combination we could construct:
- Zero resource regeneration (pool starts full but never refills)
- Agent death drains ATP from resource pools (positive feedback loop toward depletion)
- All safety mechanisms disabled (no treasury, no decay, no grants, no extinction floor)
- 10× reproduction cost (250 ATP vs. default 25 ATP)
- Mutation disabled, cortex disabled

**Rationale**: If the system survives this, survival under the default definition is established across the full tested range. If it doesn't, we know the breaking point.

**Result**: Mean population contracted to 12.8 agents. Minimum observed: 3 (exactly at the floor). Zero collapses under default definition. 97.5% collapse under P_floor = 10.

## Observation: Pathological Survival States

The most important qualitative finding of Season 2 was not about collapse — it was about **what survival looks like under extreme conditions**.

S4 full attack populations exhibit:
- Reproductive inequality: 0.952 (top fitness quartile monopolizes reproduction)
- Survival inequality: 0.873 (bottom quartile concentrates deaths)
- Wealth concentration: 0.417

This is a degenerate state. A tiny oligarchy reproduces while the majority die. The population "survives" only in the narrowest definitional sense.

**Honest assessment**: These pathological states arguably matter more than the collapse rate. A system that "survives" by producing a reproductive oligarchy with 95% inequality is not exhibiting healthy dynamics. The known failure modes document addresses this directly.

## Observation: Extended Horizon Shows Continued Existence, Not Recovery

S4 extended horizon (5,000 epochs) shows mean population 26.0 with minimum 7. This is higher than the 500-epoch full attack (12.8 mean) because the extended horizon uses a less extreme configuration (zero regen + death sink but not all S3 mechanisms off).

**Interpretation**: Populations do not recover to baseline over longer time horizons. They find a lower equilibrium and stay there. Whether this represents genuine stability or slow secular decline that would manifest at 50,000 epochs is unknown.

## Post-Season 2 Decision: Sensitivity Analysis

After Season 2, the zero-collapse result held but was clearly fragile. The next obvious question was: **how sensitive is the result to the collapse definition itself and to the fitness weights?**

This led directly to the floor sensitivity (Appendix C) and weight perturbation (Appendix D) experiments.
