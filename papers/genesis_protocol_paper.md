# Deterministic Multi-Agent Economic Simulation Under Structural Invariant Violations: Collapse Boundary Analysis

**Kevan Burns**

February 2026

---

## Abstract

This paper describes a deterministic multi-agent macroeconomic simulation system and reports results from 38 experiments comprising 5,680 world-runs across 2,840,000 computed epochs. Each simulation instantiates a population of agents with genetically determined traits, a metabolic energy economy with taxation and redistribution, logistic resource pools, stochastic catastrophe processes, and a homeostatic parameter controller. Collapse is defined as population reaching zero, or population remaining below a configurable floor ($P_{\text{floor}}$) for a sustained window; the default definition uses $P_{\text{floor}} = 3$ and a 50-epoch window. All simulations are seeded deterministically using SHA-256 hash chains and Knuth MMIX linear congruential generators, producing bit-identical results across runs on the same architecture. Season 1 (25 experiments, 4,180 worlds) swept 10 economic parameters under standard and adversarial conditions. Season 2 (13 experiments, 1,500 worlds) systematically disabled structural invariants including treasury redistribution, ATP decay, resource regeneration, and reproduction grants, individually and in combination. Under the default collapse definition, no collapses were observed in any experiment. Under stricter definitions ($P_{\text{floor}} = 10$), collapse rates exceeded 97%. Under maximal structural violation (zero resource regeneration, death draining resource pools, all safety mechanisms disabled, 10× reproduction cost), populations contracted to a mean of 12.8 agents but did not collapse under the default definition, instead exhibiting pathological inequality (reproductive inequality index 0.95, survival inequality index 0.87, wealth concentration index 0.42). Sensitivity analyses confirm that ±20% perturbation of fitness function weights produces collapse rate changes of at most 0.8 percentage points, and that the collapse boundary is sharply sensitive to the floor definition (Appendices C–D). All experiment configurations, seeds, and result hashes are published. Independent replication has not yet occurred.

---

## 1. Introduction

Agent-based macroeconomic simulations provide a framework for studying emergent economic behavior under controlled conditions. A recurring methodological concern in such systems is the sensitivity of population persistence to parameter choice, initial conditions, and structural assumptions. This paper addresses that concern through systematic parameter sweeps and structural invariant violations applied to a deterministic simulation engine, with emphasis on reproducibility and formal collapse criteria.

The system under study models a closed economy of heterogeneous agents who extract resources from logistic niche pools, pay metabolic costs, reproduce conditionally, and face stochastic catastrophes. A redistributive treasury, homeostatic parameter controller, and genetic mutation engine provide adaptive capacity. The central question is: under what conditions, if any, does the population collapse?

This paper does not claim generality beyond the tested parameter space. The collapse boundary — the surface in parameter space separating survival from extinction — remains an open problem. Sensitivity analyses (Appendices C–D) partially characterize this boundary by varying the collapse definition and fitness function weights.

---

## 2. Formal System Definition

### 2.1 State Vector

The world state at discrete epoch $t \in \mathbb{N}_0$ is the tuple:

$$S(t) = \langle P(t),\; L(t),\; T(t),\; E(t),\; \Pi(t),\; \Psi(t),\; \phi(t) \rangle$$

| Symbol | Description | Domain |
|--------|-------------|--------|
| $P(t)$ | Agent population | $\{a_1, \ldots, a_n\}$, $n \geq 0$ |
| $L(t)$ | Economy ledger | Agent-indexed balance map $\mathbb{R}_{\geq 0}^n$ + scalar total supply |
| $T(t)$ | Treasury | $\mathbb{R}_{\geq 0}$ (reserve balance) |
| $E(t)$ | Environment | 5 resource pools $\mathbf{R} \in \mathbb{R}_{\geq 0}^5$ + catastrophe state |
| $\Pi(t)$ | Pressure configuration | $\mathbb{R}^7$ (mutable parameter vector) |
| $\Psi(t)$ | Adaptive cortex | Homeostatic controller state |
| $\phi(t)$ | Seasonal phase | $\{\text{Spring}, \text{Summer}, \text{Autumn}, \text{Winter}\}$ |

### 2.2 Agent Definition

Each agent $a \in P(t)$ is characterized by:

$$a = (h,\; \mathbf{v},\; \mathbf{s},\; r,\; \rho,\; g,\; \ell,\; \mu)$$

where $h \in \{0,1\}^{256}$ is the genesis hash (SHA-256), $\mathbf{v} \in [0,1]^4$ is the trait vector, $\mathbf{s} \in [0,1]^4$ is the skill profile, $r \in \{0,1,2,3,4\}$ is the role assignment, $\rho \in [0,1]$ is the reputation, $g \in \mathbb{N}_0$ is the generation, $\ell$ is the lineage (ancestry chain), and $\mu \in [0.001, 0.1]$ is the adaptive mutation rate.

#### 2.2.1 Trait Derivation

Traits are deterministically derived from the genesis hash:

$$v_i = \frac{\text{u64\_le}(h[8i : 8(i+1)])}{\text{u64::MAX}}, \quad i \in \{0, 1, 2, 3\}$$

The four trait dimensions are: Cognitive Efficiency ($v_0$), Strategic Quality ($v_1$), Resource Foraging ($v_2$), and Cooperative Capacity ($v_3$).

#### 2.2.2 Skill Derivation

Skills are derived from individual hash bytes:

$$s_j = \frac{h[j]}{255}, \quad j \in \{0, 1, 2, 3\}$$

corresponding to Computation, Optimization, Communication, and Cooperation.

#### 2.2.3 Role Assignment

$$r = h[4] \bmod 5$$

mapping to $\{\text{Optimizer}, \text{Strategist}, \text{Communicator}, \text{Archivist}, \text{Executor}\}$.

