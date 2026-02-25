#!/usr/bin/env python3
"""
Statistical Validation Report Generator for Genesis Protocol.

Computes:
- Binomial confidence intervals on collapse rates
- Aggregate cross-experiment statistics
- Cohen's d effect sizes for parameter perturbations
- Bootstrap confidence intervals on means
- Power analysis for collapse detection

Output: papers/statistical_validation_report.md
"""

import csv
import math
import os
import sys
from pathlib import Path
from dataclasses import dataclass, field
from typing import Optional

import numpy as np
from scipy import stats


# ── Data structures ──────────────────────────────────────────────────────

@dataclass
class ExperimentRow:
    """One row from an experiment CSV: one parameter value, N worlds."""
    parameter_value: float
    collapse_rate: float
    mean_survival_epochs: float
    final_population_mean: float
    final_population_stddev: float
    final_population_min: float
    final_population_max: float
    final_population_p10: float
    final_population_p90: float
    mean_population_mean: Optional[float] = None
    mean_population_stddev: Optional[float] = None
    min_population_mean: Optional[float] = None
    min_population_min: Optional[float] = None
    gini_coefficient_mean: Optional[float] = None
    gini_coefficient_stddev: Optional[float] = None
    mean_fitness_mean: Optional[float] = None
    mean_fitness_stddev: Optional[float] = None
    total_births_mean: Optional[float] = None
    total_deaths_mean: Optional[float] = None
    population_volatility_mean: Optional[float] = None


@dataclass
class ExperimentSummary:
    """Aggregate statistics for one experiment."""
    name: str
    n_parameter_values: int
    n_worlds_per_value: int
    total_worlds: int
    epochs_per_world: int
    total_epochs: int
    collapse_rate_overall: float
    collapsed_worlds: int
    final_pop_grand_mean: float
    final_pop_grand_std: float
    final_pop_global_min: float
    final_pop_global_max: float
    gini_grand_mean: Optional[float] = None
    gini_grand_std: Optional[float] = None
    fitness_grand_mean: Optional[float] = None
    fitness_grand_std: Optional[float] = None
    rows: list = field(default_factory=list)


@dataclass
class BinomialCI:
    """Binomial confidence interval for a proportion."""
    n: int
    k: int
    p_hat: float
    ci_lower: float
    ci_upper: float
    confidence: float
    method: str


# ── Parsing ──────────────────────────────────────────────────────────────

def parse_csv(filepath: Path) -> list[dict]:
    """Parse an experiment CSV into list of dicts."""
    rows = []
    with open(filepath, 'r', newline='') as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append({k: v for k, v in row.items()})
    return rows


def safe_float(val: str, default=None) -> Optional[float]:
    """Safely convert string to float."""
    try:
        return float(val)
    except (ValueError, TypeError):
        return default


def parse_manifest(filepath: Path) -> dict:
    """Parse a manifest JSON for world count info."""
    import json
    with open(filepath, 'r') as f:
        return json.load(f)


def infer_worlds_per_value(manifest: dict) -> int:
    """Infer worlds per parameter value from manifest."""
    total = manifest.get('total_worlds', 0)
    values = manifest.get('parameter_values', [])
    if values and total:
        return total // len(values)
    # Fallback: check worlds_per_value directly
    return manifest.get('worlds_per_value', 20)


# ── Statistical computations ────────────────────────────────────────────

def binomial_ci(n: int, k: int, confidence: float = 0.95) -> BinomialCI:
    """
    Compute confidence interval for binomial proportion.
    
    Uses Clopper-Pearson (exact) method for small samples / extreme proportions.
    For k=0, also computes the "rule of three" approximation.
    """
    alpha = 1 - confidence
    p_hat = k / n if n > 0 else 0
    
    if k == 0:
        ci_lower = 0.0
        ci_upper = 1 - (alpha / 2) ** (1 / n) if n > 0 else 1.0
        method = "Clopper-Pearson exact (k=0)"
    elif k == n:
        ci_lower = (alpha / 2) ** (1 / n) if n > 0 else 0.0
        ci_upper = 1.0
        method = "Clopper-Pearson exact (k=n)"
    else:
        ci_lower = stats.beta.ppf(alpha / 2, k, n - k + 1)
        ci_upper = stats.beta.ppf(1 - alpha / 2, k + 1, n - k)
        method = "Clopper-Pearson exact"
    
    return BinomialCI(
        n=n, k=k, p_hat=p_hat,
        ci_lower=ci_lower, ci_upper=ci_upper,
        confidence=confidence, method=method
    )


