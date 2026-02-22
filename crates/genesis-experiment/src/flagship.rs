// Flagship Experiments — Pre-built research protocols
//
// Three experiments that demonstrate what Genesis can discover:
//
// 1. Entropy Sweep — "Does the cost of existing determine civilization fate?"
//    Varies entropy_coeff across an order of magnitude.
//    Measures collapse rate, inequality trajectory, survival time.
//
// 2. Catastrophe Resilience — "Can civilizations evolve to survive catastrophe?"
//    Varies catastrophe_base_prob from mild to extreme.
//    Measures survival rate, population recovery, immune response.
//
// 3. Inequality Threshold — "At what Gini threshold does wealth tax stabilize societies?"
//    Varies gini_wealth_tax_threshold from always-active to never-triggers.
//    Measures Gini coefficient, population stability, treasury health.
//
// Each is designed to produce publishable results with statistical rigor.

use crate::config::{ExperimentConfig, ParameterSweep, Metric, SweepVariable};
use genesis_multiverse::PhysicsPreset;
use gateway::world::PressureConfig;

/// Factory for the three flagship experiments.
pub struct FlagshipExperiments;

impl FlagshipExperiments {
    /// Experiment 1: Entropy Sweep
    ///
    /// Hypothesis: Higher entropy coefficients cause earlier civilization collapse
    /// and higher inequality before collapse.
    ///
    /// Sweeps entropy_coeff from 0.00001 (gentle) to 0.0001 (harsh).
    /// 10 steps × 20 runs × 500 epochs = 100 worlds, 50,000 total epochs.
    pub fn entropy_sweep() -> ExperimentConfig {
        ExperimentConfig {
            name: "Entropy Sweep: Cost of Existing".into(),
            hypothesis: "Higher entropy coefficients cause earlier civilization collapse \
                         and higher terminal inequality".into(),
            sweep: ParameterSweep::new(
                SweepVariable::EntropyCoeff,
                0.00001,
                0.0001,
                0.00001,
            ),
            runs_per_step: 20,
            epochs_per_run: 500,
            metrics: vec![
                Metric::FinalPopulation,
                Metric::Collapsed,
                Metric::SurvivalEpochs,
                Metric::GiniCoefficient,
                Metric::MeanFitness,
                Metric::TotalBirths,
                Metric::TotalDeaths,
                Metric::MeanPopulation,
                Metric::PopulationVolatility,
                Metric::TotalEntropyBurned,
                Metric::TotalPressureMutations,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            base_seed: 20260222, // Date seed: Feb 22, 2026
        }
    }

    /// Experiment 2: Catastrophe Resilience
    ///
    /// Hypothesis: Moderate catastrophe rates build resilience;
    /// extreme rates cause extinction; zero rates cause stagnation.
    ///
    /// Sweeps catastrophe_base_prob from 0.0 (peaceful) to 0.03 (apocalyptic).
    /// 7 steps × 20 runs × 500 epochs = 140 worlds, 70,000 total epochs.
    pub fn catastrophe_resilience() -> ExperimentConfig {
        ExperimentConfig {
            name: "Catastrophe Resilience: Survival Under Fire".into(),
            hypothesis: "Moderate catastrophe rates (0.005-0.01) produce more resilient \
                         civilizations than either peaceful (0.0) or apocalyptic (0.03) conditions".into(),
            sweep: ParameterSweep::new(
                SweepVariable::CatastropheBaseProb,
                0.0,
                0.03,
                0.005,
            ),
            runs_per_step: 20,
            epochs_per_run: 500,
            metrics: vec![
                Metric::FinalPopulation,
                Metric::Collapsed,
                Metric::SurvivalEpochs,
                Metric::TotalCatastropheDeaths,
                Metric::MeanPopulation,
                Metric::PopulationVolatility,
                Metric::TotalImmuneThreats,
                Metric::TotalPressureMutations,
                Metric::MeanFitness,
                Metric::BirthDeathRatio,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            base_seed: 20260222,
        }
    }

    /// Experiment 3: Inequality Threshold
    ///
    /// Hypothesis: There exists an optimal Gini threshold for wealth tax activation
    /// that maximizes both population stability and economic efficiency.
    ///
    /// Sweeps gini_wealth_tax_threshold from 0.20 (aggressive redistribution)
    /// to 0.90 (laissez-faire).
    /// 8 steps × 20 runs × 500 epochs = 160 worlds, 80,000 total epochs.
    pub fn inequality_threshold() -> ExperimentConfig {
        ExperimentConfig {
            name: "Inequality Threshold: When Does Redistribution Help?".into(),
            hypothesis: "A Gini wealth tax threshold between 0.35-0.50 optimizes both \
                         population stability and mean fitness, while extremes (too aggressive \
                         or too passive) reduce long-term outcomes".into(),
            sweep: ParameterSweep::new(
                SweepVariable::GiniWealthTaxThreshold,
                0.20,
                0.90,
                0.10,
            ),
            runs_per_step: 20,
            epochs_per_run: 500,
            metrics: vec![
                Metric::FinalPopulation,
                Metric::Collapsed,
                Metric::SurvivalEpochs,
                Metric::GiniCoefficient,
                Metric::MeanFitness,
                Metric::TreasuryRatio,
                Metric::MeanPopulation,
                Metric::PopulationVolatility,
                Metric::TotalBirths,
                Metric::TotalDeaths,
                Metric::RoleEntropy,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            base_seed: 20260222,
        }
    }

    /// Experiment 4: Treasury Stability
    ///
    /// Hypothesis: There exists an optimal treasury overflow threshold that
    /// maximizes economic stability. Too-aggressive redistribution (low threshold)
    /// depletes reserves; too-passive hoarding (high threshold) concentrates wealth.
    ///
    /// Sweeps treasury_overflow_threshold from 0.10 (deploy early) to 0.90 (hoard).
    /// 9 steps × 20 runs × 500 epochs = 180 worlds, 90,000 total epochs.
    pub fn treasury_stability() -> ExperimentConfig {
        ExperimentConfig {
            name: "Treasury Stability: Reserve Deployment Policy".into(),
            hypothesis: "An intermediate treasury overflow threshold (0.30-0.50) maximizes \
                         population stability and minimizes inequality, while extremes — \
                         aggressive deployment or passive hoarding — degrade outcomes".into(),
            sweep: ParameterSweep::new(
                SweepVariable::TreasuryOverflowThreshold,
                0.10,
                0.90,
                0.10,
            ),
            runs_per_step: 20,
            epochs_per_run: 500,
            metrics: vec![
                Metric::FinalPopulation,
                Metric::Collapsed,
                Metric::SurvivalEpochs,
                Metric::TreasuryRatio,
                Metric::GiniCoefficient,
                Metric::MeanFitness,
                Metric::MeanPopulation,
                Metric::PopulationVolatility,
                Metric::TotalBirths,
                Metric::TotalDeaths,
                Metric::BirthDeathRatio,
                Metric::TotalEntropyBurned,
                Metric::RoleEntropy,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            base_seed: 20260222,
        }
    }

    // ─── FTH Reserve Stress Suite ───────────────────────────────────────
    //
    // 4-tier shock experiment: How should reserve deployment policy
    // change when external shock frequency increases?
    //
    // Each tier sets a different baseline catastrophe_base_prob
    // (representing market shock frequency) while sweeping
    // treasury_overflow_threshold (reserve deployment policy).
    //
    // Maps FTH domain concepts to Genesis parameters:
    //   RWA-backed reserve    → treasury overflow threshold
    //   Market shock frequency → catastrophe_base_prob
    //   Economic stress        → entropy taxation
    //   Wealth concentration   → Gini threshold
    //
    // Answers: "Should reserve deployment policy change when
    // market conditions deteriorate?"

    /// FTH Reserve Stress — Calm Market (shock = 0.001)
    ///
    /// Baseline: Low shock frequency, representing stable market conditions.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn fth_reserve_calm() -> ExperimentConfig {
        Self::fth_reserve_tier("Calm", 0.001,
            "Under calm market conditions (shock=0.001), aggressive treasury \
             deployment outperforms hoarding with minimal downside risk")
    }

    /// FTH Reserve Stress — Moderate Market (shock = 0.005)
    ///
    /// Baseline: Moderate shock frequency, representing normal volatility.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn fth_reserve_moderate() -> ExperimentConfig {
        Self::fth_reserve_tier("Moderate", 0.005,
            "Under moderate shocks (shock=0.005), the optimal deployment \
             threshold shifts toward conservative reserves")
    }

    /// FTH Reserve Stress — Stressed Market (shock = 0.015)
    ///
    /// Baseline: High shock frequency, representing market stress.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn fth_reserve_stressed() -> ExperimentConfig {
        Self::fth_reserve_tier("Stressed", 0.015,
            "Under stressed conditions (shock=0.015), reserve hoarding begins \
             to outperform deployment as shock recovery demands liquidity buffers")
    }

    /// FTH Reserve Stress — Crisis Market (shock = 0.030)
    ///
    /// Baseline: Extreme shock frequency, representing crisis conditions.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn fth_reserve_crisis() -> ExperimentConfig {
        Self::fth_reserve_tier("Crisis", 0.030,
            "Under crisis conditions (shock=0.030), conservative reserve \
             management becomes critical for survival — deployment policy \
             must shift dramatically or civilizations collapse")
    }