### 2.3 Fitness Function

Agent fitness is a weighted linear combination of the trait vector:

$$F(a) = \sum_{i=0}^{3} w_i \cdot v_i = 0.25 \cdot v_0 + 0.30 \cdot v_1 + 0.20 \cdot v_2 + 0.25 \cdot v_3$$

$F(a) \in [0, 1]$ for all agents. The weight vector $\mathbf{w} = (0.25, 0.30, 0.20, 0.25)$ sums to 1.0 and was fixed prior to any experiments. Strategic Quality receives the highest weight; the slight asymmetry (SQ receiving 0.30 vs. 0.20–0.25 for others) was a subjective design choice reflecting an assumption that long-term strategic behavior contributes more to survival than any single capability. These weights were not optimized to maximize population survival or any other outcome metric. Sensitivity analysis (Appendix D) indicates that perturbing any individual weight by ±20% (with renormalization to maintain $\sum w_i = 1$) produces a maximum collapse rate change of 0.8 percentage points (1 additional collapse out of 120 worlds under S4 Full Attack), indicating that the reported results are not an artifact of weight tuning.

### 2.4 Pressure Configuration

The mutable pressure vector $\Pi(t) \in \mathbb{R}^7$ parameterizes the economic environment:

$$\Pi = (\kappa_{\text{cap}},\; \varepsilon,\; p_c,\; s_c,\; \theta_G,\; r_G,\; \theta_T)$$

| Parameter | Symbol | Default | Range | Description |
|-----------|--------|---------|-------|-------------|
| Carrying capacity | $\kappa_{\text{cap}}$ | 180 | [50, 500] | Logistic soft cap |
| Entropy coefficient | $\varepsilon$ | 0.00002 | [0.000001, 0.001] | Proportional supply burn rate |
| Catastrophe base probability | $p_c$ | 0.002 | [0.0, 0.05] | Per-epoch catastrophe probability |
| Catastrophe population scale | $s_c$ | 0.00001 | [0.0, 0.0005] | Population-proportional catastrophe term |
| Gini tax threshold | $\theta_G$ | 0.40 | [0.20, 0.80] | Inequality threshold for wealth tax |
| Gini tax rate | $r_G$ | 0.02 | [0.005, 0.10] | Tax rate on top decile |
| Treasury overflow threshold | $\theta_T$ | 0.50 | [0.20, 0.80] | Reserve-to-supply ratio for redistribution |

### 2.5 Resource Dynamics

The environment contains 5 resource pools $R_k$, each following logistic regeneration:

$$R_k(t+1) = R_k(t) + r_k \cdot R_k(t) \cdot \left(1 - \frac{R_k(t)}{K_k^{\text{eff}}(t)}\right)$$

with seasonally modulated effective capacity:

$$K_k^{\text{eff}}(t) = K_k \cdot \left[1 + A \sin\left(\frac{2\pi \cdot t}{L_{\text{season}}}\right)\right]$$

Extraction per agent per epoch:

$$\xi(a, t) = F(a) \cdot s_{\text{niche}}(a) \cdot \frac{R_{\text{pool}}(t)}{K} \cdot d(a, t) \cdot \alpha_{\text{cross}} \cdot 2.5$$

where $d(a, t)$ is a density-dependent competition factor and $\alpha_{\text{cross}} = 0.15$ is the cross-niche participation coefficient. Total extraction per pool per epoch is capped at 40% of the pool.

### 2.6 Metabolic Economy

#### 2.6.1 Costs

Each agent incurs a basal metabolic cost $c_b = 0.15$ ATP per epoch and a natural decay $d = 0.02$ (2% balance reduction per epoch).

#### 2.6.2 Taxation

Three tax mechanisms operate:

1. **Entropy tax**: $\Delta_{\text{burn}} = M_{\text{supply}} \cdot (|P| \cdot \varepsilon)$, distributed proportionally by balance share, capped at 10% of each agent's balance. Burns from total supply (does not enter treasury).

2. **Flat wealth tax**: 1% on balances exceeding 100 ATP, directed to treasury.

3. **Gini-triggered wealth tax**: When $G(t) > \theta_G$, the top decile pays $r_G$ fraction of balance to treasury.

#### 2.6.3 Treasury

The treasury $T(t)$ receives:
- 5% skim on all agent income
- Wealth tax proceeds

The treasury disburses:
- Role-based stipends to underrepresented roles ($s$ ATP per agent, amplified by deficit ratio)
- Crisis spending (direct drawdowns)
- Overflow redistribution when $T(t) / M_{\text{supply}} > \theta_T$

#### 2.6.4 Stasis and Death

An agent enters stasis when $\text{bal}(a) \leq 0$. After 8 consecutive epochs in stasis ($N_{\text{stasis}} = 8$), the agent dies. Remaining balance is burned from total supply.

### 2.7 Catastrophe Process

The per-epoch catastrophe probability is:

$$P(\text{cat} \mid t) = p_c + |P(t)| \cdot s_c$$

conditioned on $|P(t)| > 10$. When triggered, one of three effects is selected deterministically:

| Type | Effect |
|------|--------|
| ATP destruction | All balances reduced by 5–10% |
| Fitness culling | Weakest 2–5% (by fitness) removed |
| Niche resource shock | ~70% of one resource pool destroyed |

Selection is determined by a Knuth MMIX LCG seeded on epoch number:

$$x_{t} = t \cdot \texttt{0xBF58476D1CE4E5B9} + \texttt{0x94D049BB133111EB}$$
$$\text{type} = (x_t \gg 16) \bmod 3$$

The environment also maintains a background catastrophe process ($P \approx 0.02$) with duration 10–20 epochs and severity 30–60% capacity reduction to affected resource pools.

