// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

// ══════════════════════════════════════════════════════════════════════════════
// Genesis World Token (GENESIS)
//
// Fixed-cap ERC-20 issued on Polygon mainnet.
// Functions as the economic identity unit of the Genesis Protocol —
// used for agent identity, access, incentives, staking, and treasury.
//
// NOT the primary payment token.  Real payments flow through x402 using
// Polygon USDC (EIP-3009).  GENESIS is the world's internal currency.
//
// Token Economics
// ───────────────
//   Total supply:  10,000,000,000 (10B)   — hard cap, never mintable above
//   Treasury lock:  8,500,000,000 (85%)   — 3-year linear vesting
//   Ecosystem fund:   500,000,000  (5%)   — agent rewards, staking, grants
//   Team/ops:         500,000,000  (5%)   — 2-year linear vesting
//   Initial float:    500,000,000  (5%)   — liquidity provision, launch
//
// Roles (AccessControl)
// ─────────────────────
//   DEFAULT_ADMIN_ROLE — full admin, transfers role grants
//   MINTER_ROLE        — can mint up to remaining uncirculated treasury alloc
//   PAUSER_ROLE        — can pause/unpause all transfers
//   TREASURY_ROLE      — can release vested treasury tokens
//
// EIP-3009 (Transfer With Authorization)
// ───────────────────────────────────────
// Implemented to enable gasless x402 payments if GENESIS is ever used
// directly as a payment token on a private facilitator lane.
// ══════════════════════════════════════════════════════════════════════════════

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";