    /// Returns all 4 FTH reserve stress tier experiments.
    pub fn fth_reserve_stress_suite() -> Vec<(&'static str, ExperimentConfig)> {
        vec![
            ("fth_reserve_calm", Self::fth_reserve_calm()),
            ("fth_reserve_moderate", Self::fth_reserve_moderate()),
            ("fth_reserve_stressed", Self::fth_reserve_stressed()),
            ("fth_reserve_crisis", Self::fth_reserve_crisis()),
        ]
    }

    /// Internal: build a single FTH reserve stress tier experiment.
    fn fth_reserve_tier(tier_label: &str, shock_prob: f64, hypothesis: &str) -> ExperimentConfig {
        let mut base_pressure = PressureConfig::default();
        base_pressure.catastrophe_base_prob = shock_prob;

        ExperimentConfig {
            name: format!("FTH Reserve Stress — {} (shock={:.3})", tier_label, shock_prob),
            hypothesis: hypothesis.into(),
            sweep: ParameterSweep::new(
                SweepVariable::TreasuryOverflowThreshold,
                0.10,
                0.90,
                0.10,
            ),
            runs_per_step: 15,
            epochs_per_run: 500,
            metrics: vec![
                Metric::FinalPopulation,
                Metric::Collapsed,
                Metric::SurvivalEpochs,
                Metric::TreasuryRatio,
                Metric::GiniCoefficient,
                Metric::MeanFitness,
                Metric::MeanPopulation,
                Metric::PopulationVolatility,
                Metric::TotalBirths,
                Metric::TotalDeaths,
                Metric::BirthDeathRatio,
                Metric::TotalCatastropheDeaths,
                Metric::TotalEntropyBurned,
                Metric::RoleEntropy,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: Some(base_pressure),
            base_seed: 20260222,
        }
    }

    /// Quick versions of flagships for testing (fewer runs, fewer epochs).
    pub fn entropy_sweep_quick() -> ExperimentConfig {
        let mut config = Self::entropy_sweep();
        config.name = "Entropy Sweep (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn catastrophe_resilience_quick() -> ExperimentConfig {
        let mut config = Self::catastrophe_resilience();
        config.name = "Catastrophe Resilience (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn inequality_threshold_quick() -> ExperimentConfig {
        let mut config = Self::inequality_threshold();
        config.name = "Inequality Threshold (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn treasury_stability_quick() -> ExperimentConfig {
        let mut config = Self::treasury_stability();
        config.name = "Treasury Stability (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn fth_reserve_calm_quick() -> ExperimentConfig {
        let mut config = Self::fth_reserve_calm();
        config.name = "FTH Reserve Calm (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn fth_reserve_moderate_quick() -> ExperimentConfig {
        let mut config = Self::fth_reserve_moderate();
        config.name = "FTH Reserve Moderate (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn fth_reserve_stressed_quick() -> ExperimentConfig {
        let mut config = Self::fth_reserve_stressed();
        config.name = "FTH Reserve Stressed (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn fth_reserve_crisis_quick() -> ExperimentConfig {
        let mut config = Self::fth_reserve_crisis();
        config.name = "FTH Reserve Crisis (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    /// List all flagship experiment names.
    pub fn list() -> Vec<&'static str> {
        vec![
            "Entropy Sweep: Cost of Existing",
            "Catastrophe Resilience: Survival Under Fire",
            "Inequality Threshold: When Does Redistribution Help?",
            "Treasury Stability: Reserve Deployment Policy",
            "FTH Reserve Stress — Calm",
            "FTH Reserve Stress — Moderate",
            "FTH Reserve Stress — Stressed",
            "FTH Reserve Stress — Crisis",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::ExperimentRunner;
    use crate::report::ExperimentReport;

    #[test]
    fn entropy_sweep_config_valid() {
        let config = FlagshipExperiments::entropy_sweep();
        assert_eq!(config.sweep.step_count(), 10);
        assert_eq!(config.total_worlds(), 200);
        assert!(config.metrics.len() >= 10);
    }

    #[test]
    fn catastrophe_resilience_config_valid() {
        let config = FlagshipExperiments::catastrophe_resilience();
        assert_eq!(config.sweep.step_count(), 7);
        assert_eq!(config.total_worlds(), 140);
        assert!(config.metrics.len() >= 9);
    }

    #[test]
    fn inequality_threshold_config_valid() {
        let config = FlagshipExperiments::inequality_threshold();
        assert_eq!(config.sweep.step_count(), 8);
        assert_eq!(config.total_worlds(), 160);
        assert!(config.metrics.len() >= 10);
    }

    #[test]
    fn quick_entropy_sweep_runs() {
        let config = FlagshipExperiments::entropy_sweep_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 10);
        assert!(result.total_worlds > 0);
        // Should complete without panic
    }

    #[test]
    fn quick_catastrophe_resilience_runs() {
        let config = FlagshipExperiments::catastrophe_resilience_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 7);
    }

    #[test]
    fn quick_inequality_threshold_runs() {
        let config = FlagshipExperiments::inequality_threshold_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 8);
    }

