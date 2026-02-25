// Genesis Protocol — Sensitivity Analysis
//
// Collapse Definition Sensitivity + Fitness Weight Robustness
//
// Addresses two peer-review concerns:
//   1. Is the 0% collapse rate an artifact of the permissive extinction floor (P_floor=3)?
//   2. Are results sensitive to the specific fitness weight vector [0.25, 0.30, 0.20, 0.25]?
//
// Experiment A: Re-runs S4 Full Attack with extinction_floor ∈ {3, 5, 10, 15, 20}
// Experiment B: Re-runs S4 Full Attack with fitness weights perturbed ±20%
//
// Usage: cargo run --release --bin sensitivity_analysis

use genesis_experiment::{ExperimentRunner, ExperimentReport, FlagshipExperiments};
use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  SENSITIVITY ANALYSIS — COLLAPSE DEFINITION & FITNESS   ║");
    println!("║     Adversarial Hardening for Peer Review               ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let output_dir = "experiments/sensitivity";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let global_start = Instant::now();

    // ═══════════════════════════════════════════════════════════════════
    // EXPERIMENT A: Extinction Floor Sensitivity
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  PART A: EXTINCTION FLOOR SENSITIVITY");
    println!("  Base experiment: S4 Full Attack (most extreme stress test)");
    println!("  Testing P_floor ∈ {{3, 5, 10, 15, 20}}, window = 50 epochs");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let floor_values: Vec<usize> = vec![3, 5, 10, 15, 20];
    let mut floor_results: Vec<(usize, f64, usize, usize)> = Vec::new(); // (floor, collapse_rate, collapsed, total)

    for &floor in &floor_values {
        let mut config = FlagshipExperiments::s4_full_attack();
        config.name = format!("S4 Full Attack [P_floor={}]", floor);
        config.extinction_floor_override = Some(floor);
        // Keep same base_seed for reproducibility comparison

        println!("  Running: {} (floor={}, {} worlds, {} epochs/world)",
            config.name, floor, config.total_worlds(), config.epochs_per_run);

        let start = Instant::now();
        let result = ExperimentRunner::run(&config);
        let elapsed = start.elapsed();

        // Compute collapse rate from step results
        let total_trials: usize = result.steps.iter().map(|s| s.trial_count).sum();
        let total_collapses: usize = result.steps.iter()
            .map(|s| {
                let cr = s.metric_summaries.get("collapsed")
                    .map(|m| m.mean)
                    .unwrap_or(0.0);
                (cr * s.trial_count as f64).round() as usize
            })
            .sum();
        let collapse_rate = if total_trials > 0 { total_collapses as f64 / total_trials as f64 } else { 0.0 };

        println!("    Completed in {:.2}s | Collapse rate: {}/{} ({:.1}%)",
            elapsed.as_secs_f64(), total_collapses, total_trials, collapse_rate * 100.0);
        println!("    Hash: {}", &result.result_hash);

        // Save per min_population stats
        for step in &result.steps {
            if let Some(min_pop) = step.metric_summaries.get("min_population") {
                println!("    SoftCap={:.0}: min_pop mean={:.1}, min={:.0}, max={:.0}, p10={:.0}",
                    step.parameter_value, min_pop.mean, min_pop.min, min_pop.max, min_pop.p10);
            }
        }
        println!();

        floor_results.push((floor, collapse_rate, total_collapses, total_trials));

        // Save report
        let report = ExperimentReport::generate(&result, vec![
            format!("Extinction floor sensitivity test: P_floor={}", floor),
            format!("Collapse rate: {}/{} ({:.1}%)", total_collapses, total_trials, collapse_rate * 100.0),
        ]);
        let dir = format!("{}/floor_{}", output_dir, floor);
        report.save_to_dir_with_slug(&dir, Some(&format!("sensitivity_floor_{}", floor)))
            .expect("Failed to save report");
    }

    // ─── Floor Sensitivity Summary Table ─────────────────────────────
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  EXTINCTION FLOOR SENSITIVITY — SUMMARY TABLE");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  {:>8} | {:>12} | {:>10} | {:>10}", "P_floor", "Collapse %", "Collapsed", "Total");
    println!("  {:-<8}-+-{:-<12}-+-{:-<10}-+-{:-<10}", "", "", "", "");
    for (floor, rate, collapsed, total) in &floor_results {
        println!("  {:>8} | {:>11.1}% | {:>10} | {:>10}", floor, rate * 100.0, collapsed, total);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════
    // EXPERIMENT B: Fitness Weight Perturbation
    // ═══════════════════════════════════════════════════════════════════
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  PART B: FITNESS WEIGHT ROBUSTNESS");
    println!("  Default weights: [CE=0.25, SQ=0.30, RF=0.20, CC=0.25]");
    println!("  Testing ±20% perturbations on each weight (renormalized)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let default_weights = [0.25, 0.30, 0.20, 0.25];
    let weight_names = ["CE", "SQ", "RF", "CC"];
    let mut weight_results: Vec<(String, [f64; 4], f64, usize, usize)> = Vec::new();

    // Baseline (default weights, for comparison hash)
    {
        let mut config = FlagshipExperiments::s4_full_attack();
        config.name = "S4 Full Attack [default weights]".into();
        // No fitness_weights override = default behavior

        println!("  Running: {} (baseline comparison)", config.name);
        let start = Instant::now();
        let result = ExperimentRunner::run(&config);
        let elapsed = start.elapsed();

        let total_trials: usize = result.steps.iter().map(|s| s.trial_count).sum();
        let total_collapses: usize = result.steps.iter()
            .map(|s| {
                let cr = s.metric_summaries.get("collapsed")
                    .map(|m| m.mean)
                    .unwrap_or(0.0);
                (cr * s.trial_count as f64).round() as usize
            })
            .sum();
        let collapse_rate = if total_trials > 0 { total_collapses as f64 / total_trials as f64 } else { 0.0 };

        println!("    Completed in {:.2}s | Collapse rate: {}/{} ({:.1}%)",
            elapsed.as_secs_f64(), total_collapses, total_trials, collapse_rate * 100.0);
        println!("    Hash: {}", &result.result_hash);
        println!();

        weight_results.push(("default".into(), default_weights, collapse_rate, total_collapses, total_trials));
    }

    // Perturb each weight ±20%, renormalize
    for (i, wname) in weight_names.iter().enumerate() {
        for direction in &[-0.20_f64, 0.20_f64] {
            let mut weights = default_weights;
            weights[i] *= 1.0 + direction;

            // Renormalize to sum to 1.0
            let sum: f64 = weights.iter().sum();
            for w in &mut weights {
                *w /= sum;
            }

            let label = format!("{}{}20%", wname,
                if *direction > 0.0 { "+" } else { "-" });

            let mut config = FlagshipExperiments::s4_full_attack();
            config.name = format!("S4 Full Attack [{}]", label);
            config.fitness_weights = Some(weights);

            println!("  Running: {} | weights=[{:.3}, {:.3}, {:.3}, {:.3}]",
                config.name, weights[0], weights[1], weights[2], weights[3]);

            let start = Instant::now();
            let result = ExperimentRunner::run(&config);
            let elapsed = start.elapsed();

            let total_trials: usize = result.steps.iter().map(|s| s.trial_count).sum();
            let total_collapses: usize = result.steps.iter()
                .map(|s| {
                    let cr = s.metric_summaries.get("collapsed")
                        .map(|m| m.mean)
                        .unwrap_or(0.0);
                    (cr * s.trial_count as f64).round() as usize
                })
                .sum();
            let collapse_rate = if total_trials > 0 { total_collapses as f64 / total_trials as f64 } else { 0.0 };

            println!("    Completed in {:.2}s | Collapse rate: {}/{} ({:.1}%)",
                elapsed.as_secs_f64(), total_collapses, total_trials, collapse_rate * 100.0);
            println!("    Hash: {}", &result.result_hash);
            println!();

            weight_results.push((label.clone(), weights, collapse_rate, total_collapses, total_trials));

            // Save report
            let report = ExperimentReport::generate(&result, vec![
                format!("Fitness weight perturbation: {}", label),
                format!("Weights: [{:.4}, {:.4}, {:.4}, {:.4}]", weights[0], weights[1], weights[2], weights[3]),
                format!("Collapse rate: {}/{} ({:.1}%)", total_collapses, total_trials, collapse_rate * 100.0),
            ]);
            let dir = format!("{}/weights_{}", output_dir, label.to_lowercase().replace("+", "plus").replace("-", "minus"));
            report.save_to_dir_with_slug(&dir, Some(&format!("sensitivity_{}", label.to_lowercase())))
                .expect("Failed to save report");
        }
    }

    // ─── Weight Perturbation Summary Table ───────────────────────────
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  FITNESS WEIGHT ROBUSTNESS — SUMMARY TABLE");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  {:>10} | {:>7} {:>7} {:>7} {:>7} | {:>12} | {:>10}",
        "Variant", "CE", "SQ", "RF", "CC", "Collapse %", "N");
    println!("  {:-<10}-+-{:-<7}-{:-<7}-{:-<7}-{:-<7}-+-{:-<12}-+-{:-<10}", "", "", "", "", "", "", "");
    for (label, w, rate, _collapsed, total) in &weight_results {
        println!("  {:>10} | {:>7.4} {:>7.4} {:>7.4} {:>7.4} | {:>11.1}% | {:>10}",
            label, w[0], w[1], w[2], w[3], rate * 100.0, total);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════
    // GRAND SUMMARY
    // ═══════════════════════════════════════════════════════════════════
    let total_elapsed = global_start.elapsed();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  SENSITIVITY ANALYSIS COMPLETE");
    println!("  Total time: {:.1}s", total_elapsed.as_secs_f64());
    println!("  Floor tests: {} configurations", floor_values.len());
    println!("  Weight tests: {} configurations (1 baseline + {} perturbations)",
        weight_results.len(), weight_results.len() - 1);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
