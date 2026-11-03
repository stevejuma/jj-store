import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { parse as parseYaml } from "yaml";

const workflowRoles = {
  standard: ["ci.yml", "ci-ui.yml"],
  qualification: [
    "ci-health.yml",
    "ci-heavy.yml",
    "ci-heavy-backend.yml",
    "ci-heavy-cold.yml",
    "ci-heavy-contracts.yml",
    "ci-heavy-native.yml",
    "ci-heavy-storage.yml",
    "ci-heavy-ui.yml",
    "ci-qualification-build.yml",
  ],
  cacheExperiment: ["ci-cache-build.yml", "ci-cache-trial.yml"],
  publication: [
    "npm-publish-jj-conversations.yml",
    "npm-publish-jj-history.yml",
    "publish-main-image.yml",
    "publish-release.yml",
    "publish-spec-pages.yml",
    "release.yml",
  ],
};

test("every tracked workflow has an explicit current owner", () => {
  const actual = readdirSync(".github/workflows")
    .filter((name) => name.endsWith(".yml"))
    .sort();
  const owned = Object.values(workflowRoles).flat().sort();
  assert.deepEqual(actual, owned);
  assert.equal(new Set(owned).size, owned.length);
  for (const name of owned) {
    const workflow = parseYaml(
      readFileSync(`.github/workflows/${name}`, "utf8"),
    );
    assert.ok(workflow.on, `${name} has triggers`);
    assert.ok(workflow.jobs, `${name} has jobs`);
  }
  assert.match(
    readFileSync(".github/workflows/release.yml", "utf8"),
    /publish: pnpm release:orchestrate/,
  );
});