    #[test]
    fn quick_experiment_produces_report() {
        let config = FlagshipExperiments::entropy_sweep_quick();
        let result = ExperimentRunner::run(&config);
        let report = ExperimentReport::generate(
            &result,
            vec!["Quick test finding".into()],
        );
        assert!(report.text_report.contains("Entropy Sweep"));
        assert!(report.csv_data.lines().count() > 1);
        assert!(report.manifest.verify());
    }

    #[test]
    fn treasury_stability_config_valid() {
        let config = FlagshipExperiments::treasury_stability();
        assert_eq!(config.sweep.step_count(), 9);
        assert_eq!(config.total_worlds(), 180);
        assert!(config.metrics.len() >= 12);
    }

    #[test]
    fn quick_treasury_stability_runs() {
        let config = FlagshipExperiments::treasury_stability_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 9);
    }

    #[test]
    fn flagship_list() {
        let list = FlagshipExperiments::list();
        assert_eq!(list.len(), 8);
        assert!(list[0].contains("Entropy"));
        assert!(list[1].contains("Catastrophe"));
        assert!(list[2].contains("Inequality"));
        assert!(list[3].contains("Treasury"));
        assert!(list[4].contains("Calm"));
        assert!(list[5].contains("Moderate"));
        assert!(list[6].contains("Stressed"));
        assert!(list[7].contains("Crisis"));
    }

    #[test]
    fn fth_reserve_calm_config_valid() {
        let config = FlagshipExperiments::fth_reserve_calm();
        assert_eq!(config.sweep.step_count(), 9);
        assert_eq!(config.total_worlds(), 135);
        assert!(config.metrics.len() >= 13);
        assert!(config.base_pressure_override.is_some());
        let p = config.base_pressure_override.unwrap();
        assert!((p.catastrophe_base_prob - 0.001).abs() < 1e-10);
    }

    #[test]
    fn fth_reserve_crisis_config_valid() {
        let config = FlagshipExperiments::fth_reserve_crisis();
        assert_eq!(config.sweep.step_count(), 9);
        assert_eq!(config.total_worlds(), 135);
        let p = config.base_pressure_override.unwrap();
        assert!((p.catastrophe_base_prob - 0.030).abs() < 1e-10);
    }

    #[test]
    fn fth_reserve_stress_suite_valid() {
        let suite = FlagshipExperiments::fth_reserve_stress_suite();
        assert_eq!(suite.len(), 4);
        let total_worlds: usize = suite.iter().map(|(_, c)| c.total_worlds()).sum();
        assert_eq!(total_worlds, 540);
    }

    #[test]
    fn quick_fth_reserve_calm_runs() {
        let config = FlagshipExperiments::fth_reserve_calm_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 9);
        assert!(result.total_worlds > 0);
    }
}
