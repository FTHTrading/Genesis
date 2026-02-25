# Parameter Range Rationale

This document records the reasoning behind parameter ranges chosen for each sweep variable. Where a choice is arbitrary, it is stated as such.

## 1. Season 1 Parameters

### Entropy Coefficient (0.00001 – 0.0001)

The default value is 0.00002. The sweep spans 0.5× to 5× the default. The lower bound was chosen as the point where entropy tax becomes negligible relative to agent income. The upper bound was chosen to stress-test supply contraction without making every agent's balance trivially negative within a few epochs.

**Justification quality**: Moderate. Bounds are order-of-magnitude estimates, not derived from any analytical threshold.

### Catastrophe Base Probability (0.0 – 0.03)

Default: 0.002. The upper bound (0.03) produces approximately one catastrophe every 33 epochs — frequent enough to stress populations but below the frequency where catastrophes dominate all other dynamics.

**Justification quality**: Moderate. The 0.03 cap is a design judgment.

### Gini Wealth Tax Threshold (0.2 – 0.9)

Full range from aggressive redistribution (0.2 = tax triggers at very low inequality) to laissez-faire (0.9 = tax almost never triggers). This captures the full policy space.

**Justification quality**: Strong. The range covers the mechanism's full operational extent.

### Treasury Overflow Threshold (0.1 – 0.9)

Full operational range. At 0.1, the treasury redistributes aggressively (almost any reserve triggers overflow). At 0.9, redistribution almost never occurs.

**Justification quality**: Strong.

### Replication Cost Multiplier (1.0 – 5.0 / 10.0)

1× is baseline. 5× prices most agents out of reproduction. 10× (dual inversion) makes reproduction a severe bottleneck. These are stress-test extremes, not calibrated to any empirical system.

**Justification quality**: Low. Chosen for stress coverage, not theoretical grounding.

### Basal Cost Multiplier (1.0 – 10.0)

1× is baseline. 10× makes survival metabolically expensive but still possible for high-fitness agents with adequate resource extraction. Beyond 10×, agent death would occur within 1–2 epochs regardless of fitness.

**Justification quality**: Moderate. Upper bound is set at the threshold of immediate lethality.

### Soft Cap (30 – 180)

30 is the lowest carrying capacity where non-trivial population dynamics can occur (below ~20, stochastic effects dominate). 180 is the default EarthPrime value and the standard upper bound.

**Justification quality**: Moderate.

## 2. Season 2 Design

Season 2 does not sweep economic parameters — it sweeps structural mechanism enablement. The design is combinatorial:

| Stage | Mechanisms Disabled | Rationale |
|---|---|---|
| S1 | Treasury redistribution only | Isolate treasury contribution |
| S2 | ATP decay only | Isolate monetary policy contribution |
| S3 | Pairwise and complete combinations of: decay, treasury, grants, extinction floor | Factorial coverage of mechanism interactions |
| S4 | All S3 + resource regeneration + death drainage + 10× reproduction cost | Maximum structural stress |

Each stage retains the soft cap sweep (30–180) to provide an internal gradient.

**Hostile variants** (S1, S2) additionally disable mutation and cortex and increase entropy 5× and catastrophe to maximum. These represent worst-case-within-mechanism-disabled scenarios.

## 3. Sensitivity Analysis Parameters

### Floor Sensitivity (3, 5, 10, 15, 20)

Chosen to bracket the observed population equilibrium (~8 agents) under s4_full_attack. Floor = 3 is the default; floor = 5 and 10 bracket the equilibrium; floor = 15 and 20 confirm universal collapse above the equilibrium band.

**Justification quality**: Strong. Grid points are placed at diagnostically informative thresholds.

### Weight Perturbation (±20%)

20% perturbation was chosen as a meaningful but non-extreme deviation. It preserves the qualitative weight ordering (SQ highest) while testing sensitivity. Larger perturbations (e.g., ±50%) would change the qualitative structure and are noted as untested.

**Justification quality**: Moderate. 20% is a conventional sensitivity range in parameter studies but is not uniquely justified.
