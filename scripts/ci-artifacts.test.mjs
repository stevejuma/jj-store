import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtemp,
  mkdir,
  writeFile,
  rm,
  symlink,
  readFile,
} from "node:fs/promises";
import { join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  verifyReceipt,
  verifyProducer,
  verifyPayload,
  lanePayloads,
} from "./ci-artifacts.mjs";
import {
  secondsBetween,
  summarizeJobs,
  junitCases,
  localResults,
} from "./ci-report.mjs";
const expected = {
  source: "a".repeat(40),
  repository: "stevejuma/jj-store",
  run: "123",
  attempt: "2",
};
const source = () => ({ format: 1, ...expected, role: "source", payloads: {} });
const producer = () => ({
  id: 123,
  run_attempt: 2,
  repository: { full_name: expected.repository },
  event: "workflow_dispatch",
  path: ".github/workflows/ci-heavy.yml",
  conclusion: "failure",
});
const jobs = () =>
  [
    "Resolve qualification source",
    "Standard checks / required",
    "build artifact producer",
  ].map((name) => ({ name: `qualification / ${name}`, conclusion: "success" }));
test("artifact reuse requires successful exact producers, not a successful unrelated suite", () => {
  assert.doesNotThrow(() =>
    verifyProducer(producer(), jobs(), expected, ["build"]),
  );
  for (const result of ["failure", "cancelled", "skipped", null])
    for (let i = 0; i < 3; i++) {
      const changed = jobs();
      changed[i].conclusion = result;
      assert.throws(() =>
        verifyProducer(producer(), changed, expected, ["build"]),
      );
    }
  assert.throws(() =>
    verifyProducer(producer(), jobs().slice(1), expected, ["build"]),
  );
  assert.throws(() =>
    verifyProducer(producer(), [...jobs(), jobs()[1]], expected, ["build"]),
  );
  for (const changed of [
    { id: 124 },
    { run_attempt: 1 },
    { repository: { full_name: "fork/jj-store" } },
    { path: ".github/workflows/ci.yml" },
    { event: "pull_request" },
  ])
    assert.throws(() =>
      verifyProducer({ ...producer(), ...changed }, jobs(), expected, [
        "build",
      ]),
    );
});
test("receipts reject wrong source, attempts, roles, missing payloads and unsafe paths", () => {
  assert.doesNotThrow(() => verifyReceipt(source(), expected, "source"));
  for (const changed of [
    { source: "b".repeat(40) },
    { run: "124" },
    { attempt: "1" },
    { repository: "fork/jj-store" },
    { role: "build" },
    { format: 2 },
  ])
    assert.throws(() =>
      verifyReceipt({ ...source(), ...changed }, expected, "source"),
    );
  const receipt = {
    ...source(),
    role: "assets",
    payloads: {
      "web-bundle": { "index.html": { sha256: "a".repeat(64), mode: 0o644 } },
      "storybook-bundle": {
        "index.html": { sha256: "a".repeat(64), mode: 0o644 },
      },
    },
  };
  assert.doesNotThrow(() => verifyReceipt(receipt, expected, "assets"));
  for (const path of [
    "../bad",
    "/tmp/bad",
    "a/../../bad",
    "a\\..\\bad",
    "a//bad",
  ]) {
    const changed = structuredClone(receipt);
    changed.payloads["web-bundle"] = {
      [path]: { sha256: "a".repeat(64), mode: 0o644 },
    };
    assert.throws(() => verifyReceipt(changed, expected, "assets"));
  }
  delete receipt.payloads["web-bundle"];
  assert.throws(() => verifyReceipt(receipt, expected, "assets"));
});
test("downloaded payload checks reject corruption and executable symlinks", async () => {
  const previous = process.cwd(),
    temp = await mkdtemp(join(tmpdir(), "jj-ci-artifacts-"));
  try {
    process.chdir(temp);
    await mkdir(".scratch");
    await writeFile(".scratch/web-responses.json", "fixture");
    const entries = {
      "web-responses.json": {
        sha256: createHash("sha256").update("fixture").digest("hex"),
        mode: 0o644,
      },
    };
    await verifyPayload("native-browser-fixtures", entries);
    await writeFile(".scratch/web-responses.json", "corrupt");
    await assert.rejects(
      verifyPayload("native-browser-fixtures", entries),
      /checksum/,
    );
    await rm(".scratch/web-responses.json");
    await writeFile("other", "fixture");
    await symlink(resolve("other"), ".scratch/web-responses.json");
    await assert.rejects(
      verifyPayload("native-browser-fixtures", entries),
      /symlink/,
    );
  } finally {
    process.chdir(previous);
    await rm(temp, { recursive: true, force: true });
  }
});
test("fixture UI and native/API checks do not require storage artifacts", () => {
  for (const lane of [
    "storybook",
    "catalog",
    "browser",
    "help",
    "native",
    "contracts",
    "external",
  ])
    assert.equal(lanePayloads[lane].includes("linux-storage-runtime"), false);
  for (const lane of ["cold", "cold-profile", "git-cycle", "embedded"])
    assert.ok(lanePayloads[lane].includes("linux-storage-runtime"));
});
test("command reporting preserves a failure and writes its diagnostics", async () => {
  const temp = await mkdtemp(join(tmpdir(), "jj-ci-exec-"));
  try {
    assert.throws(
      () =>
        execFileSync(
          process.execPath,
          [
            resolve("scripts/ci-exec.mjs"),
            "fixture",
            "--",
            process.execPath,
            "-e",
            'console.error("expected failure"); process.exit(7)',
          ],
          { cwd: temp, stdio: "pipe" },
        ),
      (error) => error.status === 7,
    );
    assert.match(
      await readFile(join(temp, ".release/ci-results/fixture.log"), "utf8"),
      /expected failure/,
    );
    const row = JSON.parse(
      await readFile(join(temp, ".release/ci-results/commands.jsonl"), "utf8"),
    );
    assert.equal(row.exit_code, 7);
    assert.ok(row.seconds >= 0);
  } finally {
    await rm(temp, { recursive: true, force: true });
  }
});
test("timing reports retain unknowns, failures and individual JUnit results", () => {
  assert.equal(secondsBetween(null, null), null);
  assert.equal(secondsBetween("bad", "bad"), null);
  const [job] = summarizeJobs([
    {
      name: "tests",
      started_at: "2026-10-15T10:00:00Z",
      completed_at: "2026-10-15T10:01:00Z",
      conclusion: "failure",
    },
  ]);
  assert.equal(job.execution_seconds, 60);
  assert.equal(job.queue_seconds, null);
  assert.equal(job.dependency_wait_seconds, null);
  assert.equal(job.conclusion, "failure");
  const cases = junitCases(
    '<testsuites><testsuite><testcase name="ok" time="1.5"/><testcase name="bad" time="2"><failure message="no"/></testcase><testcase name="skip"><skipped/></testcase></testsuite></testsuites>',
  );
  assert.deepEqual(
    cases.map((row) => row.result),
    ["passed", "failed", "skipped"],
  );
  assert.deepEqual(
    cases.map((row) => row.seconds),
    [1.5, 2, null],
  );
});

