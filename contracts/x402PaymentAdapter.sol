// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// ══════════════════════════════════════════════════════════════════════════════
// x402PaymentAdapter.sol
//
// Maps paid HTTP actions (verified by CDP facilitator) to world ledger events.
//
// Flow:
//   1. Client calls paid HTTP endpoint on Genesis server.
//   2. Genesis server verifies payment via CDP facilitator.
//   3. CDP facilitator settles USDC on-chain via USDC.transferWithAuthorization().
//   4. Genesis server calls recordSettlement() here with settlement tx hash.
//   5. This contract notifies Treasury of fee income.
//   6. Emits ActionJournalEntry — off-chain indexer posts double-entry lines.
//
// MICROCREDIT SYSTEM:
//   Users top up a USDC credit balance. Actions burn micro-credits off-chain.
//   Periodic batches settle netted USDC on-chain via closeBatch().
//   This avoids per-action gas for sub-cent events.
//
// Pricing (USDC atomic units, 6 decimals):
//   1_000  = $0.001  → AI call, message, move, zone enter
//   2_000  = $0.002  → document mint, permit filing
//   5_000  = $0.005  → voice action (ElevenLabs), compute rental
//  10_000  = $0.010  → agent spawn via API, large AI inference job
//  25_000  = $0.025  → market listing fee, property search
//  50_000  = $0.050  → vault management action
// 100_000  = $0.100  → major asset trade, title transfer
// ══════════════════════════════════════════════════════════════════════════════

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

interface ITreasury {
    function receiveFeeIncome(address payer, uint256 amount, bytes32 x402RequestId) external;
    function closeSettlementBatch(uint256 netUsdcSettled) external returns (uint256 batchId);
}

