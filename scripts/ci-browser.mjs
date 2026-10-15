import { readInventory } from "./ci-inventory.mjs";
import { selectedPackages } from "./ci-packages.mjs";
import { execFileSync } from "node:child_process";
import { cargoCommand } from "./managed-cargo.mjs";
const pnpm = (...args) => execFileSync("pnpm", args, { stdio: "inherit" });
const lane = process.argv[2];
const prebuilt =
  process.env.CI === "true" && process.env.JJ_STORE_CI_PREBUILT === "1";
const shard = process.env.JJ_STORE_BROWSER_SHARD;
if (shard && !/^[1-9]\d*\/[1-9]\d*$/.test(shard))
  throw Error("invalid browser shard");
const shardArgs = shard ? [`--shard=${shard}`] : [];
if (lane === "storybook") pnpm("test:storybook", ...shardArgs);
else if (lane === "browser" && prebuilt) {
  pnpm(
    "exec",
    "playwright",
    "test",
    "--config",
    "web/playwright.config.ts",
    ...shardArgs,
  );
} else if (lane === "browser") {
  pnpm("web:test:browser", ...shardArgs);
} else if (lane === "catalog") {
  if (prebuilt)
    pnpm("exec", "node", "--test", "scripts/catalog-browser.test.mjs");
  else pnpm("catalog:test:browser");
} else if (lane === "help") {
  if (prebuilt)
    pnpm(
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.docs.config.ts",
    );
  else pnpm("docs:test:browser");
} else if (lane === "cold" || lane === "cold-profile") {
  if (lane === "cold-profile") {
    delete process.env.JJ_STORE_REAL_SHARD;
    process.env.JJ_STORE_REAL_TEST = "profiles-emoji.spec.ts";
  }
  pnpm("web:test:real");
} else if (lane === "git-cycle") {
  await readInventory(
    ".release/tests-heavy.tar.zst",
    "heavy",
    selectedPackages("heavy"),
  );
  process.env.JJ_STORE_RUSTFS_RUNTIME = process.env.JJ_STORE_DEV_RUSTFS_RUNTIME;
  execFileSync(
    ...cargoCommand([
      "nextest",
      "run",
      "--archive-file",
      ".release/tests-heavy.tar.zst",
      "--workspace-remap",
      process.cwd(),
      "--profile",
      "heavy",
      "--run-ignored",
      "only",
      "-E",
      "binary(server_executable) & test(=git_bridge_acceptance::real_rustfs_native_git_external_merge_cycle_survives_cold_restart)",
    ]),
    { stdio: "inherit" },
  );
} else throw Error("unknown browser lane");