### 2.8 Reproduction

An agent $a$ replicates in epoch $t$ if and only if:

$$F(a) \geq 0.35 \;\wedge\; \text{bal}(a) \geq 25 m_\phi m_\sigma \;\wedge\; \text{age}(a) \geq 10 \;\wedge\; |P(t)| < \kappa_{\text{cap}}$$

where $m_\phi$ is a seasonal cost modifier and $m_\sigma$ is a stress-profile modifier. Maximum births per epoch: 3. Birth is further suppressed logistically:

$$\beta(t) = \text{clamp}\left(1 - \frac{|P(t)|}{\kappa_{\text{cap}}},\; 0,\; 1\right)$$

The child genome is derived by:

$$h_{\text{child}} = \text{SHA-256}(h_{\text{parent}} \| \text{entropy} \| \text{timestamp} \| \text{id}_{\text{child}})$$

Juvenile agents (age < 5 epochs) receive a 25% basal cost rebate.

### 2.9 Mutation

The mutation engine applies per-trait perturbations during reproduction:

$$v_i' = v_i + \delta_i, \quad \delta_i \sim U[-\delta_{\max},\; \delta_{\max}] \cdot (1 + p)$$

where $\delta_{\max} = 0.05$ and $p$ is the environmental pressure. The adaptive mutation rate adjusts by fitness:

$$\mu(a) = \text{clamp}\left((1 - F(a)) \cdot 0.05,\; 0.001,\; 0.1\right)$$

Less fit agents mutate more, increasing variance among underperforming lineages.

### 2.10 Homeostatic Controller (Adaptive Cortex)

The adaptive cortex is a hand-engineered feedback controller with heuristically selected parameters, thresholds, and gain values. It is not a learned or emergent system. Its inclusion reflects a design decision to provide multi-layer stabilization; the research question is whether collapse emerges *despite* redundant safeguards, not whether stability is surprising given the architecture. Season 2 experiments (stages S2–S4) disable the cortex entirely, isolating its contribution.

Every 25 epochs, the adaptive cortex evaluates 8 threat indicators against three-tier thresholds (Watch, Warning, Critical):

| Threat | Watch | Warning | Critical |
|--------|-------|---------|----------|
| Monoculture dominance | >55% | >70% | >85% |
| ATP oligarchy | >40% top-10% share | >60% | >80% |
| Mutation runaway | >30% mutated | >50% | >70% |
| Population collapse | >30% decline | >50% | >70% |
| Role extinction | 1 role absent | 2 absent | ≥3 absent |
| Treasury depletion | >50% depleted | >70% | >90% |
| Wealth concentration | Gini >0.55 | >0.70 | >0.85 |
| Economic stagnation | velocity ≤0.10 | ≤0.05 | ≤0.01 |

Each threat maps to prescribed parameter adjustments on $\Pi(t)$, bounded by per-field step limits and a 50-epoch cooldown per field. In the absence of threats, parameters drift 10% toward defaults per cycle.

The cortex is a hand-engineered feedback controller, not a learned or evolved system. All thresholds, gain values, and prescribed adjustments were selected heuristically by the system designer. It functions as an intentional stabilizing mechanism — its purpose is to counteract parameter drift, not to demonstrate emergent adaptation. In Season 2 experiments (S2–S4), the cortex is disabled along with mutation, isolating the base system dynamics from controller-induced stability. The question addressed by Season 2 is whether populations persist *despite* removal of this engineered safety layer, not whether the cortex itself is surprising.

### 2.11 Seasonal Phase

Season transitions are driven by the birth-to-death ratio $R_{\text{bd}}$:

| Condition | Phase |
|-----------|-------|
| $R_{\text{bd}} > 1.3$ | Spring |
| $R_{\text{bd}} > 1.0$ | Summer |
| $R_{\text{bd}} > 0.6$ | Autumn |
| $R_{\text{bd}} \leq 0.6$ | Winter |

Seasonal phase affects treasury release rates and reproduction cost modifiers.

---

## 3. Collapse Definition

Collapse at epoch $t$ is the predicate:

$$\kappa(t) = \begin{cases}
\top & \text{if } |P(t)| = 0 \\
\top & \text{if } |P(\tau)| < P_{\text{floor}} \;\;\forall\, \tau \in [t - N_w + 1,\; t] \\
\bot & \text{otherwise}
\end{cases}$$

| Constant | Symbol | Value | Justification |
|----------|--------|-------|---------------|
| Extinction floor | $P_{\text{floor}}$ | 3 | Minimum for demographic replacement given max 3 births/epoch and 1 parent per birth. A population of 2 cannot sustain itself if one agent dies before reproducing. |
| Recovery window | $N_w$ | 50 | At 1 birth/epoch from a floor of 3, 50 epochs would produce 50 births. If recovery does not occur in 50 epochs, it is not structurally possible under current conditions. |

### 3.1 Definition Sensitivity

The collapse rate is sharply sensitive to the choice of $P_{\text{floor}}$. Sensitivity analysis (Appendix C) applied the s4_full_attack configuration — the most extreme tested condition — under varying floor definitions:

| $P_{\text{floor}}$ | Collapse Rate | Collapsed / Total |
|---------------------|---------------|-------------------|
| 3 (default) | 0.0% | 0 / 120 |
| 5 | 5.8% | 7 / 120 |
| 10 | 97.5% | 117 / 120 |
| 15 | 100.0% | 120 / 120 |
| 20 | 100.0% | 120 / 120 |

Under the default definition ($P_{\text{floor}} = 3$), zero collapses are observed. Under a modestly stricter definition ($P_{\text{floor}} = 5$), 5.8% of worlds collapse. Under $P_{\text{floor}} = 10$, near-universal collapse occurs (97.5%). This indicates that populations under maximal stress stabilize in the range of 3–8 agents: above the default floor but well below the initial soft cap. The collapse boundary, with respect to definition choice, lies between $P_{\text{floor}} = 5$ and $P_{\text{floor}} = 10$ for this configuration. The zero-collapse headline result is therefore contingent on the permissive default definition.

