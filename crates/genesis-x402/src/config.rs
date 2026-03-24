// config.rs — X402Config loaded from environment variables
//
// Required env vars (server / seller mode):
//   GENESIS_X402_ENABLED=true
//   GENESIS_X402_PAY_TO=0x...         Genesis hot wallet (receives USDC)
//   GENESIS_X402_FACILITATOR_URL=...  Default: CDP mainnet
//   GENESIS_X402_CDP_API_KEY=...      CDP API key (required for mainnet)
//
// Optional pricing (in USDC atomic units, 6 decimals):
//   GENESIS_X402_PRICE_AI_CALL=1000       ($0.001)
//   GENESIS_X402_PRICE_DATA_PULL=500      ($0.0005)
//   GENESIS_X402_PRICE_AGENT_SPAWN=10000  ($0.01)
//   GENESIS_X402_PRICE_VOICE_ACTION=5000  ($0.005)
//   GENESIS_X402_PRICE_DOC_GEN=2000       ($0.002)
//
// Vault (EVM private key, encrypted):
//   GENESIS_VAULT_PATH=C:\Users\...\genesis-vault.enc
//   GENESIS_VAULT_PASSPHRASE=...

use std::time::Duration;

/// Prices for each protected endpoint, in USDC atomic units (6 decimals).
/// 1_000_000 = $1.00 USDC.  1_000 = $0.001 USDC.
#[derive(Debug, Clone)]
pub struct PriceTable {
    /// GET /api/ai-call — per AI inference call
    pub ai_call: u64,
    /// GET /api/data — premium ecosystem data pull
    pub data_pull: u64,
    /// POST /api/agent/spawn — direct agent spawning via API
    pub agent_spawn: u64,
    /// POST /api/voice — voice action proxy
    pub voice_action: u64,
    /// POST /api/doc — document generation
    pub doc_gen: u64,
}

impl Default for PriceTable {
    fn default() -> Self {
        Self {
            ai_call:      1_000,   // $0.001
            data_pull:      500,   // $0.0005
            agent_spawn:  10_000,  // $0.01
            voice_action:  5_000,  // $0.005
            doc_gen:       2_000,  // $0.002
        }
    }
}

#[derive(Debug, Clone)]
pub struct X402Config {
    /// Whether x402 payment enforcement is active.
    pub enabled: bool,

    /// Polygon USDC contract address.
    pub usdc_contract: String,

    /// CAIP-2 network identifier.
    pub network: String,

    /// EVM address that receives USDC payments (Genesis hot wallet).
    pub pay_to: String,

    /// CDP facilitator base URL.
    pub facilitator_url: String,

    /// CDP API key (required for mainnet facilitator).
    pub cdp_api_key: Option<String>,

    /// CDP API secret.
    pub cdp_api_secret: Option<String>,

    /// Endpoint price table.
    pub prices: PriceTable,

    /// Path to the encrypted EVM vault file.
    pub vault_path: String,

    /// Vault encryption passphrase.
    pub vault_passphrase: Option<String>,

    /// Timeout for facilitator calls.
    pub facilitator_timeout: Duration,

    /// Path to the lineage ledger JSONL file.
    pub lineage_path: String,
}

impl X402Config {
    /// Load from environment.  Returns None if GENESIS_X402_ENABLED != "true".
    pub fn from_env() -> Option<Self> {
        let enabled = std::env::var("GENESIS_X402_ENABLED")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);
        if !enabled { return None; }

        let pay_to = std::env::var("GENESIS_X402_PAY_TO").unwrap_or_default();
        if pay_to.is_empty() {
            tracing::warn!("GENESIS_X402_ENABLED=true but GENESIS_X402_PAY_TO not set — x402 disabled");
            return None;
        }

        fn price_var(name: &str, default: u64) -> u64 {
            std::env::var(name)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default)
        }

        let defaults = PriceTable::default();

        Some(Self {
            enabled: true,
            usdc_contract: std::env::var("GENESIS_X402_USDC_CONTRACT")
                .unwrap_or_else(|_| crate::USDC_POLYGON.to_string()),
            network: std::env::var("GENESIS_X402_NETWORK")
                .unwrap_or_else(|_| crate::NETWORK_POLYGON.to_string()),
            pay_to,
            facilitator_url: std::env::var("GENESIS_X402_FACILITATOR_URL")
                .unwrap_or_else(|_| crate::CDP_FACILITATOR_URL.to_string()),
            cdp_api_key:    std::env::var("GENESIS_X402_CDP_API_KEY").ok(),
            cdp_api_secret: std::env::var("GENESIS_X402_CDP_API_SECRET").ok(),
            prices: PriceTable {
                ai_call:      price_var("GENESIS_X402_PRICE_AI_CALL",      defaults.ai_call),
                data_pull:    price_var("GENESIS_X402_PRICE_DATA_PULL",    defaults.data_pull),
                agent_spawn:  price_var("GENESIS_X402_PRICE_AGENT_SPAWN",  defaults.agent_spawn),
                voice_action: price_var("GENESIS_X402_PRICE_VOICE_ACTION", defaults.voice_action),
                doc_gen:      price_var("GENESIS_X402_PRICE_DOC_GEN",      defaults.doc_gen),
            },
            vault_path:       std::env::var("GENESIS_VAULT_PATH")
                .unwrap_or_else(|_| "genesis-vault.enc".to_string()),
            vault_passphrase: std::env::var("GENESIS_VAULT_PASSPHRASE").ok(),
            facilitator_timeout: Duration::from_secs(
                std::env::var("GENESIS_X402_FACILITATOR_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10)
            ),
            lineage_path: std::env::var("GENESIS_LINEAGE_PATH")
                .unwrap_or_else(|_| "lineage/ledger.jsonl".to_string()),
        })
    }

    /// Returns true if a CDP API key is configured.
    pub fn has_cdp_auth(&self) -> bool {
        self.cdp_api_key.is_some() && self.cdp_api_secret.is_some()
    }
}
