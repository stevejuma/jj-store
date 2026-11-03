import assert from "node:assert/strict";
import {
  lstat,
  readFile,
  mkdir,
  writeFile,
  mkdtemp,
  rm,
  chmod,
} from "node:fs/promises";
import { join, resolve, isAbsolute } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import { archiveHash, sourceSha } from "./ci-inventory.mjs";

import {
  payloads,
  lanePayloads,
  uiLanePayloads,
} from "./ci-artifact-layout.mjs";
import {
  isScheduledProducer,
  verifyUiProducer,
  verifyUiSource,
} from "./ci-ui-producer.mjs";
import { isConversationsProducer } from "./ci-conversations-producer.mjs";
import {
  downloadArtifact,
  filesUnder,
  githubApi,
  safeFile,
} from "./ci-artifact-io.mjs";
export { payloads, lanePayloads } from "./ci-artifact-layout.mjs";

export function identity(env = process.env) {
  const value = {
    source: env.CI_SOURCE || sourceSha(),
    repository: env.GITHUB_REPOSITORY,
    run: env.CI_PRODUCER_RUN || env.GITHUB_RUN_ID,
    attempt: env.CI_PRODUCER_ATTEMPT || env.GITHUB_RUN_ATTEMPT,
  };
  assert.match(
    value.source,
    /^[a-f0-9]{40}$/,
    "An exact source SHA is required",
  );
  assert.equal(value.repository, "stevejuma/jj-store");
  for (const field of ["run", "attempt"])
    assert.match(value[field] ?? "", /^[1-9]\d*$/, `Invalid ${field}`);
  return value;
}
export function verifyReceipt(receipt, expected, role) {
  assert.equal(receipt.format, 1);
  for (const key of ["source", "repository", "run", "attempt"])
    assert.equal(receipt[key], expected[key], `Artifact ${key} mismatch`);
  assert.equal(receipt.role, role);
  const names = Object.keys(payloads)
    .filter((name) => payloads[name].role === role)
    .sort();
  assert.deepEqual(
    Object.keys(receipt.payloads).sort(),
    names,
    "Missing or unexpected payload",
  );
  for (const name of names) {
    const files = receipt.payloads[name];
    assert.ok(Object.keys(files).length > 0, "Empty payload");
    if (payloads[name].files)
      assert.deepEqual(
        Object.keys(files).sort(),
        [...payloads[name].files].sort(),
      );
    for (const [path, entry] of Object.entries(files)) {
      assert.ok(
        path &&
          !isAbsolute(path) &&
          !path.split(/[\\/]/).some((part) => ["", ".", ".."].includes(part)),
        "Unsafe artifact path",
      );
      assert.match(entry.sha256, /^[a-f0-9]{64}$/);
      assert.ok([0o644, 0o755].includes(entry.mode));
    }
  }
  return receipt;
}
export function verifyProducer(run, jobs, expected, roles, env = process.env) {
  assert.equal(String(run.id), expected.run);
  assert.equal(
    Number(run.run_attempt),
    Number(expected.attempt),
    "Producer attempt mismatch",
  );
  assert.equal(run.repository.full_name, expected.repository);
  if (run.event === "schedule")
    assert.equal(run.head_sha, expected.source, "Scheduled source mismatch");
  assert.ok(
    run.event === "workflow_dispatch" ||
      isScheduledProducer(run) ||
      isConversationsProducer(run),
    "Unapproved producer event",
  );
  assert.ok(
    [
      ".github/workflows/ci-heavy.yml",
      ".github/workflows/publish-release.yml",
    ].includes(run.path) || isConversationsProducer(run),
    "Unapproved producer workflow",
  );
  for (const leaf of [
    "Resolve qualification source",
    ...(expected.run === env.GITHUB_RUN_ID &&
    expected.attempt === env.GITHUB_RUN_ATTEMPT
      ? []
      : ["Standard checks / required"]),
    ...roles.map((role) => `${role} artifact producer`),
  ]) {
    const found = jobs.filter(
      (job) => job.name === leaf || job.name.endsWith(`/ ${leaf}`),
    );
    assert.equal(found.length, 1, `Missing or ambiguous producer: ${leaf}`);
    assert.equal(
      found[0].conclusion,
      "success",
      `Unsuccessful producer: ${leaf}`,
    );
  }
  // The overall run may have failed an unrelated test. Only completed producers
  // and the standard gate authorize artifact reuse, never a cached test result.
}
export async function verifyPayload(name, entries) {
  const descriptor = payloads[name];
  if (!descriptor.files)
    assert.deepEqual(
      await filesUnder(descriptor.root),
      Object.keys(entries).sort(),
      "Payload inventory mismatch",
    );
  for (const [file, entry] of Object.entries(entries)) {
    const path = await safeFile(descriptor.root, file);
    assert.equal(
      await archiveHash(path),
      entry.sha256,
      `Payload checksum mismatch: ${name}/${file}`,
    );
    await chmod(path, entry.mode);
  }
}
async function main() {
  const [mode, selection] = process.argv.slice(2);
  const expected = identity();
  assert.equal(
    sourceSha(),
    expected.source,
    "Checkout does not match selected source",
  );
  if (mode === "pack") {
    assert.ok(
      [
        "source",
        "build",
        "runtime",
        "assets",
        "ui-source",
        "ui",
        "ui-catalog",
      ].includes(selection),
    );
    assert.equal(expected.run, process.env.GITHUB_RUN_ID);
    assert.equal(expected.attempt, process.env.GITHUB_RUN_ATTEMPT);
    const receipt = { format: 1, ...expected, role: selection, payloads: {} };
    for (const [name, descriptor] of Object.entries(payloads)) {
      if (descriptor.role !== selection) continue;
      const files = descriptor.files || (await filesUnder(descriptor.root));
      receipt.payloads[name] = {};
      for (const file of files) {
        const path = await safeFile(descriptor.root, file);
        receipt.payloads[name][file] = {
          sha256: await archiveHash(path),
          mode: (await lstat(path)).mode & 0o111 ? 0o755 : 0o644,
        };
      }
    }
    verifyReceipt(receipt, expected, selection);
    await mkdir(".release/qualification", { recursive: true });
    await writeFile(
      `.release/qualification/${selection}.json`,
      JSON.stringify(receipt, null, 2) + "\n",
    );
  } else if (mode === "source-ui") {
    assert.equal(selection, "storybook", "Only Storybook is source-only");
    assert.deepEqual(
      uiLanePayloads[selection],
      [],
      "Source lane consumes artifacts",
    );
    const run = await githubApi(
      `actions/runs/${expected.run}/attempts/${expected.attempt}`,
    );
    verifyUiSource(run, expected);
    console.log(`Verified source-only Storybook at ${expected.source}`);
  } else if (mode === "consume" || mode === "consume-ui") {
    const ui = mode === "consume-ui";
    const lanes = ui ? uiLanePayloads : lanePayloads;
    assert.ok(Object.hasOwn(lanes, selection), "Unknown qualification lane");
    const names = lanes[selection];
    if (ui)
      assert.ok(names.length > 0, "Use source-ui for source-only execution");
    const roles = [...new Set(names.map((name) => payloads[name].role))];
    const run = await githubApi(
      `actions/runs/${expected.run}/attempts/${expected.attempt}`,
    );
    const jobs = [];
    for (let page = 1; ; page++) {
      const data = await githubApi(
        `actions/runs/${expected.run}/attempts/${expected.attempt}/jobs?per_page=100&page=${page}`,
      );
      jobs.push(...data.jobs);
      if (data.jobs.length < 100) break;
    }
    if (ui)
      for (const role of roles)
        verifyUiProducer(run, jobs, expected, process.env, role);
    else verifyProducer(run, jobs, expected, roles);
    const temp = await mkdtemp(join(tmpdir(), "jj-qualification-"));
    try {
      for (const role of ui ? roles : ["source", ...roles]) {
        const dir = join(temp, role);
        downloadArtifact(`qualification-${role}`, dir, expected);
        const receipt = verifyReceipt(
          JSON.parse(await readFile(join(dir, `${role}.json`), "utf8")),
          expected,
          role,
        );
        for (const name of names.filter(
          (name) => payloads[name].role === role,
        )) {
          downloadArtifact(name, payloads[name].root, expected);
          await verifyPayload(name, receipt.payloads[name]);
        }
      }
    } finally {
      await rm(temp, { recursive: true, force: true });
    }
    await mkdir(".release/ci-results", { recursive: true });
    await writeFile(
      ".release/ci-results/artifact-verification.json",
      JSON.stringify(
        { ...expected, lane: selection, payloads: names },
        null,
        2,
      ),
    );
    console.log(
      `Verified ${selection} inputs at ${expected.source}, producer ${expected.run}/${expected.attempt}`,
    );
  } else throw Error("Usage: ci-artifacts.mjs pack ROLE | consume LANE");
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await main();