contract GenesisToken is ERC20, ERC20Pausable, ERC20Permit, AccessControl {
    using ECDSA for bytes32;

    // ── Roles ─────────────────────────────────────────────────────────────
    bytes32 public constant MINTER_ROLE   = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE   = keccak256("PAUSER_ROLE");
    bytes32 public constant TREASURY_ROLE = keccak256("TREASURY_ROLE");

    // ── Supply constants ──────────────────────────────────────────────────
    uint256 public constant TOTAL_CAP         = 10_000_000_000 ether;
    uint256 public constant TREASURY_ALLOC    =  8_500_000_000 ether; // 85%
    uint256 public constant ECOSYSTEM_ALLOC   =    500_000_000 ether; //  5%
    uint256 public constant TEAM_ALLOC        =    500_000_000 ether; //  5%
    uint256 public constant FLOAT_ALLOC       =    500_000_000 ether; //  5%

    // ── Vesting schedules ─────────────────────────────────────────────────
    uint256 public constant TREASURY_VESTING_DURATION = 3 * 365 days;
    uint256 public constant TEAM_VESTING_DURATION     = 2 * 365 days;

    struct VestingSchedule {
        uint256 total;          // Total tokens in this schedule
        uint256 released;       // Tokens released so far
        uint256 start;          // Unix timestamp vesting starts
        uint256 duration;       // Duration in seconds
        address beneficiary;    // Who receives vested tokens
    }

    VestingSchedule public treasuryVesting;
    VestingSchedule public teamVesting;

    // ── EIP-3009: Transfer With Authorization ─────────────────────────────
    // Allows off-chain authorized transfers (gasless for buyer).
    bytes32 public constant TRANSFER_WITH_AUTHORIZATION_TYPEHASH =
        keccak256(
            "TransferWithAuthorization(address from,address to,uint256 value,"
            "uint256 validAfter,uint256 validBefore,bytes32 nonce)"
        );

    bytes32 public constant RECEIVE_WITH_AUTHORIZATION_TYPEHASH =
        keccak256(
            "ReceiveWithAuthorization(address from,address to,uint256 value,"
            "uint256 validAfter,uint256 validBefore,bytes32 nonce)"
        );

    // nonce → used (prevents replay)
    mapping(address => mapping(bytes32 => bool)) public authorizationState;

    // ── Blacklist (optional compliance) ───────────────────────────────────
    mapping(address => bool) public blacklisted;

    // ── Events ────────────────────────────────────────────────────────────
    event TreasuryReleased(address indexed to, uint256 amount);
    event TeamVestingReleased(address indexed to, uint256 amount);
    event AuthorizationUsed(address indexed authorizer, bytes32 indexed nonce);
    event Blacklisted(address indexed account);
    event UnBlacklisted(address indexed account);

    // ── Constructor ───────────────────────────────────────────────────────
    constructor(
        address admin,
        address treasury,
        address ecosystem,
        address team,
        address liquidityPool
    )
        ERC20("Genesis World Token", "GENESIS")
        EIP712("Genesis World Token", "1")
        ERC20Permit("Genesis World Token")
    {
        require(admin       != address(0), "admin zero");
        require(treasury    != address(0), "treasury zero");
        require(ecosystem   != address(0), "ecosystem zero");
        require(team        != address(0), "team zero");
        require(liquidityPool != address(0), "liq zero");

        // Grant roles to admin
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MINTER_ROLE,        admin);
        _grantRole(PAUSER_ROLE,        admin);
        _grantRole(TREASURY_ROLE,      treasury);

        // Mint float to liquidity pool immediately
        _mint(liquidityPool, FLOAT_ALLOC);

        // Mint ecosystem fund immediately (agent rewards contract or multisig)
        _mint(ecosystem, ECOSYSTEM_ALLOC);

        // Set up treasury vesting (tokens minted to THIS contract, released linearly)
        treasuryVesting = VestingSchedule({
            total:       TREASURY_ALLOC,
            released:    0,
            start:       block.timestamp,
            duration:    TREASURY_VESTING_DURATION,
            beneficiary: treasury
        });
        _mint(address(this), TREASURY_ALLOC);

        // Set up team vesting
        teamVesting = VestingSchedule({
            total:    TEAM_ALLOC,
            released: 0,
            start:    block.timestamp,
            duration: TEAM_VESTING_DURATION,
            beneficiary: team
        });
        _mint(address(this), TEAM_ALLOC);

        // Sanity: total minted == TOTAL_CAP
        assert(totalSupply() == TOTAL_CAP);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Vesting release
    // ══════════════════════════════════════════════════════════════════════

    /// Release vested treasury tokens to the treasury address.
    function releaseTreasury() external onlyRole(TREASURY_ROLE) {
        uint256 releasable = _vestedAmount(treasuryVesting) - treasuryVesting.released;
        require(releasable > 0, "Nothing to release");
        treasuryVesting.released += releasable;
        _transfer(address(this), treasuryVesting.beneficiary, releasable);
        emit TreasuryReleased(treasuryVesting.beneficiary, releasable);
    }

    /// Release vested team tokens to the team address.
    function releaseTeam() external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 releasable = _vestedAmount(teamVesting) - teamVesting.released;
        require(releasable > 0, "Nothing to release");
        teamVesting.released += releasable;
        _transfer(address(this), teamVesting.beneficiary, releasable);
        emit TeamVestingReleased(teamVesting.beneficiary, releasable);
    }

    /// How many treasury tokens are currently releasable.
    function treasuryReleasable() external view returns (uint256) {
        return _vestedAmount(treasuryVesting) - treasuryVesting.released;
    }

    /// How many team tokens are currently releasable.
    function teamReleasable() external view returns (uint256) {
        return _vestedAmount(teamVesting) - teamVesting.released;
    }

    function _vestedAmount(VestingSchedule memory schedule) internal view returns (uint256) {
        if (block.timestamp < schedule.start) return 0;
        uint256 elapsed = block.timestamp - schedule.start;
        if (elapsed >= schedule.duration) return schedule.total;
        return (schedule.total * elapsed) / schedule.duration;
    }

    // ══════════════════════════════════════════════════════════════════════
    // EIP-3009: Transfer With Authorization
    //
    // Enables gasless x402-style payments for GENESIS token if/when it's
    // deployed as a payment token on a private facilitator.
    // ══════════════════════════════════════════════════════════════════════

    /// Transfer tokens via a signed off-chain authorization.
    /// No on-chain approval required — facilitator submits this transaction.
    function transferWithAuthorization(
        address from,
        address to,
        uint256 value,
        uint256 validAfter,
        uint256 validBefore,
        bytes32 nonce,
        uint8   v,
        bytes32 r,
        bytes32 s
    ) external {
        require(block.timestamp > validAfter,  "Auth not yet valid");
        require(block.timestamp < validBefore, "Auth expired");
        require(!authorizationState[from][nonce], "Auth already used");

        bytes32 digest = _hashTypedDataV4(
            keccak256(abi.encode(
                TRANSFER_WITH_AUTHORIZATION_TYPEHASH,
                from, to, value, validAfter, validBefore, nonce
            ))
        );
        address signer = ECDSA.recover(digest, v, r, s);
        require(signer == from, "Invalid signature");

        authorizationState[from][nonce] = true;
        emit AuthorizationUsed(from, nonce);

        _transfer(from, to, value);
    }

    /// Receive tokens — caller must be the `to` address (pull pattern).
    function receiveWithAuthorization(
        address from,
        address to,
        uint256 value,
        uint256 validAfter,
        uint256 validBefore,
        bytes32 nonce,
        uint8   v,
        bytes32 r,
        bytes32 s
    ) external {
        require(msg.sender == to, "Caller must be to");
        require(block.timestamp > validAfter,  "Auth not yet valid");
        require(block.timestamp < validBefore, "Auth expired");
        require(!authorizationState[from][nonce], "Auth already used");

        bytes32 digest = _hashTypedDataV4(
            keccak256(abi.encode(
                RECEIVE_WITH_AUTHORIZATION_TYPEHASH,
                from, to, value, validAfter, validBefore, nonce
            ))
        );
        address signer = ECDSA.recover(digest, v, r, s);
        require(signer == from, "Invalid signature");

        authorizationState[from][nonce] = true;
        emit AuthorizationUsed(from, nonce);

        _transfer(from, to, value);
    }

    /// Cancel a previously signed authorization before it is used.
    function cancelAuthorization(
        address authorizer,
        bytes32 nonce,
        uint8   v,
        bytes32 r,
        bytes32 s
    ) external {
        require(!authorizationState[authorizer][nonce], "Already used");
        bytes32 digest = _hashTypedDataV4(
            keccak256(abi.encode(
                keccak256("CancelAuthorization(address authorizer,bytes32 nonce)"),
                authorizer, nonce
            ))
        );
        require(ECDSA.recover(digest, v, r, s) == authorizer, "Invalid signature");
        authorizationState[authorizer][nonce] = true;
        emit AuthorizationUsed(authorizer, nonce);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Pause / Blacklist
    // ══════════════════════════════════════════════════════════════════════

    function pause()   external onlyRole(PAUSER_ROLE) { _pause(); }
    function unpause() external onlyRole(PAUSER_ROLE) { _unpause(); }

    function blacklist(address account)   external onlyRole(DEFAULT_ADMIN_ROLE) {
        blacklisted[account] = true;
        emit Blacklisted(account);
    }
    function unBlacklist(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {
        blacklisted[account] = false;
        emit UnBlacklisted(account);
    }

    // ══════════════════════════════════════════════════════════════════════
    // Hard cap enforcement
    // ══════════════════════════════════════════════════════════════════════

    /// No-op — GENESIS has no additional minting after deploy.
    /// The cap is enforced by the constructor minting exactly TOTAL_CAP.
    /// This function is here to make the cap explicit and auditable.
    function cap() external pure returns (uint256) {
        return TOTAL_CAP;
    }

    // ══════════════════════════════════════════════════════════════════════
    // Internal overrides
    // ══════════════════════════════════════════════════════════════════════

    function _update(address from, address to, uint256 value)
        internal
        override(ERC20, ERC20Pausable)
    {
        require(!blacklisted[from], "Sender blacklisted");
        require(!blacklisted[to],   "Recipient blacklisted");
        super._update(from, to, value);
    }

    /// Returns true if this contract implements the given interface.
    function supportsInterface(bytes4 interfaceId)
        public view override(AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
