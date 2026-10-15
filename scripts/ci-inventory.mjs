import { execFileSync } from "node:child_process";
import {
  createReadStream,
  mkdtempSync,
  readFileSync,
  writeFileSync,
  rmSync,
} from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
import assert from "node:assert/strict";
export async function archiveHash(path) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest("hex");
}
export const sourceSha = () =>
  execFileSync("git", ["rev-parse", "HEAD"], { encoding: "utf8" }).trim();
export function testsFromList(list) {
  if (!list["rust-suites"] || !Number.isInteger(list["test-count"]))
    throw Error("Invalid Nextest inventory");
  const packages = new Set(),
    tests = [],
    runnable = [];
  for (const [id, suite] of Object.entries(list["rust-suites"])) {
    if (suite.status !== "listed" || !suite.testcases || !suite["package-name"])
      throw Error("Incomplete test listing");
    packages.add(suite["package-name"]);
    for (const [name, value] of Object.entries(suite.testcases)) {
      const identity = JSON.stringify([
        suite["package-name"],
        id,
        name,
        value.ignored,
      ]);
      tests.push(identity);
      if (!value.ignored && value["filter-match"]?.status === "matches")
        runnable.push(identity);
    }
  }
  if (tests.length !== list["test-count"])
    throw Error("Incomplete test inventory count");
  return {
    packages: [...packages].sort(),
    tests: tests.sort(),
    runnable: runnable.sort(),
  };
}
export function verifyInventory(data, { source, cohort, packages, hash }) {
  assert.equal(data.format, 1);
  assert.equal(data.source, source, "Archive source mismatch");
  assert.equal(data.cohort, cohort);
  assert.equal(data.archive_sha256, hash, "Archive digest mismatch");
  assert.deepEqual(
    data.packages,
    [...packages].sort(),
    "Archive package mismatch",
  );
  assert.equal(data.partitions.length, cohort === "heavy" ? 8 : 4);
  assert.equal(new Set(data.tests).size, data.tests.length);
  assert.equal(new Set(data.runnable).size, data.runnable.length);
  assert.ok(data.runnable.every((id) => data.tests.includes(id)));
  const partitioned = data.partitions.flat().sort();
  assert.equal(
    new Set(partitioned).size,
    partitioned.length,
    "Duplicate shard tests",
  );
  assert.deepEqual(
    partitioned,
    data.runnable,
    "Missing or unexpected shard tests",
  );
  return data;
}
export async function buildInventory(archive, cohort, packages) {
  // Only extract metadata: binaries stay in the managed build target. Listing
  // reusable binaries does not compile, run tests or need another target tree.
  const dir = mkdtempSync(".release/inventory-");
  try {
    const paths = ["cargo-metadata.json", "binaries-metadata.json"].map(
      (name) => {
        const path = join(dir, name);
        writeFileSync(
          path,
          execFileSync("tar", ["-xOf", archive, `target/nextest/${name}`], {
            maxBuffer: 64 * 1024 * 1024,
          }),
        );
        return path;
      },
    );
    const list = (partition) =>
      testsFromList(
        JSON.parse(
          execFileSync(
            "cargo",
            [
              "nextest",
              "list",
              "--cargo-metadata",
              paths[0],
              "--binaries-metadata",
              paths[1],
              "--profile",
              cohort === "heavy" ? "heavy" : "ci",
              "--message-format",
              "json",
              ...(partition ? ["--partition", partition] : []),
            ],
            { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
          ),
        ),
      );
    const full = list(),
      count = cohort === "heavy" ? 8 : 4;
    assert.deepEqual(
      full.packages,
      [...packages].sort(),
      "Archive contains unplanned packages",
    );
    const partitions = Array.from({ length: count }, (_, i) => {
      const shard = list(`hash:${i + 1}/${count}`);
      assert.deepEqual(
        shard.tests,
        full.tests,
        "Shard listing lost test identities",
      );
      return shard.runnable;
    });
    const source = sourceSha(),
      hash = await archiveHash(archive);
    const data = {
      format: 1,
      source,
      cohort,
      archive_sha256: hash,
      ...full,
      partitions,
    };
    verifyInventory(data, { source, cohort, packages, hash });
    writeFileSync(archive + ".json", JSON.stringify(data, null, 2) + "\n");
    console.log(
      `Verified ${full.tests.length} test identities; ${full.runnable.length} runnable tests in ${count} disjoint shards.`,
    );
    return data;
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
export async function readInventory(archive, cohort, packages) {
  return verifyInventory(JSON.parse(readFileSync(archive + ".json", "utf8")), {
    source: sourceSha(),
    cohort,
    packages,
    hash: await archiveHash(archive),
  });
}
