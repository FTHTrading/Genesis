// Experiment Configuration — What to vary, what to measure
//
// An experiment is defined by:
//   - A variable to sweep (which pressure parameter to vary)
//   - A range (start, end, step)
//   - Runs per step (how many independent trials per parameter value)
//   - Epochs per run (how long each civilization runs)
//   - Metrics to collect (what to measure at the end)
//   - Base physics preset (starting conditions)
//
// This is the experimental protocol. Everything downstream flows from it.

use serde::{Serialize, Deserialize};
use genesis_multiverse::PhysicsPreset;
use gateway::world::PressureConfig;

/// Which pressure parameter to sweep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SweepVariable {
    /// `PressureConfig.entropy_coeff` — cost of existing per epoch.
    EntropyCoeff,
    /// `PressureConfig.soft_cap` — population ceiling before birth suppression.
    SoftCap,
    /// `PressureConfig.catastrophe_base_prob` — base probability of mass death events.
    CatastropheBaseProb,
    /// `PressureConfig.catastrophe_pop_scale` — additional catastrophe risk per agent.
    CatastrophePopScale,
    /// `PressureConfig.gini_wealth_tax_threshold` — Gini level triggering wealth tax.
    GiniWealthTaxThreshold,
    /// `PressureConfig.gini_wealth_tax_rate` — tax rate on top earners.
    GiniWealthTaxRate,
    /// `PressureConfig.treasury_overflow_threshold` — treasury redistribution trigger.
    TreasuryOverflowThreshold,
    /// `MutationEngine.base_rate` — base probability of trait mutation per cycle.
    /// Note: This modifies the world's mutation engine directly, not PressureConfig.
    MutationBaseRate,
}

impl SweepVariable {
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::EntropyCoeff => "entropy_coeff",
            Self::SoftCap => "soft_cap",
            Self::CatastropheBaseProb => "catastrophe_base_prob",
            Self::CatastrophePopScale => "catastrophe_pop_scale",
            Self::GiniWealthTaxThreshold => "gini_wealth_tax_threshold",
            Self::GiniWealthTaxRate => "gini_wealth_tax_rate",
            Self::TreasuryOverflowThreshold => "treasury_overflow_threshold",
            Self::MutationBaseRate => "mutation_base_rate",
        }
    }

    /// All available sweep variables.
    pub fn all() -> &'static [SweepVariable] {
        &[
            Self::EntropyCoeff,
            Self::SoftCap,
            Self::CatastropheBaseProb,
            Self::CatastrophePopScale,
            Self::GiniWealthTaxThreshold,
            Self::GiniWealthTaxRate,
            Self::TreasuryOverflowThreshold,
            Self::MutationBaseRate,
        ]
    }
}

/// Parameter sweep specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSweep {
    /// Which variable to sweep.
    pub variable: SweepVariable,
    /// Start value (inclusive).
    pub start: f64,
    /// End value (inclusive).
    pub end: f64,
    /// Step size.
    pub step: f64,
}

impl ParameterSweep {
    /// Create a new sweep.
    pub fn new(variable: SweepVariable, start: f64, end: f64, step: f64) -> Self {
        Self { variable, start, end, step }
    }

    /// Generate all parameter values in the sweep.
    pub fn values(&self) -> Vec<f64> {
        let mut vals = Vec::new();
        let mut v = self.start;
        // Use epsilon-based comparison for floating point
        while v <= self.end + self.step * 0.001 {
            vals.push(v);
            v += self.step;
        }
        vals
    }

    /// Number of steps in this sweep.
    pub fn step_count(&self) -> usize {
        self.values().len()
    }
}

