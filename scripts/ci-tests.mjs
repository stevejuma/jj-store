import { execFileSync } from "node:child_process";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import {
  cohortProfile,
  cohortShards,
  profileJunitPath,
  selectedPackages,
  packageGraph,
} from "./ci-packages.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
import { buildInventory, readInventory } from "./ci-inventory.mjs";
import { timed } from "./ci-timing.mjs";
const mode = process.argv[2],
  cohort = process.argv[3] || "ordinary";
if (
  !["build", "run", "inventory"].includes(mode) ||
  !["ordinary", "heavy", "targeted"].includes(cohort)
)
  throw Error("invalid CI test selection");
const archive = `.release/tests-${cohort}.tar.zst`;
mkdirSync(".release", { recursive: true });
// Archive consumers use the build's metadata; never fetch or resolve dependencies.
const metadata =
  mode === "run" || mode === "inventory"
    ? JSON.parse(
        execFileSync(
          "tar",
          ["-xOf", archive, "target/nextest/cargo-metadata.json"],
          { maxBuffer: 64 * 1024 * 1024, encoding: "utf8" },
        ),
      )
    : null;
const packages = metadata
  ? selectedPackages(
      cohort,
      process.env.CI_PACKAGES,
      packageGraph(metadata, metadata.workspace_root),
    )
  : selectedPackages(cohort);
if (mode === "build") {
  timed(`Compile and archive ${cohort}`, () =>
    execFileSync(
      ...cargoCommand([
        "nextest",
        "archive",
        ...packages.flatMap((name) => ["-p", name]),
        "--locked",
        "--archive-file",
        archive,
      ]),
      { stdio: "inherit" },
    ),
  );
  await buildInventory(archive, cohort, packages);
} else if (mode === "inventory") {
  writeFileSync(
    ".release/test-inventory.json",
    JSON.stringify(await readInventory(archive, cohort, packages), null, 2),
  );
} else {
  const shard = Number(process.env.CI_SHARD),
    count = cohortShards(cohort);
  if (!Number.isInteger(shard) || shard < 1 || shard > count)
    throw Error("invalid test shard");
  const inventory = await readInventory(archive, cohort, packages);
  const expected = inventory.partitions[shard - 1].length;
  if (process.env.JJ_STORE_CI_REPORTS === "1") {
    mkdirSync(".release/ci-results", { recursive: true });
    writeFileSync(
      ".release/ci-results/inventory.json",
      JSON.stringify(
        {
          source: inventory.source,
          cohort,
          profile: cohortProfile(cohort),
          packages: inventory.packages,
          archive_sha256: inventory.archive_sha256,
          shard,
          expected,
          tests: inventory.partitions[shard - 1],
        },
        null,
        2,
      ),
    );
  }
  console.log(
    `Verified source and archive; shard ${shard}/${count} expects ${expected} runnable tests.`,
  );
  // A shard result is valid only when this invocation produced it. Removing
  // the selected profile's report prevents an interrupted run from publishing
  // a stale JUnit file copied into the archive or retained by a retry.
  rmSync(profileJunitPath(cohortProfile(cohort)), { force: true });
  timed(`Run ${cohort} shard ${shard}/${count}`, () =>
    execFileSync(
      ...cargoCommand([
        "nextest",
        "run",
        "--archive-file",
        archive,
        "--workspace-remap",
        process.cwd(),
        "--profile",
        cohortProfile(cohort),
        "--partition",
        `hash:${shard}/${count}`,
        "--no-tests",
        expected === 0 ? "pass" : "fail",
      ]),
      { stdio: "inherit" },
    ),
  );
}
