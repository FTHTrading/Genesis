// scripts/simulate.rs — 10,000-epoch simulation with CSV output
//
// This is a standalone script that imports the gateway crate's World
// and runs 10,000 epochs, outputting CSV data for empirical analysis.
//
// Usage: cargo run --example simulate -- [epochs]
// Output: dist/simulation-10k.csv

use gateway::world::World;
use std::fs;

fn main() {
    let target_epochs: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000);

    eprintln!("[SIM] Running {} epochs...", target_epochs);

    let mut world = World::new();

    // CSV header
    let mut csv = String::from(
        "epoch,population,total_atp,mean_fitness,max_fitness,min_fitness,\
         births,deaths,mutations,stasis_count,market_solved,market_rewarded,\
         resources_extracted,total_resources,season,catastrophe_active,\
         dynamic_pop_cap,treasury_reserve,treasury_distributed,\
         optimizer_count,strategist_count,communicator_count,archivist_count,executor_count\n"
    );

    for _ in 0..target_epochs {
        let stats = world.run_epoch();

        let opt = stats.role_counts.get(&genesis_dna::AgentRole::Optimizer).copied().unwrap_or(0);
        let str = stats.role_counts.get(&genesis_dna::AgentRole::Strategist).copied().unwrap_or(0);
        let com = stats.role_counts.get(&genesis_dna::AgentRole::Communicator).copied().unwrap_or(0);
        let arc = stats.role_counts.get(&genesis_dna::AgentRole::Archivist).copied().unwrap_or(0);
        let exe = stats.role_counts.get(&genesis_dna::AgentRole::Executor).copied().unwrap_or(0);

        csv.push_str(&format!(
            "{},{},{:.2},{:.4},{:.4},{:.4},{},{},{},{},{},{:.2},{:.2},{:.2},{},{},{},{:.2},{:.2},{},{},{},{},{}\n",
            stats.epoch,
            stats.population,
            stats.total_atp,
            stats.mean_fitness,
            stats.max_fitness,
            stats.min_fitness,
            stats.births,
            stats.deaths,
            stats.mutations,
            stats.stasis_count,
            stats.market_solved,
            stats.market_rewarded,
            stats.resources_extracted,
            stats.total_resources,
            stats.season,
            stats.catastrophe_active,
            stats.dynamic_pop_cap,
            stats.treasury_reserve,
            stats.treasury_distributed,
            opt, str, com, arc, exe,
        ));

        // Progress indicator every 1000 epochs
        if (stats.epoch + 1) % 1000 == 0 {
            eprintln!(
                "  [{}] pop={} atp={:.1} fitness={:.3} treasury={:.1} births={} deaths={}",
                stats.epoch + 1,
                stats.population,
                stats.total_atp,
                stats.mean_fitness,
                stats.treasury_reserve,
                stats.births,
                stats.deaths,
            );
        }
    }

    // Write CSV
    let dist = std::path::Path::new("dist");
    if !dist.exists() {
        fs::create_dir_all(dist).unwrap();
    }
    let output_path = dist.join("simulation-10k.csv");
    fs::write(&output_path, &csv).unwrap();
    eprintln!("[SIM] Output: {}", output_path.display());

    // Summary stats
    eprintln!("\n[SIM] === SUMMARY ===");
    eprintln!("  Final population: {}", world.agents.len());
    eprintln!("  Final ATP supply: {:.1}", world.ledger.total_supply());
    eprintln!("  Treasury reserve: {:.1}", world.treasury.reserve);
    eprintln!("  Treasury collected: {:.1}", world.treasury.total_collected);
    eprintln!("  Treasury distributed: {:.1}", world.treasury.total_distributed);
    eprintln!("  Total births: {}", world.total_births);
    eprintln!("  Total deaths: {}", world.total_deaths);
    eprintln!("  Epochs run: {}", world.epoch);
}
