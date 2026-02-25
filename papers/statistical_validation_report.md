# Statistical Validation Report

**Genesis Protocol — Computational Statistical Analysis**

Generated: 2026-02-24 23:52:45

---

## 1. Dataset Overview

| Metric | Value |
|---|---|
| Total experiments analyzed | 51 |
| Season 1 experiments | 25 |
| Season 2 experiments | 13 |
| Sensitivity experiments | 13 |
| Total worlds (all phases) | 7,240 |
| Total epochs (all phases) | 3,620,000 |
| Core worlds (S1+S2, default definition) | 5,680 |
| Core collapsed worlds | 0 |

## 2. Collapse Rate — Binomial Confidence Intervals

### 2.1 Core Experiments (Season 1 + Season 2)

Under the default collapse definition ($P_{\text{floor}} = 3$, 50-epoch window):

- **Observed**: 0 collapses in 5,680 worlds
- **Point estimate**: $\hat{p} = 0.000000$
- **95% CI (Clopper-Pearson)**: [0.000000, 0.000649]
- **Rule of three**: 95% upper bound ≤ 0.000528 (3/5,680)
- **Method**: Clopper-Pearson exact (k=0)

With 5,680 independent worlds and zero observed collapses, we can state with 95% confidence that the true collapse probability under the default definition is at most **0.0006** (i.e., less than 0.06%).

### 2.2 Per-Season Breakdown

| Season | N | Collapses | $\hat{p}$ | 95% CI Lower | 95% CI Upper |
|---|---|---|---|---|---|
| Season 1 | 4,180 | 0 | 0.000000 | 0.000000 | 0.000882 |
| Season 2 | 1,500 | 0 | 0.000000 | 0.000000 | 0.002456 |
| Combined | 5,680 | 0 | 0.000000 | 0.000000 | 0.000649 |

### 2.3 Collapse Definition Sensitivity

| Floor ($P_{\text{floor}}$) | N | Collapses | $\hat{p}$ | 95% CI |
|---|---|---|---|---|
| 3 | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| 5 | 120 | 7 | 0.0583 | [0.0238, 0.1165] |
| 10 | 120 | 117 | 0.9750 | [0.9287, 0.9948] |
| 15 | 120 | 120 | 1.0000 | [0.9697, 1.0000] |
| 20 | 120 | 120 | 1.0000 | [0.9697, 1.0000] |

### 2.4 Fitness Weight Perturbation

| Perturbation | N | Collapses | $\hat{p}$ | 95% CI |
|---|---|---|---|---|
| cc-20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| cc+20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| ce-20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| ce+20% | 120 | 1 | 0.0083 | [0.0002, 0.0456] |
| rf-20% | 120 | 1 | 0.0083 | [0.0002, 0.0456] |
| rf+20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| sq-20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |
| sq+20% | 120 | 0 | 0.0000 | [0.0000, 0.0303] |

## 3. Statistical Power Analysis

Given $N$ zero-collapse worlds, what is the power to detect a true collapse rate of $p$?

Using the criterion 'reject $H_0: p=0$ if at least one collapse observed':

Power = $1 - (1-p)^N$

| True $p$ | N = 4180 | N = 5680 | N = 7240 |
|---|---|---|---|
| 1% | 1.0000 | 1.0000 | 1.0000 |
| 2% | 1.0000 | 1.0000 | 1.0000 |
| 5% | 1.0000 | 1.0000 | 1.0000 |
| 10% | 1.0000 | 1.0000 | 1.0000 |

With 5,680 core worlds, we have >99% power to detect a true collapse rate of 1% or higher. Even a 0.1% true collapse rate would be detected with probability 0.997.

## 4. Aggregate Population Statistics

### 4.1 Per-Experiment Summary (Core)

