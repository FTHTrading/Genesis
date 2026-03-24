// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// ══════════════════════════════════════════════════════════════════════════════
// Treasury.sol
//
// Central reserve and emission controller for the Genesis World.
// Holds the majority of the 10B GENESIS supply under vesting/emission schedules.
// Routes all fee income from x402 and world actions.
// Executes emissions for rewards, infrastructure, and ecosystem grants.
//
// All movements emit a TreasuryJournalEntry event so the off-chain double-entry
// ledger can reconcile without additional state storage cost.
// ══════════════════════════════════════════════════════════════════════════════

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

interface IWorldToken is IERC20 {
    function cap() external pure returns (uint256);
}

contract Treasury is AccessControl, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // ── Roles ─────────────────────────────────────────────────────────────
    bytes32 public constant EMISSION_ROLE  = keccak256("EMISSION_ROLE");
    bytes32 public constant FEE_ROUTER     = keccak256("FEE_ROUTER");
    bytes32 public constant AUDITOR_ROLE   = keccak256("AUDITOR_ROLE");

    // ── Token references ──────────────────────────────────────────────────
    IWorldToken public immutable worldToken;
    IERC20      public immutable usdc;

    // ── Emission buckets (basis points of remaining reserve) ──────────────
    // These caps enforce the emission percentages described in the world spec.
    uint256 public constant MAX_ECOSYSTEM_BPS    = 1500; // 15% of cap
    uint256 public constant MAX_PATRON_REWARDS_BPS =  700; // 7%
    uint256 public constant MAX_TEAM_BPS          =  700; // 7%
    uint256 public constant MAX_LIQUIDITY_BPS     =  300; // 3%
    uint256 public constant MAX_INSURANCE_BPS     =  200; // 2%

    uint256 public totalCap; // cached from worldToken.cap()

    // ── Revenue accounting ────────────────────────────────────────────────
    // Total USDC received from x402 settlements.
    uint256 public totalUsdcFeeIncome;
    // Total WORLD tokens emitted from this treasury.
    uint256 public totalWorldEmitted;

    // ── Settlement batch sequencing ───────────────────────────────────────
    uint256 public settlementBatchSeq;

    // ── Events (feeds the double-entry journal) ───────────────────────────
    event TreasuryJournalEntry(
        uint256 indexed seq,
        bytes32 indexed debitAccount,
        bytes32 indexed creditAccount,
        address  token,
        uint256  amount,
        bytes32  referenceType,  // "EMISSION" | "FEE_INCOME" | "REWARD" | "RESERVE"
        bytes32  referenceId,
        uint256  timestamp
    );

    event EmissionScheduleExecuted(
        bytes32 indexed bucket,
        address indexed recipient,
        uint256 amount,
        uint256 batchSeq
    );

    event UsdcFeeReceived(
        address indexed payer,
        uint256 amount,
        bytes32 indexed x402RequestId
    );

    event SettlementBatchClosed(uint256 indexed batchSeq, uint256 netUsdc, uint256 timestamp);

    // ── Journal entry sequence ────────────────────────────────────────────
    uint256 private _journalSeq;

    constructor(address admin, address _worldToken, address _usdc) {
        require(admin       != address(0), "admin zero");
        require(_worldToken != address(0), "token zero");
        require(_usdc       != address(0), "usdc zero");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(AUDITOR_ROLE,       admin);

        worldToken = IWorldToken(_worldToken);
        usdc       = IERC20(_usdc);
        totalCap   = IWorldToken(_worldToken).cap();
    }

    // ══════════════════════════════════════════════════════════════════════
    // Fee income router — called by x402PaymentAdapter after settlement
    // ══════════════════════════════════════════════════════════════════════

    /// Record incoming x402 USDC fee.  Caller must be FEE_ROUTER.
    function receiveFeeIncome(
        address payer,
        uint256 amount,
        bytes32 x402RequestId
    ) external onlyRole(FEE_ROUTER) {
        totalUsdcFeeIncome += amount;
        emit UsdcFeeReceived(payer, amount, x402RequestId);
        _journal(
            keccak256("USDC_RESERVE"),
            keccak256("TREASURY_FEE_REVENUE"),
            address(usdc),
            amount,
            "FEE_INCOME",
            x402RequestId
        );
    }

    // ══════════════════════════════════════════════════════════════════════
    // World token emissions
    // ══════════════════════════════════════════════════════════════════════

    /// Emit WORLD tokens from treasury to ecosystem fund (rewards, grants, infra).
    function emitEcosystem(address recipient, uint256 amount, bytes32 referenceId)
        external onlyRole(EMISSION_ROLE) nonReentrant
    {
        _emitChecked(recipient, amount, keccak256("ECOSYSTEM"), MAX_ECOSYSTEM_BPS, referenceId);
    }

    /// Emit WORLD tokens as patron vault rewards.
    function emitPatronRewards(address recipient, uint256 amount, bytes32 referenceId)
        external onlyRole(EMISSION_ROLE) nonReentrant
    {
        _emitChecked(recipient, amount, keccak256("PATRON_REWARDS"), MAX_PATRON_REWARDS_BPS, referenceId);
    }

    /// Emit WORLD tokens for liquidity operations.
    function emitLiquidity(address recipient, uint256 amount, bytes32 referenceId)
        external onlyRole(EMISSION_ROLE) nonReentrant
    {
        _emitChecked(recipient, amount, keccak256("LIQUIDITY"), MAX_LIQUIDITY_BPS, referenceId);
    }

    // Internal emission with cap check
    function _emitChecked(
        address recipient,
        uint256 amount,
        bytes32 bucket,
        uint256 maxBps,
        bytes32 referenceId
    ) internal {
        uint256 maxAllowed = (totalCap * maxBps) / 10_000;
        require(totalWorldEmitted + amount <= maxAllowed, "Emission cap exceeded");
        totalWorldEmitted += amount;
        worldToken.transfer(recipient, amount);
        emit EmissionScheduleExecuted(bucket, recipient, amount, settlementBatchSeq);
        _journal(
            keccak256("INCENTIVE_EXPENSE"),
            keccak256("WORLD_TREASURY_RESERVE"),
            address(worldToken),
            amount,
            "EMISSION",
            referenceId
        );
    }

    // ══════════════════════════════════════════════════════════════════════
    // Settlement batch control
    // ══════════════════════════════════════════════════════════════════════

    /// Close a microcredit settlement batch.  Called by the off-chain settler.
    function closeSettlementBatch(uint256 netUsdcSettled)
        external onlyRole(FEE_ROUTER) returns (uint256 batchId)
    {
        batchId = ++settlementBatchSeq;
        emit SettlementBatchClosed(batchId, netUsdcSettled, block.timestamp);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Journal
    // ══════════════════════════════════════════════════════════════════════

    function _journal(
        bytes32 debit,
        bytes32 credit,
        address token,
        uint256 amount,
        bytes32 refType,
        bytes32 refId
    ) internal {
        emit TreasuryJournalEntry(++_journalSeq, debit, credit, token, amount, refType, refId, block.timestamp);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Views
    // ══════════════════════════════════════════════════════════════════════

    function usdcBalance() external view returns (uint256) {
        return usdc.balanceOf(address(this));
    }

    function worldBalance() external view returns (uint256) {
        return worldToken.balanceOf(address(this));
    }
}
