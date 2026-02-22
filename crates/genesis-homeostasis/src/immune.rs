// Organism Immune System — Internal feedback detectors.
//
// These are sensors only. They detect threats and report them.
// They do NOT intervene, do NOT override economics, and do NOT
// inject governance. The organism heals itself or it doesn't.
// Information only. Sovereignty respected.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Severity of a detected threat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// Normal variance — not a threat.
    Normal,
    /// Elevated — worth monitoring.
    Watch,
    /// Active threat — organism should react.
    Warning,
    /// Critical — existential risk.
    Critical,
}

/// A detected immune event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneEvent {
    pub kind: ThreatKind,
    pub level: ThreatLevel,
    pub message: String,
    pub metric_value: f64,
    pub threshold: f64,
    pub epoch: u64,
    pub detected_at: DateTime<Utc>,
}

/// Types of threats the immune system can detect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatKind {
    /// Single role dominates (>70% of population).
    MonocultureDominance,
    /// Top agent(s) hold disproportionate ATP share.
    AtpOligarchy,
    /// Mutation rate explodes beyond useful adaptation.
    MutationRunaway,
    /// Population collapsing faster than recovery.
    PopulationCollapse,
    /// A role has zero living agents.
    RoleExtinction,
    /// Treasury is depleting dangerously.
    TreasuryDepletion,
    /// Gini coefficient too high — wealth too concentrated.
    WealthConcentration,
    /// ATP velocity near zero — economy stalled.
    EconomicStagnation,
}

/// Full immune system diagnostic report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneReport {
    pub epoch: u64,
    pub overall_health: ThreatLevel,
    pub events: Vec<ImmuneEvent>,
    pub computed_at: DateTime<Utc>,
}

impl ImmuneReport {
    /// Is the organism in a healthy state?
    pub fn is_healthy(&self) -> bool {
        self.overall_health == ThreatLevel::Normal
    }

    /// Number of active threats.
    pub fn threat_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.level != ThreatLevel::Normal)
            .count()
    }
}

// ─── Detection Functions ────────────────────────────────────────────────

/// Detect monoculture dominance.
///
/// If any single role has >70% of the population, it's a threat.
/// >85% is critical.
pub fn detect_monoculture(role_counts: &HashMap<String, usize>, epoch: u64) -> ImmuneEvent {
    let total: usize = role_counts.values().sum();
    if total == 0 {
        return event(ThreatKind::MonocultureDominance, ThreatLevel::Normal, 0.0, 0.7, epoch,
            "No agents — monoculture detection N/A".into());
    }

    let max_count = *role_counts.values().max().unwrap_or(&0);
    let fraction = max_count as f64 / total as f64;

    let level = if fraction >= 0.85 {
        ThreatLevel::Critical
    } else if fraction >= 0.70 {
        ThreatLevel::Warning
    } else if fraction >= 0.55 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    let dominant_role = role_counts
        .iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k.clone())
        .unwrap_or_default();

    event(
        ThreatKind::MonocultureDominance, level, fraction, 0.70, epoch,
        format!("Role '{dominant_role}' at {:.1}% of population", fraction * 100.0),
    )
}

/// Detect ATP oligarchy.
///
/// If top 10% of agents hold >60% of total ATP, it's a threat.
pub fn detect_atp_oligarchy(balances: &[f64], epoch: u64) -> ImmuneEvent {
    let concentration = genesis_econometrics::wealth_concentration(balances, 0.10);

    let level = if concentration >= 0.80 {
        ThreatLevel::Critical
    } else if concentration >= 0.60 {
        ThreatLevel::Warning
    } else if concentration >= 0.45 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::AtpOligarchy, level, concentration, 0.60, epoch,
        format!("Top 10% hold {:.1}% of ATP", concentration * 100.0),
    )
}

/// Detect mutation runaway.
///
/// If mutation rate exceeds useful bounds, diversity becomes noise.
pub fn detect_mutation_runaway(
    mutation_count: usize,
    population: usize,
    epoch: u64,
) -> ImmuneEvent {
    if population == 0 {
        return event(ThreatKind::MutationRunaway, ThreatLevel::Normal, 0.0, 0.5, epoch,
            "No population — mutation detection N/A".into());
    }

    let rate = mutation_count as f64 / population as f64;

    let level = if rate >= 0.80 {
        ThreatLevel::Critical
    } else if rate >= 0.50 {
        ThreatLevel::Warning
    } else if rate >= 0.30 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::MutationRunaway, level, rate, 0.50, epoch,
        format!("Mutation rate {:.2} per agent per epoch", rate),
    )
}

