// Statistical Aggregation — Mean, median, stddev, percentiles
//
// Every experiment produces distributions, not single values.
// This module computes descriptive statistics across trial populations.
//
// StatSummary is the fundamental unit: given N values, it computes
// all the statistics needed to characterize a distribution.

use serde::{Serialize, Deserialize};

/// Descriptive statistics for a set of values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatSummary {
    /// Number of values.
    pub count: usize,
    /// Arithmetic mean.
    pub mean: f64,
    /// Median (50th percentile).
    pub median: f64,
    /// Standard deviation.
    pub std_dev: f64,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// 10th percentile.
    pub p10: f64,
    /// 25th percentile.
    pub p25: f64,
    /// 75th percentile.
    pub p75: f64,
    /// 90th percentile.
    pub p90: f64,
    /// Sum of all values.
    pub sum: f64,
    /// Variance.
    pub variance: f64,
}

impl StatSummary {
    /// Compute summary statistics from a slice of values.
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self::empty();
        }

        let n = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / n as f64;

        let variance = if n > 1 {
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let std_dev = variance.sqrt();

        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let min = sorted[0];
        let max = sorted[n - 1];
        let median = percentile_sorted(&sorted, 50.0);
        let p10 = percentile_sorted(&sorted, 10.0);
        let p25 = percentile_sorted(&sorted, 25.0);
        let p75 = percentile_sorted(&sorted, 75.0);
        let p90 = percentile_sorted(&sorted, 90.0);

        Self {
            count: n,
            mean,
            median,
            std_dev,
            min,
            max,
            p10,
            p25,
            p75,
            p90,
            sum,
            variance,
        }
    }

    /// Empty summary (used when no values are provided).
    pub fn empty() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            min: 0.0,
            max: 0.0,
            p10: 0.0,
            p25: 0.0,
            p75: 0.0,
            p90: 0.0,
            sum: 0.0,
            variance: 0.0,
        }
    }

    /// Coefficient of variation (stddev / mean). Returns 0 if mean is 0.
    pub fn cv(&self) -> f64 {
        if self.mean.abs() < 1e-12 {
            0.0
        } else {
            self.std_dev / self.mean.abs()
        }
    }

    /// Interquartile range.
    pub fn iqr(&self) -> f64 {
        self.p75 - self.p25
    }

    /// One-line summary for display.
    pub fn summary_line(&self) -> String {
        format!(
            "mean={:.4} ± {:.4} | median={:.4} | [{:.4}, {:.4}] | n={}",
            self.mean, self.std_dev, self.median, self.min, self.max, self.count
        )
    }
}

/// Compute the p-th percentile from a sorted slice.
/// Uses linear interpolation (same as numpy's default).
fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }

    let n = sorted.len();
    let rank = (p / 100.0) * (n - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let frac = rank - lower as f64;

    if lower == upper || upper >= n {
        sorted[lower.min(n - 1)]
    } else {
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_values() {
        let s = StatSummary::from_values(&[]);
        assert_eq!(s.count, 0);
        assert_eq!(s.mean, 0.0);
    }

    #[test]
    fn single_value() {
        let s = StatSummary::from_values(&[42.0]);
        assert_eq!(s.count, 1);
        assert!((s.mean - 42.0).abs() < 1e-10);
        assert!((s.median - 42.0).abs() < 1e-10);
        assert!((s.min - 42.0).abs() < 1e-10);
        assert!((s.max - 42.0).abs() < 1e-10);
        assert_eq!(s.std_dev, 0.0);
    }

    #[test]
    fn known_distribution() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let s = StatSummary::from_values(&values);
        assert_eq!(s.count, 5);
        assert!((s.mean - 3.0).abs() < 1e-10);
        assert!((s.median - 3.0).abs() < 1e-10);
        assert!((s.min - 1.0).abs() < 1e-10);
        assert!((s.max - 5.0).abs() < 1e-10);
        assert!((s.sum - 15.0).abs() < 1e-10);
        // stddev of [1,2,3,4,5] with sample variance = √2.5 ≈ 1.5811
        assert!((s.std_dev - 1.5811).abs() < 0.001);
    }

    #[test]
    fn percentiles_correct() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let s = StatSummary::from_values(&values);
        assert!((s.p10 - 10.9).abs() < 0.1);
        assert!((s.p25 - 25.75).abs() < 0.1);
        assert!((s.median - 50.5).abs() < 0.1);
        assert!((s.p75 - 75.25).abs() < 0.1);
        assert!((s.p90 - 90.1).abs() < 0.1);
    }

    #[test]
    fn cv_calculation() {
        let s = StatSummary::from_values(&[10.0, 10.0, 10.0]);
        assert_eq!(s.cv(), 0.0);

        let s2 = StatSummary::from_values(&[0.0, 0.0, 0.0]);
        assert_eq!(s2.cv(), 0.0); // zero mean → zero cv
    }

    #[test]
    fn iqr_calculation() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let s = StatSummary::from_values(&values);
        assert!((s.iqr() - 49.5).abs() < 0.1);
    }

    #[test]
    fn summary_line_readable() {
        let s = StatSummary::from_values(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let line = s.summary_line();
        assert!(line.contains("mean="));
        assert!(line.contains("median="));
        assert!(line.contains("n=5"));
    }

    #[test]
    fn unsorted_input_handled() {
        let values = vec![5.0, 1.0, 3.0, 4.0, 2.0];
        let s = StatSummary::from_values(&values);
        assert!((s.min - 1.0).abs() < 1e-10);
        assert!((s.max - 5.0).abs() < 1e-10);
        assert!((s.median - 3.0).abs() < 1e-10);
    }
}
