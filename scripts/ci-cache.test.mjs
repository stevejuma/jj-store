import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { parse } from "yaml";
import { compareSamples } from "./ci-cache-compare.mjs";
function fixtures(cached = [80, 85, 89]) {
  const samples = [],
    jobs = [];
  for (const backend of ["baseline", "sccache"])
    for (const i of [1, 2, 3]) {
      const sample = `${backend}-${i}`,
        seconds =
          backend === "baseline" ? [95, 100, 105][i - 1] : cached[i - 1];
      samples.push({
        sample,
        source: "source",
        cache_hit: "true",
        packages: ["p"],
        tests: ["a"],
        runnable: ["a"],
        partitions: [["a"], [], [], []],
        build_seconds: seconds - 10,
      });
      jobs.push({
        name: `nested / Cache build ${sample}`,
        conclusion: "success",
        started_at: new Date(0).toISOString(),
        completed_at: new Date(seconds * 1000).toISOString(),
      });
    }
  for (const sample of ["baseline-1", "sccache-1"])
    jobs.push({
      name: `nested / Verify cached build ${sample}`,
      conclusion: "success",
    });
  return { samples, jobs };
}
test("cache adoption requires at least ten percent median complete-job improvement", () => {
  let f = fixtures();
  assert.equal(
    compareSamples(f.samples, f.jobs, "source").retain_sccache,
    true,
  );
  f = fixtures([89, 90, 91]);
  assert.equal(
    compareSamples(f.samples, f.jobs, "source").retain_sccache,
    true,
  );
  f = fixtures([90, 91, 92]);
  assert.equal(
    compareSamples(f.samples, f.jobs, "source").retain_sccache,
    false,
  );
});
test("cache comparisons reject missing samples cold caches changed inventories sources and failed tests", () => {
  let f = fixtures();
  assert.throws(() => compareSamples(f.samples.slice(1), f.jobs, "source"));
  for (const mutation of [
    { cache_hit: "false" },
    { source: "other" },
    { packages: ["other"] },
    { tests: [] },
    { partitions: [] },
    { build_seconds: 0 },
  ]) {
    f = fixtures();
    Object.assign(f.samples[1], mutation);
    assert.throws(() => compareSamples(f.samples, f.jobs, "source"));
  }
  f = fixtures();
  f.jobs.at(-1).conclusion = "failure";
  assert.throws(() => compareSamples(f.samples, f.jobs, "source"));
  f = fixtures();
  f.jobs.pop();
  assert.throws(() => compareSamples(f.samples, f.jobs, "source"));
});
test("cache trial is opt-in branch-only bounded and uses isolated read-only measured caches", () => {
  const ci = parse(readFileSync(".github/workflows/ci.yml", "utf8"));
  assert.equal(ci.on.workflow_dispatch.inputs.cache_trial.default, false);
  assert.match(
    ci.jobs.cache_trial.if,
    /workflow_dispatch.*inputs.cache_trial.*github.ref != 'refs\/heads\/main'/,
  );
  assert.deepEqual(ci.jobs.cache_trial.needs, ["plan", "required"]);
  assert.equal(ci.jobs.required.needs.includes("cache_trial"), false);
  const trial = parse(
    readFileSync(".github/workflows/ci-cache-trial.yml", "utf8"),
  );
  assert.deepEqual(Object.keys(trial.on), ["workflow_call"]);
  assert.deepEqual(trial.jobs.samples.strategy.matrix, {
    backend: ["baseline", "sccache"],
    run: [1, 2, 3],
  });
  const worker = parse(
    readFileSync(".github/workflows/ci-cache-build.yml", "utf8"),
  );
  assert.equal(worker.env.CARGO_INCREMENTAL, "0");
  const cache = worker.jobs.build.steps.find((s) =>
    s.uses?.startsWith("Swatinem/"),
  );
  assert.equal(cache.with["save-if"], "${{ inputs.seed }}");
  assert.match(cache.with["shared-key"], /trial-.*inputs.source/);
  const compile = worker.jobs.build.steps.find(
    (s) => s.run === "node scripts/ci-cache-sample.mjs build",
  );
  assert.equal(
    compile.env.SCCACHE_GHA_RW_MODE,
    "${{ inputs.seed && 'READ_WRITE' || 'READ_ONLY' }}",
  );
  const action = worker.jobs.build.steps.find((s) =>
    s.uses?.startsWith("mozilla-actions/"),
  );
  assert.match(action.uses, /@[0-9a-f]{40}$/);
  assert.equal(action.with.version, "v0.18.0");
});
