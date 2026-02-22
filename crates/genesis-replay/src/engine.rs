// Replay Engine — Deterministic evolutionary replay
//
// Seeds all RNG, replays each epoch step by step, captures trajectory
// data, and optionally verifies against an anchor chain.

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::trajectory::{Trajectory, EpochPoint};
use crate::errors::ReplayError;

/// Configuration for a replay run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// RNG seed for deterministic replay.
    pub seed: u64,
    /// Number of epochs to replay.
    pub epochs: u64,
    /// Initial population count.
    pub initial_population: usize,
    /// Initial ATP per agent.
    pub initial_atp: f64,
    /// Base resource capacity per niche.
    pub base_capacity: f64,
    /// Logistic regeneration rate.
    pub regen_rate: f64,
    /// Mutation base rate.
    pub mutation_rate: f64,
    /// Maximum mutation delta.
    pub mutation_max_delta: f64,
    /// Replication cost.
    pub replication_cost: f64,
    /// Decay rate per epoch.
    pub decay_rate: f64,
    /// Wealth tax threshold.
    pub wealth_tax_threshold: f64,
    /// Wealth tax rate.
    pub wealth_tax_rate: f64,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            seed: 7729,
            epochs: 10_000,
            initial_population: 20,
            initial_atp: 50.0,
            base_capacity: 150.0,
            regen_rate: 0.12,
            mutation_rate: 0.15,
            mutation_max_delta: 0.1,
            replication_cost: 25.0,
            decay_rate: 0.02,
            wealth_tax_threshold: 100.0,
            wealth_tax_rate: 0.01,
        }
    }
}

/// Deterministic replay engine — lightweight epoch simulation using
/// seeded PRNG for all stochastic decisions.
pub struct ReplayEngine {
    pub config: ReplayConfig,
    /// LCG state for deterministic random.
    rng_state: u64,
}

impl ReplayEngine {
    pub fn new(config: ReplayConfig) -> Result<Self, ReplayError> {
        if config.seed == 0 {
            return Err(ReplayError::InvalidSeed);
        }
        Ok(Self {
            rng_state: config.seed,
            config,
        })
    }

