import { ethers } from "hardhat";
import * as fs from "fs";

// ── Treasury Math ─────────────────────────────────────────────────────────
// Total supply:  10,000,000,000 GENESIS
//   85% (8.5B)  → Treasury vesting address — 3-year linear
//    5% (500M)  → Ecosystem fund (agent rewards, staking)
//    5% (500M)  → Team — 2-year linear vesting
//    5% (500M)  → Liquidity pool at launch
// ─────────────────────────────────────────────────────────────────────────

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying GenesisToken...");
  console.log("  Deployer:  ", deployer.address);
  console.log("  Network:   ", (await ethers.provider.getNetwork()).name);

  // Read addresses from env or default to deployer (for testnet convenience)
  const admin       = process.env.GENESIS_ADMIN       ?? deployer.address;
  const treasury    = process.env.GENESIS_TREASURY    ?? deployer.address;
  const ecosystem   = process.env.GENESIS_ECOSYSTEM   ?? deployer.address;
  const team        = process.env.GENESIS_TEAM        ?? deployer.address;
  const liqPool     = process.env.GENESIS_LIQUIDITY   ?? deployer.address;

  console.log("  Admin:     ", admin);
  console.log("  Treasury:  ", treasury);
  console.log("  Ecosystem: ", ecosystem);
  console.log("  Team:      ", team);
  console.log("  LiqPool:   ", liqPool);

  const factory = await ethers.getContractFactory("GenesisToken");
  const token   = await factory.deploy(admin, treasury, ecosystem, team, liqPool);
  await token.waitForDeployment();

  const address = await token.getAddress();
  console.log("\n  GenesisToken deployed to:", address);
  console.log("  Total supply:", ethers.formatUnits(await token.totalSupply(), 18), "GENESIS");

  // Save deployment info
  const chainId = (await ethers.provider.getNetwork()).chainId.toString();
  const info = {
    address,
    deployer:  deployer.address,
    chainId,
    timestamp: new Date().toISOString(),
    totalSupply: "10000000000",
    treasuryAlloc: "8500000000",
    ecosystemAlloc: "500000000",
    teamAlloc: "500000000",
    floatAlloc: "500000000",
  };

  fs.mkdirSync("deployments", { recursive: true });
  fs.writeFileSync(`deployments/${chainId}.json`, JSON.stringify(info, null, 2));
  console.log(`  Saved to deployments/${chainId}.json`);

  console.log("\n  Next steps:");
  console.log(`  1. Verify: npx hardhat verify --network polygon ${address} "${admin}" "${treasury}" "${ecosystem}" "${team}" "${liqPool}"`);
  console.log("  2. Add GENESIS_TOKEN_ADDRESS to Genesis .env");
  console.log("  3. Fund the Genesis hot wallet with Polygon USDC for x402 payments");
}

main().catch((err) => { console.error(err); process.exit(1); });
