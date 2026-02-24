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
            mutation_rate_override: None,
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
            mutation_rate_override: None,
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
            mutation_rate_override: None,
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
            mutation_rate_override: None,
            base_seed: 20260222,
        }
    }

    // ─── Reserve Stress Suite ────────────────────────────────────────────
    //
    // 4-tier shock experiment: How should reserve deployment policy
    // change when external shock frequency increases?
    //
    // Each tier sets a different baseline catastrophe_base_prob
    // (representing shock frequency) while sweeping
    // treasury_overflow_threshold (reserve deployment policy).
    //
    // Domain mapping:
    //   Reserve deployment    → treasury overflow threshold
    //   Shock frequency       → catastrophe_base_prob
    //   Economic stress       → entropy taxation
    //   Wealth concentration  → Gini threshold
    //
    // Answers: "Should reserve deployment policy change when
    // external conditions deteriorate?"

    /// Reserve Stress — Calm (shock = 0.001)
    ///
    /// Baseline: Low shock frequency, representing stable conditions.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn reserve_calm() -> ExperimentConfig {
        Self::reserve_tier("Calm", 0.001,
            "Under calm conditions (shock=0.001), aggressive treasury \
             deployment outperforms hoarding with minimal downside risk")
    }

    /// Reserve Stress — Moderate (shock = 0.005)
    ///
    /// Baseline: Moderate shock frequency, representing normal volatility.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn reserve_moderate() -> ExperimentConfig {
        Self::reserve_tier("Moderate", 0.005,
            "Under moderate shocks (shock=0.005), the optimal deployment \
             threshold shifts toward conservative reserves")
    }

    /// Reserve Stress — Stressed (shock = 0.015)
    ///
    /// Baseline: High shock frequency, representing environmental stress.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn reserve_stressed() -> ExperimentConfig {
        Self::reserve_tier("Stressed", 0.015,
            "Under stressed conditions (shock=0.015), reserve hoarding begins \
             to outperform deployment as shock recovery demands liquidity buffers")
    }

    /// Reserve Stress — Crisis (shock = 0.030)
    ///
    /// Baseline: Extreme shock frequency, representing crisis conditions.
    /// Sweeps treasury_overflow_threshold 0.10 → 0.90.
    /// 9 steps × 15 runs × 500 epochs = 135 worlds, 67,500 epochs.
    pub fn reserve_crisis() -> ExperimentConfig {
        Self::reserve_tier("Crisis", 0.030,
            "Under crisis conditions (shock=0.030), conservative reserve \
             management becomes critical for survival — deployment policy \
             must shift dramatically or civilizations collapse")
    }

    /// Returns all 4 reserve stress tier experiments.
    pub fn reserve_stress_suite() -> Vec<(&'static str, ExperimentConfig)> {
        vec![
            ("reserve_calm", Self::reserve_calm()),
            ("reserve_moderate", Self::reserve_moderate()),
            ("reserve_stressed", Self::reserve_stressed()),
            ("reserve_crisis", Self::reserve_crisis()),
        ]
    }

    /// Internal: build a single reserve stress tier experiment.
    fn reserve_tier(tier_label: &str, shock_prob: f64, hypothesis: &str) -> ExperimentConfig {
        let mut base_pressure = PressureConfig::default();
        base_pressure.catastrophe_base_prob = shock_prob;

        ExperimentConfig {
            name: format!("Reserve Stress — {} (shock={:.3})", tier_label, shock_prob),
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
            mutation_rate_override: None,
            base_seed: 20260222,
        }
    }

    // ─── Resource Depletion Crossover ───────────────────────────────────
    //
    // 4-tier carrying capacity experiment: How does metabolic cost
    // sensitivity change when the population's carrying capacity is
    // compressed?
    //
    // Each tier sets a different baseline soft_cap (carrying capacity)
    // while sweeping entropy_coeff (metabolic cost of existence).
    //
    // Domain mapping:
    //   Carrying capacity  → soft_cap
    //   Metabolic cost     → entropy_coeff
    //   Resource scarcity  → lower soft_cap = fewer sustainable agents
    //
    // Answers: "Does the relationship between metabolic cost and
    // population health change under resource scarcity?"

    /// Resource Depletion — Abundant (soft_cap = 200)
    ///
    /// Baseline: High carrying capacity, representing resource abundance.
    /// Sweeps entropy_coeff 0.00001 → 0.00010, step 0.00001.
    /// 10 steps × 15 runs × 500 epochs = 150 worlds, 75,000 epochs.
    pub fn resource_depletion_abundant() -> ExperimentConfig {
        Self::resource_depletion_tier("Abundant", 200, 
            "Under abundant resources (soft_cap=200), increasing metabolic \
             cost has minimal impact on population stability")
    }

    /// Resource Depletion — Normal (soft_cap = 120)
    ///
    /// Baseline: Normal carrying capacity, representing typical conditions.
    /// Sweeps entropy_coeff 0.00001 → 0.00010, step 0.00001.
    /// 10 steps × 15 runs × 500 epochs = 150 worlds, 75,000 epochs.
    pub fn resource_depletion_normal() -> ExperimentConfig {
        Self::resource_depletion_tier("Normal", 120,
            "Under normal capacity (soft_cap=120), metabolic cost increases \
             produce measurable but manageable population stress")
    }

    /// Resource Depletion — Constrained (soft_cap = 60)
    ///
    /// Baseline: Low carrying capacity, representing resource constraints.
    /// Sweeps entropy_coeff 0.00001 → 0.00010, step 0.00001.
    /// 10 steps × 15 runs × 500 epochs = 150 worlds, 75,000 epochs.
    pub fn resource_depletion_constrained() -> ExperimentConfig {
        Self::resource_depletion_tier("Constrained", 60,
            "Under constrained resources (soft_cap=60), metabolic cost \
             becomes a critical survival factor — small increases may \
             trigger population collapse")
    }

    /// Resource Depletion — Scarce (soft_cap = 30)
    ///
    /// Baseline: Minimal carrying capacity, representing extreme scarcity.
    /// Sweeps entropy_coeff 0.00001 → 0.00010, step 0.00001.
    /// 10 steps × 15 runs × 500 epochs = 150 worlds, 75,000 epochs.
    pub fn resource_depletion_scarce() -> ExperimentConfig {
        Self::resource_depletion_tier("Scarce", 30,
            "Under extreme scarcity (soft_cap=30), even moderate metabolic \
             cost increases become existential — population collapse \
             thresholds shift dramatically")
    }

    /// Returns all 4 resource depletion tier experiments.
    pub fn resource_depletion_suite() -> Vec<(&'static str, ExperimentConfig)> {
        vec![
            ("resource_depletion_abundant", Self::resource_depletion_abundant()),
            ("resource_depletion_normal", Self::resource_depletion_normal()),
            ("resource_depletion_constrained", Self::resource_depletion_constrained()),
            ("resource_depletion_scarce", Self::resource_depletion_scarce()),
        ]
    }

    /// Internal: build a single resource depletion tier experiment.
    fn resource_depletion_tier(tier_label: &str, soft_cap: usize, hypothesis: &str) -> ExperimentConfig {
        let mut base_pressure = PressureConfig::default();
        base_pressure.soft_cap = soft_cap;

        ExperimentConfig {
            name: format!("Resource Depletion — {} (cap={})", tier_label, soft_cap),
            hypothesis: hypothesis.into(),
            sweep: ParameterSweep::new(
                SweepVariable::EntropyCoeff,
                0.00001,
                0.00010,
                0.00001,
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
            mutation_rate_override: None,
            base_seed: 20260222,
        }
    }

    // ─── Evolution Forbidden ────────────────────────────────────────────
    //
    // Control experiment: What happens when mutation is completely disabled?
    //
    // Identical to catastrophe_resilience EXCEPT mutation_base_rate is
    // forced to 0.0. Agents cannot adapt their traits. If the organism
    // survives catastrophe only because mutation lets low-fitness agents
    // explore new trait configurations, then forbidding evolution should
    // reveal the dependency.
    //
    // The question: Is Genesis Protocol's resilience due to adaptation,
    // or is the economic structure alone sufficient?
    //
    // Domain mapping:
    //   Catastrophe frequency → catastrophe_base_prob (swept)
    //   Adaptation            → mutation_base_rate (locked at 0.0)
    //
    // Answers: "Can a civilization survive without evolution?"

    /// Evolution Forbidden: Catastrophe Resilience Without Adaptation
    ///
    /// Identical catastrophe sweep as `catastrophe_resilience()` but with
    /// mutation_base_rate locked at 0.0. No trait mutation occurs.
    /// 7 steps × 20 runs × 500 epochs = 140 worlds, 70,000 total epochs.
    pub fn evolution_forbidden() -> ExperimentConfig {
        ExperimentConfig {
            name: "Evolution Forbidden: Survival Without Adaptation".into(),
            hypothesis: "Without mutation (base_rate=0.0), civilizations lose resilience \
                         under catastrophe pressure — collapse rates increase relative to \
                         the standard catastrophe resilience experiment, proving that \
                         adaptation is a necessary survival mechanism".into(),
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
                Metric::TotalTraitMutations,
                Metric::MeanFitness,
                Metric::BirthDeathRatio,
                Metric::GiniCoefficient,
                Metric::TotalBirths,
                Metric::TotalDeaths,
            ],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            mutation_rate_override: Some(0.0), // THE KEY: no mutation allowed
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

    pub fn reserve_calm_quick() -> ExperimentConfig {
        let mut config = Self::reserve_calm();
        config.name = "Reserve Calm (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn reserve_moderate_quick() -> ExperimentConfig {
        let mut config = Self::reserve_moderate();
        config.name = "Reserve Moderate (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn reserve_stressed_quick() -> ExperimentConfig {
        let mut config = Self::reserve_stressed();
        config.name = "Reserve Stressed (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn reserve_crisis_quick() -> ExperimentConfig {
        let mut config = Self::reserve_crisis();
        config.name = "Reserve Crisis (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn resource_depletion_abundant_quick() -> ExperimentConfig {
        let mut config = Self::resource_depletion_abundant();
        config.name = "Resource Depletion Abundant (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn resource_depletion_normal_quick() -> ExperimentConfig {
        let mut config = Self::resource_depletion_normal();
        config.name = "Resource Depletion Normal (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn resource_depletion_constrained_quick() -> ExperimentConfig {
        let mut config = Self::resource_depletion_constrained();
        config.name = "Resource Depletion Constrained (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn resource_depletion_scarce_quick() -> ExperimentConfig {
        let mut config = Self::resource_depletion_scarce();
        config.name = "Resource Depletion Scarce (Quick)".into();
        config.runs_per_step = 3;
        config.epochs_per_run = 50;
        config
    }

    pub fn evolution_forbidden_quick() -> ExperimentConfig {
        let mut config = Self::evolution_forbidden();
        config.name = "Evolution Forbidden (Quick)".into();
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
            "Reserve Stress — Calm",
            "Reserve Stress — Moderate",
            "Reserve Stress — Stressed",
            "Reserve Stress — Crisis",
            "Resource Depletion — Abundant",
            "Resource Depletion — Normal",
            "Resource Depletion — Constrained",
            "Resource Depletion — Scarce",
            "Evolution Forbidden: Survival Without Adaptation",
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
        assert_eq!(list.len(), 13);
        assert!(list[0].contains("Entropy"));
        assert!(list[1].contains("Catastrophe"));
        assert!(list[2].contains("Inequality"));
        assert!(list[3].contains("Treasury"));
        assert!(list[4].contains("Calm"));
        assert!(list[5].contains("Moderate"));
        assert!(list[6].contains("Stressed"));
        assert!(list[7].contains("Crisis"));
        assert!(list[8].contains("Abundant"));
        assert!(list[9].contains("Normal"));
        assert!(list[10].contains("Constrained"));
        assert!(list[11].contains("Scarce"));
        assert!(list[12].contains("Evolution Forbidden"));
    }

    #[test]
    fn reserve_calm_config_valid() {
        let config = FlagshipExperiments::reserve_calm();
        assert_eq!(config.sweep.step_count(), 9);
        assert_eq!(config.total_worlds(), 135);
        assert!(config.metrics.len() >= 13);
        assert!(config.base_pressure_override.is_some());
        let p = config.base_pressure_override.unwrap();
        assert!((p.catastrophe_base_prob - 0.001).abs() < 1e-10);
    }

    #[test]
    fn reserve_crisis_config_valid() {
        let config = FlagshipExperiments::reserve_crisis();
        assert_eq!(config.sweep.step_count(), 9);
        assert_eq!(config.total_worlds(), 135);
        let p = config.base_pressure_override.unwrap();
        assert!((p.catastrophe_base_prob - 0.030).abs() < 1e-10);
    }

    #[test]
    fn reserve_stress_suite_valid() {
        let suite = FlagshipExperiments::reserve_stress_suite();
        assert_eq!(suite.len(), 4);
        let total_worlds: usize = suite.iter().map(|(_, c)| c.total_worlds()).sum();
        assert_eq!(total_worlds, 540);
    }

    #[test]
    fn quick_reserve_calm_runs() {
        let config = FlagshipExperiments::reserve_calm_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 9);
        assert!(result.total_worlds > 0);
    }

    #[test]
    fn resource_depletion_abundant_config_valid() {
        let config = FlagshipExperiments::resource_depletion_abundant();
        assert_eq!(config.sweep.step_count(), 10);
        assert_eq!(config.total_worlds(), 150);
        assert!(config.metrics.len() >= 13);
        assert!(config.base_pressure_override.is_some());
        let p = config.base_pressure_override.unwrap();
        assert_eq!(p.soft_cap, 200);
    }

    #[test]
    fn resource_depletion_scarce_config_valid() {
        let config = FlagshipExperiments::resource_depletion_scarce();
        assert_eq!(config.sweep.step_count(), 10);
        assert_eq!(config.total_worlds(), 150);
        let p = config.base_pressure_override.unwrap();
        assert_eq!(p.soft_cap, 30);
    }

    #[test]
    fn resource_depletion_suite_valid() {
        let suite = FlagshipExperiments::resource_depletion_suite();
        assert_eq!(suite.len(), 4);
        let total_worlds: usize = suite.iter().map(|(_, c)| c.total_worlds()).sum();
        assert_eq!(total_worlds, 600);
    }

    #[test]
    fn quick_resource_depletion_abundant_runs() {
        let config = FlagshipExperiments::resource_depletion_abundant_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 10);
        assert!(result.total_worlds > 0);
    }

    #[test]
    fn evolution_forbidden_config_valid() {
        let config = FlagshipExperiments::evolution_forbidden();
        assert_eq!(config.sweep.step_count(), 7);
        assert_eq!(config.total_worlds(), 140);
        assert!(config.metrics.len() >= 10);
        assert_eq!(config.mutation_rate_override, Some(0.0));
        assert!(config.base_pressure_override.is_none());
    }

    #[test]
    fn quick_evolution_forbidden_runs() {
        let config = FlagshipExperiments::evolution_forbidden_quick();
        let result = ExperimentRunner::run(&config);
        assert_eq!(result.steps.len(), 7);
        assert!(result.total_worlds > 0);
        // Verify mutation was actually suppressed: pressure_mutations should be 0
        for step in &result.steps {
            for trial in &step.trials {
                if let Some(&mutations) = trial.metrics.get("total_trait_mutations") {
                    assert_eq!(mutations, 0.0,
                        "Expected zero trait mutations with mutation_rate_override=0.0");
                }
            }
        }
    }
}
