// Genesis Protocol — Season 2: Structural Invariant S1
//
// Treasury Cycling Disabled — Collapse Boundary Detection
// Tests whether removing treasury redistribution causes population collapse.
//
// Usage: cargo run --release --bin s1_treasury_disabled

use genesis_experiment::{ExperimentRunner, ExperimentReport, FlagshipExperiments};
use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  SEASON 2 — COLLAPSE BOUNDARY: S1 TREASURY DISABLED    ║");
    println!("║     Structural Invariant Violation Experiment           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let output_dir = "experiments/season2";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let experiments: Vec<(&str, genesis_experiment::ExperimentConfig)> = vec![
        ("s1_treasury_disabled_baseline", FlagshipExperiments::s1_treasury_disabled_baseline()),
        ("s1_treasury_disabled_hostile", FlagshipExperiments::s1_treasury_disabled_hostile()),
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
        println!("  Treasury Cycling: DISABLED");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let start = Instant::now();
        let result = ExperimentRunner::run(config);
        let elapsed = start.elapsed();

        println!("  Completed in {:.2}s", elapsed.as_secs_f64());
        println!("  Total epochs run: {}", result.total_epochs_run);
        println!("  Result hash: {}", &result.result_hash);
        println!();

        // Analyze collapse data
        let findings = derive_s1_findings(slug, &result);
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
    println!("  S1 TREASURY DISABLED — SYNTHESIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    for (name, findings) in &all_findings {
        println!("\n  {}:", name);
        for f in findings {
            println!("    • {}", f);
        }
    }

    let elapsed = global_start.elapsed();
    println!("\n  Total time: {:.2}s", elapsed.as_secs_f64());
    println!("  Output directory: {}/", output_dir);
}

fn derive_s1_findings(_slug: &str, result: &genesis_experiment::ExperimentResult) -> Vec<String> {
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

        // Collapse by sweep step (parameter value = soft_cap tier)
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
        findings.push("NO COLLAPSES — treasury cycling removal did NOT cause extinction".to_string());
    }

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

    // Treasury reserve statistics
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

    // Mean population statistics
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

    // Mean fitness
    let fitnesses: Vec<f64> = all_trials.iter()
        .filter_map(|t| t.metrics.get("mean_fitness").copied())
        .collect();
    if !fitnesses.is_empty() {
        let mean_fit = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        findings.push(format!("Mean fitness: {:.4}", mean_fit));
    }

    // Per-step collapse rates for summary
    findings.push("── Per-step collapse rates ──".to_string());
    for step in &result.steps {
        findings.push(format!(
            "  cap={:.0}: collapse={:.1}%, mean_survival={:.0}, pop={:.1}",
            step.parameter_value, step.collapse_rate * 100.0,
            step.mean_survival_epochs,
            step.metric_summaries.get("mean_population").map(|s| s.mean).unwrap_or(0.0),
        ));
    }

    // Verdict
    if collapse_count > 0 {
        findings.push(format!(
            "VERDICT: Treasury cycling is NECESSARY — {:.1}% collapse rate proves redistribution prevents extinction",
            collapse_rate
        ));
    } else {
        findings.push(
            "VERDICT: Treasury cycling is NOT necessary for survival at this configuration — system self-stabilizes without redistribution".to_string()
        );
    }

    findings
}