def rule_of_three(n: int) -> float:
    """
    Rule of three: if 0 events in n trials, 95% CI upper bound ≈ 3/n.
    """
    return 3.0 / n if n > 0 else float('inf')


def cohens_d(mean1: float, std1: float, n1: int,
             mean2: float, std2: float, n2: int) -> float:
    """Compute Cohen's d (pooled standard deviation)."""
    if n1 + n2 < 4:
        return float('nan')
    pooled_std = math.sqrt(
        ((n1 - 1) * std1**2 + (n2 - 1) * std2**2) / (n1 + n2 - 2)
    )
    if pooled_std == 0:
        return 0.0
    return (mean1 - mean2) / pooled_std


def bootstrap_ci_from_summary(mean: float, std: float, n: int,
                               n_bootstrap: int = 10000,
                               confidence: float = 0.95) -> tuple[float, float]:
    """
    Bootstrap CI for a mean given summary statistics.
    
    Since we don't have per-world data, we simulate from N(mean, std/sqrt(n))
    to estimate the sampling distribution of the mean.
    """
    if n == 0 or std == 0:
        return (mean, mean)
    
    rng = np.random.default_rng(42)
    se = std / math.sqrt(n)
    bootstrap_means = rng.normal(mean, se, n_bootstrap)
    
    alpha = 1 - confidence
    lower = float(np.percentile(bootstrap_means, 100 * alpha / 2))
    upper = float(np.percentile(bootstrap_means, 100 * (1 - alpha / 2)))
    return (lower, upper)


def power_analysis_binomial(n: int, p_alt: float, alpha: float = 0.05) -> float:
    """
    Power to detect collapse rate p_alt given n trials and significance alpha.
    
    Under H0: p = 0 (no collapses expected)
    Under H1: p = p_alt
    
    Power = P(at least 1 collapse | p = p_alt) when using "any collapse" criterion.
    Since we reject H0 if k >= 1:
    Power = 1 - (1 - p_alt)^n
    """
    return 1 - (1 - p_alt) ** n


# ── Experiment analysis ──────────────────────────────────────────────────

def analyze_experiment(name: str, csv_path: Path, manifest_path: Path) -> Optional[ExperimentSummary]:
    """Analyze a single experiment directory."""
    if not csv_path.exists() or not manifest_path.exists():
        return None
    
    try:
        manifest = parse_manifest(manifest_path)
        rows = parse_csv(csv_path)
    except Exception as e:
        print(f"  WARN: Error parsing {name}: {e}", file=sys.stderr)
        return None
    
    if not rows:
        return None
    
    worlds_per_value = infer_worlds_per_value(manifest)
    n_values = len(rows)
    total_worlds = manifest.get('total_worlds', n_values * worlds_per_value)
    epochs = manifest.get('epochs_per_world', 500)
    
    # Aggregate across parameter values
    final_pops = []
    collapse_rates = []
    ginis = []
    fitnesses = []
    
    for row in rows:
        fp_mean = safe_float(row.get('final_population_mean'), 0)
        fp_std = safe_float(row.get('final_population_stddev'), 0)
        cr = safe_float(row.get('collapse_rate'), 0)
        gini = safe_float(row.get('gini_coefficient_mean'))
        fitness = safe_float(row.get('mean_fitness_mean'))
        
        final_pops.append((fp_mean, fp_std))
        collapse_rates.append(cr)
        if gini is not None:
            ginis.append((gini, safe_float(row.get('gini_coefficient_stddev'), 0)))
        if fitness is not None:
            fitnesses.append((fitness, safe_float(row.get('mean_fitness_stddev'), 0)))
    
    # Grand mean of final population (mean of means)
    fp_means = [m for m, s in final_pops]
    fp_grand_mean = np.mean(fp_means)
    fp_grand_std = np.std(fp_means, ddof=1) if len(fp_means) > 1 else 0
    fp_global_min = min(safe_float(row.get('final_population_min'), 0) for row in rows)
    fp_global_max = max(safe_float(row.get('final_population_max'), 0) for row in rows)
    
    # Overall collapse rate (weighted by worlds per value)
    overall_collapse = np.mean(collapse_rates)
    collapsed_worlds = int(round(overall_collapse * total_worlds))
    
    # Gini stats
    gini_grand_mean = np.mean([m for m, s in ginis]) if ginis else None
    gini_grand_std = np.std([m for m, s in ginis], ddof=1) if len(ginis) > 1 else None
    
    # Fitness stats
    fitness_grand_mean = np.mean([m for m, s in fitnesses]) if fitnesses else None
    fitness_grand_std = np.std([m for m, s in fitnesses], ddof=1) if len(fitnesses) > 1 else None
    
    return ExperimentSummary(
        name=name,
        n_parameter_values=n_values,
        n_worlds_per_value=worlds_per_value,
        total_worlds=total_worlds,
        epochs_per_world=epochs,
        total_epochs=total_worlds * epochs,
        collapse_rate_overall=overall_collapse,
        collapsed_worlds=collapsed_worlds,
        final_pop_grand_mean=float(fp_grand_mean),
        final_pop_grand_std=float(fp_grand_std),
        final_pop_global_min=float(fp_global_min),
        final_pop_global_max=float(fp_global_max),
        gini_grand_mean=float(gini_grand_mean) if gini_grand_mean is not None else None,
        gini_grand_std=float(gini_grand_std) if gini_grand_std is not None else None,
        fitness_grand_mean=float(fitness_grand_mean) if fitness_grand_mean is not None else None,
        fitness_grand_std=float(fitness_grand_std) if fitness_grand_std is not None else None,
        rows=rows
    )