    /// Deterministic random f64 in [0, 1).
    fn rand_f64(&mut self) -> f64 {
        // Linear congruential generator (same constants as Environment::event_seed)
        self.rng_state = self.rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.rng_state >> 33) as f64 / (u32::MAX as f64)
    }

    /// Run the full replay and return the trajectory.
    pub fn run(&mut self) -> Trajectory {
        let mut trajectory = Trajectory::new(self.config.seed);

        // Initialize agents: (fitness, atp_balance)
        let mut agents: Vec<(f64, f64)> = (0..self.config.initial_population)
            .map(|i| {
                // Deterministic initial fitness from seed + index
                let mut hasher = Sha256::new();
                hasher.update(self.config.seed.to_le_bytes());
                hasher.update(i.to_le_bytes());
                let hash: [u8; 32] = hasher.finalize().into();
                let fitness = (hash[0] as f64 / 255.0) * 0.5 + 0.3; // [0.3, 0.8]
                (fitness, self.config.initial_atp)
            })
            .collect();

        let mut treasury: f64 = 0.0;
        let mut _total_births: u64 = 0;
        let mut _total_deaths: u64 = 0;
        let mut resource_level = self.config.base_capacity * 0.8;

        for epoch in 1..=self.config.epochs {
            let mut births: u64 = 0;
            let mut _deaths: u64 = 0;
            let mut mutations: u64 = 0;

            // Resource regeneration (logistic)
            let growth = self.config.regen_rate * resource_level
                * (1.0 - resource_level / self.config.base_capacity);
            resource_level = (resource_level + growth).clamp(0.0, self.config.base_capacity);

            // Catastrophe check (~2%)
            let catastrophe = self.rand_f64() < 0.02;
            if catastrophe {
                resource_level *= 0.4;
            }

            // Resource extraction per agent (proportional)
            let demand_per = if agents.is_empty() { 0.0 } else {
                resource_level * 0.4 / agents.len() as f64
            };

            // Metabolic tick for each agent
            let agent_count = agents.len().max(1);
            for agent in agents.iter_mut() {
                // Extract resources
                let extracted = demand_per.min(resource_level / agent_count as f64);
                agent.1 += extracted;
                resource_level -= extracted;

                // Basal cost
                agent.1 -= 0.15;

                // Decay
                agent.1 *= 1.0 - self.config.decay_rate;

                // Mutation
                if self.rand_f64() < self.config.mutation_rate {
                    let delta = (self.rand_f64() - 0.5) * 2.0 * self.config.mutation_max_delta;
                    agent.0 = (agent.0 + delta).clamp(0.01, 1.0);
                    mutations += 1;
                }
            }

            // Wealth tax
            for agent in agents.iter_mut() {
                if agent.1 > self.config.wealth_tax_threshold {
                    let tax = (agent.1 - self.config.wealth_tax_threshold) * self.config.wealth_tax_rate;
                    agent.1 -= tax;
                    treasury += tax;
                }
            }

            // Death: agents with balance <= 0
            let before = agents.len();
            agents.retain(|a| a.1 > 0.0);
            _deaths = (before - agents.len()) as u64;
            _total_deaths += _deaths;

            // Replication: top fitness agents with enough ATP
            let mut new_agents = Vec::new();
            let birth_cap = 3;
            for agent in agents.iter_mut() {
                if births >= birth_cap {
                    break;
                }
                if agent.0 > 0.35 && agent.1 > self.config.replication_cost {
                    agent.1 -= self.config.replication_cost;
                    // Child fitness = parent ± small mutation
                    let delta = (self.rand_f64() - 0.5) * 0.1;
                    let child_fitness = (agent.0 + delta).clamp(0.01, 1.0);
                    new_agents.push((child_fitness, 8.0)); // CHILD_GRANT
                    births += 1;
                    _total_births += 1;
                }
            }
            agents.extend(new_agents);

            // Compute state hash
            let mut sorted_balances: Vec<String> = agents
                .iter()
                .map(|(f, b)| format!("{:.8}:{:.8}", f, b))
                .collect();
            sorted_balances.sort();
            let mut hasher = Sha256::new();
            for s in &sorted_balances {
                hasher.update(s.as_bytes());
            }
            let state_hash = hex::encode(hasher.finalize());

            // Fitness stats
            let mean_fitness = if agents.is_empty() {
                0.0
            } else {
                agents.iter().map(|a| a.0).sum::<f64>() / agents.len() as f64
            };
            let max_fitness = agents.iter().map(|a| a.0).fold(0.0_f64, f64::max);
            let min_fitness = agents.iter().map(|a| a.0).fold(1.0_f64, f64::min);
            let total_atp: f64 = agents.iter().map(|a| a.1).sum();

            trajectory.push(EpochPoint {
                epoch,
                population: agents.len(),
                total_atp,
                mean_fitness,
                max_fitness,
                min_fitness,
                births,
                deaths: _deaths,
                mutations,
                treasury_reserve: treasury,
                role_counts: std::collections::HashMap::new(),
                resources_total: resource_level,
                state_hash,
            });

            // Extinction
            if agents.is_empty() {
                break;
            }
        }

        trajectory
    }

    /// Verify two replay runs are identical (deterministic guarantee).
    pub fn verify_determinism(config: &ReplayConfig) -> Result<bool, ReplayError> {
        let mut engine1 = ReplayEngine::new(config.clone())?;
        let mut engine2 = ReplayEngine::new(config.clone())?;

        let traj1 = engine1.run();
        let traj2 = engine2.run();

        if traj1.points.len() != traj2.points.len() {
            return Err(ReplayError::Divergence {
                epoch: 0,
                detail: format!(
                    "Length mismatch: {} vs {}",
                    traj1.points.len(),
                    traj2.points.len()
                ),
            });
        }

        for (p1, p2) in traj1.points.iter().zip(traj2.points.iter()) {
            if p1.state_hash != p2.state_hash {
                return Err(ReplayError::Divergence {
                    epoch: p1.epoch,
                    detail: format!(
                        "State hash mismatch at epoch {}: {} vs {}",
                        p1.epoch,
                        &p1.state_hash[..16],
                        &p2.state_hash[..16]
                    ),
                });
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replay() {
        let config = ReplayConfig {
            epochs: 100,
            ..Default::default()
        };
        let mut engine = ReplayEngine::new(config).unwrap();
        let traj = engine.run();
        assert!(!traj.points.is_empty());
        assert!(traj.points[0].population > 0);
    }

    #[test]
    fn test_determinism() {
        let config = ReplayConfig {
            epochs: 500,
            ..Default::default()
        };
        assert!(ReplayEngine::verify_determinism(&config).unwrap());
    }

    #[test]
    fn test_different_seeds_diverge() {
        let config1 = ReplayConfig {
            seed: 42,
            epochs: 100,
            ..Default::default()
        };
        let config2 = ReplayConfig {
            seed: 1337,
            epochs: 100,
            ..Default::default()
        };
        let mut e1 = ReplayEngine::new(config1).unwrap();
        let mut e2 = ReplayEngine::new(config2).unwrap();
        let t1 = e1.run();
        let t2 = e2.run();

        // Trajectories should diverge
        let any_different = t1.points.iter().zip(t2.points.iter())
            .any(|(p1, p2)| p1.state_hash != p2.state_hash);
        assert!(any_different);
    }

    #[test]
    fn test_csv_export() {
        let config = ReplayConfig {
            epochs: 50,
            ..Default::default()
        };
        let mut engine = ReplayEngine::new(config).unwrap();
        let traj = engine.run();
        let csv = traj.to_csv();
        assert!(csv.starts_with("epoch,"));
        assert!(csv.lines().count() > 1);
    }
}