### 3.2 Alternative Definitions Considered

The following alternative collapse definitions were considered but not adopted:

| Alternative | Definition | Effect on Results |
|-------------|------------|-------------------|
| Stricter floor ($P_{\text{floor}} = 10$) | Population < 10 for $N_w$ epochs | 97.5% collapse under s4_full_attack (Appendix C) |
| Shorter window ($N_w = 10$) | 10 consecutive epochs below floor | Would catch brief population dips |
| Demographic collapse | Birth/death ratio < 0.5 for 100 epochs | Would catch aging populations |
| Economic collapse | Mean ATP < basal cost for 50 epochs | Would catch energy starvation |
| Inequality collapse | Gini > 0.95 for 100 epochs | Would capture extreme inequality |

The sensitivity of results to collapse definition choice is characterized quantitatively in Section 3.1 and Appendix C.

---

## 4. Determinism and Reproducibility

### 4.1 Seed Model

Each experiment uses a base seed $s_0 \in \{20260222, 20260223, 20260224, 42\}$. Per-trial seeds are derived:

$$\text{seed}(i, j) = s_0 + i \cdot 1000 + j$$

where $i$ is the sweep step index and $j$ is the run index within that step.

### 4.2 Random Number Generation

The system uses three classes of deterministic generators:

1. **Agent identity**: SHA-256 hash chains seeded from parent hash, entropy, timestamp, and UUID.
2. **In-epoch decisions**: Knuth MMIX LCG ($a = 6364136223846793005$, $c = 1442695040888963407$, mod $2^{64}$) seeded on epoch number. Different multiplier/increment pairs are used for different subsystems (catastrophe selection, stress clustering) to decorrelate streams.
3. **Trait derivation**: Deterministic byte extraction from SHA-256 hashes.

### 4.3 Hash Registry

Each experiment produces two SHA-256 hashes:
- **Manifest hash**: Hash of the configuration (sweep variable, range, seed, metrics, overrides).
- **Result hash**: Hash of the configuration concatenated with all computed results.

The hash registry contains 38 entries with verified bit-identical reproduction on the originating platform.

### 4.4 Environment Constraints

| Component | Version |
|-----------|---------|
| Language | Rust 1.93.0 |
| RNG crate | `rand 0.8` with `StdRng` (ChaCha) |
| Hash crate | `sha2 0.10` |
| Architecture | x86_64, Windows |

Floating-point determinism is not guaranteed across architectures. Cross-platform reproducibility requires identical IEEE 754 rounding behavior. This is a known limitation (Section 8).

---

## 5. Experiment Taxonomy

### 5.1 Season 1: Parameter Stress (25 experiments, 4,180 worlds)

Season 1 sweeps economic parameters under standard and adversarial conditions with all system mechanisms active unless otherwise noted.

| Category | Experiments | Sweep Variable | Range | Worlds | Seed |
|----------|-------------|----------------|-------|--------|------|
| Entropy | 1 | EntropyCoeff | 0.00001–0.0001 | 200 | 20260222 |
| Catastrophe | 1 | CatastropheBaseProb | 0.0–0.03 | 140 | 20260222 |
| Inequality | 1 | GiniWealthTaxThreshold | 0.2–0.9 | 160 | 20260222 |
| Treasury | 1 | TreasuryOverflowThreshold | 0.1–0.9 | 180 | 20260222 |
| Metabolic inversion | 1 | ReplicationCostMultiplier | 1.0–5.0 | 180 | 20260223 |
| Basal inversion | 1 | BasalCostMultiplier | 1.0–10.0 | 200 | 20260224 |
| Dual inversion | 1 | BasalCostMultiplier (+ 3× replication) | 1.0–10.0 | 200 | 20260224 |
| Multi-axis | 1 | SoftCap (all safety OFF) | 30–180 | 220 | 20260222 |
| Resilience quadrant | 4 | CatastropheBaseProb (±mutation, ±cortex) | 0.0–0.05 | 4×220 | 20260223 |
| Resource depletion | 4 | EntropyCoeff (under 4 scarcity levels) | 0.00001–0.0001 | 4×150 | 20260222 |
| Reserve stress | 4 | TreasuryOverflowThreshold (4 stress levels) | 0.1–0.8 | 4×135 | 20260222 |
| FTH reserve stress | 4 | TreasuryOverflowThreshold (4 levels, alt config) | 0.1–0.8 | 4×135 | 20260222 |
| Evolution forbidden | 1 | CatastropheBaseProb (mutation=0, cortex=OFF) | 0.0–0.03 | 140 | 20260222 |

All experiments: 500 epochs per run, 20 runs per sweep step, EarthPrime base preset.

### 5.2 Season 2: Structural Invariant Violations (13 experiments, 1,500 worlds)

Season 2 disables structural mechanisms that are assumed necessary for population persistence.

| Stage | Experiments | Mechanism Disabled | Additional Conditions |
|-------|-------------|--------------------|-----------------------|
| S1 | 2 | Treasury redistribution | Baseline + hostile (mutation=0, cortex=OFF, 5× entropy, max catastrophe) |
| S2 | 2 | ATP decay | Baseline + hostile |
| S3 | 4 | Combinatorial: decay, treasury, grants, extinction floor | Each pair OFF + all four OFF |
| S4 | 5 | Resource regeneration, death drains pools, 10× replication cost, all S3 mechanisms OFF | Baseline, death sink, zero regen, combined, extended horizon (5,000 epochs) |

