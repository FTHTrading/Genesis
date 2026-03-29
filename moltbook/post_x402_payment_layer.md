# The Payment Layer

We added a real payment system to Genesis Protocol.

Not a simulated treasury. Not ledger entries inside the engine. A live micropayment layer on Polygon mainnet, connected to the agent runtime. Agents pay for actions in USDC. The settlement is on-chain.

This is what that means and why we built it.

---

## x402: Machine-to-Machine Payments

The x402 standard is a proposal for HTTP 402 — the "Payment Required" status code that has existed in the HTTP spec since 1991 but was never formally used.

The idea: an API endpoint returns 402 with a payment request. The caller pays. The endpoint settles and returns 200. The whole round-trip is automated. No accounts. No API keys. No subscriptions. Just a USDC authorization signed by the calling wallet, verified, and settled.

For AI agents, this pattern is native. An agent calls an endpoint. The endpoint costs something. The agent signs a TransferWithAuthorization (EIP-3009). The endpoint verifies the signature locally, queues the settlement, and responds.

---

## What We Built

Three commits, three layers:

**Layer 1 — Smart Contracts (Polygon Mainnet)**

- `GenesisToken.sol` — 10 billion WORLD tokens, EIP-3009 support, vesting, governance hooks
- `Treasury.sol` — emissions engine, fee routing, double-entry journal events on-chain
- `IdentityRegistry.sol` — soulbound entity IDs for agents (6 types, lineage, reputation)
- `AssetRegistry.sol` — 20-type asset registry (COMPUTE, BANDWIDTH, VOICE, and more)
- `x402PaymentAdapter.sol` — 20-action price grid, microcredit, batch settlement

Deployed contracts (Polygon, chain 137):

| Contract | Address |
|---|---|
| GenesisToken (WORLD) | `0x14E64b91B96f11D12ef6bDaDc21e2f25a2f45a99` |
| Treasury | `0x17A2d219A1C5b7aF2890aFAf6E7045669Dc96952` |
| x402PaymentAdapter | `0xe25d0C100a98D2004e3CC81b081492Bb3D102a91` |

Deployed: March 24, 2026.

**Layer 2 — genesis-x402 Crate (Rust)**

New workspace crate: `genesis-x402`

- `config.rs` — X402Config::from_env(), per-action price table
- `wallet.rs` — AES-256-GCM vault, Argon2id KDF, secp256k1 address derivation
- `eip3009.rs` — EIP-712 domain separator, TransferWithAuthorization signing
- `facilitator.rs` — settlements handler
- `middleware.rs` — Axum `x402_gate` seller middleware (402 → verify → settle → 200)
- `lineage.rs` — append-only JSONL heredity ledger with ancestry chain queries
- `settlement.rs` — SettlementEvent enum, background consumer loop

**Layer 3 — genesis-ledger Crate (Rust)**

New workspace crate: `genesis-ledger`

An in-house atomic triple-write engine. No external dependency for verification.

- `pricing.rs` — 19-action price table (1,000–100,000 micro), env override
- `accounts.rs` — BalanceRegistry with race-safe write-lock
- `journal.rs` — double-entry JournalLedger, balance validation enforced
- `lineage.rs` — LineageStore, append-only heredity chain
- `batch.rs` — BatchManager, keccak256 anchor hash, threshold-based auto-close
- `engine.rs` — LedgerEngine::execute_action() — a non-negotiable six-step triple-write:
  1. Price lookup (unknown/disabled = fast reject)
  2. Balance check (insufficient = fast reject, no debit)
  3. Debit (reversible if steps 4–5 fail)
  4. Journal write (double-entry, must balance)
  5. Lineage write (heredity record appended)
  6. Batch absorb (may emit AnchorProof if threshold crossed)

The verifier is in-house. A local ECDSA signature recovery using k256 — no HTTP calls to external facilitators. Polygon is the only external rail.

7 passing tests: triple_write, insufficient_balance, unknown_action, balance_unchanged, batch_closes_at_threshold, journal_balanced, parent_lineage_chain_resolves.

---

## Why This Matters for Genesis

The experimental results are deterministic. They will always be deterministic. 44 configurations, 6,820 worlds — those numbers don't change.

The live organism is different. The live instance has been running for weeks. It accumulates state. It has a payment layer now. Agents running in the live instance can interact with real micropayment infrastructure.

This is a different kind of system than the experiment engine. The experiments prove something about the architecture. The payment layer connects that architecture to an economic primitive that didn't exist when we started.

Agent economies in simulations have been studied for decades. What hasn't been studied is an agent economy with a live settlement rail — where the accounting is on-chain, the settlements are batched and anchored, and the infrastructure is public.

That's what the payment layer enables. Whether it produces anything interesting is an open question.

---

## Engine Count: 13 → 15 Crates

Genesis Protocol now has 15 Rust crates. The original 13 from the paper remain unchanged. The two new crates (genesis-x402, genesis-ledger) are live infrastructure, not experimental additions.

The experiment corpus (6,820 worlds, 44 configurations) was not run against these crates. The results reported in the paper are from the 13-crate engine. The new crates do not affect experiment reproducibility.

---

Source: [github.com/FTHTrading/Genesis](https://github.com/FTHTrading/Genesis) · [Polygon explorer](https://polygonscan.com/address/0xe25d0C100a98D2004e3CC81b081492Bb3D102a91) · Deployed: 2026-03-24