test("Pages validates the spec, manual and source Storybook together", () => {
  const source = readFileSync(
    ".github/workflows/publish-spec-pages.yml",
    "utf8",
  );
  const pages = parseYaml(source);
  const render = pages.jobs.build.steps.find(
    (step) => step.name === "Render specification, manual and Storybook",
  );
  assert.equal(render.env.STORYBOOK_BASE_PATH, "/jj-store/storybook/");
  assert.match(
    render.run,
    /pnpm build-storybook --output-dir spec\/book\/storybook/,
  );
  assert.match(render.run, /test -f spec\/book\/storybook\/iframe\.html/);
  for (const trigger of [pages.on.push.paths, pages.on.pull_request.paths])
    for (const path of [".storybook/**", "catalog/**", "web/**"])
      assert.ok(trigger.includes(path), `${path} triggers Pages`);
  const landing = readFileSync("spec/src/index.md", "utf8");
  assert.match(landing, /https:\/\/stevejuma\.github\.io\/jj-store\/manual\//);
  assert.match(
    landing,
    /https:\/\/stevejuma\.github\.io\/jj-store\/storybook\//,
  );
});
test("successful main builds publish bounded source and rolling image tags", () => {
  const main = parseYaml(
    readFileSync(".github/workflows/publish-main-image.yml", "utf8"),
  );
  assert.deepEqual(main.on.workflow_run, {
    workflows: ["PR checks"],
    branches: ["main"],
    types: ["completed"],
  });
  assert.match(main.jobs.source.if, /workflow_run\.conclusion == 'success'/);
  const publication = main.jobs.index.steps.find((step) =>
    step.name?.startsWith("Publish immutable source"),
  );
  assert.match(publication.run, /scripts\/image-tags\.mjs source/);
  assert.match(publication.run, /jj-store:main/);
  assert.match(publication.run, /imagetools inspect/);
  const release = readFileSync(".github/workflows/publish-release.yml", "utf8");
  assert.match(release, /image-tags\.mjs release-architecture/);
  assert.match(release, /RELEASE_TAG: \$\{\{ inputs\.tag \}\}/);
});
test("embedded runtime installs protoc before compiling its Pulsar dependency", () => {
  const { steps } = parseYaml(
    readFileSync(".github/workflows/ci-qualification-build.yml", "utf8"),
  ).jobs.produce;
  const install = steps.findIndex((step) =>
    step.run?.includes(
      "apt-get install --no-install-recommends -y protobuf-compiler",
    ),
  );
  const build = steps.findIndex((step) =>
    step.run?.includes("pnpm build:embedded-rustfs"),
  );
  assert.ok(install >= 0 && install < build);
  assert.equal(steps[install].if, "inputs.kind == 'runtime'");
  assert.match(steps[install].run, /protoc --version/);
});
test("required check rejects missing or failed shared runtime production", () => {
  const jobs = Object.fromEntries(
    [
      "resolve",
      "standard",
      "plan",
      "fast",
      "build",
      "backend",
      "contracts",
      "external",
      "embedded",
      "ui",
      "cold",
      "rust",
      "heavy",
      "runtime",
      "storage",
      "web",
      "assets",
      "browser",
      "native",
    ].map((name) => [name, { result: "success" }]),
  );
  const verify = (jobs) =>
    execFileSync(process.execPath, ["scripts/ci-required.mjs"], {
      env: {
        ...process.env,
        CI_STAGE: "heavy",
        CI_PLAN: JSON.stringify({
          rust: true,
          web: true,
          heavy: true,
          full: true,
        }),
        CI_JOBS: JSON.stringify(jobs),
      },
      stdio: ["ignore", "pipe", "pipe"],
    });
  assert.doesNotThrow(() => verify(jobs));
  assert.throws(
    () => verify({ ...jobs, runtime: { result: "failure" } }),
    /runtime/,
  );
  const { runtime, ...missing } = jobs;
  assert.throws(() => verify(missing), /missing planned CI lane runtime/);
});
test("standard checks never schedule heavy qualification and preserve the protected gate", () => {
  const ci = parseYaml(readFileSync(".github/workflows/ci.yml", "utf8"));
  const { jobs } = ci;
  assert.deepEqual(Object.keys(ci.on).sort(), [
    "pull_request",
    "push",
    "workflow_call",
    "workflow_dispatch",
  ]);
  assert.equal(jobs.heavy, undefined);
  assert.equal(jobs.required.name, "required");
  assert.equal(jobs.required.needs.includes("heavy"), false);
  for (const trigger of ["workflow_dispatch", "workflow_call"]) {
    assert.equal(ci.on[trigger].inputs.full, undefined);
    assert.equal(ci.on[trigger].inputs.all_checks.type, "boolean");
  }
  assert.deepEqual(jobs.tooling.needs, ["plan"]);
  assert.deepEqual(jobs.governance.needs, ["plan"]);
  assert.equal(
    jobs.format.steps.find((step) => step.uses === "./.github/actions/setup")
      .with.capability,
    "format",
  );
  assert.equal(
    jobs.tooling.steps.find((step) => step.uses === "./.github/actions/setup")
      .with.capability,
    "tooling",
  );
  assert.equal(
    jobs.governance.steps.find(
      (step) => step.uses === "./.github/actions/setup",
    ).with.capability,
    "node",
  );
  for (const job of ["build", "targeted_build", "web", "catalog_unit", "ui"])
    assert.deepEqual(jobs[job].needs, ["plan"]);
  assert.equal(jobs.targeted.if, "needs.plan.outputs.backend == 'true'");
  assert.deepEqual(jobs.targeted.needs, ["plan", "targeted_build"]);
  assert.deepEqual(jobs.targeted.strategy.matrix.shard, [1, 2]);
  assert.equal(
    jobs.targeted.env.CI_PACKAGES,
    "${{ needs.plan.outputs.backend_packages }}",
  );
  for (const [id, job] of Object.entries(jobs)) {
    if (id !== "plan") {
      assert.ok(job.needs.includes("plan"));
      for (const step of job.steps ?? []) {
        if (step.uses?.startsWith("actions/checkout@"))
          assert.equal(step.with.ref, "${{ needs.plan.outputs.source }}");
      }
    }
  }
  assert.match(jobs.rust.name, /Standard Rust tests.*matrix.shard/);
  assert.deepEqual(
    jobs.web.steps.find((s) => s.uses === "./.github/actions/setup").with,
    {
      capability: "node",
      "native-dependencies": "false",
      "build-tools": "false",
    },
  );
  for (const domain of [
    "ui_source",
    "ui_browser",
    "help",
    "catalog",
    "contracts",
  ])
    assert.equal(
      jobs.plan.outputs[domain],
      `\${{ steps.plan.outputs.${domain} }}`,
    );
  assert.match(jobs.web.if, /outputs\.contracts/);
  assert.equal(jobs.catalog_unit.if, "needs.plan.outputs.catalog == 'true'");
  assert.match(jobs.ui.if, /outputs\.ui_browser/);
  const setup = parseYaml(
    readFileSync(".github/actions/setup/action.yml", "utf8"),
  );
  const validation = setup.runs.steps.find(
    (step) => step.name === "Validate setup capability",
  );
  for (const capability of ["full", "tooling", "format", "archive", "node"])
    assert.match(validation.run, new RegExp(`\\b${capability}\\b`));
  const rust = setup.runs.steps.find((step) =>
    step.uses?.startsWith("dtolnay/rust-toolchain@"),
  );
  assert.match(rust.with.components, /format.*rustfmt/);
  assert.match(rust.with.components, /full.*rustfmt,clippy/);
});
test("manual and release qualification pin one source and require every lane", () => {
  const ci = parseYaml(readFileSync(".github/workflows/ci-heavy.yml", "utf8"));
  const { jobs } = ci;
  assert.deepEqual(Object.keys(ci.on).sort(), [
    "schedule",
    "workflow_call",
    "workflow_dispatch",
  ]);
  for (const trigger of [ci.on.workflow_call, ci.on.workflow_dispatch])
    assert.deepEqual(Object.keys(trigger.inputs), ["ref"]);
  assert.equal(jobs.standard.uses, "./.github/workflows/ci.yml");
  assert.equal(jobs.standard.with.all_checks, true);
  assert.equal(jobs.standard.with.ref, "${{ needs.resolve.outputs.source }}");
  assert.deepEqual(jobs.build.needs, ["resolve"]);
  for (const name of ["runtime", "contracts", "external", "native"])
    assert.deepEqual(jobs[name].needs, ["resolve"]);
  assert.deepEqual(jobs.assets.needs, ["resolve", "build"]);
  assert.equal(jobs.ui, undefined); // Standard checks already require all UI shards.
  assert.deepEqual(jobs.cold.needs, ["resolve", "assets", "runtime"]);
  assert.deepEqual(jobs.embedded.needs, ["resolve", "runtime"]);
  for (const [name, job] of Object.entries(jobs)) {
    if (["resolve", "standard", "complete", "diagnostics"].includes(name))
      continue;
    assert.equal(job.with.source, "${{ needs.resolve.outputs.source }}");
    assert.equal(job.if, undefined);
    assert.equal(job.needs.includes("backend"), false);
  }
  for (const lane of [
    "backend",
    "contracts",
    "storage",
    "ui",
    "cold",
    "native",
  ]) {
    const workflow = parseYaml(
      readFileSync(`.github/workflows/ci-heavy-${lane}.yml`, "utf8"),
    );
    assert.deepEqual(Object.keys(workflow.on).sort(), [
      "workflow_call",
      "workflow_dispatch",
    ]);
    for (const key of ["source", "producer_run", "producer_attempt"])
      assert.equal(workflow.on.workflow_dispatch.inputs[key].required, true);
    for (const job of Object.values(workflow.jobs)) {
      if (job.strategy) assert.equal(job.strategy["fail-fast"], false);
      assert.ok(
        job.steps.some(
          (step) =>
            step.run?.includes("ci-artifacts.mjs") && /consume/.test(step.run),
        ),
      );
      assert.equal(job.steps.at(-1).if, "always()");
    }
  }
  const release = parseYaml(
    readFileSync(".github/workflows/publish-release.yml", "utf8"),
  );
  assert.equal(release.jobs.qualify.uses, "./.github/workflows/ci-heavy.yml");
  assert.deepEqual(release.jobs.qualify.permissions, {
    contents: "read",
    actions: "read",
  });
  assert.deepEqual(release.jobs.qualify.with, { ref: "${{ inputs.source }}" });
  assert.equal(release.jobs.archives.needs, "qualify");
  assert.equal(
    release.jobs.archives.steps.find((step) =>
      step.uses?.startsWith("actions/download-artifact@"),
    ).with.name,
    "web-bundle-${{ github.run_attempt }}",
  );
  assert.match(
    readFileSync(".config/nextest.toml", "utf8"),
    /\[profile.heavy\][\s\S]*?fail-fast = false[\s\S]*?test-threads = 1/,
  );
  assert.match(
    readFileSync(".github/workflows/release.yml", "utf8"),
    /-f all_checks=true/,
  );
});
test("aggregates reject missing, failed, canceled and unexpectedly skipped prerequisites", () => {
  const verify = (stage, plan, jobs) =>
    execFileSync(process.execPath, ["scripts/ci-required.mjs"], {
      env: {
        ...process.env,
        CI_STAGE: stage,
        CI_PLAN: JSON.stringify(plan),
        CI_JOBS: JSON.stringify(jobs),
      },
      stdio: "pipe",
    });
  const planned = (overrides = {}) => {
    const domains = {
      ui_source: false,
      ui_browser: false,
      help: false,
      catalog: false,
      contracts: false,
      ...Object.fromEntries(
        Object.entries(overrides).filter(([key]) =>
          ["ui_source", "ui_browser", "help", "catalog", "contracts"].includes(
            key,
          ),
        ),
      ),
    };
    return {
      rust: overrides.rust ?? false,
      web: Object.values(domains).some(Boolean),
      backend: overrides.backend ?? false,
      ...domains,
      selection: JSON.stringify({ version: 2, mode: "active", domains }),
    };
  };
  const fullPlan = planned({
    rust: true,
    backend: true,
    ui_source: true,
    ui_browser: true,
    help: true,
    catalog: true,
    contracts: true,
  });
  for (const [stage, names] of [
    ["early", ["plan", "format", "tooling", "governance"]],
    [
      "root",
      [
        "plan",
        "format",
        "tooling",
        "governance",
        "fast",
        "build",
        "rust",
        "targeted_build",
        "targeted",
        "web",
        "catalog_unit",
        "ui",
      ],
    ],
    [
      "heavy",
      [
        "resolve",
        "standard",
        "build",
        "backend",
        "runtime",
        "contracts",
        "external",
        "embedded",
        "assets",
        "cold",
        "native",
      ],
    ],
  ]) {
    const jobs = Object.fromEntries(
      names.map((name) => [name, { result: "success" }]),
    );
    assert.doesNotThrow(() => verify(stage, fullPlan, jobs));
    for (const name of names) {
      for (const result of ["failure", "cancelled", "skipped"])
        assert.throws(() =>
          verify(stage, fullPlan, { ...jobs, [name]: { result } }),
        );
      const missing = { ...jobs };
      delete missing[name];
      assert.throws(() => verify(stage, fullPlan, missing));
    }
    if (stage === "root") {
      assert.throws(() => verify(stage, {}, jobs), /selection/);
      const light = {
        ...jobs,
        build: { result: "skipped" },
        rust: { result: "skipped" },
        web: { result: "skipped" },
        catalog_unit: { result: "skipped" },
        ui: { result: "skipped" },
        targeted_build: { result: "skipped" },
        targeted: { result: "skipped" },
      };
      assert.doesNotThrow(() => verify(stage, planned(), light));
      assert.throws(() =>
        verify(stage, planned(), { ...light, rust: { result: "cancelled" } }),
      );
      const catalogOnly = {
        ...light,
        catalog_unit: { result: "success" },
        ui: { result: "success" },
      };
      assert.doesNotThrow(() =>
        verify(stage, planned({ catalog: true }), catalogOnly),
      );
      assert.throws(() => verify(stage, { ...fullPlan, web: false }, jobs));
      assert.throws(() =>
        verify(
          stage,
          { ...fullPlan, selection: JSON.stringify({ version: 1 }) },
          jobs,
        ),
      );
      assert.throws(() => verify(stage, { ...fullPlan, catalog: false }, jobs));
    }
  }
});
