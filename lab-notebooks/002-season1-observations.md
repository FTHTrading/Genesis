# Lab Notebook 002: Season 1 Observations

**Date**: 2026-01 through 2026-02
**Author**: Kevan Burns

## Observation: Population Convergence to ~48–49

Across all baseline Season 1 experiments (entropy sweep, catastrophe, inequality, treasury), mean populations converge to a narrow band of 48–49 agents regardless of the swept parameter.

**Interpretation**: The logistic soft cap (default 180) combined with resource competition, metabolic costs, and the reproduction threshold creates an effective carrying capacity far below the soft cap. This emergent equilibrium is robust to parameter perturbation within tested ranges.

**Concern**: This convergence might be over-determined by the cortex controller. Season 2 was designed partly to address this.

## Observation: Gini Sensitivity to Tax Threshold

The widest Gini range (0.49–0.72) occurs in the inequality threshold sweep. At threshold 0.90 (laissez-faire), Gini reaches 0.72 — substantial inequality but not extreme. At threshold 0.20 (aggressive redistribution), Gini compresses to ~0.55.

**Interpretation**: The tax mechanism is effective at compressing inequality but cannot drive Gini below ~0.50 because fitness-based extraction inherently produces differential income.

## Observation: Resource Depletion Pinning

Under severe resource scarcity (carrying capacity = 30), population pins to ~30 agents. This is exactly the carrying capacity.

**Interpretation**: Under scarcity, the population is supply-limited rather than demand-limited. The system finds the resource-constrained equilibrium efficiently.

**Concern**: This is very close to the point where further reduction would push toward collapse. At carrying capacity = 20, can the system maintain above P_floor = 3? Not tested.

## Observation: Metabolic Inversion Produces Massive Population Reduction

At 10× basal cost, population drops from ~49 to ~21. Cohen's d = 12.6.

**Interpretation**: The system is extremely sensitive to metabolic parameters. The population contracts rather than collapsing, finding a lower equilibrium — but the magnitude of the contraction (57% population loss) is striking.

**Question for future work**: Is there a metabolic cost threshold above which collapse occurs? The sweep only goes to 10×. At 20× or 50×, collapse seems likely but is untested.

## Observation: Zero Collapses in All 4,180 Worlds

No Season 1 experiment produced a collapse under the default definition.

**Initial reaction**: Encouraging but potentially too good. This raised the question of whether the system is over-engineered for survival (multiple redundant mechanisms making collapse nearly impossible by design).

**Response**: Season 2 was designed to answer this by systematically removing mechanisms. The fact that Season 2 also produced zero collapses (under default definition) while producing pathological states and near-floor populations provided a more nuanced answer.