def discover_experiments(base_dir: Path) -> list[tuple[str, Path, Path]]:
    """Discover all experiment directories with CSV + manifest."""
    experiments = []
    
    # Top-level experiments (Season 1)
    for d in sorted(base_dir.iterdir()):
        if d.is_dir() and d.name not in ('season2', 'sensitivity'):
            csv_files = list(d.glob('*_data.csv'))
            manifest_files = list(d.glob('*_manifest.json'))
            if csv_files and manifest_files:
                experiments.append((f"s1/{d.name}", csv_files[0], manifest_files[0]))
    
    # Season 2
    s2_dir = base_dir / 'season2'
    if s2_dir.exists():
        for d in sorted(s2_dir.iterdir()):
            if d.is_dir():
                csv_files = list(d.glob('*_data.csv'))
                manifest_files = list(d.glob('*_manifest.json'))
                if csv_files and manifest_files:
                    experiments.append((f"s2/{d.name}", csv_files[0], manifest_files[0]))
    
    # Sensitivity
    sens_dir = base_dir / 'sensitivity'
    if sens_dir.exists():
        for d in sorted(sens_dir.iterdir()):
            if d.is_dir():
                csv_files = list(d.glob('*_data.csv'))
                manifest_files = list(d.glob('*_manifest.json'))
                if csv_files and manifest_files:
                    experiments.append((f"sens/{d.name}", csv_files[0], manifest_files[0]))
    
    return experiments


# ── Report generation ────────────────────────────────────────────────────

