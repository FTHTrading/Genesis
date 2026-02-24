// Genesis Protocol — Flagship Experiment Runner
//
// Runs all three flagship experiments, exports results to experiments/ directory.
// Usage: cargo run --bin run_experiments

use genesis_experiment::{
    ExperimentRunner, ExperimentReport, FlagshipExperiments,
};
use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║       GENESIS PROTOCOL — FLAGSHIP EXPERIMENTS          ║");
    println!("║           Phase 5: The Experimental Method             ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let output_dir = "experiments";
    std::fs::create_dir_all(output_dir).expect("Failed to create experiments/ directory");

    let experiments: Vec<(&str, genesis_experiment::ExperimentConfig)> = vec![
        ("entropy_sweep", FlagshipExperiments::entropy_sweep()),
        ("catastrophe_resilience", FlagshipExperiments::catastrophe_resilience()),
        ("inequality_threshold", FlagshipExperiments::inequality_threshold()),
        ("treasury_stability", FlagshipExperiments::treasury_stability()),
        ("evolution_forbidden", FlagshipExperiments::evolution_forbidden()),
    ];

    // Reserve Stress Suite — 4 shock tiers
    let reserve_suite = FlagshipExperiments::reserve_stress_suite();

    // Resource Depletion Suite — 4 carrying capacity tiers
    let depletion_suite = FlagshipExperiments::resource_depletion_suite();

    let mut all_findings: Vec<(String, Vec<String>)> = Vec::new();
    let mut reserve_findings: Vec<(String, String, Vec<String>)> = Vec::new(); // (slug, name, findings)
    let mut depletion_findings: Vec<(String, String, Vec<String>)> = Vec::new();
    let global_start = Instant::now();

    for (slug, config) in &experiments {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Experiment: {}", config.name);
        println!("  Hypothesis: {}", config.hypothesis);
        println!("  Worlds: {} | Epochs/world: {} | Total epochs: {}",
            config.total_worlds(),
            config.epochs_per_run,
            config.total_worlds() as u64 * config.epochs_per_run,
        );
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let start = Instant::now();
        let result = ExperimentRunner::run(config);
        let elapsed = start.elapsed();

        println!("  Completed in {:.2}s", elapsed.as_secs_f64());
        println!("  Total epochs run: {}", result.total_epochs_run);
        println!("  Result hash: {}", &result.result_hash);
        println!();

        // Derive findings from the data
        let findings = derive_findings(slug, &result);
        for (i, f) in findings.iter().enumerate() {
            println!("  Finding {}: {}", i + 1, f);
        }
        println!();

        // Generate report
        let report = ExperimentReport::generate(&result, findings.clone());

        // Save to experiments/<slug>/
        let dir = format!("{}/{}", output_dir, slug);
        report.save_to_dir_with_slug(&dir, Some(slug)).expect("Failed to save report");

        println!("  Saved: {}/", dir);
        println!("    - *_report.txt");
        println!("    - *_data.csv");
        println!("    - *_manifest.json");
        println!();

        all_findings.push((config.name.clone(), findings));
    }

    // ─── Reserve Stress Suite ───────────────────────────────────────
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║     RESERVE STRESS SUITE — 4 SHOCK TIERS               ║");
    println!("║       Domain-Specific Treasury Policy Modeling          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let mut reserve_results: Vec<(&str, genesis_experiment::ExperimentResult)> = Vec::new();

    for (slug, config) in &reserve_suite {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Experiment: {}", config.name);
        println!("  Hypothesis: {}", config.hypothesis);
        println!("  Worlds: {} | Epochs/world: {} | Total epochs: {}",
            config.total_worlds(),
            config.epochs_per_run,
            config.total_worlds() as u64 * config.epochs_per_run,
        );
        if let Some(ref p) = config.base_pressure_override {
            println!("  Shock baseline: catastrophe_base_prob = {:.4}", p.catastrophe_base_prob);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let start = Instant::now();
        let result = ExperimentRunner::run(config);
        let elapsed = start.elapsed();

        println!("  Completed in {:.2}s", elapsed.as_secs_f64());
        println!("  Total epochs run: {}", result.total_epochs_run);
        println!("  Result hash: {}", &result.result_hash);
        println!();

        // Per-tier findings
        let findings = derive_reserve_tier_findings(slug, &result);
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

        reserve_findings.push((slug.to_string(), config.name.clone(), findings));
        reserve_results.push((slug, result));
    }

    // ─── Cross-Tier Synthesis ───────────────────────────────────────────
    let synthesis = derive_reserve_cross_tier_synthesis(&reserve_results);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  RESERVE STRESS — CROSS-TIER SYNTHESIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    for (i, s) in synthesis.iter().enumerate() {
        println!("  Synthesis {}: {}", i + 1, s);
    }
    println!();
    all_findings.push(("Reserve Stress — Cross-Tier Synthesis".into(), synthesis));

    // ─── Resource Depletion Crossover ───────────────────────────────────
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   RESOURCE DEPLETION CROSSOVER — 4 CAPACITY TIERS      ║");
    println!("║       Metabolic Cost Under Carrying Capacity Compression║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    let mut depletion_results: Vec<(&str, genesis_experiment::ExperimentResult)> = Vec::new();

    for (slug, config) in &depletion_suite {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Experiment: {}", config.name);
        println!("  Hypothesis: {}", config.hypothesis);
        println!("  Worlds: {} | Epochs/world: {} | Total epochs: {}",
            config.total_worlds(),
            config.epochs_per_run,
            config.total_worlds() as u64 * config.epochs_per_run,
        );
        if let Some(ref p) = config.base_pressure_override {
            println!("  Carrying capacity: soft_cap = {}", p.soft_cap);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let start = Instant::now();
        let result = ExperimentRunner::run(config);
        let elapsed = start.elapsed();

        println!("  Completed in {:.2}s", elapsed.as_secs_f64());
        println!("  Total epochs run: {}", result.total_epochs_run);
        println!("  Result hash: {}", &result.result_hash);
        println!();

        // Per-tier findings
        let findings = derive_depletion_tier_findings(slug, &result);
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

        depletion_findings.push((slug.to_string(), config.name.clone(), findings));
        depletion_results.push((slug, result));
    }

    // ─── Resource Depletion Cross-Tier Synthesis ────────────────────────
    let depletion_synthesis = derive_depletion_cross_tier_synthesis(&depletion_results);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  RESOURCE DEPLETION — CROSS-TIER SYNTHESIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    for (i, s) in depletion_synthesis.iter().enumerate() {
        println!("  Synthesis {}: {}", i + 1, s);
    }
    println!();
    all_findings.push(("Resource Depletion — Cross-Tier Synthesis".into(), depletion_synthesis));

    let total_elapsed = global_start.elapsed();

    let total_worlds: usize = experiments.iter().map(|(_, c)| c.total_worlds()).sum::<usize>()
        + reserve_suite.iter().map(|(_, c)| c.total_worlds()).sum::<usize>()
        + depletion_suite.iter().map(|(_, c)| c.total_worlds()).sum::<usize>();

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                   ALL EXPERIMENTS COMPLETE              ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("  Total time: {:.2}s", total_elapsed.as_secs_f64());
    println!("  Total worlds spawned: {}", total_worlds);
    println!("  Output directory: {}/", output_dir);
    println!();

    // Print consolidated findings
    println!("── Consolidated Findings ──");
    println!();
    for (name, findings) in &all_findings {
        println!("  {}", name);
        for f in findings {
            println!("    • {}", f);
        }
        println!();
    }

    // Print per-tier findings
    println!("── Reserve Stress — Per-Tier Findings ──");
    println!();
    for (_, name, findings) in &reserve_findings {
        println!("  {}", name);
        for f in findings {
            println!("    • {}", f);
        }
        println!();
    }

    // Print resource depletion per-tier findings
    println!("── Resource Depletion — Per-Tier Findings ──");
    println!();
    for (_, name, findings) in &depletion_findings {
        println!("  {}", name);
        for f in findings {
            println!("    • {}", f);
        }
        println!();
    }
}

/// Derive findings from experiment results by analyzing the data.
fn derive_findings(slug: &str, result: &genesis_experiment::ExperimentResult) -> Vec<String> {
    let mut findings = Vec::new();

    match slug {
        "entropy_sweep" => {
            // Find collapse transition point
            let mut first_collapse_step = None;
            let mut first_total_collapse_step = None;
            for step in &result.steps {
                if step.collapse_rate > 0.0 && first_collapse_step.is_none() {
                    first_collapse_step = Some(step.clone());
                }
                if step.collapse_rate >= 1.0 && first_total_collapse_step.is_none() {
                    first_total_collapse_step = Some(step.clone());
                }
            }

            // Collapse analysis
            if let Some(ref step) = first_collapse_step {
                findings.push(format!(
                    "First collapse observed at entropy_coeff = {:.6} (collapse rate: {:.0}%)",
                    step.parameter_value, step.collapse_rate * 100.0
                ));
            } else {
                findings.push("No collapses observed across entire entropy range".into());
            }

            if let Some(ref step) = first_total_collapse_step {
                findings.push(format!(
                    "Total collapse (100%) occurs at entropy_coeff = {:.6}",
                    step.parameter_value
                ));
            }

            // Gini trajectory
            let gini_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            let gini_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (gini_first, gini_last) {
                let change_pct = ((last - first) / first.max(0.001)) * 100.0;
                findings.push(format!(
                    "Gini coefficient moves from {:.4} to {:.4} ({:+.1}%) across entropy range",
                    first, last, change_pct
                ));
            }

            // Population impact
            let pop_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_population"))
                .map(|s| s.mean);
            let pop_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_population"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (pop_first, pop_last) {
                findings.push(format!(
                    "Mean population declines from {:.1} to {:.1} as entropy increases",
                    first, last
                ));
            }

            // Mean fitness impact
            let fit_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            let fit_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (fit_first, fit_last) {
                findings.push(format!(
                    "Mean fitness shifts from {:.4} to {:.4} under increasing entropy pressure",
                    first, last
                ));
            }
        }

        "catastrophe_resilience" => {
            // Find survival sweet spot
            let mut best_survival_step = result.steps.first().cloned();
            let mut best_survival = 0.0f64;
            for step in &result.steps {
                if step.mean_survival_epochs > best_survival {
                    best_survival = step.mean_survival_epochs;
                    best_survival_step = Some(step.clone());
                }
            }

            if let Some(ref step) = best_survival_step {
                findings.push(format!(
                    "Peak mean survival ({:.1} epochs) at catastrophe_base_prob = {:.4}",
                    step.mean_survival_epochs, step.parameter_value
                ));
            }

            // Collapse curve
            let peaceful = result.steps.first().map(|s| s.collapse_rate).unwrap_or(0.0);
            let max_cat = result.steps.last().map(|s| s.collapse_rate).unwrap_or(0.0);
            findings.push(format!(
                "Collapse rate: {:.0}% at peaceful (0.0) vs {:.0}% at extreme (0.03)",
                peaceful * 100.0, max_cat * 100.0
            ));

            // Resilience analysis: compare mid-range vs extremes
            let mid_idx = result.steps.len() / 2;
            if let Some(mid_step) = result.steps.get(mid_idx) {
                let mid_pop = mid_step.metric_summaries.get("mean_population")
                    .map(|s| s.mean).unwrap_or(0.0);
                let first_pop = result.steps.first()
                    .and_then(|s| s.metric_summaries.get("mean_population"))
                    .map(|s| s.mean).unwrap_or(0.0);
                findings.push(format!(
                    "Mid-range catastrophe ({:.4}) produces mean pop {:.1} vs {:.1} at peaceful",
                    mid_step.parameter_value, mid_pop, first_pop
                ));
            }

            // Catastrophe death toll
            let cat_deaths_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("total_catastrophe_deaths"))
                .map(|s| s.mean);
            if let Some(deaths) = cat_deaths_last {
                findings.push(format!(
                    "Mean catastrophe deaths at extreme rate: {:.1}",
                    deaths
                ));
            }
        }

        "inequality_threshold" => {
            // Find optimal threshold by population stability
            let mut best_pop_step = result.steps.first().cloned();
            let mut best_pop = 0.0f64;
            for step in &result.steps {
                let pop = step.metric_summaries.get("mean_population")
                    .map(|s| s.mean).unwrap_or(0.0);
                if pop > best_pop {
                    best_pop = pop;
                    best_pop_step = Some(step.clone());
                }
            }

            if let Some(ref step) = best_pop_step {
                findings.push(format!(
                    "Optimal population stability at gini_threshold = {:.2} (mean pop: {:.1})",
                    step.parameter_value, best_pop
                ));
            }

            // Gini coefficient vs threshold
            let gini_low = result.steps.first()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            let gini_high = result.steps.last()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            if let (Some(low), Some(high)) = (gini_low, gini_high) {
                findings.push(format!(
                    "Gini: {:.4} at aggressive threshold (0.20) vs {:.4} at laissez-faire (0.90)",
                    low, high
                ));
            }

            // Fitness vs threshold
            let fit_low = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            let fit_high = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            if let (Some(low), Some(high)) = (fit_low, fit_high) {
                findings.push(format!(
                    "Mean fitness: {:.4} at aggressive redistribution vs {:.4} at laissez-faire",
                    low, high
                ));
            }

            // Collapse comparison
            let collapse_low = result.steps.first().map(|s| s.collapse_rate).unwrap_or(0.0);
            let collapse_high = result.steps.last().map(|s| s.collapse_rate).unwrap_or(0.0);
            findings.push(format!(
                "Collapse rate: {:.0}% (aggressive redistribution) vs {:.0}% (laissez-faire)",
                collapse_low * 100.0, collapse_high * 100.0
            ));

            // Treasury health
            let treasury_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("treasury_ratio"))
                .map(|s| s.mean);
            let treasury_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("treasury_ratio"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (treasury_first, treasury_last) {
                findings.push(format!(
                    "Treasury ratio: {:.4} at threshold 0.20 vs {:.4} at threshold 0.90",
                    first, last
                ));
            }
        }

        "treasury_stability" => {
            // Treasury ratio trajectory across overflow thresholds
            let treasury_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("treasury_ratio"))
                .map(|s| s.mean);
            let treasury_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("treasury_ratio"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (treasury_first, treasury_last) {
                let change_pct = ((last - first) / first.max(0.001)) * 100.0;
                findings.push(format!(
                    "Treasury ratio: {:.4} at aggressive deployment (0.10) vs {:.4} at hoarding (0.90) ({:+.1}%)",
                    first, last, change_pct
                ));
            }

            // Find optimal threshold by population stability
            let mut best_pop_step = result.steps.first().cloned();
            let mut best_pop = 0.0f64;
            for step in &result.steps {
                let pop = step.metric_summaries.get("mean_population")
                    .map(|s| s.mean).unwrap_or(0.0);
                if pop > best_pop {
                    best_pop = pop;
                    best_pop_step = Some(step.clone());
                }
            }
            if let Some(ref step) = best_pop_step {
                findings.push(format!(
                    "Peak population stability at overflow_threshold = {:.2} (mean pop: {:.1})",
                    step.parameter_value, best_pop
                ));
            }

            // Gini trajectory
            let gini_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            let gini_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("gini_coefficient"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (gini_first, gini_last) {
                let change_pct = ((last - first) / first.max(0.001)) * 100.0;
                findings.push(format!(
                    "Gini coefficient: {:.4} at deployment (0.10) vs {:.4} at hoarding (0.90) ({:+.1}%)",
                    first, last, change_pct
                ));
            }

            // Collapse analysis
            let any_collapse = result.steps.iter().any(|s| s.collapse_rate > 0.0);
            if any_collapse {
                let max_collapse_step = result.steps.iter()
                    .max_by(|a, b| a.collapse_rate.partial_cmp(&b.collapse_rate).unwrap())
                    .unwrap();
                findings.push(format!(
                    "Highest collapse rate: {:.0}% at overflow_threshold = {:.2}",
                    max_collapse_step.collapse_rate * 100.0, max_collapse_step.parameter_value
                ));
            } else {
                findings.push("No collapses observed across entire threshold range".into());
            }

            // Fitness comparison
            let fit_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            let fit_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (fit_first, fit_last) {
                findings.push(format!(
                    "Mean fitness: {:.4} at aggressive deployment vs {:.4} at hoarding",
                    first, last
                ));
            }

            // Population volatility comparison
            let vol_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("population_volatility"))
                .map(|s| s.mean);
            let vol_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("population_volatility"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (vol_first, vol_last) {
                findings.push(format!(
                    "Population volatility: {:.2} at deployment vs {:.2} at hoarding",
                    first, last
                ));
            }

            // Birth/death ratio
            let bdr_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("birth_death_ratio"))
                .map(|s| s.mean);
            let bdr_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("birth_death_ratio"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (bdr_first, bdr_last) {
                findings.push(format!(
                    "Birth/death ratio: {:.4} at deployment vs {:.4} at hoarding",
                    first, last
                ));
            }
        }

        "evolution_forbidden" => {
            // Compare collapse trajectory to catastrophe_resilience (which has evolution ON)
            // Key question: does forbidding mutation increase collapse rates?

            // Collapse curve across catastrophe intensities
            let peaceful_collapse = result.steps.first().map(|s| s.collapse_rate).unwrap_or(0.0);
            let extreme_collapse = result.steps.last().map(|s| s.collapse_rate).unwrap_or(0.0);
            findings.push(format!(
                "Collapse rate WITHOUT evolution: {:.0}% at peaceful (0.0) vs {:.0}% at extreme (0.03)",
                peaceful_collapse * 100.0, extreme_collapse * 100.0
            ));

            // Survival epochs
            let survival_first = result.steps.first().map(|s| s.mean_survival_epochs).unwrap_or(0.0);
            let survival_last = result.steps.last().map(|s| s.mean_survival_epochs).unwrap_or(0.0);
            findings.push(format!(
                "Mean survival (no evolution): {:.1} epochs at peaceful vs {:.1} at extreme catastrophe",
                survival_first, survival_last
            ));

            // Trait mutations should be zero
            let total_trait_mutations: f64 = result.steps.iter()
                .flat_map(|s| s.trials.iter())
                .filter_map(|t| t.metrics.get("total_trait_mutations"))
                .sum();
            findings.push(format!(
                "Total trait mutations across all worlds: {:.0} (expected: 0)",
                total_trait_mutations
            ));

            // Mean fitness degradation
            let fit_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            let fit_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_fitness"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (fit_first, fit_last) {
                findings.push(format!(
                    "Mean fitness (frozen traits): {:.4} at peaceful vs {:.4} at extreme catastrophe",
                    first, last
                ));
            }

            // Population impact
            let pop_first = result.steps.first()
                .and_then(|s| s.metric_summaries.get("mean_population"))
                .map(|s| s.mean);
            let pop_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("mean_population"))
                .map(|s| s.mean);
            if let (Some(first), Some(last)) = (pop_first, pop_last) {
                findings.push(format!(
                    "Mean population (no evolution): {:.1} at peaceful vs {:.1} at extreme",
                    first, last
                ));
            }

            // Catastrophe death toll
            let cat_deaths_last = result.steps.last()
                .and_then(|s| s.metric_summaries.get("total_catastrophe_deaths"))
                .map(|s| s.mean);
            if let Some(deaths) = cat_deaths_last {
                findings.push(format!(
                    "Mean catastrophe deaths at extreme rate (no evolution): {:.1}",
                    deaths
                ));
            }
        }

        _ => {}
    }

    findings
}

/// Derive per-tier findings for a reserve stress experiment.
fn derive_reserve_tier_findings(_slug: &str, result: &genesis_experiment::ExperimentResult) -> Vec<String> {
    let mut findings = Vec::new();

    // Extract shock level from the config's pressure override
    let shock_level = result.config.base_pressure_override.as_ref()
        .map(|p| p.catastrophe_base_prob)
        .unwrap_or(0.002);

    // Find optimal treasury threshold by mean fitness
    let mut best_fitness_step = result.steps.first().cloned();
    let mut best_fitness = f64::NEG_INFINITY;
    for step in &result.steps {
        let fit = step.metric_summaries.get("mean_fitness")
            .map(|s| s.mean).unwrap_or(0.0);
        if fit > best_fitness {
            best_fitness = fit;
            best_fitness_step = Some(step.clone());
        }
    }
    if let Some(ref step) = best_fitness_step {
        findings.push(format!(
            "Optimal treasury threshold by fitness: {:.2} (mean fitness: {:.4}) at shock={:.3}",
            step.parameter_value, best_fitness, shock_level
        ));
    }

    // Find optimal by population stability
    let mut best_pop_step = result.steps.first().cloned();
    let mut best_pop = 0.0f64;
    for step in &result.steps {
        let pop = step.metric_summaries.get("mean_population")
            .map(|s| s.mean).unwrap_or(0.0);
        if pop > best_pop {
            best_pop = pop;
            best_pop_step = Some(step.clone());
        }
    }
    if let Some(ref step) = best_pop_step {
        findings.push(format!(
            "Peak population stability at threshold = {:.2} (mean pop: {:.1})",
            step.parameter_value, best_pop
        ));
    }

    // Treasury ratio: deployment vs hoarding
    let treasury_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("treasury_ratio"))
        .map(|s| s.mean);
    let treasury_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("treasury_ratio"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (treasury_first, treasury_last) {
        findings.push(format!(
            "Treasury ratio: {:.4} at deployment (0.10) vs {:.4} at hoarding (0.90)",
            first, last
        ));
    }

    // Collapse analysis across threshold range
    let any_collapse = result.steps.iter().any(|s| s.collapse_rate > 0.0);
    if any_collapse {
        let max_collapse_step = result.steps.iter()
            .max_by(|a, b| a.collapse_rate.partial_cmp(&b.collapse_rate).unwrap())
            .unwrap();
        let min_collapse_step = result.steps.iter()
            .min_by(|a, b| a.collapse_rate.partial_cmp(&b.collapse_rate).unwrap())
            .unwrap();
        findings.push(format!(
            "Collapse range: {:.0}% (threshold={:.2}) to {:.0}% (threshold={:.2})",
            min_collapse_step.collapse_rate * 100.0, min_collapse_step.parameter_value,
            max_collapse_step.collapse_rate * 100.0, max_collapse_step.parameter_value
        ));
    } else {
        findings.push("No collapses observed across entire threshold range".into());
    }

    // Catastrophe death toll trajectory
    let cat_deaths_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("total_catastrophe_deaths"))
        .map(|s| s.mean);
    let cat_deaths_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("total_catastrophe_deaths"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (cat_deaths_first, cat_deaths_last) {
        findings.push(format!(
            "Mean catastrophe deaths: {:.1} at deployment vs {:.1} at hoarding",
            first, last
        ));
    }

    // Population volatility comparison
    let vol_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("population_volatility"))
        .map(|s| s.mean);
    let vol_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("population_volatility"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (vol_first, vol_last) {
        findings.push(format!(
            "Population volatility: {:.2} at deployment vs {:.2} at hoarding",
            first, last
        ));
    }

    findings
}

/// Cross-tier synthesis: compare optimal thresholds across shock regimes.
fn derive_reserve_cross_tier_synthesis(
    results: &[(&str, genesis_experiment::ExperimentResult)],
) -> Vec<String> {
    let mut synthesis = Vec::new();

    // Extract optimal threshold (by fitness) per tier
    let mut tier_optima: Vec<(String, f64, f64, f64)> = Vec::new(); // (tier, shock, optimal_threshold, fitness)

    for (slug, result) in results {
        let shock = result.config.base_pressure_override.as_ref()
            .map(|p| p.catastrophe_base_prob)
            .unwrap_or(0.002);

        let mut best_threshold = 0.0f64;
        let mut best_fitness = f64::NEG_INFINITY;
        for step in &result.steps {
            let fit = step.metric_summaries.get("mean_fitness")
                .map(|s| s.mean).unwrap_or(0.0);
            if fit > best_fitness {
                best_fitness = fit;
                best_threshold = step.parameter_value;
            }
        }

        let tier_name = slug.replace("reserve_", "").to_uppercase();
        tier_optima.push((tier_name, shock, best_threshold, best_fitness));
    }

    // Report optimal thresholds per tier
    if tier_optima.len() == 4 {
        synthesis.push(format!(
            "Optimal treasury thresholds by shock tier: {} (shock={:.3}) → {:.2}, {} (shock={:.3}) → {:.2}, {} (shock={:.3}) → {:.2}, {} (shock={:.3}) → {:.2}",
            tier_optima[0].0, tier_optima[0].1, tier_optima[0].2,
            tier_optima[1].0, tier_optima[1].1, tier_optima[1].2,
            tier_optima[2].0, tier_optima[2].1, tier_optima[2].2,
            tier_optima[3].0, tier_optima[3].1, tier_optima[3].2,
        ));

        // Check if optimal threshold shifts
        let calm_opt = tier_optima[0].2;
        let crisis_opt = tier_optima[3].2;
        let shift = crisis_opt - calm_opt;
        if shift.abs() > 0.05 {
            synthesis.push(format!(
                "Policy shift detected: optimal threshold moves {:+.2} from calm to crisis \
                 ({:.2} → {:.2}), suggesting reserve policy must adapt to market conditions",
                shift, calm_opt, crisis_opt
            ));
        } else {
            synthesis.push(format!(
                "Optimal threshold stable across shock regimes ({:.2} → {:.2}), \
                 suggesting robust policy exists independent of market conditions",
                calm_opt, crisis_opt
            ));
        }
    }

    // Compare collapse rates across tiers
    let mut tier_collapse_rates: Vec<(String, f64, f64)> = Vec::new(); // (tier, max_collapse, min_collapse)
    for (slug, result) in results {
        let tier_name = slug.replace("reserve_", "").to_uppercase();
        let max_collapse = result.steps.iter()
            .map(|s| s.collapse_rate)
            .fold(0.0f64, f64::max);
        let min_collapse = result.steps.iter()
            .map(|s| s.collapse_rate)
            .fold(1.0f64, f64::min);
        tier_collapse_rates.push((tier_name, min_collapse, max_collapse));
    }

    let calm_max_collapse = tier_collapse_rates.first().map(|t| t.2).unwrap_or(0.0);
    let crisis_max_collapse = tier_collapse_rates.last().map(|t| t.2).unwrap_or(0.0);
    synthesis.push(format!(
        "Collapse escalation: max collapse rate {:.0}% (calm) vs {:.0}% (crisis)",
        calm_max_collapse * 100.0, crisis_max_collapse * 100.0
    ));

    // Fitness comparison: deployment vs hoarding across tiers
    let mut fitness_comparison: Vec<(String, f64, f64)> = Vec::new(); // (tier, deploy_fitness, hoard_fitness)
    for (slug, result) in results {
        let tier_name = slug.replace("reserve_", "").to_uppercase();
        let deploy_fit = result.steps.first()
            .and_then(|s| s.metric_summaries.get("mean_fitness"))
            .map(|s| s.mean).unwrap_or(0.0);
        let hoard_fit = result.steps.last()
            .and_then(|s| s.metric_summaries.get("mean_fitness"))
            .map(|s| s.mean).unwrap_or(0.0);
        fitness_comparison.push((tier_name, deploy_fit, hoard_fit));
    }

    // Check for crossover: does hoarding ever beat deployment?
    let mut crossover_found = false;
    for fc in &fitness_comparison {
        if fc.2 > fc.1 {
            synthesis.push(format!(
                "CROSSOVER: Hoarding outperforms deployment in {} tier \
                 (fitness {:.4} hoard vs {:.4} deploy)",
                fc.0, fc.2, fc.1
            ));
            crossover_found = true;
        }
    }
    if !crossover_found {
        synthesis.push(
            "No crossover detected: deployment outperforms hoarding across all shock tiers".into()
        );
    }

    // Summary: mean fitness degradation from calm to crisis
    if fitness_comparison.len() == 4 {
        let calm_avg = (fitness_comparison[0].1 + fitness_comparison[0].2) / 2.0;
        let crisis_avg = (fitness_comparison[3].1 + fitness_comparison[3].2) / 2.0;
        let degradation = ((crisis_avg - calm_avg) / calm_avg.max(0.001)) * 100.0;
        synthesis.push(format!(
            "Fitness degradation calm → crisis: {:.1}% (mean fitness {:.4} → {:.4})",
            degradation, calm_avg, crisis_avg
        ));
    }

    synthesis
}

/// Derive per-tier findings for a resource depletion experiment.
fn derive_depletion_tier_findings(_slug: &str, result: &genesis_experiment::ExperimentResult) -> Vec<String> {
    let mut findings = Vec::new();

    let soft_cap = result.config.base_pressure_override.as_ref()
        .map(|p| p.soft_cap)
        .unwrap_or(180);

    // Find optimal entropy_coeff by mean fitness
    let mut best_fitness_step = result.steps.first().cloned();
    let mut best_fitness = f64::NEG_INFINITY;
    for step in &result.steps {
        let fit = step.metric_summaries.get("mean_fitness")
            .map(|s| s.mean).unwrap_or(0.0);
        if fit > best_fitness {
            best_fitness = fit;
            best_fitness_step = Some(step.clone());
        }
    }
    if let Some(ref step) = best_fitness_step {
        findings.push(format!(
            "Best fitness at entropy_coeff = {:.6} (mean fitness: {:.4}) at cap={}",
            step.parameter_value, best_fitness, soft_cap
        ));
    }

    // Population stability across entropy range
    let pop_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("mean_population"))
        .map(|s| s.mean);
    let pop_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("mean_population"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (pop_first, pop_last) {
        let decline = ((first - last) / first.max(0.001)) * 100.0;
        findings.push(format!(
            "Population decline from low to high entropy: {:.1}% ({:.1} → {:.1})",
            decline, first, last
        ));
    }

    // Gini coefficient range
    let gini_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("gini_coefficient"))
        .map(|s| s.mean);
    let gini_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("gini_coefficient"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (gini_first, gini_last) {
        findings.push(format!(
            "Gini range: {:.4} (low entropy) → {:.4} (high entropy)",
            first, last
        ));
    }

    // Collapse analysis
    let any_collapse = result.steps.iter().any(|s| s.collapse_rate > 0.0);
    if any_collapse {
        let first_collapse = result.steps.iter()
            .find(|s| s.collapse_rate > 0.0);
        if let Some(step) = first_collapse {
            findings.push(format!(
                "First collapse at entropy_coeff = {:.6} (collapse rate: {:.0}%)",
                step.parameter_value, step.collapse_rate * 100.0
            ));
        }
    } else {
        findings.push("No collapses observed across entire entropy range".into());
    }

    // Entropy burned comparison
    let burn_first = result.steps.first()
        .and_then(|s| s.metric_summaries.get("total_entropy_burned"))
        .map(|s| s.mean);
    let burn_last = result.steps.last()
        .and_then(|s| s.metric_summaries.get("total_entropy_burned"))
        .map(|s| s.mean);
    if let (Some(first), Some(last)) = (burn_first, burn_last) {
        findings.push(format!(
            "Total entropy burned: {:.1} (low) → {:.1} (high) — {:.0}× increase",
            first, last, last / first.max(0.001)
        ));
    }

    findings
}

/// Cross-tier synthesis: compare metabolic cost sensitivity across carrying capacity regimes.
fn derive_depletion_cross_tier_synthesis(
    results: &[(&str, genesis_experiment::ExperimentResult)],
) -> Vec<String> {
    let mut synthesis = Vec::new();

    // Extract population decline percentage per tier
    let mut tier_declines: Vec<(String, usize, f64, f64)> = Vec::new(); // (tier, cap, pop_at_low_entropy, pop_at_high_entropy)

    for (slug, result) in results {
        let soft_cap = result.config.base_pressure_override.as_ref()
            .map(|p| p.soft_cap)
            .unwrap_or(180);

        let pop_first = result.steps.first()
            .and_then(|s| s.metric_summaries.get("mean_population"))
            .map(|s| s.mean)
            .unwrap_or(0.0);
        let pop_last = result.steps.last()
            .and_then(|s| s.metric_summaries.get("mean_population"))
            .map(|s| s.mean)
            .unwrap_or(0.0);

        let tier_name = slug.replace("resource_depletion_", "").to_uppercase();
        tier_declines.push((tier_name, soft_cap, pop_first, pop_last));
    }

    // Report population sensitivity per tier
    if tier_declines.len() == 4 {
        for td in &tier_declines {
            let decline_pct = ((td.2 - td.3) / td.2.max(0.001)) * 100.0;
            synthesis.push(format!(
                "{} (cap={}): population decline {:.1}% across entropy sweep ({:.1} → {:.1})",
                td.0, td.1, decline_pct, td.2, td.3
            ));
        }

        // Check for nonlinear amplification
        let abundant_decline = ((tier_declines[0].2 - tier_declines[0].3) / tier_declines[0].2.max(0.001)) * 100.0;
        let scarce_decline = ((tier_declines[3].2 - tier_declines[3].3) / tier_declines[3].2.max(0.001)) * 100.0;

        if scarce_decline > abundant_decline * 1.5 {
            synthesis.push(format!(
                "AMPLIFICATION: Metabolic cost sensitivity {:.1}× higher under scarcity \
                 ({:.1}% decline at cap={}) vs abundance ({:.1}% decline at cap={})",
                scarce_decline / abundant_decline.max(0.001),
                scarce_decline, tier_declines[3].1,
                abundant_decline, tier_declines[0].1
            ));
        } else {
            synthesis.push(format!(
                "Linear response: Metabolic cost sensitivity scales proportionally with capacity \
                 ({:.1}% abundant vs {:.1}% scarce)",
                abundant_decline, scarce_decline
            ));
        }
    }

    // Compare collapse thresholds across tiers
    let mut tier_collapse_thresholds: Vec<(String, Option<f64>)> = Vec::new();
    for (slug, result) in results {
        let tier_name = slug.replace("resource_depletion_", "").to_uppercase();
        let first_collapse = result.steps.iter()
            .find(|s| s.collapse_rate > 0.0)
            .map(|s| s.parameter_value);
        tier_collapse_thresholds.push((tier_name, first_collapse));
    }

    let collapse_tiers: Vec<_> = tier_collapse_thresholds.iter()
        .filter(|(_, t)| t.is_some())
        .collect();
    if collapse_tiers.is_empty() {
        synthesis.push("No collapses observed in any capacity tier — system resilient across full entropy range".into());
    } else {
        for (name, threshold) in &collapse_tiers {
            synthesis.push(format!(
                "COLLAPSE THRESHOLD: {} tier first collapses at entropy_coeff = {:.6}",
                name, threshold.unwrap()
            ));
        }
    }

    // Fitness degradation comparison
    let mut fitness_by_tier: Vec<(String, f64, f64)> = Vec::new();
    for (slug, result) in results {
        let tier_name = slug.replace("resource_depletion_", "").to_uppercase();
        let fit_first = result.steps.first()
            .and_then(|s| s.metric_summaries.get("mean_fitness"))
            .map(|s| s.mean).unwrap_or(0.0);
        let fit_last = result.steps.last()
            .and_then(|s| s.metric_summaries.get("mean_fitness"))
            .map(|s| s.mean).unwrap_or(0.0);
        fitness_by_tier.push((tier_name, fit_first, fit_last));
    }

    if fitness_by_tier.len() == 4 {
        let abundant_avg = (fitness_by_tier[0].1 + fitness_by_tier[0].2) / 2.0;
        let scarce_avg = (fitness_by_tier[3].1 + fitness_by_tier[3].2) / 2.0;
        let degradation = ((scarce_avg - abundant_avg) / abundant_avg.max(0.001)) * 100.0;
        synthesis.push(format!(
            "Fitness degradation abundant → scarce: {:.1}% (mean fitness {:.4} → {:.4})",
            degradation, abundant_avg, scarce_avg
        ));
    }

    synthesis
}
