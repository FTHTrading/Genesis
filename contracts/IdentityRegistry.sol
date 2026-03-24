// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// ══════════════════════════════════════════════════════════════════════════════
// IdentityRegistry.sol
//
// Persistent identity layer for every entity in the Genesis World:
// citizens, AI agents, organizations, machines, and autonomous processes.
//
// Identity tokens are SOULBOUND (non-transferable).
// Each entity gets exactly one identity token.
// Economic activity is tracked in off-chain ledgers keyed to this registry.
// ══════════════════════════════════════════════════════════════════════════════

import "@openzeppelin/contracts/access/AccessControl.sol";

contract IdentityRegistry is AccessControl {
    bytes32 public constant REGISTRAR_ROLE = keccak256("REGISTRAR_ROLE");
    bytes32 public constant ORACLE_ROLE    = keccak256("ORACLE_ROLE");

    /// Entity classifications
    enum EntityType {
        CITIZEN,        // human participant
        AI_AGENT,       // Genesis Protocol autonomous agent
        ORGANIZATION,   // business, guild, or collective
        MACHINE,        // IoT, compute node, physical asset proxy
        VAULT,          // financial position entity (PatronVault proxy)
        CONTRACT        // autonomous smart contract entity
    }

    /// Status of an entity in the world
    enum EntityStatus {
        ACTIVE,
        SUSPENDED,
        DECEASED,       // agent death or permanent shutdown
        MIGRATED        // moved to another world shard
    }

    struct Identity {
        uint256     id;
        EntityType  entityType;
        EntityStatus status;
        address     walletAddress;     // controlling EVM wallet (may be zero for AI)
        bytes32     genesisHash;       // deterministic identity hash from genesis
        string      displayName;
        uint64      createdAt;         // unix timestamp
        uint64      lastActiveAt;
        uint64      generation;        // for AI agents: generation number
        bytes32     parentId;          // for agents born from another agent
        uint256     reputationScore;   // 0-10000 basis points
    }

    // id → Identity
    mapping(uint256 => Identity)           public identities;
    // wallet address → entity id
    mapping(address => uint256)            public walletToId;
    // genesisHash → entity id (for AI agents)
    mapping(bytes32 => uint256)            public hashToId;

    uint256 private _nextId = 1;

    event EntityRegistered(
        uint256 indexed id,
        EntityType entityType,
        address indexed wallet,
        bytes32 genesisHash,
        uint64 timestamp
    );

    event EntityStatusChanged(
        uint256 indexed id,
        EntityStatus oldStatus,
        EntityStatus newStatus,
        uint64 timestamp
    );

    event ReputationUpdated(
        uint256 indexed id,
        uint256 oldScore,
        uint256 newScore,
        bytes32 reason
    );

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(REGISTRAR_ROLE, admin);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Registration
    // ══════════════════════════════════════════════════════════════════════

    function register(
        EntityType  entityType,
        address     wallet,
        bytes32     genesisHash,
        string calldata displayName,
        uint64      generation,
        bytes32     parentId
    ) external onlyRole(REGISTRAR_ROLE) returns (uint256 id) {
        if (wallet != address(0)) {
            require(walletToId[wallet] == 0, "Wallet already registered");
        }
        if (genesisHash != bytes32(0)) {
            require(hashToId[genesisHash] == 0, "Genesis hash already registered");
        }

        id = _nextId++;
        identities[id] = Identity({
            id:              id,
            entityType:      entityType,
            status:          EntityStatus.ACTIVE,
            walletAddress:   wallet,
            genesisHash:     genesisHash,
            displayName:     displayName,
            createdAt:       uint64(block.timestamp),
            lastActiveAt:    uint64(block.timestamp),
            generation:      generation,
            parentId:        parentId,
            reputationScore: 5000 // start at 50%
        });

        if (wallet != address(0))   walletToId[wallet]      = id;
        if (genesisHash != bytes32(0)) hashToId[genesisHash] = id;

        emit EntityRegistered(id, entityType, wallet, genesisHash, uint64(block.timestamp));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Status and reputation
    // ══════════════════════════════════════════════════════════════════════

    function setStatus(uint256 id, EntityStatus newStatus)
        external onlyRole(REGISTRAR_ROLE)
    {
        EntityStatus old = identities[id].status;
        identities[id].status = newStatus;
        emit EntityStatusChanged(id, old, newStatus, uint64(block.timestamp));
    }

    function updateReputation(uint256 id, uint256 newScore, bytes32 reason)
        external onlyRole(ORACLE_ROLE)
    {
        require(newScore <= 10_000, "Score > 10000");
        uint256 old = identities[id].reputationScore;
        identities[id].reputationScore = newScore;
        emit ReputationUpdated(id, old, newScore, reason);
    }

    function touchActivity(uint256 id) external onlyRole(REGISTRAR_ROLE) {
        identities[id].lastActiveAt = uint64(block.timestamp);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Views
    // ══════════════════════════════════════════════════════════════════════

    function isActive(uint256 id) external view returns (bool) {
        return identities[id].status == EntityStatus.ACTIVE;
    }

    function totalEntities() external view returns (uint256) {
        return _nextId - 1;
    }
}
