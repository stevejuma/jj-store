import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { parse as parseYaml } from "yaml";
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
  assert.deepEqual(jobs.tooling.needs, ["plan", "format"]);
  assert.deepEqual(jobs.governance.needs, ["plan", "format"]);
  for (const job of ["build", "web"])
    assert.ok(jobs[job].needs.includes("fast"));
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
      "native-dependencies": "false",
      "build-tools": "false",
    },
  );
});
test("manual and release qualification pin one source and require every lane", () => {
  const ci = parseYaml(readFileSync(".github/workflows/ci-heavy.yml", "utf8"));
  const { jobs } = ci;
  assert.deepEqual(Object.keys(ci.on).sort(), [
    "workflow_call",
    "workflow_dispatch",
  ]);
  for (const trigger of Object.values(ci.on))
    assert.deepEqual(Object.keys(trigger.inputs), ["ref"]);
  assert.equal(jobs.standard.uses, "./.github/workflows/ci.yml");
  assert.equal(jobs.standard.with.all_checks, true);
  assert.equal(jobs.standard.with.ref, "${{ needs.resolve.outputs.source }}");
  assert.ok(jobs.build.needs.includes("standard"));
  for (const name of ["runtime", "contracts", "external", "native"])
    assert.ok(jobs[name].needs.includes("standard"));
  assert.deepEqual(jobs.assets.needs, ["resolve", "build"]);
  assert.deepEqual(jobs.ui.needs, ["resolve", "assets"]);
  assert.deepEqual(jobs.cold.needs, ["resolve", "assets", "runtime"]);
  assert.deepEqual(jobs.embedded.needs, ["resolve", "runtime"]);
  for (const [name, job] of Object.entries(jobs)) {
    if (["resolve", "standard", "complete"].includes(name)) continue;
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
        job.steps.some((step) =>
          step.run?.includes("ci-artifacts.mjs consume"),
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
        "web",
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
        "ui",
        "cold",
        "native",
      ],
    ],
  ]) {
    const jobs = Object.fromEntries(
      names.map((name) => [name, { result: "success" }]),
    );
    assert.doesNotThrow(() => verify(stage, { rust: true, web: true }, jobs));
    for (const name of names) {
      for (const result of ["failure", "cancelled", "skipped"])
        assert.throws(() =>
          verify(
            stage,
            { rust: true, web: true },
            { ...jobs, [name]: { result } },
          ),
        );
      const missing = { ...jobs };
      delete missing[name];
      assert.throws(() => verify(stage, { rust: true, web: true }, missing));
    }
    if (stage === "root") {
      assert.throws(() => verify(stage, {}, jobs), /planned selection/);
      const light = {
        ...jobs,
        build: { result: "skipped" },
        rust: { result: "skipped" },
        web: { result: "skipped" },
      };
      assert.doesNotThrow(() =>
        verify(stage, { rust: false, web: false }, light),
      );
      assert.throws(() =>
        verify(
          stage,
          { rust: false, web: false },
          { ...light, rust: { result: "cancelled" } },
        ),
      );
    }
  }
});
