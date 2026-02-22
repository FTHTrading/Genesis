// ATP Economic Analytics — Econometric indicators for the organism economy.
//
// Pure analytical functions. None of these mutate state. They read
// balances, compute metrics, and return numbers. Zero governance.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Complete econometric snapshot for an epoch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconSnapshot {
    pub epoch: u64,
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub wealth_concentration_top10: f64,
    pub wealth_concentration_top1: f64,
    pub total_supply: f64,
    pub mean_balance: f64,
    pub median_balance: f64,
    pub std_dev: f64,
    pub role_entropy: f64,
    pub replication_success_ratio: f64,
    pub survival_half_life: Option<f64>,
    pub velocity: f64,
}

/// Compute Gini coefficient from a list of balances.
///
/// Gini = 0 ⟹ perfect equality; Gini = 1 ⟹ maximum inequality.
/// Uses the relative mean absolute difference formula:
///   G = Σ_i Σ_j |x_i − x_j| / (2 n² μ)
pub fn gini_coefficient(balances: &[f64]) -> f64 {
    let n = balances.len();
    if n <= 1 {
        return 0.0;
    }
    let mean: f64 = balances.iter().sum::<f64>() / n as f64;
    if mean <= 0.0 {
        return 0.0;
    }

    let mut sum_diff = 0.0;
    for i in 0..n {
        for j in 0..n {
            sum_diff += (balances[i] - balances[j]).abs();
        }
    }
    sum_diff / (2.0 * n as f64 * n as f64 * mean)
}

/// Compute Lorenz curve points from balances.
///
/// Returns a Vec of (cumulative population fraction, cumulative wealth fraction)
/// sorted from lowest to highest balance holder.
pub fn lorenz_curve(balances: &[f64]) -> Vec<(f64, f64)> {
    if balances.is_empty() {
        return vec![(0.0, 0.0), (1.0, 1.0)];
    }

    let mut sorted = balances.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let total: f64 = sorted.iter().sum();
    if total <= 0.0 {
        return vec![(0.0, 0.0), (1.0, 1.0)];
    }

    let n = sorted.len() as f64;
    let mut curve = Vec::with_capacity(sorted.len() + 1);
    curve.push((0.0, 0.0));

    let mut cumulative = 0.0;
    for (i, &val) in sorted.iter().enumerate() {
        cumulative += val;
        curve.push(((i + 1) as f64 / n, cumulative / total));
    }
    curve
}

/// Wealth concentration: fraction of total ATP held by top N%.
pub fn wealth_concentration(balances: &[f64], top_fraction: f64) -> f64 {
    if balances.is_empty() {
        return 0.0;
    }
    let total: f64 = balances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }

    let mut sorted = balances.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)); // descending

    let count = ((balances.len() as f64 * top_fraction).ceil() as usize).max(1);
    let top_sum: f64 = sorted.iter().take(count).sum();
    top_sum / total
}

/// Compute median of a slice.
pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

