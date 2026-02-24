// Genesis Protocol -- Basal Inversion
//
// "You attacked speed. You didn't attack oxygen."
//
// Metabolic Inversion increased reproduction cost 5x.
// The organism downsized from 46 to 20. Did not collapse.
// It throttled reproduction, not viability.
//
// Basal Inversion attacks existence itself.
// Every agent burns more ATP per epoch just to stay alive.
// BASAL_TICK = 0.15 ATP/epoch at 1x. At 10x = 1.5 ATP/epoch.
//
// This is the oxygen attack. Not rent. Air.
//
// 10 steps × 20 runs × 500 epochs = 200 worlds, 100,000 total epochs.
//
// Usage: cargo run --release --bin basal_inversion

use genesis_experiment::{
    ExperimentRunner, ExperimentReport, FlagshipExperiments,
};
use std::time::Instant;

fn main() {
    println!("======================================================================");
    println!("  GENESIS PROTOCOL -- BASAL INVERSION");
    println!("  The Starvation");
    println!("======================================================================");
    println!();

    let config = FlagshipExperiments::basal_inversion();

    println!("  Hypothesis: {}", config.hypothesis);
    println!();
    println!("  Protocol:");
    println!("    Sweep variable:  BasalCostMultiplier");
    println!("    Range:           1.0x -> 10.0x (step 1.0, 10 levels)");
    println!("    Effective cost:  0.15 ATP/epoch -> 1.5 ATP/epoch per agent");
    println!("    Runs per step:   {}", config.runs_per_step);
    println!("    Epochs per run:  {}", config.epochs_per_run);
    println!("    Total worlds:    {}", config.total_worlds());
    println!("    Total epochs:    {}", config.total_worlds() as u64 * config.epochs_per_run);
    println!("    Base seed:       {}", config.base_seed);
    println!();
    println!("  Hostile axes (ALL locked from multi-axis):");
    println!("    Mutation:        DISABLED (rate = 0.0)");
    println!("    Cortex/Immune:   DISABLED");
    println!("    Redistribution:  DISABLED (threshold = 1.0, rate = 0.0)");
    println!("    Treasury deploy: DISABLED (threshold = 1.0)");
    println!("    Catastrophe:     MAXIMUM (0.03)");
    println!("    Entropy:         MAXIMUM (0.0001, 10x default)");
    println!();
    println!("  Metabolic attack:");
    println!("    BASAL_TICK base = 0.15 ATP/epoch");
    println!("    Multiplier sweep = 1.0 .. 10.0");
    println!("    At 10.0x: cost = 1.5 ATP/epoch per agent (chronic drain)");
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
    println!("  RESULTS BY BASAL COST MULTIPLIER");
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
        println!("  *** BASAL BOUNDARY FOUND ***");
        println!("  Chronic energy drain causes population collapse.");
        println!("  The organism cannot survive when existence itself is too expensive.");
        println!();
        if let Some(first) = first_collapse_mult {
            println!("  First collapse at:     {:.1}x (basal tick: {:.3} ATP/epoch)",
                first, 0.15 * first);
        }
        if let Some(boundary) = boundary_mult {
            println!("  Boundary (>=50%):      {:.1}x (basal tick: {:.3} ATP/epoch)",
                boundary, 0.15 * boundary);
        }
        println!();

        // Phase transition analysis
        println!("  Phase transition:");
        for step in &result.steps {
            let n = step.trials.len();
            let collapsed: usize = step.trials.iter()
                .filter(|t| t.collapse_epoch.is_some())
                .count();
            let pct = (collapsed as f64 / n as f64) * 100.0;
            let mean_pop: f64 = step.trials.iter()
                .map(|t| *t.metrics.get("mean_population").unwrap_or(&0.0))
                .sum::<f64>() / n as f64;
            let marker = if pct >= 50.0 { " <<<" } else { "" };
            println!("    {:.1}x -> {:>5.1}% collapse, {:>5.1} mean pop{}",
                step.parameter_value, pct, mean_pop, marker);
        }
    } else {
        println!("  *** NO COLLAPSE FOUND ***");
        println!("  Even at 10.0x basal cost (1.5 ATP/epoch), the system survives.");
        println!("  The organism adapts to chronic energy drain.");
        println!("  Existence cost alone cannot kill it.");
    }
    println!();

    // ---- Demographic visualization ----
    println!("======================================================================");
    println!("  ENERGY FLOW ANALYSIS");
    println!("======================================================================");
    println!();
    for step in &result.steps {
        let mult = step.parameter_value;
        let n = step.trials.len();
        let mean_pop: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("mean_population").unwrap_or(&0.0))
            .sum::<f64>() / n as f64;
        let entropy_burned: f64 = step.trials.iter()
            .map(|t| *t.metrics.get("total_entropy_burned").unwrap_or(&0.0))
            .sum::<f64>() / n as f64;
        let collapsed: usize = step.trials.iter()
            .filter(|t| t.collapse_epoch.is_some())
            .count();

        let bar_len = (mean_pop / 50.0 * 30.0).min(30.0) as usize;
        let bar: String = if collapsed > 0 {
            let alive = ((n - collapsed) as f64 / n as f64 * bar_len as f64) as usize;
            format!("{}{}", "#".repeat(alive), "X".repeat(bar_len - alive))
        } else {
            "#".repeat(bar_len)
        };
        let collapse_marker = if collapsed > 0 {
            format!(" [{}% DEAD]", (collapsed * 100) / n)
        } else {
            String::new()
        };

        println!("  {:>4.1}x | {:>5.1} pop | entropy={:>8.0} | {}{}",
            mult, mean_pop, entropy_burned, bar, collapse_marker);
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
            println!("  basal_mult={:.1}x (tick={:.3} ATP/epoch): {} of {} collapsed",
                step.parameter_value,
                0.15 * step.parameter_value,
                collapsed_trials.len(), step.trials.len());
            for t in &collapsed_trials {
                println!("    Trial {} (seed {}): collapsed at epoch {}, final_pop={}",
                    t.run_index, t.seed,
                    t.collapse_epoch.unwrap(),
                    t.final_population);
            }
            println!();
        }
    }
    if !any_detail {
        println!("  No collapses recorded at any basal multiplier level.");
        println!();
    }

    // ---- Save full report ----
    let output_dir = "experiments/basal_inversion";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let findings = if any_collapse {
        let mut f = vec![
            format!("BASAL BOUNDARY FOUND: {} of {} worlds collapsed ({:.1}%)",
                total_collapsed, total_trials,
                (total_collapsed as f64 / total_trials as f64) * 100.0),
            "Chronic energy drain causes population collapse".into(),
            "The organism cannot survive when existence itself is too expensive".into(),
        ];
        if let Some(first) = first_collapse_mult {
            f.push(format!("First collapse at {:.1}x (basal tick {:.3} ATP/epoch)", first, 0.15 * first));
        }
        if let Some(boundary) = boundary_mult {
            f.push(format!("Boundary (>=50% collapse) at {:.1}x (basal tick {:.3} ATP/epoch)", boundary, 0.15 * boundary));
        }
        f
    } else {
        vec![
            format!("NO COLLAPSE: 0 of {} worlds survived at all basal multiplier levels", total_trials),
            "Basal inversion up to 10.0x (1.5 ATP/epoch) insufficient to break attractor".into(),
            "The organism adapts to chronic energy drain through population contraction".into(),
        ]
    };

    let report = ExperimentReport::generate(&result, findings);
    report.save_to_dir_with_slug(output_dir, Some("basal_inversion"))
        .expect("Failed to save report");

    println!("  Full report saved to {}/", output_dir);
    println!("    - basal_inversion_report.txt");
    println!("    - basal_inversion_data.csv");
    println!("    - basal_inversion_manifest.json");
    println!();
    println!("======================================================================");
    println!("  EXPERIMENT COMPLETE");
    println!("======================================================================");
}