test("timing reports include RustFS counts and preserve startup failures", async () => {
  const previous = process.cwd();
  const temp = await mkdtemp(join(tmpdir(), "jj-ci-storage-report-"));
  try {
    process.chdir(temp);
    await mkdir(".release/ci-results", { recursive: true });
    await mkdir(".scratch");
    assert.equal(localResults().test_counts_available, false);
    const report = {
      result: "fail",
      failures: 1,
      tests: [
        { name: "cas", result: "pass", failures: 0, seconds: 0.1 },
        { name: "restart", result: "fail", failures: 1, seconds: 0.2 },
      ],
    };
    await writeFile(".scratch/rustfs-contract.json", JSON.stringify(report));
    const result = localResults();
    assert.equal(result.test_counts_available, true);
    assert.deepEqual(result.rustfs[0].counts, {
      total: 2,
      passed: 1,
      failed: 1,
      skipped: 0,
    });
    assert.deepEqual(result.rustfs[0].tests, report.tests);
    report.tests = [];
    await writeFile(".scratch/rustfs-contract.json", JSON.stringify(report));
    assert.equal(localResults().rustfs[0].failures, 1);
    assert.equal(localResults().rustfs[0].result, "fail");
  } finally {
    process.chdir(previous);
    await rm(temp, { recursive: true, force: true });
  }
});