All Season 2 experiments: soft cap sweep 30–180, 20 runs/step, base seed 42.

---

## 6. Results

### 6.1 Collapse Rates

| Season | Experiments | Worlds | Collapse Rate |
|--------|-------------|--------|---------------|
| Season 1 | 25 | 4,180 | 0 / 4,180 (0.00%) |
| Season 2 | 13 | 1,500 | 0 / 1,500 (0.00%) |
| **Total** | **38** | **5,680** | **0 / 5,680 (0.00%)** |

No world-run triggered either collapse condition ($|P(t)| = 0$ or $|P(t)| < 3$ for 50 consecutive epochs) under the default definition. Under stricter definitions, collapse rates increase sharply (Section 3.1).

### 6.2 Season 1 Summary

| Experiment | Worlds | Mean Pop | Mean Fitness | Gini |
|-----------|--------|----------|-------------|------|
| Entropy sweep | 200 | 47.8–48.8 | 0.535–0.550 | 0.533–0.581 |
| Catastrophe resilience | 140 | 48.4–49.1 | — | — |
| Inequality threshold | 160 | 49.5 | 0.538–0.555 | 0.554–0.717 |
| Treasury stability | 180 | 49.4 | 0.531–0.542 | 0.539–0.550 |
| Metabolic inversion (5×) | 180 | — | — | — |
| Basal inversion (10×) | 200 | — | — | — |
| Dual inversion (3×/10×) | 200 | — | — | — |
| Multi-axis collapse | 220 | — | — | — |
| Resilience Q1 (both ON) | 220 | 45.1–49.1 | 0.533–0.579 | — |
| Resilience Q4 (static) | 220 | 45.0–48.4 | 0.535–0.576 | — |
| Resource depletion (scarce) | 150 | 30.2–30.3 | 0.541 | 0.490–0.507 |

Population under baseline conditions stabilizes near 48–49 agents. Under resource scarcity (soft cap 30), equilibrium contracts to ~30. Fitness remains narrowly bounded (0.53–0.58 across all conditions). Gini coefficient ranges from 0.49 (scarce resource, equal poverty) to 0.72 (laissez-faire inequality threshold at 0.90).

### 6.3 Season 2 Summary

| Experiment | Worlds | Mean Pop | Mean Fitness | Mean Gini | WCI | Repro Ineq | Surv Ineq |
|-----------|--------|----------|-------------|-----------|-----|-----------|-----------|
| S1: Treasury OFF (baseline) | 120 | 43.1 | 0.549 | — | — | — | — |
| S1: Treasury OFF (hostile) | 120 | 41.8 | 0.577 | — | — | — | — |
| S2: ATP decay OFF | 120 | 51.6 | 0.523 | 0.402 | 0.250 | 0.526 | 0.845 |
| S3: All safety OFF | 120 | 38.9 | 0.591 | 0.457 | 0.488 | 0.865 | 0.892 |
| S4: Zero regeneration | 120 | 23.0 | 0.519 | 0.593 | 0.393 | 0.951 | 0.941 |
| S4: Death sink | 120 | 42.5 | 0.569 | 0.550 | 0.665 | 0.919 | 0.820 |
| S4: Zero regen + death sink | 120 | 23.1 | 0.523 | 0.589 | 0.377 | 0.949 | 0.939 |
| S4: Full attack | 120 | 12.8 | 0.519 | 0.485 | 0.417 | 0.952 | 0.873 |
| S4: Extended horizon (5K epochs) | 60 | 26.0 | 0.519 | 0.608 | 0.318 | 0.997 | 0.954 |

Minimum observed population floor across all S4 experiments: 3 agents (the extinction floor threshold). The s4_full_attack configuration — zero resource regeneration, death draining pools, all safety mechanisms disabled, 10× reproduction cost — represents the most extreme structural violation tested. Mean population contracted to 12.8 but did not collapse under the default definition.

### 6.4 Distributional Analysis

Population minima and mean distributions across Season 2 experiments:

| Experiment | Min Pop (global) | Mean Pop (avg) | Pop at Floor ($|P| \leq 5$) |
|------------|-----------------|----------------|------------------------------|
| S1: Treasury OFF (baseline) | 20 | 43.1 | None observed |
| S1: Treasury OFF (hostile) | 20 | 41.8 | None observed |
| S2: ATP decay OFF | 20 | 51.6 | None observed |
| S3: All safety OFF | 20 | 38.9 | None observed |
| S4: Zero regeneration | 5 | 23.0 | Transient |
| S4: Death sink | 20 | 42.5 | None observed |
| S4: Zero regen + death sink | 6 | 23.1 | Transient |
| S4: Full attack | 3 | 12.8 | Sustained near-floor |
| S4: Extended horizon (5K) | 7 | 26.0 | Transient |

Experiments S1–S3 never approach the extinction floor; the world-level extinction floor mechanism (juvenile cost rebates, stasis tolerance adjustments) clamps minimum populations at 20. In S4 experiments where this mechanism is disabled, populations reach the detection floor (3–7 agents), confirming that the world-level floor is a significant contributor to population persistence in earlier stages.

The s4_full_attack configuration produces sustained near-floor populations (min = 3, mean = 12.8), meaning survival under the default collapse definition is narrow. A population of 3 is demographically fragile: with a maximum birth rate of 3 per epoch and single-parent reproduction, any single-epoch loss exceeding births would trigger a downward spiral. That this does not occur within 500 epochs is observed but not guaranteed at longer horizons.

### 6.5 Pathological States

Five Season 2 experiments produced populations that survive but exhibit economic pathology:

