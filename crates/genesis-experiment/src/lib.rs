// Genesis Experiment Engine — Hypothesis-driven macroeconomic research
//
// This crate converts the multiverse from infrastructure into instrument.
//
// Core workflow:
//   1. Define an ExperimentConfig — what variable to sweep, what metrics to collect
//   2. Run the experiment — spawn worlds, advance them, collect EpochStats
//   3. Aggregate results — mean, median, stddev, percentiles per metric per step
//   4. Export — CSV, text summary, SHA-256 anchored manifest
//   5. Replay — given a manifest, reproduce any civilization exactly
//
// Every experiment is reproducible. Every result is hashable.
// This is how Genesis becomes scientific.

pub mod config;
pub mod runner;
pub mod stats;
pub mod manifest;
pub mod report;
pub mod flagship;

pub use config::{ExperimentConfig, ParameterSweep, Metric, SweepVariable};
pub use runner::{ExperimentRunner, ExperimentResult, TrialResult, StepResult};
pub use stats::StatSummary;
pub use manifest::ReplayManifest;
pub use report::ExperimentReport;
pub use flagship::FlagshipExperiments;
