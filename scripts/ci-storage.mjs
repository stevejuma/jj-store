import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
const lane = process.argv[2];
const pnpm = (...args) => execFileSync("pnpm", args, { stdio: "inherit" });
if (lane === "external") pnpm("test:rustfs");
else if (lane === "contracts") pnpm("contracts:check");
else if (lane === "embedded") {
  const prebuilt =
    process.env.CI === "true" && process.env.JJ_STORE_CI_PREBUILT === "1";
  if (!prebuilt) pnpm("build:embedded-rustfs");
  process.env.JJ_STORE_RUSTFS_RUNTIME = resolve(
    "runtime/embedded-rustfs/target/debug/jj-store-rustfs",
  );
  if (!prebuilt)
    pnpm(
      "dev:cargo",
      "--",
      "clippy",
      "--manifest-path",
      "runtime/embedded-rustfs/Cargo.toml",
      "--all-targets",
      "--locked",
      "--",
      "-D",
      "warnings",
    );
  pnpm(
    "dev:cargo",
    "--",
    "test",
    "-p",
    "jj-store-server",
    "--test",
    "embedded_storage",
    "--test",
    "custom_data_indexes",
    "--locked",
    "--",
    "--ignored",
    "--test-threads=1",
  );
  pnpm(
    "dev:cargo",
    "--",
    "test",
    "-p",
    "jj-store",
    "--test",
    "server_executable",
    "--locked",
    "--",
    "--ignored",
    "--exact",
    "local_trusted_executable_bootstraps_registers_and_serves_without_tokens",
  );
} else throw Error("unknown storage lane");
