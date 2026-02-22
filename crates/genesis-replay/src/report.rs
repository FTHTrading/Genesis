// Replay Report — Summary of a replay run with comparisons and statistics.

use serde::{Serialize, Deserialize};

use crate::trajectory::Trajectory;

/// Full report from a replay run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayReport {
    pub seed: u64,
    pub total_epochs: u64,
    pub final_population: usize,
    pub went_extinct: bool,
    pub extinction_epoch: Option<u64>,
    pub fitness_slope: f64,
    pub equilibrium_epoch: Option<u64>,
    pub peak_population: usize,
    pub peak_atp: f64,
    pub min_population: usize,
    pub total_births: u64,
    pub total_deaths: u64,
    pub total_mutations: u64,
    pub treasury_peak: f64,
    pub deterministic: bool,
}

impl ReplayReport {
    /// Generate a report from a trajectory.
    pub fn from_trajectory(traj: &Trajectory, deterministic: bool) -> Self {
        let peak_pop = traj.points.iter().map(|p| p.population).max().unwrap_or(0);
        let min_pop = traj.points.iter().map(|p| p.population).min().unwrap_or(0);
        let peak_atp = traj.points.iter().map(|p| p.total_atp).fold(0.0_f64, f64::max);
        let total_births: u64 = traj.points.iter().map(|p| p.births).sum();
        let total_deaths: u64 = traj.points.iter().map(|p| p.deaths).sum();
        let total_mutations: u64 = traj.points.iter().map(|p| p.mutations).sum();
        let treasury_peak = traj.points.iter()
            .map(|p| p.treasury_reserve)
            .fold(0.0_f64, f64::max);

        Self {
            seed: traj.seed,
            total_epochs: traj.total_epochs,
            final_population: traj.final_population(),
            went_extinct: traj.went_extinct(),
            extinction_epoch: traj.extinction_epoch(),
            fitness_slope: traj.fitness_slope(),
            equilibrium_epoch: traj.equilibrium_epoch(50, 4.0),
            peak_population: peak_pop,
            peak_atp,
            min_population: min_pop,
            total_births,
            total_deaths,
            total_mutations,
            treasury_peak,
            deterministic,
        }
    }

    /// Format as human-readable summary.
    pub fn summary(&self) -> String {
        let mut s = String::new();
        s.push_str("╔══════════════════════════════════════════════════════╗\n");
        s.push_str("║              GENESIS REPLAY REPORT                  ║\n");
        s.push_str("╚══════════════════════════════════════════════════════╝\n");
        s.push_str(&format!("  Seed:               {}\n", self.seed));
        s.push_str(&format!("  Epochs:             {}\n", self.total_epochs));
        s.push_str(&format!("  Deterministic:      {}\n", self.deterministic));
        s.push_str(&format!("  Final population:   {}\n", self.final_population));
        s.push_str(&format!("  Peak population:    {}\n", self.peak_population));
        s.push_str(&format!("  Min population:     {}\n", self.min_population));
        s.push_str(&format!("  Went extinct:       {}\n", self.went_extinct));
        if let Some(e) = self.extinction_epoch {
            s.push_str(&format!("  Extinction at:      epoch {}\n", e));
        }
        s.push_str(&format!("  Fitness slope:      {:.8}\n", self.fitness_slope));
        if let Some(eq) = self.equilibrium_epoch {
            s.push_str(&format!("  Equilibrium at:     epoch {}\n", eq));
        }
        s.push_str(&format!("  Total births:       {}\n", self.total_births));
        s.push_str(&format!("  Total deaths:       {}\n", self.total_deaths));
        s.push_str(&format!("  Total mutations:    {}\n", self.total_mutations));
        s.push_str(&format!("  Peak ATP:           {:.2}\n", self.peak_atp));
        s.push_str(&format!("  Treasury peak:      {:.2}\n", self.treasury_peak));
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{ReplayConfig, ReplayEngine};

    #[test]
    fn test_report_generation() {
        let config = ReplayConfig {
            epochs: 200,
            ..Default::default()
        };
        let mut engine = ReplayEngine::new(config).unwrap();
        let traj = engine.run();
        let report = ReplayReport::from_trajectory(&traj, true);
        assert!(report.total_epochs > 0);
        assert!(!report.summary().is_empty());
    }
}
