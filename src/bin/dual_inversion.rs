// Genesis Protocol -- Dual Inversion
//
// "If neither single-axis inversion finds collapse,
//  the dual is the last chance to break the organism."
//
// Metabolic Inversion (replication 5x): downsized to 20. Survived.
// Basal Inversion (existence 10x): TBD.
//
// Dual Inversion attacks BOTH metabolic pathways simultaneously:
//   - Replication cost FIXED at 3x (75 ATP per birth)
//   - Basal cost SWEPT from 1x to 10x (0.15 → 1.5 ATP/epoch)
//
// This is the final escalation.
// If the organism survives this, nothing in the current
// parameter space can kill it.
//
// 10 steps × 20 runs × 500 epochs = 200 worlds, 100,000 total epochs.
//
// Usage: cargo run --release --bin dual_inversion

use genesis_experiment::{
    ExperimentRunner, ExperimentReport, FlagshipExperiments,
};
use std::time::Instant;

fn main() {
    println!("======================================================================");
    println!("  GENESIS PROTOCOL -- DUAL INVERSION");
    println!("  The Final Escalation");
    println!("======================================================================");
    println!();

    let config = FlagshipExperiments::dual_inversion();

    println!("  Hypothesis: {}", config.hypothesis);
    println!();
    println!("  Protocol:");
    println!("    Sweep variable:  BasalCostMultiplier (with fixed replication 3x)");
    println!("    Range:           1.0x -> 10.0x (step 1.0, 10 levels)");
    println!("    Replication cost: FIXED at 3.0x (75 ATP per birth)");
    println!("    Basal tick:      0.15 ATP/epoch -> 1.5 ATP/epoch per agent");
    println!("    Runs per step:   {}", config.runs_per_step);
    println!("    Epochs per run:  {}", config.epochs_per_run);
    println!("    Total worlds:    {}", config.total_worlds());
    println!("    Total epochs:    {}", config.total_worlds() as u64 * config.epochs_per_run);
    println!("    Base seed:       {}", config.base_seed);
    println!();
    println!("  Hostile axes (ALL locked):");
    println!("    Mutation:        DISABLED (rate = 0.0)");
    println!("    Cortex/Immune:   DISABLED");
    println!("    Redistribution:  DISABLED (threshold = 1.0, rate = 0.0)");
    println!("    Treasury deploy: DISABLED (threshold = 1.0)");
    println!("    Catastrophe:     MAXIMUM (0.03)");
    println!("    Entropy:         MAXIMUM (0.0001, 10x default)");
    println!();
    println!("  Dual metabolic attack:");
    println!("    REPLICATION_COST = 75.0 ATP (3x fixed)");
    println!("    BASAL_TICK = 0.15 × multiplier (swept 1-10x)");
    println!("    Both metabolic pathways under simultaneous stress");
    println!();
    println!("  Running experiment...");
    println!();

    let start = Instant::now();
    let result = ExperimentRunner::run(&config);
    let elapsed = start.elapsed();

    println!("  Completed in {:.2}s", elapsed.as_secs_f64());
    println!("  Total epochs run: {}", result.total_epochs_run);
    println!("  Result hash: {}", &result.result_hash);
    println!();

    // ---- Per-Step Analysis ----

    println!("======================================================================");
    println!("  RESULTS BY BASAL COST (WITH 3x REPLICATION)");
    println!("======================================================================");
    println!();
    println!("{:<10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "basal_mul", "eff_tick", "collapse%", "surv_ep", "mean_pop",
        "mean_fit", "births", "deaths", "b/d_ratio");
    println!("{}", "-".repeat(100));

    let mut any_collapse = false;
    let mut total_collapsed = 0usize;
    let mut total_trials = 0usize;
    let mut first_collapse_mult: Option<f64> = None;
    let mut boundary_mult: Option<f64> = None;

    for step in &result.steps {
        let mult = step.parameter_value;
        let effective_tick = 0.15 * mult;
        let n_trials = step.trials.len();
        total_trials += n_trials;

        let collapsed: usize = step.trials.iter()
            .filter(|t| t.collapse_epoch.is_some())
            .count();
        total_collapsed += collapsed;
        let collapse_pct = (collapsed as f64 / n_trials as f64) * 100.0;

        if collapsed > 0 {
            any_collapse = true;
            if first_collapse_mult.is_none() {
                first_collapse_mult = Some(mult);
            }
        }
        if collapse_pct >= 50.0 && boundary_mult.is_none() {
            boundary_mult = Some(mult);
        }

        let survival_epochs: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("survival_epochs").unwrap_or(&500.0))
            .sum::<f64>() / n_trials as f64;
        let mean_pop: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("mean_population").unwrap_or(&0.0))
            .sum::<f64>() / n_trials as f64;
        let mean_fit: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("mean_fitness").unwrap_or(&0.0))
            .sum::<f64>() / n_trials as f64;
        let births: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("total_births").unwrap_or(&0.0))
            .sum::<f64>() / n_trials as f64;
        let deaths: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("total_deaths").unwrap_or(&0.0))
            .sum::<f64>() / n_trials as f64;
        let bd_ratio = if deaths > 0.0 { births / deaths } else { f64::INFINITY };

        println!("{:<10.1} {:>10.3} {:>9.1}% {:>10.1} {:>10.1} {:>10.4} {:>10.0} {:>10.0} {:>10.4}",
            mult, effective_tick, collapse_pct, survival_epochs, mean_pop,
            mean_fit, births, deaths, bd_ratio);
    }

    println!();
    println!("======================================================================");
    println!("  SUMMARY");
    println!("======================================================================");
    println!();
    println!("  Total worlds:          {}", total_trials);
    println!("  Total collapsed:       {}", total_collapsed);
    println!("  Overall collapse rate: {:.1}%", (total_collapsed as f64 / total_trials as f64) * 100.0);
    println!("  Duration:              {:.2}s", elapsed.as_secs_f64());
    println!();

    if any_collapse {
        println!("  *** DUAL INVERSION BOUNDARY FOUND ***");
        println!("  Combined metabolic stress causes population collapse.");
        println!("  The organism cannot survive when BOTH pathways are attacked.");
        println!();
        if let Some(first) = first_collapse_mult {
            println!("  First collapse at:     basal {:.1}x + replication 3.0x", first);
            println!("                         (tick={:.3} ATP/epoch, birth=75 ATP)", 0.15 * first);
        }
        if let Some(boundary) = boundary_mult {
            println!("  Boundary (>=50%):      basal {:.1}x + replication 3.0x", boundary);
            println!("                         (tick={:.3} ATP/epoch, birth=75 ATP)", 0.15 * boundary);
        }
        println!();

        // Phase transition
        println!("  Phase transition map:");
        for step in &result.steps {
            let n = step.trials.len();
            let collapsed: usize = step.trials.iter()
                .filter(|t| t.collapse_epoch.is_some())
                .count();
            let pct = (collapsed as f64 / n as f64) * 100.0;
            let mean_pop: f64 = step.trials.iter()
                .map(|t| *t.metrics.get("mean_population").unwrap_or(&0.0))
                .sum::<f64>() / n as f64;
            let surv: f64 = step.trials.iter()
                .map(|t| *t.metrics.get("survival_epochs").unwrap_or(&500.0))
                .sum::<f64>() / n as f64;
            let marker = if pct >= 100.0 {
                " <<< TOTAL EXTINCTION"
            } else if pct >= 50.0 {
                " <<< BOUNDARY"
            } else if pct > 0.0 {
                " < partial"
            } else {
                ""
            };
            println!("    basal {:>4.1}x | {:>5.1}% collapse | {:>5.1} pop | {:>5.0} epochs{}",
                step.parameter_value, pct, mean_pop, surv, marker);
        }
    } else {
        println!("  *** NO COLLAPSE FOUND ***");
        println!("  Even with BOTH metabolic pathways under maximum stress,");
        println!("  the organism survives.");
        println!();
        println!("  The Genesis Protocol has no collapse boundary within");
        println!("  the tested parameter space.");
        println!();
        println!("  This is flow-stabilized structural immunity.");
    }
    println!();

    // ---- Comparative analysis ----
    println!("======================================================================");
    println!("  DUAL STRESS ENERGY FLOW");
    println!("======================================================================");
    println!();
    println!("  replication cost: 75 ATP (fixed 3x)");
    println!("  basal cost: swept 1-10x");
    println!();
    for step in &result.steps {
        let mult = step.parameter_value;
        let n = step.trials.len();
        let mean_pop: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("mean_population").unwrap_or(&0.0))
            .sum::<f64>() / n as f64;
        let births: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("total_births").unwrap_or(&0.0))
            .sum::<f64>() / n as f64;
        let deaths: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("total_deaths").unwrap_or(&0.0))
            .sum::<f64>() / n as f64;
        let collapsed: usize = step.trials.iter()
            .filter(|t| t.collapse_epoch.is_some())
            .count();

        let bar_len = (mean_pop / 50.0 * 30.0).min(30.0).max(0.0) as usize;
        let bar: String = if collapsed > 0 {
            let alive = ((n - collapsed) as f64 / n as f64 * bar_len as f64) as usize;
            let dead = bar_len.saturating_sub(alive);
            format!("{}{}", "#".repeat(alive), "X".repeat(dead))
        } else {
            "#".repeat(bar_len)
        };
        let collapse_marker = if collapsed == n {
            " [EXTINCT]".to_string()
        } else if collapsed > 0 {
            format!(" [{}% DEAD]", (collapsed * 100) / n)
        } else {
            String::new()
        };

        println!("  {:>4.1}x | {:>5.1} pop | b={:>5.0} d={:>5.0} | {}{}",
            mult, mean_pop, births, deaths, bar, collapse_marker);
    }
    println!();

    // ---- Collapse detail ----
    println!("======================================================================");
    println!("  COLLAPSE DETAIL");
    println!("======================================================================");
    println!();
    let mut any_detail = false;
    for step in &result.steps {
        let collapsed_trials: Vec<_> = step.trials.iter()
            .filter(|t| t.collapse_epoch.is_some())
            .collect();
        if !collapsed_trials.is_empty() {
            any_detail = true;
            println!("  basal_mult={:.1}x + repl=3.0x: {} of {} collapsed",
                step.parameter_value,
                collapsed_trials.len(), step.trials.len());
            let mut epochs: Vec<u64> = collapsed_trials.iter()
                .map(|t| t.collapse_epoch.unwrap())
                .collect();
            epochs.sort();
            let min_ep = epochs[0];
            let max_ep = epochs[epochs.len() - 1];
            let mean_ep: f64 = epochs.iter().sum::<u64>() as f64 / epochs.len() as f64;
            println!("    Collapse epochs: min={}, mean={:.0}, max={}", min_ep, mean_ep, max_ep);
            for t in &collapsed_trials {
                println!("    Trial {} (seed {}): epoch {}, final_pop={}",
                    t.run_index, t.seed,
                    t.collapse_epoch.unwrap(),
                    t.final_population);
            }
            println!();
        }
    }
    if !any_detail {
        println!("  No collapses recorded at any stress level.");
        println!();
    }

    // ---- Save full report ----
    let output_dir = "experiments/dual_inversion";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let findings = if any_collapse {
        let mut f = vec![
            format!("DUAL INVERSION BOUNDARY FOUND: {} of {} worlds collapsed ({:.1}%)",
                total_collapsed, total_trials,
                (total_collapsed as f64 / total_trials as f64) * 100.0),
            "Combined metabolic stress (reproduction + existence) causes collapse".into(),
            "The organism requires at least one metabolic pathway to be affordable".into(),
        ];
        if let Some(first) = first_collapse_mult {
            f.push(format!("First collapse at basal {:.1}x + replication 3.0x", first));
        }
        if let Some(boundary) = boundary_mult {
            f.push(format!("Boundary (>=50%) at basal {:.1}x + replication 3.0x", boundary));
        }
        f
    } else {
        vec![
            format!("NO COLLAPSE: 0 of {} worlds survived dual metabolic stress", total_trials),
            "Even with BOTH pathways attacked, flow stabilization holds".into(),
            "The Genesis organism has no collapse boundary in tested parameter space".into(),
        ]
    };

    let report = ExperimentReport::generate(&result, findings);
    report.save_to_dir_with_slug(output_dir, Some("dual_inversion"))
        .expect("Failed to save report");

    println!("  Full report saved to {}/", output_dir);
    println!("    - dual_inversion_report.txt");
    println!("    - dual_inversion_data.csv");
    println!("    - dual_inversion_manifest.json");
    println!();
    println!("======================================================================");
    println!("  EXPERIMENT COMPLETE");
    println!("======================================================================");
}