| Experiment | Worlds | Pop Mean | Pop Std | Pop Min | Pop Max | Gini Mean | Collapse |
|---|---|---|---|---|---|---|---|
| s1/basal_inversion | 200 | 40.0 | 3.8 | 31 | 52 | 0.7186 | 0.000 |
| s1/catastrophe_resilience | 140 | 52.6 | 1.1 | 47 | 57 | — | 0.000 |
| s1/dual_inversion | 200 | 20.9 | 3.2 | 14 | 32 | 0.7281 | 0.000 |
| s1/entropy_sweep | 200 | 53.2 | 0.8 | 47 | 57 | 0.5597 | 0.000 |
| s1/evolution_forbidden | 140 | 52.0 | 1.0 | 47 | 58 | 0.5481 | 0.000 |
| s1/fth_reserve_calm | 135 | 53.5 | 0.5 | 47 | 57 | 0.5453 | 0.000 |
| s1/fth_reserve_crisis | 135 | 50.9 | 0.6 | 46 | 56 | 0.5560 | 0.000 |
| s1/fth_reserve_moderate | 135 | 53.3 | 0.6 | 47 | 58 | 0.5447 | 0.000 |
| s1/fth_reserve_stressed | 135 | 52.7 | 0.7 | 48 | 57 | 0.5593 | 0.000 |
| s1/inequality_threshold | 160 | 53.2 | 0.4 | 47 | 58 | 0.6068 | 0.000 |
| s1/metabolic_inversion | 180 | 31.9 | 10.8 | 17 | 55 | 0.5621 | 0.000 |
| s1/multi_axis_collapse | 220 | 46.5 | 5.9 | 28 | 55 | 0.7383 | 0.000 |
| s1/reserve_calm | 135 | 53.6 | 0.5 | 47 | 57 | 0.5421 | 0.000 |
| s1/reserve_crisis | 135 | 50.9 | 0.8 | 46 | 56 | 0.5564 | 0.000 |
| s1/reserve_moderate | 135 | 53.3 | 0.9 | 46 | 57 | 0.5458 | 0.000 |
| s1/reserve_stressed | 135 | 52.7 | 0.6 | 47 | 57 | 0.5534 | 0.000 |
| s1/resilience_q1_both | 220 | 51.4 | 2.0 | 46 | 58 | 0.5588 | 0.000 |
| s1/resilience_q2_immune_only | 220 | 51.7 | 1.8 | 46 | 57 | 0.5575 | 0.000 |
| s1/resilience_q3_genetic_only | 220 | 50.9 | 1.8 | 47 | 57 | 0.5650 | 0.000 |
| s1/resilience_q4_static | 220 | 51.1 | 1.6 | 45 | 57 | 0.5673 | 0.000 |
| s1/resource_depletion_abundant | 150 | 52.8 | 0.7 | 47 | 57 | 0.5538 | 0.000 |
| s1/resource_depletion_constrained | 150 | 51.8 | 0.6 | 45 | 56 | 0.5537 | 0.000 |
| s1/resource_depletion_normal | 150 | 52.7 | 1.0 | 46 | 58 | 0.5574 | 0.000 |
| s1/resource_depletion_scarce | 150 | 30.7 | 0.3 | 28 | 32 | 0.5059 | 0.000 |
| s1/treasury_stability | 180 | 53.5 | 0.5 | 47 | 58 | 0.5463 | 0.000 |
| s2/s1_treasury_disabled_baseline | 120 | 46.2 | 7.7 | 29 | 54 | 0.6403 | 0.000 |
| s2/s1_treasury_disabled_hostile | 120 | 44.0 | 7.3 | 27 | 52 | 0.7260 | 0.000 |
| s2/s2_atp_decay_disabled_baseline | 120 | 53.2 | 10.9 | 30 | 59 | 0.3745 | 0.000 |
| s2/s2_atp_decay_disabled_hostile | 120 | 50.4 | 10.2 | 28 | 58 | 0.6031 | 0.000 |
| s2/s3_all_off | 120 | 44.6 | 7.5 | 28 | 52 | 0.5506 | 0.000 |
| s2/s3_decay_floor_off | 120 | 49.8 | 10.0 | 28 | 58 | 0.5897 | 0.000 |
| s2/s3_decay_grants_off | 120 | 47.8 | 9.0 | 28 | 56 | 0.5857 | 0.000 |
| s2/s3_decay_treasury_off | 120 | 49.2 | 9.6 | 28 | 57 | 0.6693 | 0.000 |
| s2/s4_death_sink | 120 | 44.4 | 8.0 | 23 | 54 | 0.7287 | 0.000 |
| s2/s4_extended_horizon | 60 | 26.5 | 0.6 | 22 | 31 | 0.4924 | 0.000 |
| s2/s4_full_attack | 120 | 8.1 | 0.4 | 3 | 12 | 0.5921 | 0.000 |
| s2/s4_zero_regen_death_sink | 120 | 19.3 | 0.4 | 14 | 24 | 0.5294 | 0.000 |
| s2/s4_zero_regeneration | 120 | 19.0 | 0.7 | 6 | 23 | 0.5411 | 0.000 |

### 4.2 Grand Aggregates

- **Grand mean final population** (across 38 experiments): 45.27
- **Std across experiment means**: 11.94
- **95% Bootstrap CI**: [41.45, 49.08]
- **Global minimum final population** (any world): 3
- **Global maximum final population** (any world): 59

- **Grand mean Gini coefficient**: 0.5798
- **Gini range across experiments**: [0.3745, 0.7383]

## 5. Effect Sizes — Parameter Impact on Final Population

Cohen's d between first and last parameter value within each Season 1 experiment:

