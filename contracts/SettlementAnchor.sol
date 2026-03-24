// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/// @title SettlementAnchor
/// @notice In-house settlement batches are computed off-chain and anchored here.
///         Polygon is only involved at batch-close time. No per-action gas.
///         Every anchor is a tamper-evident proof that a batch containing N
///         micropayment movements was net-settled at a specific block.
///
/// Flow:
///   1. In-house ledger accumulates actions debit/credit until threshold.
///   2. Settlement service computes:  batchHash = keccak256(abi.encode(lines))
///   3. Settlement service calls anchorBatch().
///   4. Event is emitted — off-chain indexers update treasury positions.
///
/// Verification:
///   Anyone can call verifyBatch(batchId, batchHash) to confirm the hash
///   matches what was anchored. The actual line-level data lives in the
///   in-house ledger (PostgreSQL + JSONL lineage).
contract SettlementAnchor is AccessControl, Pausable {
    bytes32 public constant SETTLER_ROLE = keccak256("SETTLER_ROLE");
    bytes32 public constant PAUSER_ROLE  = keccak256("PAUSER_ROLE");

    // ── State ─────────────────────────────────────────────────────────────

    struct BatchRecord {
        bytes32 batchHash;          // keccak256 of serialized batch lines
        address settler;            // who submitted
        uint256 anchoredAt;         // block.timestamp
        uint256 blockNumber;        // block.number
        uint64  lineCount;          // number of settlement lines in batch
        uint256 totalNetUsdc;       // net USDC moved (atomic units, 6 dec)
        bool    reversed;           // true if this batch was reversed
    }

    mapping(bytes32 => BatchRecord) public batches;
    bytes32[] public batchIds;

    uint256 public totalBatchesAnchored;
    uint256 public totalUsdcAnchored;

    // ── Events ────────────────────────────────────────────────────────────

    event BatchAnchored(
        bytes32 indexed batchId,
        bytes32          batchHash,
        address indexed  settler,
        uint64           lineCount,
        uint256          totalNetUsdc,
        uint256          anchoredAt
    );

    event BatchReversed(
        bytes32 indexed batchId,
        address indexed  reversedBy,
        string           reason
    );

    // ── Constructor ───────────────────────────────────────────────────────

    constructor(address admin, address settler) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(SETTLER_ROLE, settler);
        _grantRole(PAUSER_ROLE, admin);
    }

    // ── Anchor ────────────────────────────────────────────────────────────

    /// @notice Anchor a settlement batch. Called by the in-house settlement service.
    /// @param batchId      Off-chain batch UUID as bytes32 (keccak256 of UUID string).
    /// @param batchHash    keccak256(abi.encode(all settlement lines in canonical order)).
    /// @param lineCount    Number of line items in this batch.
    /// @param totalNetUsdc Net USDC amount settled in atomic units (6 decimals).
    function anchorBatch(
        bytes32 batchId,
        bytes32 batchHash,
        uint64  lineCount,
        uint256 totalNetUsdc
    )
        external
        onlyRole(SETTLER_ROLE)
        whenNotPaused
    {
        require(batchId != bytes32(0),    "SettlementAnchor: zero batchId");
        require(batchHash != bytes32(0),  "SettlementAnchor: zero batchHash");
        require(batches[batchId].anchoredAt == 0, "SettlementAnchor: already anchored");

        batches[batchId] = BatchRecord({
            batchHash:    batchHash,
            settler:      msg.sender,
            anchoredAt:   block.timestamp,
            blockNumber:  block.number,
            lineCount:    lineCount,
            totalNetUsdc: totalNetUsdc,
            reversed:     false
        });
        batchIds.push(batchId);

        totalBatchesAnchored++;
        totalUsdcAnchored += totalNetUsdc;

        emit BatchAnchored(batchId, batchHash, msg.sender, lineCount, totalNetUsdc, block.timestamp);
    }

    /// @notice Flag a batch as reversed (for dispute resolution).
    ///         Does NOT refund — off-chain ledger handles the credit.
    function reverseBatch(
        bytes32 batchId,
        string calldata reason
    )
        external
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(batches[batchId].anchoredAt > 0, "SettlementAnchor: not found");
        require(!batches[batchId].reversed,      "SettlementAnchor: already reversed");
        batches[batchId].reversed = true;
        emit BatchReversed(batchId, msg.sender, reason);
    }

    // ── Verify ────────────────────────────────────────────────────────────

    /// @notice Anyone can verify that a batchHash matches what was anchored.
    function verifyBatch(bytes32 batchId, bytes32 batchHash) external view returns (bool) {
        return batches[batchId].batchHash == batchHash && !batches[batchId].reversed;
    }

    /// @notice Returns the most recent N batch IDs (for off-chain indexers).
    function recentBatchIds(uint256 n) external view returns (bytes32[] memory) {
        uint256 len = batchIds.length;
        uint256 count = n < len ? n : len;
        bytes32[] memory result = new bytes32[](count);
        for (uint256 i = 0; i < count; i++) {
            result[i] = batchIds[len - count + i];
        }
        return result;
    }

    function pause()   external onlyRole(PAUSER_ROLE) { _pause(); }
    function unpause() external onlyRole(PAUSER_ROLE) { _unpause(); }
}
