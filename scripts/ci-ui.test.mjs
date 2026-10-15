import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { parse } from "yaml";
import { verifyUiProducer } from "./ci-ui-producer.mjs";
import { verifyProducer, verifyReceipt } from "./ci-artifacts.mjs";
import { payloads, uiLanePayloads } from "./ci-artifact-layout.mjs";

const expected = {
  source: "a".repeat(40),
  repository: "stevejuma/jj-store",
  run: "12",
  attempt: "2",
};
const env = { GITHUB_RUN_ID: "12", GITHUB_RUN_ATTEMPT: "2" };
const run = {
  id: 12,
  run_attempt: 2,
  repository: { full_name: expected.repository },
  event: "pull_request",
  path: ".github/workflows/ci.yml",
};
const jobs = [
  { name: "UI acceptance / UI acceptance inputs", conclusion: "success" },
];

test("UI inputs admit only successful same-run producers and never qualify release reuse", () => {
  assert.doesNotThrow(() => verifyUiProducer(run, jobs, expected, env));
  for (const changed of [
    { ...expected, run: "13" },
    { ...expected, attempt: "3" },
    { ...expected, repository: "someone/else" },
  ])
    assert.throws(() => verifyUiProducer(run, jobs, changed, env));
  for (const conclusion of ["failure", "cancelled", "skipped", null])
    assert.throws(() =>
      verifyUiProducer(run, [{ ...jobs[0], conclusion }], expected, env),
    );
  for (const candidates of [[], [...jobs, ...jobs]])
    assert.throws(() => verifyUiProducer(run, candidates, expected, env));
  assert.throws(() =>
    verifyUiProducer(
      { ...run, path: ".github/workflows/unknown.yml" },
      jobs,
      expected,
      env,
    ),
  );
  assert.throws(() => verifyProducer(run, jobs, expected, ["ui"]));
  assert.throws(() =>
    verifyProducer(
      {
        ...run,
        event: "workflow_dispatch",
        path: ".github/workflows/ci-ui.yml",
      },
      jobs,
      expected,
      ["ui"],
    ),
  );
});

test("UI receipts require complete source-bound payload inventories", () => {
  const receipt = { format: 1, ...expected, role: "ui", payloads: {} };
  for (const [name, descriptor] of Object.entries(payloads).filter(
    ([, d]) => d.role === "ui",
  ))
    receipt.payloads[name] = Object.fromEntries(
      (descriptor.files ?? ["index.html"]).map((file) => [
        file,
        { sha256: "b".repeat(64), mode: 0o644 },
      ]),
    );
  assert.doesNotThrow(() => verifyReceipt(receipt, expected, "ui"));
  assert.throws(() =>
    verifyReceipt({ ...receipt, source: "c".repeat(40) }, expected, "ui"),
  );
  assert.throws(() =>
    verifyReceipt({ ...receipt, attempt: "1" }, expected, "ui"),
  );
  const missing = structuredClone(receipt);
  delete missing.payloads["ui-tools"].browser_preview;
  assert.throws(() => verifyReceipt(missing, expected, "ui"));
  assert.deepEqual(uiLanePayloads.browser, [
    "ui-tools",
    "ui-fixtures",
    "ui-web",
  ]);
  assert.deepEqual(uiLanePayloads.help, uiLanePayloads.browser);
});

test("core UI follows preflight, gates PRs and can run alone without full qualification", () => {
  const workflow = (name) =>
    parse(readFileSync(`.github/workflows/${name}.yml`, "utf8"));
  const standard = workflow("ci").jobs,
    ui = workflow("ci-ui");
  assert.deepEqual(standard.ui.needs, ["plan", "fast"]);
  assert.equal(standard.ui.if, "needs.plan.outputs.web == 'true'");
  assert.equal(standard.ui.uses, "./.github/workflows/ci-ui.yml");
  assert.ok(standard.required.needs.includes("ui"));
  assert.deepEqual(Object.keys(ui.on).sort(), [
    "workflow_call",
    "workflow_dispatch",
  ]);
  assert.equal(ui.on.workflow_dispatch.inputs.source.required, true);
  assert.equal(ui.jobs.tests.needs, "prepare");
  assert.equal(ui.jobs.tests.with.input_profile, "ui");
  assert.equal(ui.jobs.tests.uses, "./.github/workflows/ci-heavy-ui.yml");
  assert.equal(
    Object.values(ui.jobs).some((job) => job.uses?.endsWith("/ci-heavy.yml")),
    false,
  );
  const heavy = workflow("ci-heavy").jobs;
  assert.equal(heavy.ui, undefined);
  assert.equal(heavy.standard.with.all_checks, true);
  assert.ok(heavy.complete.needs.includes("standard"));
  assert.equal(workflow("publish-release").jobs.archives.needs, "qualify");
  const build = readFileSync("scripts/ci-ui-build.mjs", "utf8");
  assert.match(build, /browser_preview/);
  assert.match(build, /docs_reference/);
  assert.match(build, /browser-fixtures/);
  assert.doesNotMatch(build, /ci-build|nextest|embedded-rustfs/);
});