/// Detect population collapse.
///
/// If population dropped >50% in last N epochs, it's a threat.
pub fn detect_population_collapse(
    population_history: &[usize],
    window: usize,
    epoch: u64,
) -> ImmuneEvent {
    if population_history.len() < 2 {
        return event(ThreatKind::PopulationCollapse, ThreatLevel::Normal, 0.0, 0.50, epoch,
            "Insufficient history for collapse detection".into());
    }

    let recent = &population_history[population_history.len().saturating_sub(window)..];
    if recent.is_empty() || recent[0] == 0 {
        return event(ThreatKind::PopulationCollapse, ThreatLevel::Normal, 0.0, 0.50, epoch,
            "No population baseline".into());
    }

    let start = recent[0] as f64;
    let end = *recent.last().unwrap() as f64;
    let decline = 1.0 - (end / start);

    let level = if decline >= 0.70 {
        ThreatLevel::Critical
    } else if decline >= 0.50 {
        ThreatLevel::Warning
    } else if decline >= 0.30 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::PopulationCollapse, level, decline, 0.50, epoch,
        format!(
            "Population changed from {} to {} ({:.1}% {})",
            start as usize, end as usize,
            decline.abs() * 100.0,
            if decline > 0.0 { "decline" } else { "growth" },
        ),
    )
}

/// Detect role extinction — any role with zero living agents.
pub fn detect_role_extinction(
    role_counts: &HashMap<String, usize>,
    expected_roles: &[&str],
    epoch: u64,
) -> ImmuneEvent {
    let mut extinct: Vec<String> = Vec::new();
    for role in expected_roles {
        let count = role_counts.get(*role).copied().unwrap_or(0);
        if count == 0 {
            extinct.push(role.to_string());
        }
    }

    if extinct.is_empty() {
        event(
            ThreatKind::RoleExtinction, ThreatLevel::Normal, 0.0, 1.0, epoch,
            "All roles represented".into(),
        )
    } else {
        let level = if extinct.len() >= 3 {
            ThreatLevel::Critical
        } else if extinct.len() >= 2 {
            ThreatLevel::Warning
        } else {
            ThreatLevel::Watch
        };
        event(
            ThreatKind::RoleExtinction, level,
            extinct.len() as f64, 1.0, epoch,
            format!("Extinct roles: {}", extinct.join(", ")),
        )
    }
}

/// Detect treasury depletion.
pub fn detect_treasury_depletion(
    reserve: f64,
    peak_reserve: f64,
    epoch: u64,
) -> ImmuneEvent {
    if peak_reserve <= 0.0 {
        return event(ThreatKind::TreasuryDepletion, ThreatLevel::Normal, 0.0, 0.0, epoch,
            "No treasury history".into());
    }

    let ratio = reserve / peak_reserve;
    let depletion = 1.0 - ratio;

    let level = if depletion >= 0.90 {
        ThreatLevel::Critical
    } else if depletion >= 0.70 {
        ThreatLevel::Warning
    } else if depletion >= 0.50 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::TreasuryDepletion, level, depletion, 0.70, epoch,
        format!("Treasury at {:.1}% of peak ({reserve:.1} / {peak_reserve:.1})", ratio * 100.0),
    )
}

/// Detect wealth concentration via Gini coefficient.
pub fn detect_wealth_concentration(balances: &[f64], epoch: u64) -> ImmuneEvent {
    let gini = genesis_econometrics::gini_coefficient(balances);

    let level = if gini >= 0.85 {
        ThreatLevel::Critical
    } else if gini >= 0.70 {
        ThreatLevel::Warning
    } else if gini >= 0.55 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::WealthConcentration, level, gini, 0.70, epoch,
        format!("Gini coefficient: {gini:.3}"),
    )
}

/// Detect economic stagnation — ATP velocity near zero.
pub fn detect_stagnation(
    transacted: f64,
    total_supply: f64,
    epoch: u64,
) -> ImmuneEvent {
    let velocity = genesis_econometrics::atp_velocity(transacted, total_supply);

    let level = if velocity <= 0.01 {
        ThreatLevel::Critical
    } else if velocity <= 0.05 {
        ThreatLevel::Warning
    } else if velocity <= 0.10 {
        ThreatLevel::Watch
    } else {
        ThreatLevel::Normal
    };

    event(
        ThreatKind::EconomicStagnation, level, velocity, 0.05, epoch,
        format!("ATP velocity: {velocity:.3} ({transacted:.1} transacted / {total_supply:.1} supply)"),
    )
}

// ─── Full Diagnostic ────────────────────────────────────────────────────