/// What to measure at the end of each trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Metric {
    /// Final population at last epoch.
    FinalPopulation,
    /// Did the civilization collapse (population = 0)?
    Collapsed,
    /// Mean fitness at last epoch.
    MeanFitness,
    /// Max fitness at last epoch.
    MaxFitness,
    /// Gini coefficient at last epoch.
    GiniCoefficient,
    /// Role entropy at last epoch (Shannon).
    RoleEntropy,
    /// Total births across all epochs.
    TotalBirths,
    /// Total deaths across all epochs.
    TotalDeaths,
    /// Birth-to-death ratio at last epoch.
    BirthDeathRatio,
    /// Treasury ratio at last epoch.
    TreasuryRatio,
    /// Total entropy tax burned across all epochs.
    TotalEntropyBurned,
    /// Total catastrophe deaths across all epochs.
    TotalCatastropheDeaths,
    /// Number of epochs survived before collapse (or max epochs).
    SurvivalEpochs,
    /// Mean population across all epochs.
    MeanPopulation,
    /// Population standard deviation across epochs.
    PopulationVolatility,
    /// Total agent trait mutations across all epochs (from MutationEngine).
    TotalTraitMutations,
    /// Number of pressure mutations applied across all epochs.
    TotalPressureMutations,
    /// Total immune threats detected across all epochs.
    TotalImmuneThreats,
}

impl Metric {
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::FinalPopulation => "final_population",
            Self::Collapsed => "collapsed",
            Self::MeanFitness => "mean_fitness",
            Self::MaxFitness => "max_fitness",
            Self::GiniCoefficient => "gini_coefficient",
            Self::RoleEntropy => "role_entropy",
            Self::TotalBirths => "total_births",
            Self::TotalDeaths => "total_deaths",
            Self::BirthDeathRatio => "birth_death_ratio",
            Self::TreasuryRatio => "treasury_ratio",
            Self::TotalEntropyBurned => "total_entropy_burned",
            Self::TotalCatastropheDeaths => "total_catastrophe_deaths",
            Self::SurvivalEpochs => "survival_epochs",
            Self::MeanPopulation => "mean_population",
            Self::PopulationVolatility => "population_volatility",
            Self::TotalTraitMutations => "total_trait_mutations",
            Self::TotalPressureMutations => "total_pressure_mutations",
            Self::TotalImmuneThreats => "total_immune_threats",
        }
    }

    /// All available metrics.
    pub fn all() -> &'static [Metric] {
        &[
            Self::FinalPopulation,
            Self::Collapsed,
            Self::MeanFitness,
            Self::MaxFitness,
            Self::GiniCoefficient,
            Self::RoleEntropy,
            Self::TotalBirths,
            Self::TotalDeaths,
            Self::BirthDeathRatio,
            Self::TreasuryRatio,
            Self::TotalEntropyBurned,
            Self::TotalCatastropheDeaths,
            Self::SurvivalEpochs,
            Self::MeanPopulation,
            Self::PopulationVolatility,
            Self::TotalTraitMutations,
            Self::TotalPressureMutations,
            Self::TotalImmuneThreats,
        ]
    }

    /// Default set of core metrics for most experiments.
    pub fn core_set() -> Vec<Metric> {
        vec![
            Self::FinalPopulation,
            Self::Collapsed,
            Self::MeanFitness,
            Self::GiniCoefficient,
            Self::TotalBirths,
            Self::TotalDeaths,
            Self::SurvivalEpochs,
            Self::MeanPopulation,
        ]
    }
}

/// Complete experiment configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Human-readable name for this experiment.
    pub name: String,
    /// Description / hypothesis.
    pub hypothesis: String,
    /// The parameter sweep.
    pub sweep: ParameterSweep,
    /// Number of independent trials per parameter value.
    pub runs_per_step: usize,
    /// How many epochs to run each trial.
    pub epochs_per_run: u64,
    /// Which metrics to collect.
    pub metrics: Vec<Metric>,
    /// Base physics preset (before sweep variable is applied).
    pub base_preset: PhysicsPreset,
    /// Optional: custom base pressure config (overrides preset's pressure).
    /// When set, the sweep variable is applied on top of this config
    /// instead of the preset's default pressure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_pressure_override: Option<PressureConfig>,
    /// Optional: override the world's mutation engine base rate.
    /// When set, `world.mutation_engine.base_rate` is forced to this value
    /// before running epochs (independent of the sweep variable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_rate_override: Option<f64>,
    /// Base seed for reproducibility (trial i at step j uses seed = base_seed + j * 1000 + i).
    pub base_seed: u64,
}

impl ExperimentConfig {
    /// Total number of worlds this experiment will spawn.
    pub fn total_worlds(&self) -> usize {
        self.sweep.step_count() * self.runs_per_step
    }

