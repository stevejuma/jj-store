import { execFileSync } from "node:child_process";
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
mkdirSync(".scratch", { recursive: true });
const stdout = execFileSync(
  "cargo",
  [
    "test",
    "--quiet",
    "--locked",
    "-p",
    "jj-store-cli",
    "--test",
    "native_lifecycle",
    "content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes",
    "--",
    "--exact",
  ],
  {
    encoding: "utf8",
    env: {
      ...process.env,
      JJ_STORE_CONTRACT_CAPTURE: resolve(".scratch/web-responses.json"),
    },
    maxBuffer: 4 * 1024 * 1024,
  },
);
if (!stdout.includes("1 passed; 0 failed"))
  throw new Error("Native browser response fixture did not run");
console.log("Captured native HTTP/SSE browser fixtures.");