/// Run the full immune diagnostic suite and produce a report.
pub fn diagnose(
    epoch: u64,
    role_counts: &HashMap<String, usize>,
    balances: &[f64],
    mutation_count: usize,
    population: usize,
    population_history: &[usize],
    pop_window: usize,
    expected_roles: &[&str],
    treasury_reserve: f64,
    peak_treasury: f64,
    transacted_atp: f64,
    total_supply: f64,
) -> ImmuneReport {
    let events = vec![
        detect_monoculture(role_counts, epoch),
        detect_atp_oligarchy(balances, epoch),
        detect_mutation_runaway(mutation_count, population, epoch),
        detect_population_collapse(population_history, pop_window, epoch),
        detect_role_extinction(role_counts, expected_roles, epoch),
        detect_treasury_depletion(treasury_reserve, peak_treasury, epoch),
        detect_wealth_concentration(balances, epoch),
        detect_stagnation(transacted_atp, total_supply, epoch),
    ];

    // Overall health = worst threat level detected
    let overall_health = events
        .iter()
        .map(|e| e.level)
        .max_by_key(|l| match l {
            ThreatLevel::Normal => 0,
            ThreatLevel::Watch => 1,
            ThreatLevel::Warning => 2,
            ThreatLevel::Critical => 3,
        })
        .unwrap_or(ThreatLevel::Normal);

    ImmuneReport {
        epoch,
        overall_health,
        events,
        computed_at: Utc::now(),
    }
}

// ─── Helper ─────────────────────────────────────────────────────────────

fn event(
    kind: ThreatKind,
    level: ThreatLevel,
    metric_value: f64,
    threshold: f64,
    epoch: u64,
    message: String,
) -> ImmuneEvent {
    ImmuneEvent {
        kind,
        level,
        message,
        metric_value,
        threshold,
        epoch,
        detected_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn role_map(vals: &[(&str, usize)]) -> HashMap<String, usize> {
        vals.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn test_monoculture_normal() {
        let roles = role_map(&[("A", 5), ("B", 5), ("C", 5)]);
        let e = detect_monoculture(&roles, 1);
        assert_eq!(e.level, ThreatLevel::Normal);
    }

    #[test]
    fn test_monoculture_warning() {
        let roles = role_map(&[("A", 75), ("B", 10), ("C", 10), ("D", 5)]);
        let e = detect_monoculture(&roles, 1);
        assert_eq!(e.level, ThreatLevel::Warning);
    }

    #[test]
    fn test_monoculture_critical() {
        let roles = role_map(&[("A", 90), ("B", 5), ("C", 5)]);
        let e = detect_monoculture(&roles, 1);
        assert_eq!(e.level, ThreatLevel::Critical);
    }

    #[test]
    fn test_oligarchy_normal() {
        let balances = vec![10.0; 100]; // all equal
        let e = detect_atp_oligarchy(&balances, 1);
        assert_eq!(e.level, ThreatLevel::Normal);
    }

    #[test]
    fn test_oligarchy_critical() {
        let mut balances = vec![1.0; 100];
        balances[0] = 10000.0; // One whale
        let e = detect_atp_oligarchy(&balances, 1);
        assert!(e.level == ThreatLevel::Warning || e.level == ThreatLevel::Critical);
    }

    #[test]
    fn test_mutation_runaway() {
        let e = detect_mutation_runaway(90, 100, 1);
        assert_eq!(e.level, ThreatLevel::Critical);
    }

    #[test]
    fn test_population_collapse() {
        let history = vec![100, 90, 70, 40, 30];
        let e = detect_population_collapse(&history, 5, 1);
        assert!(e.level == ThreatLevel::Warning || e.level == ThreatLevel::Critical);
    }

    #[test]
    fn test_role_extinction() {
        let roles = role_map(&[("Optimizer", 10), ("Strategist", 0)]);
        let e = detect_role_extinction(&roles, &["Optimizer", "Strategist", "Communicator"], 1);
        assert!(e.level != ThreatLevel::Normal);
    }

    #[test]
    fn test_treasury_depletion_warning() {
        let e = detect_treasury_depletion(30.0, 100.0, 1);
        assert_eq!(e.level, ThreatLevel::Warning);
    }

    #[test]
    fn test_stagnation() {
        let e = detect_stagnation(1.0, 10000.0, 1);
        assert!(e.level != ThreatLevel::Normal);
    }

    #[test]
    fn test_full_diagnose_healthy() {
        let roles = role_map(&[
            ("Optimizer", 10), ("Strategist", 10), ("Communicator", 10),
            ("Archivist", 10), ("Executor", 10),
        ]);
        let balances = vec![50.0; 50];
        let history = vec![50, 50, 50, 50, 50];

        let report = diagnose(
            100, &roles, &balances, 5, 50, &history, 5,
            &["Optimizer", "Strategist", "Communicator", "Archivist", "Executor"],
            500.0, 500.0, 500.0, 2500.0,
        );

        assert!(report.is_healthy());
        assert_eq!(report.threat_count(), 0);
    }

    #[test]
    fn test_full_diagnose_sick() {
        let roles = role_map(&[("Optimizer", 90), ("Strategist", 5), ("Communicator", 5)]);
        let mut balances = vec![1.0; 100];
        balances[0] = 5000.0;
        let history = vec![200, 150, 100, 50, 30];

        let report = diagnose(
            100, &roles, &balances, 80, 100, &history, 5,
            &["Optimizer", "Strategist", "Communicator", "Archivist", "Executor"],
            10.0, 500.0, 5.0, 5100.0,
        );

        assert!(!report.is_healthy());
        assert!(report.threat_count() > 0);
    }
}
