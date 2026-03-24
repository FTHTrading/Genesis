import { HardhatUserConfig, subtask } from "hardhat/config";
import { TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS } from "hardhat/builtin-tasks/task-names";
import "@nomicfoundation/hardhat-toolbox";
import "hardhat-contract-sizer";
import * as dotenv from "dotenv";
import * as path from "path";
import * as fs from "fs";

dotenv.config();

// ── Source path fix ───────────────────────────────────────────────────────────
// .sol files live in the same directory as hardhat.config.ts (not in a
// "contracts/" subdirectory). Override the default discovery so Hardhat
// picks up ONLY the .sol files here — never recursing into node_modules,
// which would trigger error HH1006.
subtask(TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS, async () => {
  const dir = __dirname;
  return fs
    .readdirSync(dir)
    .filter((f) => f.endsWith(".sol"))
    .map((f) => path.join(dir, f));
});

const DEPLOYER_PK = process.env.DEPLOYER_PRIVATE_KEY ?? "0x" + "0".repeat(64);
const POLYGONSCAN_API_KEY = process.env.POLYGONSCAN_API_KEY ?? "";
const POLYGON_RPC = process.env.POLYGON_RPC_URL ?? "https://polygon-rpc.com";
const AMOY_RPC   = process.env.AMOY_RPC_URL    ?? "https://rpc-amoy.polygon.technology";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.24",
    settings: {
      optimizer: { enabled: true, runs: 200 },
      viaIR: true,
      evmVersion: "cancun",   // needed for mcopy opcode (OZ v5 Bytes.sol)
    },
  },
  networks: {
    hardhat: {},
    polygon: {
      url: POLYGON_RPC,
      accounts: [DEPLOYER_PK],
      chainId: 137,
    },
    amoy: {
      url: AMOY_RPC,
      accounts: [DEPLOYER_PK],
      chainId: 80002,
    },
  },
  etherscan: {
    apiKey: POLYGONSCAN_API_KEY,  // Etherscan v2 unified key
    customChains: [
      {
        network: "polygon",
        chainId: 137,
        urls: {
          apiURL: "https://api.polygonscan.com/api",
          browserURL: "https://polygonscan.com",
        },
      },
    ],
  },
  contractSizer: {
    alphaSort: true,
    runOnCompile: false,
    disambiguatePaths: false,
  },
};

export default config;
