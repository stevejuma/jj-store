import assert from "node:assert/strict";
import {
  readFileSync,
  existsSync,
  copyFileSync,
  rmSync,
  mkdirSync,
} from "node:fs";
import { execFileSync } from "node:child_process";
import { readInventory } from "./ci-inventory.mjs";
import { packageGraph, selectedPackages } from "./ci-packages.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
const kind = process.argv[2];
assert.ok(
  ["stories", "browser", "rust"].includes(kind),
  "Unknown quarantine family",
);
const registry = JSON.parse(readFileSync(".config/ci-quarantine.json", "utf8"));
assert.equal(registry.version, 1);
assert.deepEqual(registry.cases.map((entry) => entry.id).sort(), [
  "Q-BROWSER-001",
  "Q-RUST-002",
  "Q-STORY-001",
  "Q-STORY-002",
]);
const cases = registry.cases.filter((entry) => entry.kind === kind);
assert.ok(cases.length);
if (kind === "rust") {
  const archive = ".release/tests-ordinary.tar.zst";
  const metadata = JSON.parse(
    execFileSync(
      "tar",
      ["-xOf", archive, "target/nextest/cargo-metadata.json"],
      { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
    ),
  );
  const inventory = await readInventory(
    archive,
    "ordinary",
    selectedPackages(
      "ordinary",
      undefined,
      packageGraph(metadata, metadata.workspace_root),
    ),
  );
  assert.equal(
    inventory.tests.filter((identity) => {
      const [, , name, ignored] = JSON.parse(identity);
      return name === cases[0].name && ignored === true;
    }).length,
    1,
    "Quarantined Rust body absent from archive",
  );
}
mkdirSync(".release/ci-results", { recursive: true });
let failed = false;
// Fresh executions, retries off. A later pass never erases an earlier failure.
for (let repetition = 1; repetition <= 3; repetition++) {
  console.log(
    `Quarantine diagnostics ${kind}, repetition ${repetition}/3: ${cases.map((entry) => entry.id).join(", ")}`,
  );
  if (kind === "rust") rmSync("target/nextest/ci/junit.xml", { force: true });
  try {
    if (kind === "stories")
      execFileSync(
        "pnpm",
        [
          "test:storybook",
          ...cases.map((entry) => entry.file),
          "--retry=0",
          "--reporter=default",
          "--reporter=junit",
          `--outputFile=.release/ci-results/quarantine-stories-${repetition}.xml`,
        ],
        {
          stdio: "inherit",
          env: { ...process.env, JJ_STORE_QUARANTINE: "stories" },
        },
      );
    else if (kind === "browser")
      execFileSync(
        "pnpm",
        [
          "exec",
          "playwright",
          "test",
          "--config",
          "web/playwright.config.ts",
          cases[0].file,
          "--grep",
          `${cases[0].name}$`,
          "--retries=0",
        ],
        {
          stdio: "inherit",
          env: {
            ...process.env,
            JJ_STORE_QUARANTINE: "Q-BROWSER-001",
            PLAYWRIGHT_JUNIT_OUTPUT_FILE: `${process.cwd()}/.release/ci-results/quarantine-browser-${repetition}.xml`,
          },
        },
      );
    else
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
          "--run-ignored",
          "only",
          "--retries",
          "0",
          "--no-tests",
          "fail",
          "-E",
          `binary(${cases[0].binary}) & test(=${cases[0].name})`,
        ]),
        { stdio: "inherit" },
      );
  } catch {
    failed = true;
  } finally {
    if (kind === "rust" && existsSync("target/nextest/ci/junit.xml"))
      copyFileSync(
        "target/nextest/ci/junit.xml",
        `.release/ci-results/quarantine-rust-${repetition}.xml`,
      );
  }
}
if (failed) process.exitCode = 1;
