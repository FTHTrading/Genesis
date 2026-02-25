# Known Failure Modes

**Genesis Protocol — Systematic Documentation of System Failures and Pathological States**

February 2026

---

## Purpose

This document catalogues every observed failure mode, pathological state, and degradation pattern discovered during 51 experiments across 7,240 world simulations. Transparent documentation of where the system breaks, nearly breaks, or degrades is essential for honest scientific reporting.

---

## 1. Hard Failures (Population Collapse)

### 1.1 Strict Collapse Definition ($P_{\text{floor}} \geq 10$)

Under the default collapse definition ($P_{\text{floor}} = 3$, sustained 50 epochs), zero collapses occur across 5,680 core worlds. However, this result is **entirely contingent on the permissive definition**.

| Floor | Collapses / N | Rate | 95% CI |
|---|---|---|---|
| 3 (default) | 0 / 120 | 0.0% | [0.0%, 3.0%] |
| 5 | 7 / 120 | 5.8% | [2.4%, 11.6%] |
| 10 | 117 / 120 | **97.5%** | [92.9%, 99.5%] |
| 15 | 120 / 120 | **100%** | [97.0%, 100%] |
| 20 | 120 / 120 | **100%** | [97.0%, 100%] |

**Phase transition**: The collapse rate jumps 91.7 percentage points between floor = 5 and floor = 10. This is a definitional cliff, not a smooth degradation.

**Interpretation**: Most populations stabilize somewhere between 3 and 10 agents under stress. The system does not achieve "robust" population sizes. It achieves minimum viable populations that happen to exceed the permissive default threshold.

### 1.2 Weight Perturbation Collapses

Two specific fitness weight perturbations produce collapses at the default definition:

| Perturbation | Collapses / N | Rate | 95% CI |
|---|---|---|---|
| CE +20% (cognitive emphasis ↑) | 1 / 120 | 0.83% | [0.02%, 4.6%] |
| RF −20% (resource foraging ↓) | 1 / 120 | 0.83% | [0.02%, 4.6%] |

Both reduce the effective weight of resource foraging in the selection function, indicating that the system's survival depends non-trivially on maintaining selection pressure toward foraging behavior.

---

## 2. Pathological Survival States

The system can "survive" (avoid collapse under the default definition) while exhibiting severe economic pathology. These states are arguably worse than clean collapse because they represent persistent dysfunction.

### 2.1 Full Attack Configuration (s4_full_attack)

Configuration: zero resource regeneration + death drains resource pools + all safety mechanisms OFF + 10× reproduction cost (250 ATP).

| Metric | Value | Healthy Baseline |
|---|---|---|
| Mean population | 12.8 | 40–55 |
| Final population | 8.1 ± 1.5 | 50–55 |
| **Min population (any world)** | **3** | 28–48 |
| Reproductive inequality | **0.952** | 0.1–0.3 |
| Survival inequality | **0.873** | 0.1–0.3 |
| Wealth concentration | 0.417 | 0.1–0.2 |
| Birth/death ratio | 1.72 | 1.0–1.1 |
| Mean Gini | 0.485 | 0.45–0.55 |

**Interpretation**: A tiny oligarchy monopolizes reproduction. The bottom 75% of agents contribute <50% of births but >50% of deaths. The population persists only because the extinction floor mechanism prevents the final 3 agents from dying. This is "survival" only in the narrowest definitional sense.

### 2.2 All Stabilizers Disabled (s3_all_off)

Configuration: treasury redistribution OFF + ATP decay OFF + grants OFF + floor protection OFF.

| Metric | Value |
|---|---|
| Mean population | 38.9 |
| Population floor (observed) | 20 |
| Reproductive inequality | 0.865 |
| Survival inequality | 0.892 |
| Wealth concentration | 0.488 |
| Birth/death ratio | 0.988 |

**Interpretation**: Birth/death ratio < 1.0 indicates the population is slowly declining. With 500-epoch simulations, the decline is not fast enough to trigger collapse, but extended horizons might show continued erosion.

### 2.3 Death-Sink Economy (s4_death_sink)

Configuration: agent death drains ATP from resource pools.

| Metric | Value |
|---|---|
| Mean population | 37.5 |
| Final population | 44.4 ± 8.0 |
| Min population (any world) | 23 |
| Gini coefficient | 0.729 |

**Interpretation**: Inequality spikes because resource scarcity from death drainage differentially affects agents with lower foraging traits.

---

## 3. Degradation Patterns (Not Collapse, But Concerning)

### 3.1 Population Compression Under Metabolic Stress

