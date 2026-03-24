// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// ══════════════════════════════════════════════════════════════════════════════
// AssetRegistry.sol
//
// On-chain registry for every world asset.
// Assets are the first bucket: things owned, controlled, or consumable.
//
// Asset taxonomy covers:
//   CIVIC:      land, zones, buildings, roads, utility grids
//   COMPUTE:    OpenAI inference units, Cloudflare Worker hours, Edge nodes
//   BANDWIDTH:  Cloudflare CDN/R2 egress, network throughput
//   VOICE:      ElevenLabs character credits, audio synthesis units
//   ENERGY:     power grid units
//   MATERIAL:   raw world materials
//   INVENTORY:  warehouse stock
//   LICENSE:    IP, software, API access grants
//   VAULT_SHARE:patron vault position (linked to PatronVault contract)
//   TITLE:      property title (linked to TitleNFT)
//   DATA:       datasets, analytics packages
//   INSURANCE:  coverage pool units
//
// Rule: NO asset changes state without emitting an AssetJournalEntry event.
// ══════════════════════════════════════════════════════════════════════════════

import "@openzeppelin/contracts/access/AccessControl.sol";

contract AssetRegistry is AccessControl {
    bytes32 public constant REGISTRAR_ROLE = keccak256("REGISTRAR_ROLE");
    bytes32 public constant VALUATOR_ROLE  = keccak256("VALUATOR_ROLE");
    bytes32 public constant ORACLE_ROLE    = keccak256("ORACLE_ROLE");

    // ── Asset type taxonomy ───────────────────────────────────────────────
    enum AssetType {
        LAND,           // 0
        BUILDING,       // 1
        ZONE,           // 2
        UTILITY_GRID,   // 3
        COMPUTE,        // 4  — OpenAI inference, Cloudflare Workers
        BANDWIDTH,      // 5  — Cloudflare CDN, R2, DNS
        VOICE,          // 6  — ElevenLabs synthesis credits
        ENERGY,         // 7
        MATERIAL,       // 8
        FOOD,           // 9
        WATER,          // 10
        INVENTORY,      // 11
        LICENSE,        // 12
        VAULT_SHARE,    // 13
        DATA,           // 14
        INSURANCE,      // 15
        VEHICLE,        // 16
        EQUIPMENT,      // 17
        RENTAL_RIGHT,   // 18
        DEBT_NOTE       // 19 (asset side: receivable)
    }

    // Liquidity class determines how freely transferable the asset is
    enum LiquidityClass {
        LOCKED,         // non-transferable (soulbound-equivalent assets)
        RESTRICTED,     // requires approval or policy clearance
        SEMI_LIQUID,    // market with friction (vesting, lock periods)
        LIQUID          // freely tradeable
    }

    // Income model
    enum IncomeModel {
        NONE,
        RENT,
        FEE_PER_USE,
        YIELD,
        ROYALTY,
        STIPEND
    }

    struct Asset {
        uint256     id;
        AssetType   assetType;
        uint256     ownerId;           // IdentityRegistry entity id
        string      name;
        string      unitOfMeasure;     // "sqm", "kwh", "token", "credit", "hour", "mb"
        uint256     quantity;          // amount in unitOfMeasure (scaled by 1e6)
        uint256     bookValueUsdc;     // carrying value × 1e6 (USDC 6-decimal)
        uint256     marketValueUsdc;   // last oracle valuation × 1e6
        LiquidityClass liquidity;
        IncomeModel incomeModel;
        uint256     maintenanceCostMicroPerEpoch; // USDC microcredits per world epoch
        bytes32     oracleSource;      // bytes32 identifier of price oracle
        uint64      lastValuationAt;
        bool        active;

        // Provider metadata (for compute/bandwidth/voice assets)
        bytes32     providerKey;       // keccak256("CLOUDFLARE"|"OPENAI"|"ELEVENLABS")
        bytes32     serviceId;         // provider-specific resource ID
    }

    // ── State ─────────────────────────────────────────────────────────────
    mapping(uint256 => Asset)          public assets;
    // ownerId → list of asset ids
    mapping(uint256 => uint256[])      public ownerAssets;

    uint256 private _nextId = 1;

    // ── Known provider keys ───────────────────────────────────────────────
    bytes32 public constant PROVIDER_CLOUDFLARE  = keccak256("CLOUDFLARE");
    bytes32 public constant PROVIDER_OPENAI      = keccak256("OPENAI");
    bytes32 public constant PROVIDER_ELEVENLABS  = keccak256("ELEVENLABS");

    // ── Events ────────────────────────────────────────────────────────────
    event AssetRegistered(
        uint256 indexed id,
        AssetType indexed assetType,
        uint256 indexed ownerId,
        bytes32 providerKey,
        uint64 timestamp
    );

    event AssetJournalEntry(
        uint256 indexed assetId,
        bytes32 indexed eventType,   // "CONSUME" | "TRANSFER" | "REVALUE" | "DEPRECIATE" | "REPLENISH"
        uint256 quantityDelta,       // always positive; eventType determines direction
        uint256 newBookValue,
        bytes32 referenceId,
        uint64  timestamp
    );

    event AssetTransferred(
        uint256 indexed assetId,
        uint256 indexed fromOwnerId,
        uint256 indexed toOwnerId,
        uint256 quantity,
        uint256 priceUsdc,
        bytes32 referenceId
    );

    event AssetValuationUpdated(
        uint256 indexed assetId,
        uint256 oldMarketValue,
        uint256 newMarketValue,
        bytes32 oracleSource,
        uint64  timestamp
    );

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(REGISTRAR_ROLE, admin);
        _grantRole(VALUATOR_ROLE, admin);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Registration
    // ══════════════════════════════════════════════════════════════════════

    function registerAsset(
        AssetType   assetType,
        uint256     ownerId,
        string calldata name,
        string calldata unitOfMeasure,
        uint256     quantity,
        uint256     bookValueUsdc,
        LiquidityClass liquidity,
        IncomeModel incomeModel,
        uint256     maintenanceCostMicroPerEpoch,
        bytes32     oracleSource,
        bytes32     providerKey,
        bytes32     serviceId
    ) external onlyRole(REGISTRAR_ROLE) returns (uint256 id) {
        id = _nextId++;
        assets[id] = Asset({
            id:                          id,
            assetType:                   assetType,
            ownerId:                     ownerId,
            name:                        name,
            unitOfMeasure:               unitOfMeasure,
            quantity:                    quantity,
            bookValueUsdc:               bookValueUsdc,
            marketValueUsdc:             bookValueUsdc,
            liquidity:                   liquidity,
            incomeModel:                 incomeModel,
            maintenanceCostMicroPerEpoch: maintenanceCostMicroPerEpoch,
            oracleSource:                oracleSource,
            lastValuationAt:             uint64(block.timestamp),
            active:                      true,
            providerKey:                 providerKey,
            serviceId:                   serviceId
        });
        ownerAssets[ownerId].push(id);
        emit AssetRegistered(id, assetType, ownerId, providerKey, uint64(block.timestamp));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Consumption (burn quantity, record expense)
    // ══════════════════════════════════════════════════════════════════════

    function consumeAsset(uint256 id, uint256 quantity, bytes32 referenceId)
        external onlyRole(REGISTRAR_ROLE)
    {
        Asset storage a = assets[id];
        require(a.active, "Asset inactive");
        require(a.quantity >= quantity, "Insufficient quantity");
        a.quantity -= quantity;
        // book value decreases proportionally
        if (a.quantity == 0) {
            a.bookValueUsdc = 0;
        } else {
            a.bookValueUsdc = (a.bookValueUsdc * a.quantity) / (a.quantity + quantity);
        }
        emit AssetJournalEntry(id, "CONSUME", quantity, a.bookValueUsdc, referenceId, uint64(block.timestamp));
    }

    /// Replenish asset quantity (e.g. Cloudflare quota reset, energy top-up).
    function replenishAsset(uint256 id, uint256 quantity, uint256 costUsdc, bytes32 referenceId)
        external onlyRole(REGISTRAR_ROLE)
    {
        Asset storage a = assets[id];
        require(a.active, "Asset inactive");
        a.quantity      += quantity;
        a.bookValueUsdc += costUsdc;
        emit AssetJournalEntry(id, "REPLENISH", quantity, a.bookValueUsdc, referenceId, uint64(block.timestamp));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Valuation oracle update
    // ══════════════════════════════════════════════════════════════════════

    function updateValuation(uint256 id, uint256 newMarketValueUsdc)
        external onlyRole(VALUATOR_ROLE)
    {
        Asset storage a = assets[id];
        uint256 old = a.marketValueUsdc;
        a.marketValueUsdc = newMarketValueUsdc;
        a.lastValuationAt = uint64(block.timestamp);
        emit AssetValuationUpdated(id, old, newMarketValueUsdc, a.oracleSource, uint64(block.timestamp));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Views
    // ══════════════════════════════════════════════════════════════════════

    function getOwnerAssets(uint256 ownerId) external view returns (uint256[] memory) {
        return ownerAssets[ownerId];
    }

    function totalAssets() external view returns (uint256) {
        return _nextId - 1;
    }
}
