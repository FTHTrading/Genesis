// Trajectory — Time series data captured during a replay run.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A single epoch snapshot in the trajectory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochPoint {
    pub epoch: u64,
    pub population: usize,
    pub total_atp: f64,
    pub mean_fitness: f64,
    pub max_fitness: f64,
    pub min_fitness: f64,
    pub births: u64,
    pub deaths: u64,
    pub mutations: u64,
    pub treasury_reserve: f64,
    pub role_counts: HashMap<String, usize>,
    pub resources_total: f64,
    /// SHA-256 hash of the sorted agent balance vector.
    pub state_hash: String,
}

/// Complete trajectory of a replay run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub seed: u64,
    pub total_epochs: u64,
    pub points: Vec<EpochPoint>,
}

impl Trajectory {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            total_epochs: 0,
            points: Vec::new(),
        }
    }

    pub fn push(&mut self, point: EpochPoint) {
        self.total_epochs = point.epoch;
        self.points.push(point);
    }

    /// Export trajectory to CSV format.
    pub fn to_csv(&self) -> String {
        let mut csv = String::from(
            "epoch,population,total_atp,mean_fitness,max_fitness,min_fitness,births,deaths,mutations,treasury_reserve,resources_total\n"
        );
        for p in &self.points {
            csv.push_str(&format!(
                "{},{},{:.4},{:.6},{:.6},{:.6},{},{},{},{:.4},{:.4}\n",
                p.epoch, p.population, p.total_atp, p.mean_fitness,
                p.max_fitness, p.min_fitness, p.births, p.deaths,
                p.mutations, p.treasury_reserve, p.resources_total,
            ));
        }
        csv
    }

    /// Compute fitness growth slope via linear regression over the trajectory.
    pub fn fitness_slope(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let n = self.points.len() as f64;
        let sum_x: f64 = self.points.iter().map(|p| p.epoch as f64).sum();
        let sum_y: f64 = self.points.iter().map(|p| p.mean_fitness).sum();
        let sum_xy: f64 = self.points.iter().map(|p| p.epoch as f64 * p.mean_fitness).sum();
        let sum_xx: f64 = self.points.iter().map(|p| (p.epoch as f64).powi(2)).sum();

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < 1e-12 {
            return 0.0;
        }
        (n * sum_xy - sum_x * sum_y) / denom
    }

    /// Find the epoch at which population first stabilizes (variance < threshold
    /// over a window of `window` epochs).
    pub fn equilibrium_epoch(&self, window: usize, threshold: f64) -> Option<u64> {
        if self.points.len() < window {
            return None;
        }
        for i in window..self.points.len() {
            let slice = &self.points[i - window..i];
            let mean_pop: f64 = slice.iter().map(|p| p.population as f64).sum::<f64>()
                / window as f64;
            let variance: f64 = slice
                .iter()
                .map(|p| (p.population as f64 - mean_pop).powi(2))
                .sum::<f64>()
                / window as f64;
            if variance < threshold {
                return Some(self.points[i].epoch);
            }
        }
        None
    }

    /// Population at the final epoch.
    pub fn final_population(&self) -> usize {
        self.points.last().map(|p| p.population).unwrap_or(0)
    }

    /// Whether the organism went extinct during the run.
    pub fn went_extinct(&self) -> bool {
        self.points.iter().any(|p| p.population == 0)
    }

    /// Extinction epoch (if any).
    pub fn extinction_epoch(&self) -> Option<u64> {
        self.points.iter().find(|p| p.population == 0).map(|p| p.epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_export() {
        let mut traj = Trajectory::new(42);
        traj.push(EpochPoint {
            epoch: 1,
            population: 20,
            total_atp: 1000.0,
            mean_fitness: 0.5,
            max_fitness: 0.8,
            min_fitness: 0.2,
            births: 0,
            deaths: 0,
            mutations: 0,
            treasury_reserve: 0.0,
            role_counts: HashMap::new(),
            resources_total: 500.0,
            state_hash: "abc".into(),
        });
        let csv = traj.to_csv();
        assert!(csv.contains("1,20,1000.0000"));
    }

    #[test]
    fn test_fitness_slope() {
        let mut traj = Trajectory::new(42);
        for i in 0..100 {
            traj.push(EpochPoint {
                epoch: i,
                population: 20,
                total_atp: 1000.0,
                mean_fitness: 0.5 + i as f64 * 0.001,
                max_fitness: 0.8,
                min_fitness: 0.2,
                births: 0,
                deaths: 0,
                mutations: 0,
                treasury_reserve: 0.0,
                role_counts: HashMap::new(),
                resources_total: 500.0,
                state_hash: String::new(),
            });
        }
        let slope = traj.fitness_slope();
        assert!((slope - 0.001).abs() < 0.0001);
    }
}
