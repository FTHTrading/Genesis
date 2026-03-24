// pricing.rs — Action price table and lookup
//
// Every action in the world has a price in micro-units.
// 1,000,000 micro = $1.00 USDC (6 USDC decimals).
//
// Default prices match the on-chain x402PaymentAdapter price grid.
// The table is modifiable at runtime (admin-only, audited).

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;

/// Every action the world recognizes as a priced event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    // Compute / AI
    AiCall,
    VoiceAction,
    ComputeRent,
    DataPull,

    // Agent lifecycle
    AgentSpawn,
    AgentMessage,
    AgentMove,
    EnergyConsume,

    // World travel
    ZoneEnter,

    // Economy
    TradeExecute,
    ItemMint,
    ItemRepair,
    AssetTransfer,
    AssetList,

    // Governance / docs
    PermitFile,
    PropertySearch,

    // Infra
    AnalyticsRead,
    RewardClaim,
    VaultManage,

    // Custom (keyed by string label)
    Custom,
}

impl ActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AiCall          => "AI_CALL",
            Self::VoiceAction     => "VOICE_ACTION",
            Self::ComputeRent     => "COMPUTE_RENT",
            Self::DataPull        => "DATA_PULL",
            Self::AgentSpawn      => "AGENT_SPAWN",
            Self::AgentMessage    => "AGENT_MESSAGE",
            Self::AgentMove       => "AGENT_MOVE",
            Self::EnergyConsume   => "ENERGY_CONSUME",
            Self::ZoneEnter       => "ZONE_ENTER",
            Self::TradeExecute    => "TRADE_EXECUTE",
            Self::ItemMint        => "ITEM_MINT",
            Self::ItemRepair      => "ITEM_REPAIR",
            Self::AssetTransfer   => "ASSET_TRANSFER",
            Self::AssetList       => "ASSET_LIST",
            Self::PermitFile      => "PERMIT_FILE",
            Self::PropertySearch  => "PROPERTY_SEARCH",
            Self::AnalyticsRead   => "ANALYTICS_READ",
            Self::RewardClaim     => "REWARD_CLAIM",
            Self::VaultManage     => "VAULT_MANAGE",
            Self::Custom          => "CUSTOM",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionCategory {
    Compute,
    Social,
    Asset,
    Governance,
    Lifecycle,
}

/// One entry in the price table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricedEntry {
    pub action_type: String,
    pub price_units: u64,       // micro-units
    pub category:   ActionCategory,
    pub description: String,
    pub enabled:     bool,
    pub min_balance: u64,       // minimum balance needed to execute
}

/// The canonical price table for all world actions.
/// Thread-safe: reads are lock-free concurrent, writes are exclusive.
pub struct PriceTable {
    inner: RwLock<HashMap<String, PricedEntry>>,
}

