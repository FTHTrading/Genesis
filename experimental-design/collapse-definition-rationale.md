# Collapse Definition Rationale

## Chosen Definition

Population collapse at epoch $t$ is defined as:
- $|P(t)| = 0$ (total extinction), OR
- $|P(\tau)| < P_{\text{floor}}$ for all $\tau \in [t - N_w + 1, t]$ (sustained sub-threshold population)

With default parameters: $P_{\text{floor}} = 3$, $N_w = 50$.

## Why P_floor = 3

The minimum population for **demographic replacement** given the system's constraints:
- Maximum births per epoch: 3
- Single parent per birth (asexual reproduction)
- A population of 2 can sustain itself only if both agents reach reproductive threshold AND at least one reproduces before either dies
- A population of 3 provides a minimal buffer: if one dies, two remain, each capable of reproduction

This is the **lowest non-trivial threshold**. It is explicitly permissive. We document this permissiveness rather than hiding it.

### What would change at higher floors

| $P_{\text{floor}}$ | Consequence |
|---|---|
| 5 | 5.8% collapse under s4_full_attack (CI: [2.4%, 11.6%]) |
| 10 | 97.5% collapse under s4_full_attack (CI: [92.9%, 99.5%]) |
| 20 | 100% collapse under all extreme conditions |

The headline zero-collapse result exists only because the definition is permissive. This is stated explicitly in the paper abstract, Section 3.1, Section 8 (Limitations #4), and the Known Failure Modes document (Section 1.1).

## Why N_w = 50 Epochs

The recovery window must be long enough that transient dips are not falsely classified as collapse, but short enough that genuine secular decline is detected.

At $P_{\text{floor}} = 3$ with max 3 births/epoch and single-parent reproduction:
- From a population of 3, if all three agents are reproductively viable, the population could grow by up to 9 per epoch
- 50 epochs is sufficient for a viable population to recover to normal levels (~48 agents) from any non-lethal state
- If recovery has not occurred in 50 epochs, the conditions are structurally preventing it

### What would change at shorter windows

- $N_w = 10$: Would catch brief population dips that recover. Would increase collapse count in volatile runs without changing the fundamental finding.
- $N_w = 100$: Would make the definition even more permissive. Populations could spend long periods at floor levels without being classified as collapsed.

## Alternatives Considered

| Alternative | Definition | Reason Not Adopted |
|---|---|---|
| Demographic collapse | Birth/death ratio < 0.5 for 100 epochs | Would require birth/death tracking not currently in CSV output. Noted for future work. |
| Economic collapse | Mean ATP < basal cost for 50 epochs | Conflates poverty with population failure. A poor but surviving population is not collapsed. |
| Inequality collapse | Gini > 0.95 for 100 epochs | Conflates pathology with collapse. Documented separately in failure modes. |
| Stricter floor ($P_{\text{floor}} = 10$) | Population < 10 sustained | Would capture the majority of extreme-condition worlds (97.5%). Characterized in Appendix C. |

## Disclosure Statement

The collapse definition was chosen before experiments were run. No definition shopping occurred. However, the definition is objectively permissive, and the sensitivity analysis demonstrates that results reverse under stricter definitions. The paper foregrounds this sensitivity rather than presenting the zero-collapse result without context.
