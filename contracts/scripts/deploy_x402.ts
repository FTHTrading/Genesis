/**
 * deploy_x402.ts
 *
 * Deploys Treasury.sol and x402PaymentAdapter.sol to Polygon mainnet.
 *
 * Deployment order matters:
 *   1. Treasury   — needs worldToken + usdc
 *   2. x402       — needs treasury address
 *   3. Grant Treasury FEE_ROUTER to x402 adapter
 *   4. Grant GENESIS_ADMIN all roles on both contracts (then deployer retains roles too for testing)
 *
 * Usage:
 *   cd C:\Users\Kevan\Genesis\contracts
 *   npx hardhat run scripts/deploy_x402.ts --network polygon
 */

import { ethers } from "hardhat";
import * as fs from "fs";
import * as path from "path";
import * as dotenv from "dotenv";

dotenv.config();

// ── Constants ──────────────────────────────────────────────────────────────────
const NATIVE_USDC     = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"; // native USDC on Polygon (NOT bridged USDC.e)
const GENESIS_TOKEN   = "0x14E64b91B96f11D12ef6bDaDc21e2f25a2f45a99"; // already deployed
const GENESIS_ADMIN   = process.env.GENESIS_ADMIN   ?? "";
const GENESIS_TREASURY_WALLET = process.env.GENESIS_TREASURY  ?? "";
const GENESIS_ECOSYSTEM = process.env.GENESIS_ECOSYSTEM ?? "";
const GENESIS_LIQUIDITY = process.env.GENESIS_LIQUIDITY  ?? "";

// Roles (keccak256 hashes matching the contracts)
const DEFAULT_ADMIN_ROLE = "0x0000000000000000000000000000000000000000000000000000000000000000";
const EMISSION_ROLE   = ethers.id("EMISSION_ROLE");
const FEE_ROUTER      = ethers.id("FEE_ROUTER");
const AUDITOR_ROLE    = ethers.id("AUDITOR_ROLE");
const SETTLER_ROLE    = ethers.id("SETTLER_ROLE");
const RELAYER_ROLE    = ethers.id("RELAYER_ROLE");

// ── Deployment record ──────────────────────────────────────────────────────────
const DEPLOYMENTS_FILE = path.join(__dirname, "../deployments/137.json");

function loadDeployments(): Record<string, any> {
  try {
    return JSON.parse(fs.readFileSync(DEPLOYMENTS_FILE, "utf8"));
  } catch {
    return {};
  }
}

function saveDeployments(data: Record<string, any>): void {
  fs.mkdirSync(path.dirname(DEPLOYMENTS_FILE), { recursive: true });
  fs.writeFileSync(DEPLOYMENTS_FILE, JSON.stringify(data, null, 2));
}

