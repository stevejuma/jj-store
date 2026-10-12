import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { cargoCommand } from "./managed-cargo.mjs";
mkdirSync(".scratch", { recursive: true });
function capture(test, environment) {
  const stdout = execFileSync(
    ...cargoCommand([
      "test",
      "--quiet",
      "--locked",
      "-p",
      "jj-store-cli",
      "--test",
      "native_lifecycle",
      test,
      "--",
      "--exact",
    ]),
    {
      encoding: "utf8",
      env: { ...process.env, ...environment },
      maxBuffer: 4 * 1024 * 1024,
    },
  );
  if (!stdout.includes("1 passed; 0 failed"))
    throw new Error(`Native browser response fixture did not run: ${test}`);
}
const reads = resolve(".scratch/web-read-responses.json");
const mutations = resolve(".scratch/web-mutation-responses.json");
capture(
  "content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes",
  { JJ_STORE_CONTRACT_CAPTURE: reads },
);
capture(
  "http_mutation_acceptance::http_mutations_preview_apply_status_replay_and_contract_capture",
  { JJ_STORE_MUTATION_CONTRACT_CAPTURE: mutations },
);
writeFileSync(
  resolve(".scratch/web-responses.json"),
  JSON.stringify([
    ...JSON.parse(readFileSync(reads, "utf8")),
    ...JSON.parse(readFileSync(mutations, "utf8")),
  ]),
);
console.log("Captured native HTTP/SSE and mutation browser fixtures.");
