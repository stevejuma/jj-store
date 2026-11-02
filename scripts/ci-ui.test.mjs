import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { parse } from "yaml";
import { verifyUiProducer, verifyUiSource } from "./ci-ui-producer.mjs";
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

test("Conversations qualification admits canonical main or scoped-tag UI producers", () => {
  const npmRun = {
    ...run,
    event: "workflow_dispatch",
    path: ".github/workflows/npm-publish-jj-conversations.yml",
    head_branch: "main",
  };
  assert.doesNotThrow(() => verifyUiProducer(npmRun, jobs, expected, env));
  assert.doesNotThrow(() =>
    verifyUiProducer(
      { ...npmRun, event: "push", head_branch: "jj-conversations-v0.1.0" },
      jobs,
      expected,
      env,
    ),
  );
  for (const changed of [
    { event: "pull_request" },
    { head_branch: "feature" },
    { event: "push", head_branch: "main" },
    { event: "push", head_branch: "v0.1.0" },
    { path: ".github/workflows/npm-publish-jj-history.yml" },
  ])
    assert.throws(() =>
      verifyUiProducer({ ...npmRun, ...changed }, jobs, expected, env),
    );
  assert.throws(() =>
    verifyUiProducer(npmRun, jobs, { ...expected, run: "13" }, env),
  );
  assert.throws(() =>
    verifyUiProducer(
      npmRun,
      [{ ...jobs[0], conclusion: "failure" }],
      expected,
      env,
    ),
  );
});

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

test("core UI runs alongside preflight, gates PRs and can run alone without full qualification", () => {
  const workflow = (name) =>
    parse(readFileSync(`.github/workflows/${name}.yml`, "utf8"));
  const standard = workflow("ci").jobs,
    ui = workflow("ci-ui");
  assert.deepEqual(standard.ui.needs, ["plan"]);
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
  assert.equal(ui.jobs.tests.with.suite, "application");
  assert.equal(ui.jobs.storybook.needs, undefined);
  assert.equal(ui.jobs.storybook.with.suite, "source");
  assert.equal(ui.jobs.catalog.needs, undefined);
  assert.equal(ui.jobs["catalog-tests"].needs, "catalog");
  assert.equal(ui.jobs["catalog-tests"].with.suite, "catalog");
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

test("source Storybook admission does not admit missing artifact producers", () => {
  assert.doesNotThrow(() => verifyUiSource(run, expected, env));
  for (const role of ["ui", "ui-catalog"]) {
    assert.throws(() => verifyUiProducer(run, [], expected, env, role));
    assert.throws(() => verifyUiProducer(run, jobs, expected, env, "unknown"));
  }
  const catalog = [
    { name: "UI acceptance / Static catalog inputs", conclusion: "success" },
  ];
  assert.doesNotThrow(() =>
    verifyUiProducer(run, catalog, expected, env, "ui-catalog"),
  );
  assert.throws(() => verifyUiProducer(run, jobs, expected, env, "ui-catalog"));
  assert.throws(() => verifyUiProducer(run, catalog, expected, env, "ui"));
  for (const conclusion of ["failure", "cancelled", "skipped", null])
    assert.throws(() =>
      verifyUiProducer(
        run,
        [{ ...catalog[0], conclusion }],
        expected,
        env,
        "ui-catalog",
      ),
    );
  assert.throws(() => verifyUiSource(run, { ...expected, attempt: "1" }, env));
  assert.throws(() =>
    verifyUiSource({ ...run, event: "schedule" }, expected, env),
  );
});

test("UI partitions preserve every original lane once and source jobs use Node setup", () => {
  const ui = parse(readFileSync(".github/workflows/ci-heavy-ui.yml", "utf8"));
  const expression = ui.jobs.tests.strategy.matrix.include;
  const arrays = [...expression.matchAll(/'(\[.*?\])'/g)].map((m) =>
    JSON.parse(m[1]),
  );
  assert.equal(arrays.length, 4);
  const identity = (lane) => JSON.stringify([lane.lane, lane.shard ?? null]);
  const partitioned = arrays.slice(0, 3).flat().map(identity).sort();
  assert.deepEqual(partitioned, arrays[3].map(identity).sort());
  assert.equal(new Set(partitioned).size, 7);
  const steps = ui.jobs.tests.steps;
  assert.equal(
    steps.find((s) => s.run?.includes("source-ui storybook")).if,
    "matrix.lane == 'storybook' && inputs.input_profile == 'ui'",
  );
  assert.equal(
    steps.find((s) => s.run?.includes("consume-ui")).if,
    "matrix.lane != 'storybook' || inputs.input_profile != 'ui'",
  );
  assert.equal(
    steps.find((s) => s.uses === "./.github/actions/setup").with.capability,
    "node",
  );
  const setup = parse(readFileSync(".github/actions/setup/action.yml", "utf8"));
  for (const step of setup.runs.steps.filter(
    (s) =>
      /rust-toolchain|setup-python|install-action/.test(s.uses ?? "") ||
      /ci-lapis-source|ci-redaction-source|ci-setup-jj|cargo fetch|ci-provision|shared AI/.test(
        (s.run ?? "") + (s.name ?? ""),
      ),
  ))
    assert.match(
      step.if,
      /inputs.capability (?:== '(?:full|archive)'|!= 'node')/,
    );
});
