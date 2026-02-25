# Lab Notebook 001: System Design Decisions

**Date**: 2025-12 through 2026-01
**Author**: Kevan Burns

## Decision: Agent Representation via SHA-256 Hash Chains

**Choice**: Agents are identified by SHA-256 hashes. Traits derived deterministically from hash bytes.

**Rationale**: Deterministic trait derivation eliminates RNG-induced variance in population genetics. Every agent's phenotype is fully determined by its genesis hash. This makes populations reproducible without saving per-agent state.

**Trade-off**: Trait distributions are governed by hash byte uniformity, not any empirical or theoretical distribution. The resulting trait space is uniformly distributed in [0,1]^4, which may not reflect realistic heterogeneity.

**Alternative considered**: Gaussian trait distributions with configurable mean/variance. Rejected because it introduces additional parameters without clear empirical grounding.

## Decision: Fitness Function as Weighted Linear Combination

**Choice**: F(a) = 0.25·CE + 0.30·SQ + 0.20·RF + 0.25·CC

**Rationale**: Linear combination is the simplest non-trivial aggregation. The slight asymmetry (SQ = 0.30) reflects a subjective judgment that strategic quality contributes more to long-term survival than any single capability.

**Trade-off**: The weights are arbitrary. Sensitivity analysis (Appendix D) shows ±20% perturbation produces ≤0.8% collapse rate change, but larger perturbations and nonlinear functions are untested.

**What I would do differently**: Pre-register multiple fitness functions (linear, multiplicative, min-of-traits) and run all three. Would strengthen the robustness claim significantly.

## Decision: ATP-Based Energy Economy

**Choice**: All transactions denominated in a single energy currency (ATP). Agents earn by extraction, pay basal costs, reproduce by spending.

**Rationale**: Single currency simplifies accounting and makes all agent interactions commensurable. The analogy to cellular metabolism (ATP as universal energy currency) provides biological grounding.

**Trade-off**: A single currency precludes barter, credit, or multi-commodity dynamics. The economy is simpler than any real economy.

## Decision: Adaptive Cortex as Engineered Controller

**Choice**: A hand-engineered PID-like controller with heuristic thresholds, evaluated every 25 epochs.

**Rationale**: Provides a stabilization layer that can be systematically disabled in Season 2. The research question is whether collapse emerges *despite* redundant safeguards, not whether collapse occurs in a minimal system.

**Trade-off**: The cortex confounds interpretation in Season 1 (is stability from parameters or from controller?). This is partially addressed by Season 2's staged disablement.

**Honest assessment**: If I were redesigning, I would run all Season 1 experiments both with and without the cortex active. The current design addresses this confound incompletely.

## Decision: Extinction Floor at P = 3

See [experimental-design/collapse-definition-rationale.md](../experimental-design/collapse-definition-rationale.md) for full analysis.

**Honest assessment**: The floor is permissive. This was a deliberate choice to define collapse conservatively, but the permissiveness is the single largest source of skepticism about the zero-collapse result. The sensitivity analysis in Appendix C was added specifically to address this.
