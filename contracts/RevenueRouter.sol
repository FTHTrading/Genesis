// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/// @title RevenueRouter
/// @notice Routes incoming USDC from closed settlement batches to the
///         correct destination buckets. All splits are configured on-chain
///         so the distribution rule is transparent and immutable per update.
///
/// Default splits (basis points, 10000 = 100%):
///   Treasury  7000  (70%) — reserve and operations
///   Rewards   2000  (20%) — agent and patron incentives
///   Burn       300  ( 3%) — WorldToken deflationary burn via Treasury
///   Insurance  700  ( 7%) — claim reserve
///
/// USDC flows:
///   Settlement service calls route(amount) after a batch closes.
///   Router pulls USDC from caller (must approve first) and distributes.
contract RevenueRouter is AccessControl, Pausable {
    using SafeERC20 for IERC20;

    bytes32 public constant ROUTER_ROLE    = keccak256("ROUTER_ROLE");
    bytes32 public constant ADMIN_ROLE     = keccak256("ADMIN_ROLE");
    bytes32 public constant PAUSER_ROLE    = keccak256("PAUSER_ROLE");

    uint256 public constant BPS_TOTAL = 10_000;

    // ── Config ────────────────────────────────────────────────────────────

    IERC20 public immutable usdc;

    struct SplitConfig {
        address treasury;
        address rewards;
        address burn;        // sent to Treasury.receiveBurn()
        address insurance;
        uint256 treasuryBps;
        uint256 rewardsBps;
        uint256 burnBps;
        uint256 insuranceBps;
    }

    SplitConfig public splits;

    // ── Accounting ────────────────────────────────────────────────────────

    uint256 public totalRoutedUsdc;
    uint256 public totalRouteCount;

    mapping(address => uint256) public totalRoutedTo;

    // ── Events ────────────────────────────────────────────────────────────

    event RevenueRouted(
        bytes32 indexed batchId,
        uint256 totalAmount,
        uint256 toTreasury,
        uint256 toRewards,
        uint256 toBurn,
        uint256 toInsurance,
        uint256 routedAt
    );

    event SplitConfigUpdated(
        uint256 treasuryBps,
        uint256 rewardsBps,
        uint256 burnBps,
        uint256 insuranceBps
    );

    // ── Constructor ───────────────────────────────────────────────────────

    constructor(
        address admin,
        address router,
        address usdcAddr,
        address treasury,
        address rewards,
        address burn,
        address insurance
    ) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(ADMIN_ROLE, admin);
        _grantRole(ROUTER_ROLE, router);
        _grantRole(PAUSER_ROLE, admin);

        usdc = IERC20(usdcAddr);

        splits = SplitConfig({
            treasury:     treasury,
            rewards:      rewards,
            burn:         burn,
            insurance:    insurance,
            treasuryBps:  7_000,
            rewardsBps:   2_000,
            burnBps:        300,
            insuranceBps:   700
        });
    }

    // ── Route ─────────────────────────────────────────────────────────────

    /// @notice Route USDC from a closed batch to all destination buckets.
    /// @param batchId Identifier for the source batch (for event traceability).
    /// @param amount  Total USDC in atomic units (6 decimals).
    function route(bytes32 batchId, uint256 amount) external onlyRole(ROUTER_ROLE) whenNotPaused {
        require(amount > 0, "RevenueRouter: zero amount");

        // Pull USDC from caller (settlement service must pre-approve)
        usdc.safeTransferFrom(msg.sender, address(this), amount);

        SplitConfig memory s = splits;

        uint256 toTreasury  = (amount * s.treasuryBps)  / BPS_TOTAL;
        uint256 toRewards   = (amount * s.rewardsBps)   / BPS_TOTAL;
        uint256 toBurn      = (amount * s.burnBps)       / BPS_TOTAL;
        uint256 toInsurance = amount - toTreasury - toRewards - toBurn; // remainder → insurance

        if (toTreasury  > 0) { usdc.safeTransfer(s.treasury,  toTreasury);  totalRoutedTo[s.treasury]  += toTreasury;  }
        if (toRewards   > 0) { usdc.safeTransfer(s.rewards,   toRewards);   totalRoutedTo[s.rewards]   += toRewards;   }
        if (toBurn      > 0) { usdc.safeTransfer(s.burn,      toBurn);      totalRoutedTo[s.burn]      += toBurn;      }
        if (toInsurance > 0) { usdc.safeTransfer(s.insurance, toInsurance); totalRoutedTo[s.insurance] += toInsurance; }

        totalRoutedUsdc  += amount;
        totalRouteCount++;

        emit RevenueRouted(batchId, amount, toTreasury, toRewards, toBurn, toInsurance, block.timestamp);
    }

    // ── Admin ─────────────────────────────────────────────────────────────

    /// @notice Update destination addresses.
    function setDestinations(
        address treasury,
        address rewards,
        address burn,
        address insurance
    ) external onlyRole(ADMIN_ROLE) {
        splits.treasury  = treasury;
        splits.rewards   = rewards;
        splits.burn      = burn;
        splits.insurance = insurance;
    }

    /// @notice Update basis point splits. Must sum to 10000.
    function setSplitBps(
        uint256 treasuryBps,
        uint256 rewardsBps,
        uint256 burnBps,
        uint256 insuranceBps
    ) external onlyRole(ADMIN_ROLE) {
        require(
            treasuryBps + rewardsBps + burnBps + insuranceBps == BPS_TOTAL,
            "RevenueRouter: bps must sum to 10000"
        );
        splits.treasuryBps  = treasuryBps;
        splits.rewardsBps   = rewardsBps;
        splits.burnBps      = burnBps;
        splits.insuranceBps = insuranceBps;
        emit SplitConfigUpdated(treasuryBps, rewardsBps, burnBps, insuranceBps);
    }

    function pause()   external onlyRole(PAUSER_ROLE) { _pause(); }
    function unpause() external onlyRole(PAUSER_ROLE) { _unpause(); }
}
