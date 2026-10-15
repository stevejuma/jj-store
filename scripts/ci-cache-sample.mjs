import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { selectedPackages } from "./ci-packages.mjs";
import { readInventory, sourceSha } from "./ci-inventory.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
const mode = process.argv[2],
  sample = process.env.CI_SAMPLE;
if (mode === "build") {
  if (!/^(seed|(?:baseline|sccache)-[123])$/.test(sample ?? ""))
    throw Error("Invalid cache trial sample");
  if (sample !== "seed" && process.env.CI_CACHE_HIT !== "true")
    throw Error("Warm trial requires an exact dependency cache hit");
  const start = performance.now();
  execFileSync(process.execPath, ["scripts/ci-build.mjs", "ordinary"], {
    stdio: "inherit",
  });
  const build_seconds = (performance.now() - start) / 1000;
  const sccache_stats =
    process.env.RUSTC_WRAPPER === "sccache"
      ? JSON.parse(
          execFileSync("sccache", ["--show-stats", "--stats-format", "json"], {
            encoding: "utf8",
          }),
        )
      : null;
  const inventory = await readInventory(
    ".release/tests-ordinary.tar.zst",
    "ordinary",
    selectedPackages("ordinary"),
  );
  mkdirSync(".release/cache-trial", { recursive: true });
  writeFileSync(
    `.release/cache-trial/${sample}.json`,
    JSON.stringify(
      {
        sample,
        source: sourceSha(),
        build_seconds,
        sccache_stats,
        cache_hit: process.env.CI_CACHE_HIT,
        packages: inventory.packages,
        tests: inventory.tests,
        runnable: inventory.runnable,
        partitions: inventory.partitions,
      },
      null,
      2,
    ) + "\n",
  );
} else if (mode === "test") {
  await readInventory(
    ".release/tests-ordinary.tar.zst",
    "ordinary",
    selectedPackages("ordinary"),
  );
  execFileSync(
    ...cargoCommand([
      "nextest",
      "run",
      "--archive-file",
      ".release/tests-ordinary.tar.zst",
      "--workspace-remap",
      process.cwd(),
      "--profile",
      "ci",
      "--no-tests",
      "fail",
    ]),
    { stdio: "inherit" },
  );
} else throw Error("Invalid cache trial command");