| Experiment | Pop (low param) | Pop (high param) | |d| | Magnitude |
|---|---|---|---|---|
| s1/metabolic_inversion | 49.1 | 20.7 | 12.569 | large |
| s1/multi_axis_collapse | 29.5 | 49.0 | 11.493 | large |
| s1/basal_inversion | 48.2 | 35.2 | 5.902 | large |
| s1/dual_inversion | 28.4 | 17.1 | 4.463 | large |
| s1/resilience_q1_both | 53.7 | 48.0 | 3.457 | large |
| s1/resilience_q4_static | 53.0 | 48.8 | 2.073 | large |
| s1/resilience_q2_immune_only | 53.5 | 49.0 | 1.845 | large |
| s1/resilience_q3_genetic_only | 52.4 | 48.5 | 1.811 | large |
| s1/catastrophe_resilience | 54.0 | 50.6 | 1.591 | large |
| s1/evolution_forbidden | 52.8 | 50.1 | 1.262 | large |
| s1/entropy_sweep | 54.1 | 51.7 | 1.170 | large |
| s1/reserve_moderate | 54.0 | 52.5 | 0.782 | medium |
| s1/fth_reserve_moderate | 53.7 | 52.7 | 0.505 | medium |
| s1/fth_reserve_calm | 54.0 | 53.1 | 0.474 | small |
| s1/inequality_threshold | 53.5 | 52.5 | 0.452 | small |
| s1/resource_depletion_constrained | 51.8 | 51.1 | 0.404 | small |
| s1/fth_reserve_stressed | 52.5 | 51.8 | 0.384 | small |
| s1/treasury_stability | 53.4 | 54.1 | 0.343 | small |
| s1/reserve_calm | 53.3 | 53.9 | 0.260 | small |
| s1/reserve_crisis | 50.4 | 50.9 | 0.227 | small |
| s1/fth_reserve_crisis | 50.5 | 50.1 | 0.200 | small |
| s1/resource_depletion_normal | 52.9 | 52.4 | 0.183 | negligible |
| s1/resource_depletion_scarce | 30.5 | 30.4 | 0.122 | negligible |
| s1/reserve_stressed | 52.8 | 52.6 | 0.101 | negligible |
| s1/resource_depletion_abundant | 52.5 | 52.5 | 0.000 | negligible |

## 6. Sensitivity Analysis — Detailed Statistics

### 6.1 Collapse Floor Phase Transition

The collapse rate exhibits a sharp phase transition between $P_{\text{floor}} = 5$ and $P_{\text{floor}} = 10$:

- **Floor = 3**: 0.0% collapse (95% CI [0.0%, 3.0%])
- **Floor = 5**: 5.8% collapse (95% CI [2.4%, 11.6%])
- **Floor = 10**: 97.5% collapse (95% CI [92.9%, 99.5%])
- **Floor = 15**: 100.0% collapse (95% CI [97.0%, 100.0%])
- **Floor = 20**: 100.0% collapse (95% CI [97.0%, 100.0%])

Phase transition magnitude: collapse rate increases from 5.8% to 97.5% (a 91.7 percentage point increase) when the floor definition changes from 5 to 10.

### 6.2 Weight Perturbation Detail

- **cc-20%**: 0.0% collapse, final pop mean = 8.1 (95% CI [0.0%, 3.0%])
- **cc+20%**: 0.0% collapse, final pop mean = 8.0 (95% CI [0.0%, 3.0%])
- **ce-20%**: 0.0% collapse, final pop mean = 7.8 (95% CI [0.0%, 3.0%])
- **ce+20%**: 0.8% collapse, final pop mean = 8.2 (95% CI [0.0%, 4.6%])
- **rf-20%**: 0.8% collapse, final pop mean = 8.0 (95% CI [0.0%, 4.6%])
- **rf+20%**: 0.0% collapse, final pop mean = 8.2 (95% CI [0.0%, 3.0%])
- **sq-20%**: 0.0% collapse, final pop mean = 8.0 (95% CI [0.0%, 3.0%])
- **sq+20%**: 0.0% collapse, final pop mean = 8.1 (95% CI [0.0%, 3.0%])

## 7. Methodological Notes

### 7.1 Limitations of This Analysis

1. **Aggregate-only data**: Per-world time series are not preserved in the current CSV format. All statistics are computed from per-parameter-value aggregate summaries (mean, stddev, min, max, p10, p90 across worlds at each parameter value).
2. **Bootstrap approximation**: Bootstrap CIs on means are approximated by resampling from N(mean, SE) rather than from raw per-world data.
3. **Independence assumption**: The binomial CI assumes each world is independent. Worlds share the same codebase and parameter structure but differ by deterministic seed.
4. **Single architecture**: All results are from x86_64 Windows. Cross-platform determinism is not verified.
5. **No time-to-event analysis**: Survival analysis (Kaplan-Meier, hazard rates) requires per-world survival epoch data, which would need per-world CSV export.

### 7.2 Recommended Upgrades for Publication

1. Export per-world CSV data (one row per world per epoch, or at minimum one row per world with final state)
2. Compute Kaplan-Meier survival curves across sensitivity conditions
3. Fit logistic regression: P(collapse) ~ f(floor, weight_perturbation, experiment_type)
4. Cross-platform determinism audit (Linux, ARM)
5. Formal hypothesis tests with Bonferroni correction for multiple comparisons

---

*Analysis performed on 51 experiments, 7,240 worlds, 3,620,000 epochs.*