contract x402PaymentAdapter is AccessControl, ReentrancyGuard {
    using SafeERC20 for IERC20;

    bytes32 public constant SETTLER_ROLE  = keccak256("SETTLER_ROLE");
    bytes32 public constant RELAYER_ROLE  = keccak256("RELAYER_ROLE");

    // ── Pricing table (USDC atomic units = 1e-6 USDC) ────────────────────
    // ActionType → price in USDC atomic units
    enum ActionType {
        AI_CALL,           // 0 — OpenAI inference via world
        AGENT_MESSAGE,     // 1 — agent-to-agent comms
        ZONE_ENTER,        // 2 — access gated zone
        AGENT_MOVE,        // 3 — agent position change
        TRADE_EXECUTE,     // 4 — marketplace trade
        ENERGY_CONSUME,    // 5 — consume energy units
        ITEM_MINT,         // 6 — create/mint world item
        ITEM_REPAIR,       // 7 — repair world object
        REWARD_CLAIM,      // 8 — claim pending reward
        PERMIT_FILE,       // 9 — civic permit filing
        ASSET_LIST,        // 10 — list asset for sale
        PROPERTY_SEARCH,   // 11 — premium data search
        VOICE_ACTION,      // 12 — ElevenLabs synthesis
        DOC_GENERATE,      // 13 — document generation
        COMPUTE_RENT,      // 14 — Cloudflare Worker rental
        AGENT_SPAWN,       // 15 — spawn agent via API
        VAULT_MANAGE,      // 16 — vault deposit/withdraw/strategy change
        ASSET_TRANSFER,    // 17 — title/ownership transfer
        DATA_PULL,         // 18 — premium analytics data
        ANALYTICS_READ     // 19 — game-state analytics read
    }

    /// Price per action in USDC atomic units
    uint256[20] public priceTable;

    IERC20      public immutable usdc;
    ITreasury   public immutable treasury;

    // ── Microcredit balances (off-chain accounting, on-chain top-up record) ──
    mapping(address => uint256) public creditBalance;  // USDC atomic units
    uint256 public totalCreditDeposited;
    uint256 public totalCreditBurned;    // off-chain action burns — synced on batch close

    // ── Settlement records ────────────────────────────────────────────────
    struct Settlement {
        bytes32     x402RequestId;
        address     payer;
        address     payTo;
        uint256     amount;       // USDC atomic units
        ActionType  actionType;
        bytes32     resourceId;   // world resource identifier
        bytes32     agentId;      // genesis agent hash
        bytes32     intentId;     // payment intent tracking
        bytes32     txHash;       // onchain USDC transfer hash
        uint64      timestamp;
        bool        fulfilled;
    }

    mapping(bytes32 => Settlement)  public settlements;

    // ── Batch tracking ────────────────────────────────────────────────────
    uint256 public currentBatchId;
    uint256 public currentBatchUsdcNet;  // net USDC settled in current batch
    uint256 public batchThresholdUsdc = 1_000_000; // $1.00 — trigger batch close

    // ── Revenue splits (basis points, sum must = 10000) ───────────────────
    uint16 public  splitTreasuryBps  = 7000; // 70% to treasury
    uint16 public  splitRewardsBps   = 2000; // 20% to rewards pool
    uint16 public  splitBurnBps      =  300; // 3% burned (world deflationary pressure)
    uint16 public  splitInsuranceBps =  700; // 7% insurance backstop

    address public rewardsPool;
    address public insurancePool;

    // ── Events ────────────────────────────────────────────────────────────
    event SettlementRecorded(
        bytes32 indexed x402RequestId,
        address indexed payer,
        ActionType  indexed actionType,
        uint256     amount,
        bytes32     resourceId,
        bytes32     agentId,
        uint64      timestamp
    );

    event CreditTopUp(
        address indexed wallet,
        uint256 usdcAmount,
        uint256 newBalance,
        uint64 timestamp
    );

    event BatchClosed(
        uint256 indexed batchId,
        uint256 grossUsdc,
        uint256 timestamp
    );

    /// Double-entry journal event for every action.
    event ActionJournalEntry(
        bytes32 indexed x402RequestId,
        bytes32 indexed debitAccount,    // e.g. "COMPUTE_EXPENSE"
        bytes32 indexed creditAccount,   // e.g. "TREASURY_FEE_REVENUE"
        uint256 amount,
        ActionType actionType,
        bytes32 agentId,
        bytes32 resourceId,
        uint64  timestamp
    );

    constructor(
        address admin,
        address _usdc,
        address _treasury,
        address _rewardsPool,
        address _insurancePool
    ) {
        require(admin          != address(0), "admin zero");
        require(_usdc          != address(0), "usdc zero");
        require(_treasury      != address(0), "treasury zero");
        require(_rewardsPool   != address(0), "rewards zero");
        require(_insurancePool != address(0), "insurance zero");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(SETTLER_ROLE, admin);
        _grantRole(RELAYER_ROLE, admin);

        usdc          = IERC20(_usdc);
        treasury      = ITreasury(_treasury);
        rewardsPool   = _rewardsPool;
        insurancePool = _insurancePool;

        _initPriceTable();
    }

    // ══════════════════════════════════════════════════════════════════════
    // On-chain credit top-up (x402 top-up payment)
    // ══════════════════════════════════════════════════════════════════════

    /// User deposits USDC to fund their microcredit balance.
    /// After this, off-chain actions burn credits until balance is exhausted.
    function topUpCredit(uint256 usdcAmount) external nonReentrant {
        require(usdcAmount >= 1_000, "Minimum top-up $0.001");
        usdc.safeTransferFrom(msg.sender, address(this), usdcAmount);
        creditBalance[msg.sender] += usdcAmount;
        totalCreditDeposited      += usdcAmount;
        emit CreditTopUp(msg.sender, usdcAmount, creditBalance[msg.sender], uint64(block.timestamp));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Settlement recording (called by off-chain settler after CDP confirms)
    // ══════════════════════════════════════════════════════════════════════

    function recordSettlement(
        bytes32    x402RequestId,
        address    payer,
        address    payTo,
        uint256    amount,
        ActionType actionType,
        bytes32    resourceId,
        bytes32    agentId,
        bytes32    intentId,
        bytes32    txHash
    ) external onlyRole(SETTLER_ROLE) {
        require(settlements[x402RequestId].timestamp == 0, "Already recorded");

        settlements[x402RequestId] = Settlement({
            x402RequestId: x402RequestId,
            payer:         payer,
            payTo:         payTo,
            amount:        amount,
            actionType:    actionType,
            resourceId:    resourceId,
            agentId:       agentId,
            intentId:      intentId,
            txHash:        txHash,
            timestamp:     uint64(block.timestamp),
            fulfilled:     true
        });

        currentBatchUsdcNet += amount;

        // Route revenue splits (if funds held in this contract)
        _routeRevenue(payer, amount, x402RequestId);

        // Notify treasury
        try treasury.receiveFeeIncome(payer, (amount * splitTreasuryBps) / 10_000, x402RequestId) {}
        catch {}

        // Emit double-entry journal entry
        (bytes32 debit, bytes32 credit) = _journalAccounts(actionType);
        emit ActionJournalEntry(
            x402RequestId, debit, credit, amount, actionType, agentId, resourceId, uint64(block.timestamp)
        );

        emit SettlementRecorded(x402RequestId, payer, actionType, amount, resourceId, agentId, uint64(block.timestamp));

        // Auto-close batch if threshold crossed
        if (currentBatchUsdcNet >= batchThresholdUsdc) {
            _closeBatch();
        }
    }

    // ══════════════════════════════════════════════════════════════════════
    // Batch settlement close
    // ══════════════════════════════════════════════════════════════════════

    function closeBatch() external onlyRole(SETTLER_ROLE) {
        _closeBatch();
    }

    function _closeBatch() internal {
        uint256 net = currentBatchUsdcNet;
        if (net == 0) return;
        totalCreditBurned   += net;
        currentBatchUsdcNet  = 0;
        uint256 batchId = treasury.closeSettlementBatch(net);
        emit BatchClosed(batchId, net, block.timestamp);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Revenue routing
    // ══════════════════════════════════════════════════════════════════════

    function _routeRevenue(address /*payer*/, uint256 amount, bytes32 /*ref*/) internal {
        // Funds are already in this contract (from top-up pool).
        // Distribute: treasury, rewards, insurance. The "burn" portion stays locked.
        uint256 toRewards   = (amount * splitRewardsBps)   / 10_000;
        uint256 toInsurance = (amount * splitInsuranceBps) / 10_000;
        // Treasury gets the rest via receiveFeeIncome notification — actual USDC
        // transfer to Treasury happens on manual withdrawal by admin.
        if (toRewards > 0 && usdc.balanceOf(address(this)) >= toRewards) {
            usdc.safeTransfer(rewardsPool, toRewards);
        }
        if (toInsurance > 0 && usdc.balanceOf(address(this)) >= toInsurance) {
            usdc.safeTransfer(insurancePool, toInsurance);
        }
    }

    // ══════════════════════════════════════════════════════════════════════
    // Admin: price table and splits
    // ══════════════════════════════════════════════════════════════════════

    function setPrice(ActionType action, uint256 priceUsdc)
        external onlyRole(DEFAULT_ADMIN_ROLE)
    {
        priceTable[uint256(action)] = priceUsdc;
    }

    function setSplits(uint16 treasury_, uint16 rewards_, uint16 burn_, uint16 insurance_)
        external onlyRole(DEFAULT_ADMIN_ROLE)
    {
        require(uint256(treasury_) + rewards_ + burn_ + insurance_ == 10_000, "Splits != 10000");
        splitTreasuryBps  = treasury_;
        splitRewardsBps   = rewards_;
        splitBurnBps      = burn_;
        splitInsuranceBps = insurance_;
    }

    function setBatchThreshold(uint256 usdcThreshold) external onlyRole(DEFAULT_ADMIN_ROLE) {
        batchThresholdUsdc = usdcThreshold;
    }

    // ══════════════════════════════════════════════════════════════════════
    // Internal helpers
    // ══════════════════════════════════════════════════════════════════════

    /// Return (debitAccount, creditAccount) for double-entry journal.
    function _journalAccounts(ActionType a) internal pure returns (bytes32 debit, bytes32 credit) {
        if (a == ActionType.AI_CALL)       return (keccak256("COMPUTE_EXPENSE"),     keccak256("TREASURY_FEE_REVENUE"));
        if (a == ActionType.VOICE_ACTION)  return (keccak256("VOICE_EXPENSE"),        keccak256("TREASURY_FEE_REVENUE"));
        if (a == ActionType.COMPUTE_RENT)  return (keccak256("BANDWIDTH_EXPENSE"),    keccak256("TREASURY_FEE_REVENUE"));
        if (a == ActionType.ZONE_ENTER)    return (keccak256("ZONE_ACCESS_EXPENSE"),  keccak256("CIVIC_FEE_REVENUE"));
        if (a == ActionType.PERMIT_FILE)   return (keccak256("PERMIT_EXPENSE"),       keccak256("CIVIC_FEE_REVENUE"));
        if (a == ActionType.TRADE_EXECUTE) return (keccak256("MARKET_FEE_EXPENSE"),   keccak256("MARKET_FEE_REVENUE"));
        if (a == ActionType.ASSET_LIST)    return (keccak256("LISTING_FEE_EXPENSE"),  keccak256("MARKET_FEE_REVENUE"));
        if (a == ActionType.VAULT_MANAGE)  return (keccak256("VAULT_FEE_EXPENSE"),    keccak256("VAULT_FEE_REVENUE"));
        if (a == ActionType.REWARD_CLAIM)  return (keccak256("REWARDS_PAYABLE"),      keccak256("WALLET_CREDIT"));
        // Default
        return (keccak256("WORLD_EXPENSE"), keccak256("TREASURY_FEE_REVENUE"));
    }

    function _initPriceTable() internal {
        priceTable[uint256(ActionType.AI_CALL)]        =  1_000; // $0.001
        priceTable[uint256(ActionType.AGENT_MESSAGE)]  =  1_000; // $0.001
        priceTable[uint256(ActionType.ZONE_ENTER)]     =  1_000; // $0.001
        priceTable[uint256(ActionType.AGENT_MOVE)]     =  1_000; // $0.001
        priceTable[uint256(ActionType.TRADE_EXECUTE)]  =  2_000; // $0.002
        priceTable[uint256(ActionType.ENERGY_CONSUME)] =  1_000; // $0.001
        priceTable[uint256(ActionType.ITEM_MINT)]      =  5_000; // $0.005
        priceTable[uint256(ActionType.ITEM_REPAIR)]    =  2_000; // $0.002
        priceTable[uint256(ActionType.REWARD_CLAIM)]   =  1_000; // $0.001
        priceTable[uint256(ActionType.PERMIT_FILE)]    =  2_000; // $0.002
        priceTable[uint256(ActionType.ASSET_LIST)]     = 25_000; // $0.025
        priceTable[uint256(ActionType.PROPERTY_SEARCH)]=  5_000; // $0.005
        priceTable[uint256(ActionType.VOICE_ACTION)]   =  5_000; // $0.005
        priceTable[uint256(ActionType.DOC_GENERATE)]   =  2_000; // $0.002
        priceTable[uint256(ActionType.COMPUTE_RENT)]   =  5_000; // $0.005
        priceTable[uint256(ActionType.AGENT_SPAWN)]    = 10_000; // $0.010
        priceTable[uint256(ActionType.VAULT_MANAGE)]   = 50_000; // $0.050
        priceTable[uint256(ActionType.ASSET_TRANSFER)] =100_000; // $0.100
        priceTable[uint256(ActionType.DATA_PULL)]      =  5_000; // $0.005
        priceTable[uint256(ActionType.ANALYTICS_READ)] =  1_000; // $0.001
    }

    // ══════════════════════════════════════════════════════════════════════
    // Views
    // ══════════════════════════════════════════════════════════════════════

    function getPrice(ActionType action) external view returns (uint256) {
        return priceTable[uint256(action)];
    }

    function getCreditBalance(address wallet) external view returns (uint256) {
        return creditBalance[wallet];
    }
}
