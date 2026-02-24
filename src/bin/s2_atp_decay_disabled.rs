// Genesis Protocol — Season 2: Structural Invariant S2
//
// ATP Decay Disabled — Collapse Boundary Detection
// Tests whether removing the 2% per-epoch ATP decay causes wealth immortality,
// demographic stagnation, and reproductive starvation cascade.
//
// Hypothesis: Without ATP decay, wealthy organisms persist indefinitely,
// monopolizing reproductive capacity and starving new entrants.
//
// Enhanced inequality instrumentation captures:
// - ATP distribution variance and wealth concentration
// - Reproductive and survival inequality by ATP quartile
// - Top-decile persistence (wealth immortality proxy)
// - Median/mean ATP divergence (skew detector)
//
// Usage: cargo run --release --bin s2_atp_decay_disabled

use genesis_experiment::{ExperimentRunner, ExperimentReport, FlagshipExperiments};
use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  SEASON 2 — COLLAPSE BOUNDARY: S2 ATP DECAY DISABLED   ║");
    println!("║     Structural Invariant Violation Experiment           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let output_dir = "experiments/season2";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let experiments: Vec<(&str, genesis_experiment::ExperimentConfig)> = vec![
        ("s2_atp_decay_disabled_baseline", FlagshipExperiments::s2_atp_decay_disabled_baseline()),
        ("s2_atp_decay_disabled_hostile", FlagshipExperiments::s2_atp_decay_disabled_hostile()),
    ];

    let global_start = Instant::now();
    let mut all_findings: Vec<(String, Vec<String>)> = Vec::new();

    for (slug, config) in &experiments {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Experiment: {}", config.name);
        println!("  Hypothesis: {}", config.hypothesis);
        println!("  Worlds: {} | Epochs/world: {} | Total epochs: {}",
            config.total_worlds(),
            config.epochs_per_run,
            config.total_worlds() as u64 * config.epochs_per_run,
        );
        println!("  ATP Decay: DISABLED (0% per epoch, normally 2%)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let start = Instant::now();
        let result = ExperimentRunner::run(config);
        let elapsed = start.elapsed();

        println!("  Completed in {:.2}s", elapsed.as_secs_f64());
        println!("  Total epochs run: {}", result.total_epochs_run);
        println!("  Result hash: {}", &result.result_hash);
        println!();

        // Analyze collapse data + inequality metrics
        let findings = derive_s2_findings(slug, &result);
        for (i, f) in findings.iter().enumerate() {
            println!("  Finding {}: {}", i + 1, f);
        }
        println!();

        // Generate report
        let report = ExperimentReport::generate(&result, findings.clone());
        let dir = format!("{}/{}", output_dir, slug);
        report.save_to_dir_with_slug(&dir, Some(slug)).expect("Failed to save report");
        println!("  Saved: {}/", dir);
        println!();

        all_findings.push((config.name.clone(), findings));
    }

    // ─── Cross-experiment synthesis ─────────────────────────────────
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  S2 ATP DECAY DISABLED — SYNTHESIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    for (name, findings) in &all_findings {
        println!("\n  {}:", name);
        for f in findings {
            println!("    • {}", f);
        }
    }

    // Cross-experiment inequality comparison
    println!("\n  ── CROSS-EXPERIMENT INEQUALITY COMPARISON ──");
    let baseline_trials: Vec<_> = all_findings.iter()
        .find(|(n, _)| n.contains("Baseline"))
        .map(|(_, f)| f.clone())
        .unwrap_or_default();
    let hostile_trials: Vec<_> = all_findings.iter()
        .find(|(n, _)| n.contains("Hostile"))
        .map(|(_, f)| f.clone())
        .unwrap_or_default();

    if !baseline_trials.is_empty() && !hostile_trials.is_empty() {
        println!("    Baseline findings: {}", baseline_trials.len());
        println!("    Hostile findings: {}", hostile_trials.len());
    }

    let elapsed = global_start.elapsed();
    println!("\n  Total time: {:.2}s", elapsed.as_secs_f64());
    println!("  Output directory: {}/", output_dir);
}