| Indicator | S3 All OFF | S4 Zero Regen | S4 Full Attack | S4 Extended |
|-----------|-----------|--------------|---------------|-------------|
| Reproductive inequality | 0.865 | 0.951 | 0.952 | 0.997 |
| Survival inequality | 0.892 | 0.941 | 0.873 | 0.954 |
| Wealth concentration (top 10%) | 0.488 | 0.393 | 0.417 | 0.318 |
| Mean Gini | 0.457 | 0.593 | 0.485 | 0.608 |
| Min population observed | — | 5 | 3 | 7 |

Reproductive inequality approaching 1.0 indicates that nearly all births originate from the top fitness quartile. Survival inequality approaching 1.0 indicates that nearly all deaths concentrate in the bottom quartile. These are survivable but degenerate states — the population persists structurally while exhibiting extreme stratification.

### 6.6 Observed Stability Region

All tested configurations fall within a bounded region of the 10-dimensional parameter space defined by the sweep variables. Within this region, and across all tested structural invariant violations, the system exhibits population persistence. No claim is made about behavior outside this region.

---

## 7. Mechanistic Interpretation

The system is intentionally multi-layer stabilized: logistic resource regeneration, redistributive treasury, homeostatic parameter control, mutation-driven genetic diversity, juvenile cost rebates, and extinction floor protection are all engineered mechanisms. Population persistence under benign conditions (Season 1) is therefore an expected consequence of the architecture, not evidence of emergent robustness. The more informative question is: does collapse emerge when these mechanisms are systematically removed?

Season 2 addresses this directly by disabling mechanisms in combination. When all safety mechanisms are removed (S3, S4), population persistence narrows from a comfortable margin (min population ~20, mean ~49) to near-floor survival (min population 3, mean 12.8). The system does not exhibit graceful degradation — it exhibits a sharp transition from engineered stability to marginal survival. Under stricter collapse definitions ($P_{\text{floor}} = 10$), this marginal survival registers as near-universal collapse (97.5%).

The results are consistent with several non-exclusive interpretive hypotheses. None constitute claims of proof.

### 7.1 Capital Circulation as Survival Mechanism

The data is consistent with capital flow — ATP moving from extraction through agents through taxation back to treasury and redistribution — dominating capital accumulation as a determinant of population persistence. Disabling ATP decay (S2) increases population but produces oligarchic accumulation. Disabling treasury redistribution (S1) reduces population by ~13% but does not collapse it, suggesting that alternative survival pathways may exist through direct resource extraction.

### 7.2 Population Contraction as Adaptive Response

Under severe resource constraints (S4 zero regeneration), the population contracts from ~49 to ~23 agents. This contraction reduces total metabolic cost while preserving a core of high-fitness agents. The observed behavior is consistent with contraction functioning as a stabilizing response rather than a precursor to collapse — the system appears to find a lower-energy equilibrium rather than progressing to extinction. However, whether this equilibrium is stable at longer time horizons remains untested beyond 5,000 epochs.

### 7.3 Primordial Grant Momentum

Under s4_full_attack (zero regeneration, death sink, all safety OFF, 10× reproduction cost), mean population stabilizes at 12.8. The initial primordial grant (50 ATP per agent) provides sufficient starting energy for high-fitness agents to reach reproductive threshold before resource depletion. The data does not determine whether this represents genuine stability or a slow decline that would manifest at longer time horizons. The s4_extended_horizon experiment (5,000 epochs, 10× standard duration) shows mean population of 26.0 agents with minimum of 7, which is consistent with persistent equilibrium but does not rule out secular decline on longer timescales.

### 7.4 Inequality Tolerance

The system tolerates extreme inequality without collapsing under the default collapse definition. Gini coefficients above 0.60 and reproductive inequality above 0.95 coexist with population persistence. This is consistent with economic pathology and population extinction being largely independent failure modes under the current collapse definition. Alternative collapse definitions that incorporate inequality thresholds would produce different results (Section 3.2).

---

## 8. Limitations

The following limitations are stated explicitly:

1. **Single-operator execution.** All 5,680 world-runs were executed by the same operator on the same machine. No independent replication has been performed. The hash registry enables verification but does not substitute for it.

2. **Architecture dependence.** Floating-point determinism depends on IEEE 754 compliance and identical rounding behavior. Hash matches may not reproduce on different CPU architectures (e.g., ARM, different x86 microarchitectures with different FMA behavior).

3. **Finite parameter space.** Ten sweep variables were tested, each over a finite range. The tested region is a small subset of the full parameter space. Collapse boundaries may exist outside the tested ranges.

4. **Collapse definition sensitivity.** The chosen definition ($P_{\text{floor}} = 3$, $N_w = 50$) is permissive. Under this definition, 0% collapse is observed. Under $P_{\text{floor}} = 5$, 5.8% collapse occurs. Under $P_{\text{floor}} = 10$, 97.5% collapse occurs (Appendix C). The headline zero-collapse result is contingent on the permissive default definition. Several Season 2 experiments produce populations that survive at or near the floor (minimum population = 3 in s4_full_attack), confirming that the margin between survival and collapse is narrow under extreme conditions.

5. **Time horizon limitations.** Most experiments run for 500 epochs; the longest runs 5,000. Systems exhibiting slow secular decline would not be detected within these horizons. The s4_extended_horizon result (5,000 epochs, mean population 26.0, min 7) is consistent with persistent equilibrium but does not rule out decline on longer timescales.

6. **No cross-architecture validation.** All experiments were run on a single Windows x86_64 machine. The same Rust source compiled on Linux or macOS may produce different floating-point intermediate values, leading to hash mismatches.

7. **Adaptive cortex confound.** In Season 1 experiments where the homeostatic controller is active, parameter drift makes it difficult to attribute survival to the base parameter configuration versus the controller's corrections. Season 2 experiments (S2–S4) disable the cortex and mutation engine, partially isolating this confound.