// ── Main ───────────────────────────────────────────────────────────────────────
async function main() {
  const [deployer] = await ethers.getSigners();
  const balance    = await ethers.provider.getBalance(deployer.address);

  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("  Genesis x402 Deployment — Polygon Mainnet (Chain 137)");
  console.log("═══════════════════════════════════════════════════════════");
  console.log(`  Deployer:       ${deployer.address}`);
  console.log(`  Balance:        ${ethers.formatEther(balance)} POL`);
  console.log(`  USDC (native):  ${NATIVE_USDC}`);
  console.log(`  GenesisToken:   ${GENESIS_TOKEN}`);
  console.log(`  GENESIS_ADMIN:  ${GENESIS_ADMIN}`);
  console.log(`  Rewards pool:   ${GENESIS_ECOSYSTEM}`);
  console.log(`  Insurance pool: ${GENESIS_LIQUIDITY}`);
  console.log("═══════════════════════════════════════════════════════════\n");

  // Sanity checks
  if (!GENESIS_ADMIN || GENESIS_ADMIN === "")  throw new Error("GENESIS_ADMIN not set in .env");
  if (!GENESIS_ECOSYSTEM)                       throw new Error("GENESIS_ECOSYSTEM not set in .env");
  if (!GENESIS_LIQUIDITY)                       throw new Error("GENESIS_LIQUIDITY not set in .env");
  if (balance < ethers.parseEther("0.5"))       throw new Error("Low POL balance — need at least 0.5 POL for gas");

  const deployments = loadDeployments();

  // ── Step 1: Deploy Treasury ──────────────────────────────────────────────────
  console.log("Step 1/4 — Deploying Treasury.sol...");
  const TreasuryFactory = await ethers.getContractFactory("Treasury");
  const treasury = await TreasuryFactory.deploy(
    deployer.address,   // admin = deployer initially (so we can grant FEE_ROUTER in this script)
    GENESIS_TOKEN,      // worldToken
    NATIVE_USDC         // usdc
  );
  await treasury.waitForDeployment();
  const treasuryAddr = await treasury.getAddress();
  console.log(`  ✅ Treasury deployed at: ${treasuryAddr}`);

  // ── Step 2: Deploy x402PaymentAdapter ───────────────────────────────────────
  console.log("\nStep 2/4 — Deploying x402PaymentAdapter.sol...");
  const AdapterFactory = await ethers.getContractFactory("x402PaymentAdapter");
  const adapter = await AdapterFactory.deploy(
    deployer.address,  // admin = deployer initially
    NATIVE_USDC,       // _usdc
    treasuryAddr,      // _treasury  (just deployed above)
    GENESIS_ECOSYSTEM, // _rewardsPool
    GENESIS_LIQUIDITY  // _insurancePool
  );
  await adapter.waitForDeployment();
  const adapterAddr = await adapter.getAddress();
  console.log(`  ✅ x402PaymentAdapter deployed at: ${adapterAddr}`);

  // ── Step 3: Grant Treasury FEE_ROUTER to x402 adapter ─────────────────────
  console.log("\nStep 3/4 — Granting roles...");

  // Treasury: grant FEE_ROUTER to x402 adapter (so it can call receiveFeeIncome / closeSettlementBatch)
  const tx1 = await treasury.grantRole(FEE_ROUTER, adapterAddr);
  await tx1.wait();
  console.log(`  ✅ Treasury FEE_ROUTER → x402PaymentAdapter (${adapterAddr.slice(0,10)}...)`);

  // Treasury: grant full roles to GENESIS_ADMIN (multisig/operations wallet)
  const tx2 = await treasury.grantRole(DEFAULT_ADMIN_ROLE, GENESIS_ADMIN);
  await tx2.wait();
  console.log(`  ✅ Treasury DEFAULT_ADMIN_ROLE → GENESIS_ADMIN`);

  const tx3 = await treasury.grantRole(EMISSION_ROLE, GENESIS_ADMIN);
  await tx3.wait();
  console.log(`  ✅ Treasury EMISSION_ROLE → GENESIS_ADMIN`);

  const tx4 = await treasury.grantRole(AUDITOR_ROLE, GENESIS_ADMIN);
  await tx4.wait();
  console.log(`  ✅ Treasury AUDITOR_ROLE → GENESIS_ADMIN`);

  // x402PaymentAdapter: grant full roles to GENESIS_ADMIN
  const tx5 = await adapter.grantRole(DEFAULT_ADMIN_ROLE, GENESIS_ADMIN);
  await tx5.wait();
  console.log(`  ✅ x402Adapter DEFAULT_ADMIN_ROLE → GENESIS_ADMIN`);

  const tx6 = await adapter.grantRole(SETTLER_ROLE, GENESIS_ADMIN);
  await tx6.wait();
  console.log(`  ✅ x402Adapter SETTLER_ROLE → GENESIS_ADMIN`);

  const tx7 = await adapter.grantRole(RELAYER_ROLE, GENESIS_ADMIN);
  await tx7.wait();
  console.log(`  ✅ x402Adapter RELAYER_ROLE → GENESIS_ADMIN`);

  // ── Step 4: Save deployment record ──────────────────────────────────────────
  console.log("\nStep 4/4 — Saving deployment record...");
  deployments.Treasury           = { address: treasuryAddr,  deployedAt: new Date().toISOString(), deployer: deployer.address };
  deployments.x402PaymentAdapter = { address: adapterAddr,   deployedAt: new Date().toISOString(), deployer: deployer.address };
  saveDeployments(deployments);
  console.log(`  ✅ deployments/137.json updated`);

  // ── Summary ──────────────────────────────────────────────────────────────────
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("  DEPLOYMENT COMPLETE");
  console.log("═══════════════════════════════════════════════════════════");
  console.log(`  Treasury:           ${treasuryAddr}`);
  console.log(`  x402PaymentAdapter: ${adapterAddr}`);
  console.log("\n  ⚠️  NEXT STEPS:");
  console.log(`  1. Set in Gmiie .env:  X402_ADAPTER_ADDRESS=${adapterAddr}`);
  console.log(`  2. Fund wallet with native USDC (not USDC.e) to test topUpCredit()`);
  console.log(`  3. Verify on PolygonScan:`);
  console.log(`     npx hardhat verify --network polygon ${treasuryAddr} "${deployer.address}" "${GENESIS_TOKEN}" "${NATIVE_USDC}"`);
  console.log(`     npx hardhat verify --network polygon ${adapterAddr} "${deployer.address}" "${NATIVE_USDC}" "${treasuryAddr}" "${GENESIS_ECOSYSTEM}" "${GENESIS_LIQUIDITY}"`);
  console.log("═══════════════════════════════════════════════════════════\n");
}

main().catch((err) => {
  console.error("\n❌ Deployment failed:", err.message ?? err);
  process.exit(1);
});
