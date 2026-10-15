import { execFileSync } from "node:child_process";
import { cargoCommand } from "./managed-cargo.mjs";

// Core browser acceptance needs the production static host, generated Help and
// two captured HTTP contracts, not the backend Nextest archive or RustFS.
for (const [pkg, example] of [
  ["jj-store-http", "browser_preview"],
  ["jj-store", "docs_reference"],
])
  execFileSync(
    ...cargoCommand(["build", "--locked", "-p", pkg, "--example", example]),
    { stdio: "inherit" },
  );
execFileSync(process.execPath, ["scripts/browser-fixtures.mjs"], {
  stdio: "inherit",
});