impl PriceTable {
    /// Initialize with the default production price grid.
    pub fn default_production() -> Self {
        let mut map = HashMap::new();

        let entries = vec![
            // 1000 micro = $0.001
            ("AI_CALL",          1_000,  ActionCategory::Compute,    "AI inference call"),
            ("AGENT_MESSAGE",    1_000,  ActionCategory::Social,     "Agent-to-agent message"),
            ("ZONE_ENTER",       1_000,  ActionCategory::Social,     "Enter a world zone"),
            ("AGENT_MOVE",       1_000,  ActionCategory::Lifecycle,  "Agent movement step"),
            ("ENERGY_CONSUME",   1_000,  ActionCategory::Lifecycle,  "Energy consumption event"),
            ("ANALYTICS_READ",   1_000,  ActionCategory::Compute,    "Read analytics data"),
            // 2000 micro = $0.002
            ("TRADE_EXECUTE",    2_000,  ActionCategory::Asset,      "Execute a trade"),
            ("ITEM_REPAIR",      2_000,  ActionCategory::Asset,      "Repair an item"),
            ("PERMIT_FILE",      2_000,  ActionCategory::Governance, "File a permit"),
            ("REWARD_CLAIM",     2_000,  ActionCategory::Governance, "Claim a reward"),
            // 5000 micro = $0.005
            ("ITEM_MINT",        5_000,  ActionCategory::Asset,      "Mint an item"),
            ("PROPERTY_SEARCH",  5_000,  ActionCategory::Asset,      "Search property records"),
            ("VOICE_ACTION",     5_000,  ActionCategory::Compute,    "Voice synthesis action"),
            ("COMPUTE_RENT",     5_000,  ActionCategory::Compute,    "Rent compute resources"),
            ("DATA_PULL",        5_000,  ActionCategory::Compute,    "Pull external data"),
            // 10000 micro = $0.010
            ("AGENT_SPAWN",     10_000,  ActionCategory::Lifecycle,  "Spawn a new agent"),
            // 25000 micro = $0.025
            ("ASSET_LIST",      25_000,  ActionCategory::Asset,      "List an asset"),
            // 50000 micro = $0.050
            ("VAULT_MANAGE",    50_000,  ActionCategory::Governance, "Manage a vault"),
            // 100000 micro = $0.100
            ("ASSET_TRANSFER", 100_000,  ActionCategory::Asset,      "Transfer asset ownership"),
        ];

        for (action, price, category, desc) in entries {
            map.insert(action.to_string(), PricedEntry {
                action_type:  action.to_string(),
                price_units:  price,
                category,
                description:  desc.to_string(),
                enabled:      true,
                min_balance:  price, // must have at least action price
            });
        }

        Self { inner: RwLock::new(map) }
    }

    /// Override from environment variables if set.
    /// Format: GENESIS_PRICE_<ACTION>=<micro_units>
    pub fn apply_env_overrides(&self) {
        let mut map = self.inner.write();
        for (key, val) in std::env::vars() {
            if let Some(action) = key.strip_prefix("GENESIS_PRICE_") {
                if let Ok(price) = val.parse::<u64>() {
                    if let Some(entry) = map.get_mut(action) {
                        entry.price_units = price;
                        entry.min_balance = price;
                        tracing::info!("Price override: {} = {} micro", action, price);
                    }
                }
            }
        }
    }

    /// Look up the price for an action. Returns None if unknown or disabled.
    pub fn price(&self, action_type: &str) -> Option<u64> {
        let map = self.inner.read();
        map.get(action_type)
            .filter(|e| e.enabled)
            .map(|e| e.price_units)
    }

    /// Look up minimum balance required to execute an action.
    pub fn min_balance(&self, action_type: &str) -> u64 {
        let map = self.inner.read();
        map.get(action_type)
            .filter(|e| e.enabled)
            .map(|e| e.min_balance)
            .unwrap_or(0)
    }

    /// Upsert a price entry (admin operation).
    pub fn set_price(&self, action_type: impl Into<String>, price_units: u64, category: ActionCategory, description: impl Into<String>) {
        let action_type = action_type.into();
        let mut map = self.inner.write();
        let entry = map.entry(action_type.clone()).or_insert(PricedEntry {
            action_type: action_type.clone(),
            price_units: 0,
            category,
            description: String::new(),
            enabled: true,
            min_balance: 0,
        });
        entry.price_units  = price_units;
        entry.min_balance  = price_units;
        entry.category     = category;
        entry.description  = description.into();
    }

    /// Disable an action (no-op without error, just rejected at execution time).
    pub fn disable(&self, action_type: &str) {
        let mut map = self.inner.write();
        if let Some(entry) = map.get_mut(action_type) {
            entry.enabled = false;
        }
    }

    /// Snapshot the full price table.
    pub fn snapshot(&self) -> Vec<PricedEntry> {
        let map = self.inner.read();
        map.values().cloned().collect()
    }
}

impl Default for PriceTable {
    fn default() -> Self {
        Self::default_production()
    }
}