fn derive_s2_findings(_slug: &str, result: &genesis_experiment::ExperimentResult) -> Vec<String> {
    let mut findings = Vec::new();

    // Flatten all trials from all steps
    let all_trials: Vec<_> = result.steps.iter()
        .flat_map(|s| s.trials.iter())
        .collect();

    let total_trials = all_trials.len();
    let collapsed_trials: Vec<_> = all_trials.iter()
        .filter(|t| t.collapse_epoch.is_some())
        .collect();
    let collapse_count = collapsed_trials.len();
    let collapse_rate = collapse_count as f64 / total_trials as f64 * 100.0;

    findings.push(format!(
        "Collapse rate: {}/{} trials ({:.1}%)",
        collapse_count, total_trials, collapse_rate
    ));

    if collapse_count > 0 {
        // Time to collapse statistics
        let collapse_epochs: Vec<u64> = collapsed_trials.iter()
            .map(|t| t.collapse_epoch.unwrap())
            .collect();
        let min_collapse = *collapse_epochs.iter().min().unwrap();
        let max_collapse = *collapse_epochs.iter().max().unwrap();
        let mean_collapse = collapse_epochs.iter().sum::<u64>() as f64 / collapse_epochs.len() as f64;

        findings.push(format!(
            "Time to collapse: min={}, max={}, mean={:.1} epochs",
            min_collapse, max_collapse, mean_collapse
        ));

        // Collapse by sweep step
        for step in &result.steps {
            let step_total = step.trials.len();
            let step_collapsed = step.trials.iter().filter(|t| t.collapse_epoch.is_some()).count();
            let rate = step_collapsed as f64 / step_total as f64 * 100.0;
            findings.push(format!(
                "  cap={:.0}: {}/{} collapsed ({:.1}%)",
                step.parameter_value, step_collapsed, step_total, rate
            ));
        }
    } else {
        findings.push("NO COLLAPSES — ATP decay removal did NOT cause extinction".to_string());
    }

    // ─── INEQUALITY METRICS (S2-specific) ───────────────────────────

    findings.push("── Inequality Instrumentation ──".to_string());

    // Wealth Concentration Index (top 10% share)
    let wci: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("wealth_concentration_index").copied())
        .collect();
    if !wci.is_empty() {
        let mean_wci = wci.iter().sum::<f64>() / wci.len() as f64;
        let max_wci = wci.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_wci = wci.iter().cloned().fold(f64::INFINITY, f64::min);
        findings.push(format!(
            "Wealth concentration (top 10%): mean={:.4}, max={:.4}, min={:.4}",
            mean_wci, max_wci, min_wci
        ));
        if mean_wci > 0.5 {
            findings.push("  ⚠ WEALTH OLIGARCHY: Top 10% controls >50% of ATP".to_string());
        }
    }

    // Mean Gini Coefficient
    let mgini: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("mean_gini_coefficient").copied())
        .collect();
    if !mgini.is_empty() {
        let mean_mgini = mgini.iter().sum::<f64>() / mgini.len() as f64;
        let max_mgini = mgini.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Mean Gini coefficient: mean={:.4}, max={:.4}",
            mean_mgini, max_mgini
        ));
        if mean_mgini > 0.6 {
            findings.push("  ⚠ SEVERE INEQUALITY: Gini > 0.6 indicates extreme wealth stratification".to_string());
        }
    }

    // Max Gini Coefficient (peak inequality)
    let xgini: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("max_gini_coefficient").copied())
        .collect();
    if !xgini.is_empty() {
        let mean_xgini = xgini.iter().sum::<f64>() / xgini.len() as f64;
        let max_xgini = xgini.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Max Gini coefficient: mean_of_maxes={:.4}, absolute_max={:.4}",
            mean_xgini, max_xgini
        ));
    }

    // Median/Mean ATP Divergence (skew detector)
    let mmd: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("median_mean_atp_divergence").copied())
        .collect();
    if !mmd.is_empty() {
        let mean_mmd = mmd.iter().sum::<f64>() / mmd.len() as f64;
        let max_mmd = mmd.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Median/Mean ATP divergence: mean={:.4}, max={:.4}",
            mean_mmd, max_mmd
        ));
        if mean_mmd > 0.3 {
            findings.push("  ⚠ RIGHT-SKEWED: Mean >> Median indicates wealth hoarding by few".to_string());
        }
    }

    // ATP Variance
    let atpv: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("atp_variance").copied())
        .collect();
    if !atpv.is_empty() {
        let mean_atpv = atpv.iter().sum::<f64>() / atpv.len() as f64;
        let max_atpv = atpv.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "ATP variance: mean={:.1}, max={:.1}",
            mean_atpv, max_atpv
        ));
    }

    // Reproductive Inequality Index
    let rii: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("reproductive_inequality_index").copied())
        .collect();
    if !rii.is_empty() {
        let mean_rii = rii.iter().sum::<f64>() / rii.len() as f64;
        let max_rii = rii.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Reproductive inequality: mean={:.4}, max={:.4} (fraction of births from top quartile)",
            mean_rii, max_rii
        ));
        if mean_rii > 0.5 {
            findings.push("  ⚠ REPRODUCTIVE MONOPOLY: Top quartile produces >50% of offspring".to_string());
        }
    }

    // Survival Inequality Index
    let sii: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("survival_inequality_index").copied())
        .collect();
    if !sii.is_empty() {
        let mean_sii = sii.iter().sum::<f64>() / sii.len() as f64;
        let max_sii = sii.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Survival inequality: mean={:.4}, max={:.4} (fraction of deaths from bottom quartile)",
            mean_sii, max_sii
        ));
        if mean_sii > 0.5 {
            findings.push("  ⚠ SURVIVAL APARTHEID: Bottom quartile suffers >50% of deaths".to_string());
        }
    }

    // Top Decile Persistence (wealth immortality proxy)
    let tdp: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("top_decile_persistence").copied())
        .collect();
    if !tdp.is_empty() {
        let mean_tdp = tdp.iter().sum::<f64>() / tdp.len() as f64;
        let max_tdp = tdp.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        findings.push(format!(
            "Top decile persistence: mean={:.4}, max={:.4} (fraction of epochs with concentrated wealth)",
            mean_tdp, max_tdp
        ));
        if mean_tdp > 0.8 {
            findings.push("  ⚠ WEALTH IMMORTALITY: Top decile maintains dominance >80% of simulation".to_string());
        }
    }

    // ─── POPULATION STATISTICS ──────────────────────────────────────

    findings.push("── Population & Economy ──".to_string());

    // Population statistics
    let min_pops: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("min_population").copied())
        .collect();
    if !min_pops.is_empty() {
        let global_min = min_pops.iter().cloned().fold(f64::INFINITY, f64::min);
        let mean_min = min_pops.iter().sum::<f64>() / min_pops.len() as f64;
        findings.push(format!(
            "Population floor: global_min={:.0}, mean_min={:.1}",
            global_min, mean_min
        ));
    }

    // Mean population
    let mean_pops: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("mean_population").copied())
        .collect();
    if !mean_pops.is_empty() {
        let mean_pop = mean_pops.iter().sum::<f64>() / mean_pops.len() as f64;
        let min_pop = mean_pops.iter().cloned().fold(f64::INFINITY, f64::min);
        findings.push(format!(
            "Mean population: {:.1} (min trial: {:.1})", mean_pop, min_pop
        ));
    }

    // Treasury reserve (should grow without decay)
    let max_reserves: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("max_treasury_reserve").copied())
        .collect();
    if !max_reserves.is_empty() {
        let global_max = max_reserves.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean_max = max_reserves.iter().sum::<f64>() / max_reserves.len() as f64;
        findings.push(format!(
            "Treasury accumulation: global_max={:.1}, mean_max={:.1} ATP",
            global_max, mean_max
        ));
    }

    // Mean fitness
    let fitnesses: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("mean_fitness").copied())
        .collect();
    if !fitnesses.is_empty() {
        let mean_fit = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        findings.push(format!("Mean fitness: {:.4}", mean_fit));
    }

    // ─── PER-STEP BREAKDOWN ─────────────────────────────────────────

    findings.push("── Per-step breakdown ──".to_string());
    for step in &result.steps {
        let step_collapse_pct = step.collapse_rate * 100.0;
        let step_pop = step.metric_summaries.get("mean_population").map(|s| s.mean).unwrap_or(0.0);
        let step_gini = step.metric_summaries.get("mean_gini_coefficient").map(|s| s.mean).unwrap_or(0.0);
        let step_wci = step.metric_summaries.get("wealth_concentration_index").map(|s| s.mean).unwrap_or(0.0);
        let step_rii = step.metric_summaries.get("reproductive_inequality_index").map(|s| s.mean).unwrap_or(0.0);
        findings.push(format!(
            "  cap={:.0}: collapse={:.1}%, pop={:.1}, gini={:.4}, wci={:.4}, repro_ineq={:.4}",
            step.parameter_value, step_collapse_pct, step_pop, step_gini, step_wci, step_rii
        ));
    }

    // ─── VERDICT ────────────────────────────────────────────────────

    findings.push("── VERDICT ──".to_string());

    // Classify the outcome
    let has_collapse = collapse_count > 0;
    let mean_wci_val = wci.iter().sum::<f64>() / wci.len().max(1) as f64;
    let mean_rii_val = rii.iter().sum::<f64>() / rii.len().max(1) as f64;
    let mean_sii_val = sii.iter().sum::<f64>() / sii.len().max(1) as f64;
    let mean_tdp_val = tdp.iter().sum::<f64>() / tdp.len().max(1) as f64;
    let mean_mgini_val = mgini.iter().sum::<f64>() / mgini.len().max(1) as f64;

    if has_collapse {
        findings.push(format!(
            "ATP DECAY IS NECESSARY — {:.1}% collapse rate proves decay prevents extinction",
            collapse_rate
        ));
        if mean_wci_val > 0.4 || mean_rii_val > 0.4 {
            findings.push(
                "Collapse mechanism: WEALTH CONCENTRATION → REPRODUCTIVE STARVATION CASCADE".to_string()
            );
        }
    } else {
        // Check for sub-collapse pathologies
        let mut pathologies = Vec::new();
        if mean_wci_val > 0.4 {
            pathologies.push(format!("wealth_concentration={:.3}", mean_wci_val));
        }
        if mean_mgini_val > 0.5 {
            pathologies.push(format!("gini={:.3}", mean_mgini_val));
        }
        if mean_rii_val > 0.4 {
            pathologies.push(format!("repro_inequality={:.3}", mean_rii_val));
        }
        if mean_sii_val > 0.4 {
            pathologies.push(format!("survival_inequality={:.3}", mean_sii_val));
        }
        if mean_tdp_val > 0.7 {
            pathologies.push(format!("wealth_persistence={:.3}", mean_tdp_val));
        }

        if pathologies.is_empty() {
            findings.push(
                "ATP decay is NOT necessary — system self-stabilizes without decay and shows no inequality pathology".to_string()
            );
        } else {
            findings.push(format!(
                "NO COLLAPSE but PATHOLOGICAL INEQUALITY detected: [{}]",
                pathologies.join(", ")
            ));
            findings.push(
                "ATP decay absence creates survivable but degenerate economy — wealth immortality without extinction".to_string()
            );
        }
    }

    findings
}
