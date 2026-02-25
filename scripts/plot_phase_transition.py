#!/usr/bin/env python3
"""
Generate Figure 1: Collapse Phase Transition Under Strict Definition Criteria.

Plots collapse probability vs. extinction floor definition (P_floor) with
95% Clopper-Pearson confidence intervals. Data from s4_full_attack sensitivity
analysis (120 worlds per floor value).
"""

import numpy as np
import os
import sys

try:
    import matplotlib
    matplotlib.use('Agg')  # Non-interactive backend
    import matplotlib.pyplot as plt
    from matplotlib.patches import FancyArrowPatch
except ImportError:
    print("ERROR: matplotlib is required. Install with: pip install matplotlib")
    sys.exit(1)

try:
    from scipy.stats import beta as beta_dist
except ImportError:
    print("ERROR: scipy is required. Install with: pip install scipy")
    sys.exit(1)


def clopper_pearson(k, n, alpha=0.05):
    """Exact Clopper-Pearson confidence interval."""
    if k == 0:
        lo = 0.0
        hi = 1.0 - (alpha / 2.0) ** (1.0 / n)
    elif k == n:
        lo = (alpha / 2.0) ** (1.0 / n)
        hi = 1.0
    else:
        lo = beta_dist.ppf(alpha / 2.0, k, n - k + 1)
        hi = beta_dist.ppf(1.0 - alpha / 2.0, k + 1, n - k)
    return lo, hi


# --- Data: s4_full_attack floor sensitivity ---
floors =     [3,    5,    10,    15,    20]
collapses =  [0,    7,    117,   120,   120]
n_worlds =   [120,  120,  120,   120,   120]

rates = [k / n for k, n in zip(collapses, n_worlds)]
ci_lo = []
ci_hi = []
for k, n in zip(collapses, n_worlds):
    lo, hi = clopper_pearson(k, n)
    ci_lo.append(lo)
    ci_hi.append(hi)

rates_pct = [r * 100 for r in rates]
ci_lo_pct = [r * 100 for r in ci_lo]
ci_hi_pct = [r * 100 for r in ci_hi]
err_lo = [r - lo for r, lo in zip(rates_pct, ci_lo_pct)]
err_hi = [hi - r for r, hi in zip(rates_pct, ci_hi_pct)]

# --- Plot ---
fig, ax = plt.subplots(1, 1, figsize=(7, 4.5), dpi=200)

# Confidence band (shaded)
ax.fill_between(floors, ci_lo_pct, ci_hi_pct, alpha=0.15, color='#2166ac',
                label='95% CI (Clopper-Pearson)')

# Main line with markers
ax.plot(floors, rates_pct, 'o-', color='#2166ac', linewidth=2.2,
        markersize=8, markerfacecolor='white', markeredgewidth=2,
        markeredgecolor='#2166ac', zorder=5)

# Error bars
ax.errorbar(floors, rates_pct, yerr=[err_lo, err_hi], fmt='none',
            ecolor='#2166ac', elinewidth=1.2, capsize=4, capthick=1.2, zorder=4)

# Phase transition annotation
ax.annotate('Phase transition\n91.7 pp jump',
            xy=(7.5, 51.7), fontsize=9, fontweight='bold',
            color='#b2182b', ha='center', va='center',
            bbox=dict(boxstyle='round,pad=0.4', facecolor='#fddbc7',
                      edgecolor='#b2182b', alpha=0.85))

# Arrow from annotation to the transition region
ax.annotate('', xy=(5.3, 8), xytext=(6.8, 42),
            arrowprops=dict(arrowstyle='->', color='#b2182b', lw=1.5))
ax.annotate('', xy=(9.7, 95), xytext=(8.2, 61),
            arrowprops=dict(arrowstyle='->', color='#b2182b', lw=1.5))

# Default floor marker
ax.axvline(x=3, color='#4daf4a', linewidth=1, linestyle='--', alpha=0.6)
ax.text(3.15, 50, 'Default\n($P_{floor}=3$)', fontsize=8, color='#4daf4a',
        va='center', fontstyle='italic')

# Formatting
ax.set_xlabel('Extinction Floor ($P_{\\mathrm{floor}}$)', fontsize=11)
ax.set_ylabel('Collapse Rate (%)', fontsize=11)
ax.set_title('Figure 1: Collapse Phase Transition Under Strict Definition Criteria',
             fontsize=11, fontweight='bold', pad=12)

ax.set_xlim(1, 22)
ax.set_ylim(-5, 108)
ax.set_xticks(floors)
ax.set_yticks([0, 20, 40, 60, 80, 100])
ax.tick_params(labelsize=9)
ax.legend(loc='center left', fontsize=9, framealpha=0.9)

# Subtitle
ax.text(0.5, -0.14,
        's4_full_attack configuration · 120 worlds per floor · 500 epochs · seed 42',
        transform=ax.transAxes, fontsize=8, ha='center', color='#666666')

ax.spines['top'].set_visible(False)
ax.spines['right'].set_visible(False)
ax.grid(axis='y', alpha=0.3, linewidth=0.5)

plt.tight_layout()

# Save
out_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'papers', 'figures')
os.makedirs(out_dir, exist_ok=True)
out_path = os.path.join(out_dir, 'fig1_collapse_phase_transition.png')
fig.savefig(out_path, bbox_inches='tight', facecolor='white')
print(f"Saved: {out_path}")

# Also save PDF for LaTeX submission
out_pdf = os.path.join(out_dir, 'fig1_collapse_phase_transition.pdf')
fig.savefig(out_pdf, bbox_inches='tight', facecolor='white')
print(f"Saved: {out_pdf}")
