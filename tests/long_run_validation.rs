// Long Run Validation — Scientific stress tests for Genesis Protocol
//
// Simulates 50,000+ epoch runs under extreme conditions to validate:
// - Equilibrium convergence
// - Fitness growth slope
// - Role diversity persistence
// - Treasury cycling ratio
// - ATP concentration curve
//
// These tests are #[ignore]d by default — run with:
//   cargo test --test long_run_validation -- --ignored --nocapture

use genesis_replay::engine::{ReplayConfig, ReplayEngine};
use genesis_replay::report::ReplayReport;
use genesis_replay::trajectory::Trajectory;

/// Helper: run a replay and return its report.
fn run_scenario(name: &str, config: ReplayConfig) -> (Trajectory, ReplayReport) {
    eprintln!("\n═══ {} ═══", name);
    let mut engine = ReplayEngine::new(config).unwrap();
    let traj = engine.run();
    let report = ReplayReport::from_trajectory(&traj, true);
    eprintln!("{}", report.summary());
    (traj, report)
}

#[test]
#[ignore]
fn test_50k_baseline() {
    let config = ReplayConfig {
        seed: 7729,
        epochs: 50_000,
        ..Default::default()
    };
    let (traj, report) = run_scenario("50K BASELINE", config);

    // Organism should survive 50K epochs
    assert!(
        !report.went_extinct,
        "Baseline organism went extinct at epoch {:?}",
        report.extinction_epoch
    );

    // Population should stabilize
    assert!(
        report.final_population > 5,
        "Population too low: {}",
        report.final_population
    );

    // Fitness should trend upward or stay stable
    assert!(
        report.fitness_slope >= -0.0001,
        "Fitness declining rapidly: slope = {}",
        report.fitness_slope
    );

    // Export CSV for plotting
    let csv = traj.to_csv();
    std::fs::create_dir_all("test_output").ok();
    std::fs::write("test_output/baseline_50k.csv", csv).ok();
}

#[test]
#[ignore]
fn test_resource_collapse() {
    let config = ReplayConfig {
        seed: 42,
        epochs: 20_000,
        base_capacity: 30.0,  // Severely reduced resources
        regen_rate: 0.05,     // Slow regeneration
        ..Default::default()
    };
    let (traj, report) = run_scenario("RESOURCE COLLAPSE", config);

    // Organism may or may not survive — but if it does, it's meaningful
    eprintln!("  Survived: {}", !report.went_extinct);
    eprintln!("  Final pop: {}", report.final_population);

    let csv = traj.to_csv();
    std::fs::create_dir_all("test_output").ok();
    std::fs::write("test_output/resource_collapse.csv", csv).ok();
}

#[test]
#[ignore]
fn test_high_catastrophe_mode() {
    // Override catastrophe chance is baked at 2% in replay engine,
    // but we can stress with low resources + high decay
    let config = ReplayConfig {
        seed: 1337,
        epochs: 20_000,
        base_capacity: 80.0,
        decay_rate: 0.04,     // Double normal decay
        ..Default::default()
    };
    let (_traj, report) = run_scenario("HIGH CATASTROPHE", config);

    eprintln!("  Survived: {}", !report.went_extinct);
    eprintln!("  Peak pop: {}", report.peak_population);
}

#[test]
#[ignore]
fn test_extreme_mutation() {
    let config = ReplayConfig {
        seed: 9999,
        epochs: 20_000,
        mutation_rate: 0.5,       // 50% mutation rate
        mutation_max_delta: 0.25, // Large deltas
        ..Default::default()
    };
    let (traj, report) = run_scenario("EXTREME MUTATION", config);

    // With extreme mutation, fitness slope should still be measurable
    eprintln!("  Fitness slope: {:.8}", report.fitness_slope);

    let csv = traj.to_csv();
    std::fs::create_dir_all("test_output").ok();
    std::fs::write("test_output/extreme_mutation.csv", csv).ok();
}

#[test]
#[ignore]
fn test_determinism_10k() {
    let config = ReplayConfig {
        seed: 7729,
        epochs: 10_000,
        ..Default::default()
    };
    eprintln!("\n═══ DETERMINISM 10K ═══");
    let result = ReplayEngine::verify_determinism(&config);
    assert!(result.unwrap(), "Replay engine is non-deterministic!");
    eprintln!("  10,000 epoch replay is fully deterministic ✓");
}

#[test]
#[ignore]
fn test_equilibrium_convergence_time() {
    let seeds = [42, 1337, 7729, 31337, 65536];
    let mut convergence_times = Vec::new();

    eprintln!("\n═══ EQUILIBRIUM CONVERGENCE ═══");
    for seed in seeds {
        let config = ReplayConfig {
            seed,
            epochs: 10_000,
            ..Default::default()
        };
        let mut engine = ReplayEngine::new(config).unwrap();
        let traj = engine.run();

        if let Some(eq) = traj.equilibrium_epoch(100, 9.0) {
            convergence_times.push(eq);
            eprintln!("  Seed {}: equilibrium at epoch {}", seed, eq);
        } else {
            eprintln!("  Seed {}: no equilibrium found", seed);
        }
    }

    if !convergence_times.is_empty() {
        let avg = convergence_times.iter().sum::<u64>() as f64
            / convergence_times.len() as f64;
        eprintln!("  Mean convergence: {:.0} epochs ({} of {} converged)",
            avg, convergence_times.len(), seeds.len());
    }
}

#[test]
#[ignore]
fn test_treasury_cycling_ratio() {
    let config = ReplayConfig {
        seed: 7729,
        epochs: 10_000,
        ..Default::default()
    };
    let mut engine = ReplayEngine::new(config).unwrap();
    let traj = engine.run();

    eprintln!("\n═══ TREASURY CYCLING ═══");
    if let Some(last) = traj.points.last() {
        let peak_treasury = traj.points.iter()
            .map(|p| p.treasury_reserve)
            .fold(0.0_f64, f64::max);
        let ratio = if peak_treasury > 0.0 {
            last.treasury_reserve / peak_treasury
        } else {
            0.0
        };
        eprintln!("  Final treasury: {:.2}", last.treasury_reserve);
        eprintln!("  Peak treasury:  {:.2}", peak_treasury);
        eprintln!("  Cycling ratio:  {:.4}", ratio);
    }
}