/// Standard deviation of a slice.
pub fn std_deviation(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

/// Role entropy: Shannon entropy of role distribution.
/// Higher = more diverse. Maximum = ln(num_roles).
pub fn role_entropy(role_counts: &HashMap<String, usize>) -> f64 {
    let total: usize = role_counts.values().sum();
    if total == 0 {
        return 0.0;
    }
    let n = total as f64;
    let mut h = 0.0;
    for &count in role_counts.values() {
        if count > 0 {
            let p = count as f64 / n;
            h -= p * p.ln();
        }
    }
    h
}

/// Survival half-life from a time series of population counts.
///
/// Finds the first epoch where population drops to ≤ 50% of initial.
/// Returns None if it hasn't happened yet.
pub fn survival_half_life(population_history: &[usize]) -> Option<f64> {
    if population_history.is_empty() {
        return None;
    }
    let initial = population_history[0] as f64;
    if initial <= 0.0 {
        return Some(0.0);
    }
    let half = initial / 2.0;
    for (i, &pop) in population_history.iter().enumerate() {
        if (pop as f64) <= half {
            return Some(i as f64);
        }
    }
    None
}

/// Replication success ratio: successful replications / attempts.
pub fn replication_success_ratio(births: u64, attempts: u64) -> f64 {
    if attempts == 0 {
        return 0.0;
    }
    births as f64 / attempts as f64
}

/// ATP velocity: total ATP transacted / total supply in an epoch.
/// Higher = more economic activity.
pub fn atp_velocity(transacted: f64, total_supply: f64) -> f64 {
    if total_supply <= 0.0 {
        return 0.0;
    }
    transacted / total_supply
}

/// Build a full EconSnapshot from raw data.
pub fn snapshot(
    epoch: u64,
    balance_values: &[f64],
    role_counts: &HashMap<String, usize>,
    births: u64,
    replication_attempts: u64,
    population_history: &[usize],
    transacted_atp: f64,
) -> EconSnapshot {
    let total_supply: f64 = balance_values.iter().sum();
    let mean_balance = if balance_values.is_empty() {
        0.0
    } else {
        total_supply / balance_values.len() as f64
    };

    EconSnapshot {
        epoch,
        gini_coefficient: gini_coefficient(balance_values),
        lorenz_curve: lorenz_curve(balance_values),
        wealth_concentration_top10: wealth_concentration(balance_values, 0.10),
        wealth_concentration_top1: wealth_concentration(balance_values, 0.01),
        total_supply,
        mean_balance,
        median_balance: median(balance_values),
        std_dev: std_deviation(balance_values),
        role_entropy: role_entropy(role_counts),
        replication_success_ratio: replication_success_ratio(births, replication_attempts),
        survival_half_life: survival_half_life(population_history),
        velocity: atp_velocity(transacted_atp, total_supply),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gini_perfect_equality() {
        let balances = vec![100.0, 100.0, 100.0, 100.0];
        let g = gini_coefficient(&balances);
        assert!(g.abs() < 0.001, "Perfect equality should yield Gini ≈ 0, got {g}");
    }

    #[test]
    fn test_gini_extreme_inequality() {
        let balances = vec![0.0, 0.0, 0.0, 1000.0];
        let g = gini_coefficient(&balances);
        assert!(g > 0.7, "Extreme inequality should yield high Gini, got {g}");
    }

    #[test]
    fn test_gini_moderate() {
        let balances = vec![10.0, 20.0, 30.0, 40.0, 100.0];
        let g = gini_coefficient(&balances);
        assert!(g > 0.1 && g < 0.6, "Moderate distribution, got {g}");
    }

    #[test]
    fn test_lorenz_curve_shape() {
        let balances = vec![10.0, 20.0, 30.0, 40.0];
        let curve = lorenz_curve(&balances);
        // First point is (0, 0)
        assert_eq!(curve[0], (0.0, 0.0));
        // Last point is (1, 1)
        let last = curve.last().unwrap();
        assert!((last.0 - 1.0).abs() < 0.001);
        assert!((last.1 - 1.0).abs() < 0.001);
        // Monotonically increasing
        for w in curve.windows(2) {
            assert!(w[1].0 >= w[0].0);
            assert!(w[1].1 >= w[0].1);
        }
    }

    #[test]
    fn test_wealth_concentration() {
        // Agent with 90 has 90% of total (100)
        let balances = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 90.0];
        let top10 = wealth_concentration(&balances, 0.10);
        assert!((top10 - 0.90).abs() < 0.01, "Top 10% should hold ~90%, got {top10}");
    }

    #[test]
    fn test_median_odd() {
        assert!((median(&[1.0, 3.0, 5.0]) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_median_even() {
        assert!((median(&[1.0, 2.0, 3.0, 4.0]) - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_role_entropy() {
        // Uniform distribution across 3 roles: H = ln(3) ≈ 1.099
        let mut roles = HashMap::new();
        roles.insert("A".into(), 10);
        roles.insert("B".into(), 10);
        roles.insert("C".into(), 10);
        let h = role_entropy(&roles);
        assert!((h - 3.0_f64.ln()).abs() < 0.01, "Expected ~{}, got {h}", 3.0_f64.ln());
    }

    #[test]
    fn test_role_entropy_monoculture() {
        let mut roles = HashMap::new();
        roles.insert("A".into(), 30);
        roles.insert("B".into(), 0);
        let h = role_entropy(&roles);
        assert!(h.abs() < 0.001, "Monoculture should have entropy ≈ 0, got {h}");
    }

    #[test]
    fn test_survival_half_life() {
        let pops = vec![100, 80, 60, 50, 40];
        let hl = survival_half_life(&pops);
        assert_eq!(hl, Some(3.0)); // First <= 50 at index 3
    }

    #[test]
    fn test_survival_half_life_never() {
        let pops = vec![100, 90, 85, 80];
        assert!(survival_half_life(&pops).is_none());
    }

    #[test]
    fn test_atp_velocity() {
        let v = atp_velocity(500.0, 1000.0);
        assert!((v - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_full_snapshot() {
        let balances = vec![10.0, 20.0, 30.0, 40.0];
        let mut roles = HashMap::new();
        roles.insert("Optimizer".into(), 2);
        roles.insert("Strategist".into(), 2);
        let snap = snapshot(100, &balances, &roles, 5, 10, &[20, 18, 15, 12], 200.0);
        assert_eq!(snap.epoch, 100);
        assert!(snap.gini_coefficient > 0.0);
        assert!(snap.total_supply > 0.0);
        assert!((snap.replication_success_ratio - 0.5).abs() < 0.001);
    }
}