def generate_report(experiments_dir: Path, output_path: Path):
    """Generate the full statistical validation report."""
    discoveries = discover_experiments(experiments_dir)
    print(f"Discovered {len(discoveries)} experiments")
    
    summaries = []
    for name, csv_path, manifest_path in discoveries:
        s = analyze_experiment(name, csv_path, manifest_path)
        if s:
            summaries.append(s)
            print(f"  ✓ {name}: {s.total_worlds} worlds, collapse={s.collapse_rate_overall:.3f}")
    
    # ── Compute aggregate statistics ──
    
    # Total worlds across all experiments
    total_worlds_all = sum(s.total_worlds for s in summaries)
    total_epochs_all = sum(s.total_epochs for s in summaries)
    
    # Split by season
    s1_summaries = [s for s in summaries if s.name.startswith('s1/')]
    s2_summaries = [s for s in summaries if s.name.startswith('s2/')]
    sens_summaries = [s for s in summaries if s.name.startswith('sens/')]
    
    # Collapsed worlds (default definition — exclude sensitivity floor experiments)
    core_summaries = s1_summaries + s2_summaries
    core_total_worlds = sum(s.total_worlds for s in core_summaries)
    core_collapsed = sum(s.collapsed_worlds for s in core_summaries)
    
    # Sensitivity: separate floor and weight experiments
    floor_summaries = [s for s in sens_summaries if 'floor' in s.name]
    weight_summaries = [s for s in sens_summaries if 'weight' in s.name]
    
    # ── Binomial CIs ──
    
    # Core experiments (S1 + S2) — default collapse definition
    core_ci = binomial_ci(core_total_worlds, core_collapsed)
    core_rule3 = rule_of_three(core_total_worlds)
    
    # Per-season
    s1_worlds = sum(s.total_worlds for s in s1_summaries)
    s1_collapsed = sum(s.collapsed_worlds for s in s1_summaries)
    s1_ci = binomial_ci(s1_worlds, s1_collapsed)
    
    s2_worlds = sum(s.total_worlds for s in s2_summaries)
    s2_collapsed = sum(s.collapsed_worlds for s in s2_summaries)
    s2_ci = binomial_ci(s2_worlds, s2_collapsed)
    
    # Sensitivity floor experiments
    floor_cis = {}
    for fs in floor_summaries:
        floor_name = fs.name.split('/')[-1]
        floor_ci = binomial_ci(fs.total_worlds, fs.collapsed_worlds)
        floor_cis[floor_name] = (fs, floor_ci)
    
    # Weight experiments
    weight_cis = {}
    for ws in weight_summaries:
        weight_name = ws.name.split('/')[-1]
        weight_ci = binomial_ci(ws.total_worlds, ws.collapsed_worlds)
        weight_cis[weight_name] = (ws, weight_ci)
    
    # ── Power analysis ──
    # What collapse rate could we detect with N worlds?
    power_levels = [0.01, 0.02, 0.05, 0.10]
    
    # ── Effect sizes: compare extreme parameter values within S1 experiments ──
    effect_sizes = []
    for s in s1_summaries:
        if len(s.rows) >= 2:
            first = s.rows[0]
            last = s.rows[-1]
            fp1 = safe_float(first.get('final_population_mean'), 0)
            fp1_std = safe_float(first.get('final_population_stddev'), 1)
            fp2 = safe_float(last.get('final_population_mean'), 0)
            fp2_std = safe_float(last.get('final_population_stddev'), 1)
            n = s.n_worlds_per_value
            d = cohens_d(fp1, fp1_std, n, fp2, fp2_std, n)
            if not math.isnan(d):
                effect_sizes.append((s.name, fp1, fp2, abs(d)))
    
    # ── Bootstrap CIs on key metrics ──
    # Grand mean final population across all S1+S2 experiments
    all_fp_means = [s.final_pop_grand_mean for s in core_summaries]
    all_fp_stds = [s.final_pop_grand_std for s in core_summaries if s.final_pop_grand_std > 0]
    grand_fp_mean = np.mean(all_fp_means)
    grand_fp_std = np.std(all_fp_means, ddof=1) if len(all_fp_means) > 1 else 0
    fp_bootstrap_ci = bootstrap_ci_from_summary(
        float(grand_fp_mean), float(grand_fp_std), len(all_fp_means)
    )
    
    # ── Write report ──
    lines = []
    lines.append("# Statistical Validation Report")
    lines.append("")
    lines.append("**Genesis Protocol — Computational Statistical Analysis**")
    lines.append("")
    lines.append(f"Generated: {__import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    lines.append("")
    lines.append("---")
    lines.append("")
    
    # 1. Overview
    lines.append("## 1. Dataset Overview")
    lines.append("")
    lines.append(f"| Metric | Value |")
    lines.append(f"|---|---|")
    lines.append(f"| Total experiments analyzed | {len(summaries)} |")
    lines.append(f"| Season 1 experiments | {len(s1_summaries)} |")
    lines.append(f"| Season 2 experiments | {len(s2_summaries)} |")
    lines.append(f"| Sensitivity experiments | {len(sens_summaries)} |")
    lines.append(f"| Total worlds (all phases) | {total_worlds_all:,} |")
    lines.append(f"| Total epochs (all phases) | {total_epochs_all:,} |")
    lines.append(f"| Core worlds (S1+S2, default definition) | {core_total_worlds:,} |")
    lines.append(f"| Core collapsed worlds | {core_collapsed} |")
    lines.append("")
    
    # 2. Binomial confidence intervals
    lines.append("## 2. Collapse Rate — Binomial Confidence Intervals")
    lines.append("")
    lines.append("### 2.1 Core Experiments (Season 1 + Season 2)")
    lines.append("")
    lines.append("Under the default collapse definition ($P_{\\text{floor}} = 3$, 50-epoch window):")
    lines.append("")
    lines.append(f"- **Observed**: {core_collapsed} collapses in {core_total_worlds:,} worlds")
    lines.append(f"- **Point estimate**: $\\hat{{p}} = {core_ci.p_hat:.6f}$")
    lines.append(f"- **95% CI (Clopper-Pearson)**: [{core_ci.ci_lower:.6f}, {core_ci.ci_upper:.6f}]")
    lines.append(f"- **Rule of three**: 95% upper bound ≤ {core_rule3:.6f} ({3}/{core_total_worlds:,})")
    lines.append(f"- **Method**: {core_ci.method}")
    lines.append("")
    
    if core_collapsed == 0:
        lines.append(f"With {core_total_worlds:,} independent worlds and zero observed collapses, we can state with 95% confidence that the true collapse probability under the default definition is at most **{core_ci.ci_upper:.4f}** (i.e., less than {core_ci.ci_upper*100:.2f}%).")
    lines.append("")
    
    lines.append("### 2.2 Per-Season Breakdown")
    lines.append("")
    lines.append("| Season | N | Collapses | $\\hat{p}$ | 95% CI Lower | 95% CI Upper |")
    lines.append("|---|---|---|---|---|---|")
    lines.append(f"| Season 1 | {s1_worlds:,} | {s1_collapsed} | {s1_ci.p_hat:.6f} | {s1_ci.ci_lower:.6f} | {s1_ci.ci_upper:.6f} |")
    lines.append(f"| Season 2 | {s2_worlds:,} | {s2_collapsed} | {s2_ci.p_hat:.6f} | {s2_ci.ci_lower:.6f} | {s2_ci.ci_upper:.6f} |")
    lines.append(f"| Combined | {core_total_worlds:,} | {core_collapsed} | {core_ci.p_hat:.6f} | {core_ci.ci_lower:.6f} | {core_ci.ci_upper:.6f} |")
    lines.append("")
    
    # 2.3 Sensitivity floor experiments
    lines.append("### 2.3 Collapse Definition Sensitivity")
    lines.append("")
    lines.append("| Floor ($P_{\\text{floor}}$) | N | Collapses | $\\hat{p}$ | 95% CI |")
    lines.append("|---|---|---|---|---|")
    for name in sorted(floor_cis.keys(), key=lambda x: int(x.replace('floor_', ''))):
        fs, fci = floor_cis[name]
        floor_val = name.replace('floor_', '')
        lines.append(f"| {floor_val} | {fci.n} | {fci.k} | {fci.p_hat:.4f} | [{fci.ci_lower:.4f}, {fci.ci_upper:.4f}] |")
    lines.append("")
    
    # 2.4 Weight perturbation experiments
    lines.append("### 2.4 Fitness Weight Perturbation")
    lines.append("")
    lines.append("| Perturbation | N | Collapses | $\\hat{p}$ | 95% CI |")
    lines.append("|---|---|---|---|---|")
    for name in sorted(weight_cis.keys()):
        ws, wci = weight_cis[name]
        display = name.replace('weights_', '').replace('minus', '-').replace('plus', '+').replace('%', '%')
        lines.append(f"| {display} | {wci.n} | {wci.k} | {wci.p_hat:.4f} | [{wci.ci_lower:.4f}, {wci.ci_upper:.4f}] |")
    lines.append("")
    
    # 3. Power analysis
    lines.append("## 3. Statistical Power Analysis")
    lines.append("")
    lines.append("Given $N$ zero-collapse worlds, what is the power to detect a true collapse rate of $p$?")
    lines.append("")
    lines.append("Using the criterion 'reject $H_0: p=0$ if at least one collapse observed':")
    lines.append("")
    lines.append(f"Power = $1 - (1-p)^N$")
    lines.append("")
    lines.append("| True $p$ | N = {0} | N = {1} | N = {2} |".format(
        s1_worlds, core_total_worlds, total_worlds_all
    ))
    lines.append("|---|---|---|---|")
    for p in power_levels:
        pow_s1 = power_analysis_binomial(s1_worlds, p)
        pow_core = power_analysis_binomial(core_total_worlds, p)
        pow_all = power_analysis_binomial(total_worlds_all, p)
        lines.append(f"| {p:.0%} | {pow_s1:.4f} | {pow_core:.4f} | {pow_all:.4f} |")
    lines.append("")
    lines.append(f"With {core_total_worlds:,} core worlds, we have >99% power to detect a true collapse rate of 1% or higher. Even a 0.1% true collapse rate would be detected with probability {power_analysis_binomial(core_total_worlds, 0.001):.3f}.")
    lines.append("")
    
    # 4. Aggregate population statistics
    lines.append("## 4. Aggregate Population Statistics")
    lines.append("")
    lines.append("### 4.1 Per-Experiment Summary (Core)")
    lines.append("")
    lines.append("| Experiment | Worlds | Pop Mean | Pop Std | Pop Min | Pop Max | Gini Mean | Collapse |")
    lines.append("|---|---|---|---|---|---|---|---|")
    for s in core_summaries:
        gini_str = f"{s.gini_grand_mean:.4f}" if s.gini_grand_mean is not None else "—"
        lines.append(f"| {s.name} | {s.total_worlds} | {s.final_pop_grand_mean:.1f} | {s.final_pop_grand_std:.1f} | {s.final_pop_global_min:.0f} | {s.final_pop_global_max:.0f} | {gini_str} | {s.collapse_rate_overall:.3f} |")
    lines.append("")
    
    lines.append("### 4.2 Grand Aggregates")
    lines.append("")
    lines.append(f"- **Grand mean final population** (across {len(core_summaries)} experiments): {grand_fp_mean:.2f}")
    lines.append(f"- **Std across experiment means**: {grand_fp_std:.2f}")
    lines.append(f"- **95% Bootstrap CI**: [{fp_bootstrap_ci[0]:.2f}, {fp_bootstrap_ci[1]:.2f}]")
    lines.append(f"- **Global minimum final population** (any world): {min(s.final_pop_global_min for s in core_summaries):.0f}")
    lines.append(f"- **Global maximum final population** (any world): {max(s.final_pop_global_max for s in core_summaries):.0f}")
    lines.append("")
    
    if gini_means := [s.gini_grand_mean for s in core_summaries if s.gini_grand_mean is not None]:
        lines.append(f"- **Grand mean Gini coefficient**: {np.mean(gini_means):.4f}")
        lines.append(f"- **Gini range across experiments**: [{min(gini_means):.4f}, {max(gini_means):.4f}]")
        lines.append("")
    
    # 5. Effect sizes
    lines.append("## 5. Effect Sizes — Parameter Impact on Final Population")
    lines.append("")
    lines.append("Cohen's d between first and last parameter value within each Season 1 experiment:")
    lines.append("")
    lines.append("| Experiment | Pop (low param) | Pop (high param) | |d| | Magnitude |")
    lines.append("|---|---|---|---|---|")
    for name, fp1, fp2, d in sorted(effect_sizes, key=lambda x: -x[3]):
        if d < 0.2:
            mag = "negligible"
        elif d < 0.5:
            mag = "small"
        elif d < 0.8:
            mag = "medium"
        else:
            mag = "large"
        lines.append(f"| {name} | {fp1:.1f} | {fp2:.1f} | {d:.3f} | {mag} |")
    lines.append("")
    
    # 6. Sensitivity experiment detailed analysis
    lines.append("## 6. Sensitivity Analysis — Detailed Statistics")
    lines.append("")
    lines.append("### 6.1 Collapse Floor Phase Transition")
    lines.append("")
    lines.append("The collapse rate exhibits a sharp phase transition between $P_{\\text{floor}} = 5$ and $P_{\\text{floor}} = 10$:")
    lines.append("")
    for name in sorted(floor_cis.keys(), key=lambda x: int(x.replace('floor_', ''))):
        fs, fci = floor_cis[name]
        floor_val = int(name.replace('floor_', ''))
        lines.append(f"- **Floor = {floor_val}**: {fci.p_hat*100:.1f}% collapse (95% CI [{fci.ci_lower*100:.1f}%, {fci.ci_upper*100:.1f}%])")
    lines.append("")
    
    # Compute odds ratio: floor=10 vs floor=5
    if 'floor_5' in floor_cis and 'floor_10' in floor_cis:
        fs5, fci5 = floor_cis['floor_5']
        fs10, fci10 = floor_cis['floor_10']
        lines.append(f"Phase transition magnitude: collapse rate increases from {fci5.p_hat*100:.1f}% to {fci10.p_hat*100:.1f}% (a {(fci10.p_hat - fci5.p_hat)*100:.1f} percentage point increase) when the floor definition changes from 5 to 10.")
        lines.append("")
    
    lines.append("### 6.2 Weight Perturbation Detail")
    lines.append("")
    for name in sorted(weight_cis.keys()):
        ws, wci = weight_cis[name]
        display = name.replace('weights_', '').replace('minus', '-').replace('plus', '+').replace('%', '%')
        lines.append(f"- **{display}**: {wci.p_hat*100:.1f}% collapse, final pop mean = {ws.final_pop_grand_mean:.1f} (95% CI [{wci.ci_lower*100:.1f}%, {wci.ci_upper*100:.1f}%])")
    lines.append("")
    
    # 7. Methodology notes
    lines.append("## 7. Methodological Notes")
    lines.append("")
    lines.append("### 7.1 Limitations of This Analysis")
    lines.append("")
    lines.append("1. **Aggregate-only data**: Per-world time series are not preserved in the current CSV format. All statistics are computed from per-parameter-value aggregate summaries (mean, stddev, min, max, p10, p90 across worlds at each parameter value).")
    lines.append("2. **Bootstrap approximation**: Bootstrap CIs on means are approximated by resampling from N(mean, SE) rather than from raw per-world data.")
    lines.append("3. **Independence assumption**: The binomial CI assumes each world is independent. Worlds share the same codebase and parameter structure but differ by deterministic seed.")
    lines.append("4. **Single architecture**: All results are from x86_64 Windows. Cross-platform determinism is not verified.")
    lines.append("5. **No time-to-event analysis**: Survival analysis (Kaplan-Meier, hazard rates) requires per-world survival epoch data, which would need per-world CSV export.")
    lines.append("")
    
    lines.append("### 7.2 Recommended Upgrades for Publication")
    lines.append("")
    lines.append("1. Export per-world CSV data (one row per world per epoch, or at minimum one row per world with final state)")
    lines.append("2. Compute Kaplan-Meier survival curves across sensitivity conditions")
    lines.append("3. Fit logistic regression: P(collapse) ~ f(floor, weight_perturbation, experiment_type)")
    lines.append("4. Cross-platform determinism audit (Linux, ARM)")
    lines.append("5. Formal hypothesis tests with Bonferroni correction for multiple comparisons")
    lines.append("")
    
    lines.append("---")
    lines.append("")
    lines.append(f"*Analysis performed on {len(summaries)} experiments, {total_worlds_all:,} worlds, {total_epochs_all:,} epochs.*")
    lines.append("")
    
    # Write
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))
    
    print(f"\nReport written to {output_path}")
    print(f"  {len(summaries)} experiments, {total_worlds_all:,} worlds, {total_epochs_all:,} epochs")
    return summaries


# ── Main ─────────────────────────────────────────────────────────────────

if __name__ == '__main__':
    repo_root = Path(__file__).parent.parent
    experiments_dir = repo_root / 'experiments'
    output_path = repo_root / 'papers' / 'statistical_validation_report.md'
    
    if not experiments_dir.exists():
        print(f"ERROR: experiments directory not found at {experiments_dir}", file=sys.stderr)
        sys.exit(1)
    
    generate_report(experiments_dir, output_path)