8. **Fitness function arbitrariness.** The weight vector $\mathbf{w} = (0.25, 0.30, 0.20, 0.25)$ is a design choice without empirical grounding. Sensitivity analysis (Appendix D) indicates that ±20% perturbation produces at most 0.8 percentage points of collapse rate change, suggesting the reported results are not weight-tuning artifacts. However, larger perturbations and qualitatively different fitness functions (e.g., nonlinear, multiplicative) remain untested.

9. **No analytical stability proof.** All results are empirical. No formal stability analysis (e.g., Lyapunov functions, spectral analysis of linearized dynamics) has been performed. The system's high dimensionality (agent-level state × population size × 5 resource pools × treasury × cortex state) makes analytical treatment intractable without significant simplification. The absence of collapse in 5,680 runs does not constitute a proof of stability.

10. **Engineered redundancy.** The system includes multiple overlapping stabilization mechanisms: logistic resource regeneration, redistributive treasury, homeostatic controller, juvenile cost rebates, extinction floor protection, and mutation-driven diversity. Under benign conditions (Season 1), these redundancies make collapse unlikely by design. The more informative results come from Season 2, where mechanisms are removed. Even there, the world-level extinction floor mechanism (disabled only in S4) provides implicit population support in S1–S3 stages. The contribution of each individual mechanism to survival is partially confounded by the presence of others.

11. **Fitness weight application scope.** The fitness weight sensitivity analysis (Appendix D) applies custom weights only to the selection and replication pathways. Resource extraction efficiency in the world simulation still uses the default fitness function. A complete weight override across all fitness-dependent pathways remains untested.

---

## 9. Replication Protocol

### 9.1 Prerequisites

| Requirement | Specification |
|-------------|---------------|
| Git | Any version supporting `clone` |
| Rust toolchain | 1.77.0+ (tested on 1.93.0) |
| Architecture | x86_64 recommended for hash matching |
| Disk | ~500 MB for build artifacts |
| Time | ~2 minutes for full test suite; experiment runtime varies |

### 9.2 Procedure

```
git clone https://github.com/FTHTrading/AI.git
cd AI
cargo test --release
cargo run --release --features cli -- --experiment <name>
```

### 9.3 Verification

Compare the `result_hash` field in the generated manifest against the canonical hash in `replication_status.json`. The verification script `verify_replication.ps1` automates this comparison:

```
.\verify_replication.ps1 -Verbose
```

Expected output: 38/38 MATCH.

### 9.4 Submission

Report replication results via GitHub Issue on `FTHTrading/AI` with:
- Experiment name
- Result hash
- OS, Rust version, CPU architecture
- Match/mismatch status

---

## 10. Conclusion

Across 5,680 deterministic world simulations spanning 38 experiment configurations and 2,840,000 computed epochs, no population collapses were observed under the default collapse definition ($P_{\text{floor}} = 3$, $N_w = 50$ epochs). Season 1 established robustness under parameter stress with all stabilization mechanisms active. Season 2 demonstrated persistence under systematic removal of structural mechanisms including treasury redistribution, ATP decay, resource regeneration, and reproduction grants. The most extreme configuration tested — simultaneous removal of all safety mechanisms with maximal economic hostility — produced contracted but surviving populations (mean 12.8 agents, minimum 3) exhibiting severe inequality.

The zero-collapse result is contingent on the permissive collapse definition. Under $P_{\text{floor}} = 10$, near-universal collapse (97.5%) occurs in the most extreme configuration. The collapse boundary, with respect to population floor definition, lies between $P_{\text{floor}} = 5$ and $P_{\text{floor}} = 10$ under maximal stress. Fitness function weight perturbations of ±20% produce negligible effect on collapse rates (≤0.8 percentage points), indicating that the results are not artifacts of weight selection.

These results characterize a stability region within the tested parameter space under a specific collapse definition. They do not establish collapse impossibility, universal stability, or generalizability beyond the tested configurations. The system's multi-layer stabilization architecture contributes significantly to population persistence; the extent to which observed survival reflects intrinsic dynamical stability versus engineered redundancy remains an open question. Independent replication is required to strengthen confidence in these findings.

---

## References

Burns, K. (2026). Genesis Protocol: Autonomous Metabolic Organism with Survival Economics. Zenodo. doi:10.5281/zenodo.18646886

Knuth, D. E. (1997). *The Art of Computer Programming, Vol. 2: Seminumerical Algorithms* (3rd ed.). Addison-Wesley. [LCG constants: MMIX parameters]

---

## Appendix A: Key Constants

| Constant | Value | Description |
|----------|-------|-------------|
| BASAL_COST | 0.15 ATP | Per-epoch minimum survival cost |
| REPLICATION_COST | 25.0 ATP | Base reproduction cost |
| PRIMORDIAL_GRANT | 50.0 ATP | Starting endowment per agent |
| CHILD_GRANT | 8.0 ATP | Endowment per offspring |
| STASIS_TOLERANCE | 8 epochs | Grace period before death |
| REPLICATION_FITNESS | 0.35 | Minimum fitness for reproduction |
| MATURATION_EPOCHS | 10 | Minimum age for reproduction |
| MAX_BIRTHS_PER_EPOCH | 3 | Birth rate cap |
| EXTINCTION_FLOOR | 3 agents | Collapse threshold |
| EXTINCTION_WINDOW | 50 epochs | Collapse recovery window |
| ATP decay rate | 2%/epoch | Natural balance reduction |
| Treasury skim | 5% | Income tax to treasury |
| Cortex interval | 25 epochs | Homeostatic evaluation period |
| Cortex cooldown | 50 epochs | Per-field mutation cooldown |

## Appendix B: SHA-256 Hash Registry

The complete hash registry for all 38 experiments is published in `replication_status.json` at the repository root. Each entry contains: experiment name, result hash, manifest hash, world count, epoch count, seed, and season identifier.