Experiment `s1/metabolic_inversion`: Increasing metabolic cost produces massive population reduction.

| Metabolic Cost | Final Population |
|---|---|
| Low (base) | 49.1 |
| High (10×) | 20.7 |

Cohen's d = 12.6 — an enormous effect size. The system is extremely sensitive to metabolic parameters even though it doesn't collapse under default definitions.

### 3.2 Resource Scarcity Regime

Experiment `s1/resource_depletion_scarce` (carrying capacity = 30):

| Metric | Value | Normal (cap=120) |
|---|---|---|
| Final population | 30.7 | 52.7 |
| Gini coefficient | 0.506 | 0.557 |

Population is pinned near the carrying capacity. Any further reduction would push toward the collapse boundary.

### 3.3 Multi-Axis Stress

Experiment `s1/multi_axis_collapse`: Combined stressor escalation.

| Stress Level | Final Population |
|---|---|
| Lowest | 29.5 |
| Highest | 49.0 |

Population varies by a factor of 1.7× across the stress gradient (d = 11.5).

### 3.4 Extended Horizon Contraction

Experiment `s2/s4_extended_horizon` (1000 epochs instead of 500):

| Metric | Value | 500-epoch equiv |
|---|---|---|
| Final population | 26.5 | 40–50 |
| Global min population | 22 | 28–48 |

Longer simulations reveal continued population decline that 500-epoch runs may not capture. This suggests that some "surviving" populations are in slow decline.

---

## 4. Architectural Failure Modes

### 4.1 Extinction Floor Dependency

The world-level extinction floor mechanism in `stress.rs` prevents populations below 3 from declining further. Without this mechanism, the actual collapse rate under the default definition would be higher — but "how much higher" is unknown because the floor intervenes before the natural population trajectory can be observed.

The global minimum population observed in any core experiment is **3** — exactly at the floor. This is not a coincidence.

### 4.2 Cortex Controller Limitations

The adaptive Cortex is a hand-engineered PID-like controller with heuristically selected gain parameters. It is not:
- Learned from data
- Proven optimal
- Guaranteed to stabilize all parameter regimes

Under extreme stress (s4_full_attack), the Cortex is disabled, and the system still technically survives — but only because of the extinction floor, not because of emergent stability.

### 4.3 Fitness Function Fragility

The fitness function weights (CE=0.25, SQ=0.30, RF=0.20, CC=0.25) are hand-tuned. Perturbation of RF by −20% causes collapses, suggesting the system's survival depends on maintaining adequate resource foraging selection pressure. This is a single-point-of-fragility: the system's most important behavior (resource acquisition) is governed by one weight that, if undervalued by ≥20%, can cause population extinction.

### 4.4 Cross-Platform Determinism Not Verified

All 7,240 worlds were computed on x86_64 Windows with Rust 1.93.0. SHA-256 hash verification is only meaningful within this platform. Cross-platform floating-point differences may produce divergent results, invalidating the reproducibility claim for non-x86_64 architectures.

---

## 5. What We Haven't Tested

1. **Epochs > 1000**: Longest simulation is 1000 epochs. Long-term (10,000+ epoch) stability is unknown.
2. **Population > 200**: All simulations start with 20 agents and soft caps ≤ 180. Scalability is untested.
3. **Correlated shocks**: All stochastic events are i.i.d. Real economies experience correlated crises.
4. **Adversarial agents**: No agents have adversarial strategies. All behavior is trait-determined.
5. **Network topology**: Agents interact in a fully-connected mesh. Spatial structure or limited connectivity may change dynamics.
6. **Continuous parameter spaces**: Sweeps use discrete parameter grids. Behavior between grid points is interpolated, not observed.
7. **Multiple simultaneous weight perturbations**: Only one weight is perturbed at a time. Joint perturbation effects are unknown.
8. **Stochastic seed sensitivity**: All experiments use seed 20260222. Other seeds may produce different baseline populations.

---

## 6. Summary

The system does not fail in the narrow sense of the default collapse definition (floor = 3). It fails extensively under stricter definitions and exhibits severe pathological states even when "surviving." The honest characterization is:

> **The system maintains minimum viable population (≥3) under all tested conditions, but achieves healthy population dynamics only within a relatively narrow parameter regime. Outside this regime, the system degrades to pathological oligarchic states where a tiny minority monopolizes reproduction and survival.**

This is a useful computational result. It is not evidence of robust emergent stability.

---

*Based on 51 experiments, 7,240 worlds, 3,620,000+ epochs. All failure data is independently reproducible from seeds and configurations published in the experiment manifests.*
