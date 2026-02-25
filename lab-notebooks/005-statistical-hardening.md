# Lab Notebook 005: Statistical Hardening

**Date**: 2026-02-24/25
**Author**: Kevan Burns

## Motivation

The paper reported "zero collapses in 5,680 worlds" without confidence intervals, power analysis, or effect sizes. An adversarial reviewer would correctly identify this as statistically naive.

Specific gaps identified:
1. Zero events does not mean the true rate is zero — need upper bound
2. No statement about whether N = 5,680 is sufficient to detect low rates
3. No distributional data (means without CIs)
4. No effect sizes for treatment comparisons
5. No formal null hypothesis test

## Statistical Method Choices

### Clopper-Pearson Exact CIs (Not Wilson or Wald)

For binomial proportions, three common CI methods exist:
- **Wald**: Normal approximation. Known to be poor near p = 0 or p = 1.
- **Wilson**: Corrected normal approximation. Better coverage.
- **Clopper-Pearson**: Exact, based on inverting the binomial test. Conservative (true coverage ≥ nominal).

Chose Clopper-Pearson because:
1. Many of our proportions are at or near 0 or 1, where Wald and Wilson degrade
2. It is the most conservative choice — reviewers cannot critique it as anti-conservative
3. It is the standard in medical device testing and clinical trials when sample sizes are moderate

### Rule of Three

For zero-event data: p < 3/N at 95% confidence.

This is a well-known approximation that follows from 1 - (1-p)^N = 0.05 → p ≈ 3/N for large N. It gives an intuitive upper bound without requiring distributional calculations.

### Bootstrap CIs for Distributional Statistics

Used BCa (bias-corrected and accelerated) bootstrap with 10,000 resamples for means and other non-proportion statistics. BCa corrects for skewness in the bootstrap distribution.

### Cohen's d for Effect Sizes

Standard pooled-standard-deviation effect size. Used because:
1. Widely understood across disciplines
2. Provides magnitude context that p-values alone do not
3. Effect sizes > 1.0 are unambiguously large; sizes > 10 (as in metabolic_inversion, d = 12.6) indicate qualitatively different regimes

## Key Statistical Findings

### Power Analysis Result

At N = 5,680, power to detect p = 0.001 is 99.66%.

This means: if the true collapse rate were 0.1%, we have only a 0.34% chance of seeing zero collapses. The zero count is therefore informative — it constrains the true rate to well below 0.1%.

### Phase Transition Statistical Significance

The jump from 5.8% (CI: [2.4%, 11.6%]) to 97.5% (CI: [92.9%, 99.5%]) is statistically unambiguous. The CIs do not overlap. No statistical test is needed — the effect is visible by inspection.

### Weight Perturbation Non-Significance

The two collapses (1/120 each, CI: [0.02%, 4.6%]) are consistent with noise. The CIs overlap with zero. A reviewer might argue these represent Type I error rather than true fragility. This is a valid interpretation that we do not claim to resolve.

## Decisions Not Made

1. **Multiple testing correction**: We compute CIs for many experiments but do not apply Bonferroni or FDR correction. Reason: the CIs are descriptive, not used for hypothesis testing. No single "significant" result is claimed from multiple comparisons.

2. **Bayesian intervals**: Chose frequentist CIs because they are more widely understood and require no prior specification. A Bayesian analysis with a uniform prior would give similar results (Jeffreys interval is numerically close to Clopper-Pearson for these sample sizes).

3. **Regression modeling**: The data structure (discrete experiments with aggregate statistics) does not lend itself to regression. Per-world time series would be needed for proper modeling. This is an acknowledged limitation.