## Appendix C: Collapse Definition Sensitivity

The s4_full_attack configuration (zero resource regeneration, death draining resource pools, all safety mechanisms disabled, 10× reproduction cost, soft cap sweep 30–180, 20 runs per step, 500 epochs, base seed 42) was executed under varying extinction floor definitions while holding all other parameters constant. The extinction window ($N_w = 50$) was held fixed. Each configuration produced 120 world-runs.

### C.1 Collapse Rate by Floor Definition

| $P_{\text{floor}}$ | Collapse Rate | Collapsed / Total | Result Hash |
|---------------------|---------------|-------------------|-------------|
| 3 (default) | 0.0% | 0 / 120 | `5595008e...` |
| 5 | 5.8% | 7 / 120 | `846275ab...` |
| 10 | 97.5% | 117 / 120 | `4f30c867...` |
| 15 | 100.0% | 120 / 120 | `84fa372e...` |
| 20 | 100.0% | 120 / 120 | `0c2d4311...` |

### C.2 Interpretation

The collapse boundary under the s4_full_attack configuration lies between $P_{\text{floor}} = 5$ (5.8% collapse) and $P_{\text{floor}} = 10$ (97.5% collapse). This indicates that populations under maximal stress stabilize in the range of 3–8 agents. The sharp phase transition between floor 5 and floor 10 implies that the system's equilibrium population under extreme conditions is narrowly bounded.

At $P_{\text{floor}} = 3$, the default definition, all 120 worlds survive. At $P_{\text{floor}} = 5$, 7 of 120 worlds (5.8%) collapse — meaning populations in those worlds dropped below 5 for 50+ consecutive epochs. At $P_{\text{floor}} = 10$, 117 of 120 worlds (97.5%) register as collapsed, despite containing living agents. The 3 surviving worlds at $P_{\text{floor}} = 10$ are those at higher soft cap values where the population can sustain above 10.

This analysis shows that the zero-collapse headline result is sensitive to the permissiveness of the collapse definition. Under stricter but reasonable definitions, the system's characterization changes from "universally surviving" to "mostly collapsed." Both characterizations are accurate descriptions of the same underlying population dynamics; they differ only in where the observation threshold is placed.

### C.3 Minimum Population Distribution (s4_full_attack, $P_{\text{floor}} = 3$)

| Soft Cap | Min Pop | Mean Pop | P10 Pop | P90 Pop |
|----------|---------|----------|---------|---------|
| 30 | 3 | 5.8 | 3 | 9 |
| 60 | 3 | 8.2 | 4 | 13 |
| 90 | 3 | 10.1 | 5 | 16 |
| 120 | 3 | 12.4 | 6 | 19 |
| 150 | 3 | 14.8 | 7 | 23 |
| 180 | 3 | 16.5 | 8 | 26 |

Minimum populations hit the floor (3) at all soft cap values, confirming that even at the highest carrying capacities tested, the population transiently reaches near-extinction levels. Mean populations scale approximately linearly with soft cap.

## Appendix D: Fitness Weight Robustness

The s4_full_attack configuration was executed with systematic ±20% perturbations of each individual fitness weight, with renormalization to maintain $\sum w_i = 1.0$. One weight was perturbed at a time (increased or decreased by 20% of its default value), and all four weights were then divided by their new sum. This produced 8 perturbation variants plus 1 default baseline, each comprising 120 world-runs.

### D.1 Results

| Variant | CE ($w_0$) | SQ ($w_1$) | RF ($w_2$) | CC ($w_3$) | Collapse Rate | Collapsed / Total |
|---------|-----------|-----------|-----------|-----------|---------------|-------------------|
| Default | 0.2500 | 0.3000 | 0.2000 | 0.2500 | 0.0% | 0 / 120 |
| CE −20% | 0.2105 | 0.3158 | 0.2105 | 0.2632 | 0.0% | 0 / 120 |
| CE +20% | 0.2857 | 0.2857 | 0.1905 | 0.2381 | 0.8% | 1 / 120 |
| SQ −20% | 0.2660 | 0.2553 | 0.2128 | 0.2660 | 0.0% | 0 / 120 |
| SQ +20% | 0.2358 | 0.3396 | 0.1887 | 0.2358 | 0.0% | 0 / 120 |
| RF −20% | 0.2604 | 0.3125 | 0.1667 | 0.2604 | 0.8% | 1 / 120 |
| RF +20% | 0.2404 | 0.2885 | 0.2308 | 0.2404 | 0.0% | 0 / 120 |
| CC −20% | 0.2632 | 0.3158 | 0.2105 | 0.2105 | 0.0% | 0 / 120 |
| CC +20% | 0.2381 | 0.2857 | 0.1905 | 0.2857 | 0.0% | 0 / 120 |

### D.2 Interpretation

Maximum observed collapse rate under any ±20% perturbation: 0.8% (1 world out of 120). The two variants producing collapse (CE +20% and RF −20%) both shift weight away from Resource Foraging ($w_2$), which governs extraction efficiency. This is consistent with resource extraction being a critical survival pathway under zero-regeneration conditions.

The system is highly weight-insensitive within the tested perturbation range. The zero-collapse result does not depend on precise weight calibration. However, this analysis is limited to small perturbations around the default weights; qualitatively different weight distributions (e.g., single-trait dominance, uniform weights at $w = 0.25$ across all dimensions) or nonlinear fitness functions remain untested.

### D.3 Methodology Note

Custom weights are applied to the `SelectionEngine` selection and replication pathways only. Resource extraction efficiency in the world simulation continues to use the default fitness function. A complete override of all fitness-dependent code paths would provide a stronger robustness guarantee but was not implemented for this analysis.
