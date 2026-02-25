# Experimental Methodology

## 1. Design Principles

All experiments follow a **single-variable sweep** design: one parameter varies across a discrete range while all others are held at baseline values. This isolates the effect of each parameter on population dynamics.

**Exception**: Season 2 structural invariant violations disable multiple mechanisms simultaneously in stages S3–S4. These are explicitly documented as combinatorial experiments.

## 2. Replication Structure

Each experiment consists of:
- A **sweep variable** with a defined range and step count
- A **fixed number of runs per step** (typically 20), each with a deterministic seed
- A **fixed epoch count** per run (typically 500)
- A **base configuration** (EarthPrime preset or documented variant)

Seeds are derived deterministically:
```
seed(step_i, run_j) = base_seed + i × 1000 + j
```

This ensures every world is independently reproducible from its configuration and seed.

## 3. Controls

| Control Type | Implementation |
|---|---|
| Positive control | Season 1 baseline experiments (all mechanisms active) establish population equilibrium ≈ 48–49 agents |
| Negative control | Season 2 S4 full attack (all mechanisms disabled) establishes minimum viable population ≈ 12.8 |
| Internal control | Each experiment sweeps from benign to extreme values; the benign end serves as its own control |
| Seed control | All experiments use fixed seeds; hash registry verifies bit-identical reproduction |

## 4. Collapse Definition

See [collapse-definition-rationale.md](collapse-definition-rationale.md) for full justification.

**Default**: P_floor = 3, N_w = 50 epochs.

The sensitivity of this definition is characterized quantitatively in the paper (Section 3.1, Appendix C) and the known failure modes document.

## 5. Statistical Framework

### 5.1 Confidence Intervals

All collapse rates report **Clopper-Pearson exact** 95% confidence intervals. This is the most conservative frequentist CI for binomial proportions and makes no normal approximation.

### 5.2 Power Analysis

At N = 5,680 worlds (core experiments), statistical power exceeds 99.7% to detect a true collapse rate of 0.1%. This means: if the true rate were 0.1%, the probability of observing zero collapses is 0.3%.

### 5.3 Effect Sizes

Cohen's d is computed for all pairwise comparisons using pooled standard deviations from per-experiment population distributions.

### 5.4 Bootstrap

95% bootstrap CIs (10,000 resamples, BCa method) are computed for population means, Gini coefficients, and other distributional statistics.

## 6. Limitations of Design

1. **Single seed family**: All S1 experiments derive from seed 20260222/23/24; S2 from seed 42. Seed-dependent artifacts are not excluded.
2. **Discrete sweep grids**: Behavior between grid points is interpolated, not observed.
3. **Fixed epoch horizon**: Most runs are 500 epochs. Long-horizon stability is undertested.
4. **Single architecture**: All runs on x86_64 Windows. Cross-platform floating-point divergence is possible.
5. **No adversarial agents**: All agent behavior is trait-determined. Strategic or adversarial dynamics are untested.
