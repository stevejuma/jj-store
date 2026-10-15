import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { selectedPackages } from "./ci-packages.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
import { buildInventory, readInventory } from "./ci-inventory.mjs";
import { timed } from "./ci-timing.mjs";
const mode = process.argv[2],
  cohort = process.argv[3] || "ordinary";
if (
  !["build", "run", "inventory"].includes(mode) ||
  !["ordinary", "heavy"].includes(cohort)
)
  throw Error("invalid CI test selection");
const archive = `.release/tests-${cohort}.tar.zst`;
mkdirSync(".release", { recursive: true });
const packages = selectedPackages(cohort);
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
    count = cohort === "heavy" ? 8 : 4;
  if (!Number.isInteger(shard) || shard < 1 || shard > count)
    throw Error("invalid test shard");
  const inventory = await readInventory(archive, cohort, packages);
  const expected = inventory.partitions[shard - 1].length;
  console.log(
    `Verified source and archive; shard ${shard}/${count} expects ${expected} runnable tests.`,
  );
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
        cohort === "heavy" ? "heavy" : "ci",
        "--partition",
        `hash:${shard}/${count}`,
        "--no-tests",
        expected === 0 ? "pass" : "fail",
      ]),
      { stdio: "inherit" },
    ),
  );
}