    /// Total epochs this experiment will run.
    pub fn total_epochs(&self) -> u64 {
        self.total_worlds() as u64 * self.epochs_per_run
    }

    /// Compute the seed for a specific trial.
    pub fn trial_seed(&self, step_index: usize, run_index: usize) -> u64 {
        self.base_seed + (step_index as u64) * 1000 + run_index as u64
    }

    /// Short label for display.
    pub fn label(&self) -> String {
        format!(
            "{} | {} sweep [{:.6}..{:.6}] × {} runs × {} epochs",
            self.name,
            self.sweep.variable.name(),
            self.sweep.start,
            self.sweep.end,
            self.runs_per_step,
            self.epochs_per_run,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_values_generated_correctly() {
        let sweep = ParameterSweep::new(
            SweepVariable::EntropyCoeff,
            0.00001,
            0.00005,
            0.00001,
        );
        let vals = sweep.values();
        assert_eq!(vals.len(), 5);
        assert!((vals[0] - 0.00001).abs() < 1e-10);
        assert!((vals[4] - 0.00005).abs() < 1e-10);
    }

    #[test]
    fn sweep_single_value() {
        let sweep = ParameterSweep::new(
            SweepVariable::SoftCap,
            100.0,
            100.0,
            1.0,
        );
        assert_eq!(sweep.step_count(), 1);
        assert_eq!(sweep.values(), vec![100.0]);
    }

    #[test]
    fn config_total_worlds() {
        let config = ExperimentConfig {
            name: "Test".into(),
            hypothesis: "Testing".into(),
            sweep: ParameterSweep::new(SweepVariable::EntropyCoeff, 0.0, 0.0001, 0.00005),
            runs_per_step: 10,
            epochs_per_run: 100,
            metrics: Metric::core_set(),
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            mutation_rate_override: None,
            base_seed: 42,
        };
        // 3 steps × 10 runs = 30 worlds
        assert_eq!(config.total_worlds(), 30);
        assert_eq!(config.total_epochs(), 3000);
    }

    #[test]
    fn trial_seeds_unique() {
        let config = ExperimentConfig {
            name: "Seed test".into(),
            hypothesis: "Seeds are unique".into(),
            sweep: ParameterSweep::new(SweepVariable::SoftCap, 80.0, 120.0, 20.0),
            runs_per_step: 5,
            epochs_per_run: 50,
            metrics: vec![Metric::FinalPopulation],
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            mutation_rate_override: None,
            base_seed: 1000,
        };
        let mut seeds = std::collections::HashSet::new();
        for step in 0..config.sweep.step_count() {
            for run in 0..config.runs_per_step {
                let s = config.trial_seed(step, run);
                assert!(seeds.insert(s), "Duplicate seed: {}", s);
            }
        }
        assert_eq!(seeds.len(), config.total_worlds());
    }

    #[test]
    fn config_label_readable() {
        let config = ExperimentConfig {
            name: "Entropy Sweep".into(),
            hypothesis: "Higher entropy causes collapse".into(),
            sweep: ParameterSweep::new(SweepVariable::EntropyCoeff, 0.00001, 0.0001, 0.00001),
            runs_per_step: 50,
            epochs_per_run: 1000,
            metrics: Metric::core_set(),
            base_preset: PhysicsPreset::EarthPrime,
            base_pressure_override: None,
            mutation_rate_override: None,
            base_seed: 42,
        };
        let label = config.label();
        assert!(label.contains("Entropy Sweep"));
        assert!(label.contains("entropy_coeff"));
        assert!(label.contains("50 runs"));
    }

    #[test]
    fn all_metrics_listed() {
        let all = Metric::all();
        assert!(all.len() >= 17);
        // Each has a unique name
        let names: std::collections::HashSet<&str> = all.iter().map(|m| m.name()).collect();
        assert_eq!(names.len(), all.len());
    }

    #[test]
    fn all_sweep_variables_listed() {
        let all = SweepVariable::all();
        assert_eq!(all.len(), 8);
        let names: std::collections::HashSet<&str> = all.iter().map(|v| v.name()).collect();
        assert_eq!(names.len(), all.len());
    }
}
