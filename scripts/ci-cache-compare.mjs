import assert from "node:assert/strict";
import { readFileSync, writeFileSync, appendFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
const median = (values) => [...values].sort((a, b) => a - b)[1];
export function compareSamples(samples, jobs, source) {
  assert.equal(samples.length, 6, "Require exactly three samples per backend");
  const reference = samples[0],
    rows = [];
  for (const backend of ["baseline", "sccache"])
    for (const iteration of [1, 2, 3]) {
      const sample = `${backend}-${iteration}`,
        matches = samples.filter((s) => s.sample === sample);
      assert.equal(matches.length, 1, `Missing or duplicate ${sample}`);
      const data = matches[0];
      assert.equal(data.source, source);
      assert.equal(data.cache_hit, "true", "Warm dependency cache required");
      for (const field of ["packages", "tests", "runnable", "partitions"])
        assert.deepEqual(
          data[field],
          reference[field],
          `Changed inventory: ${sample}`,
        );
      assert.ok(data.tests.length > 0 && data.runnable.length > 0);
      assert.ok(Number.isFinite(data.build_seconds) && data.build_seconds > 0);
      const builds = jobs.filter((j) =>
        j.name.endsWith(`Cache build ${sample}`),
      );
      assert.equal(
        builds.length,
        1,
        `Missing or duplicate build job ${sample}`,
      );
      assert.equal(builds[0].conclusion, "success");
      const job_seconds =
        (Date.parse(builds[0].completed_at) -
          Date.parse(builds[0].started_at)) /
        1000;
      assert.ok(Number.isFinite(job_seconds) && job_seconds > 0);
      rows.push({
        sample,
        backend,
        iteration,
        job_seconds,
        build_seconds: data.build_seconds,
      });
    }
  for (const sample of ["baseline-1", "sccache-1"]) {
    const tests = jobs.filter((j) =>
      j.name.endsWith(`Verify cached build ${sample}`),
    );
    assert.equal(tests.length, 1, `Missing validation ${sample}`);
    assert.equal(tests[0].conclusion, "success", `Failed validation ${sample}`);
  }
  const baseline = median(
    rows.filter((r) => r.backend === "baseline").map((r) => r.job_seconds),
  );
  const sccache = median(
    rows.filter((r) => r.backend === "sccache").map((r) => r.job_seconds),
  );
  const improvement = (baseline - sccache) / baseline;
  return {
    source,
    rows,
    baseline_median_seconds: baseline,
    sccache_median_seconds: sccache,
    improvement,
    retain_sccache: improvement >= 0.1,
    package_count: reference.packages.length,
    test_count: reference.tests.length,
    runnable_count: reference.runnable.length,
  };
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(process.argv[1]).href
) {
  const upstream = JSON.parse(process.env.CI_TRIAL_JOBS);
  for (const name of ["seed", "samples"])
    assert.equal(
      upstream[name]?.result,
      "success",
      `Incomplete cache trial ${name}`,
    );
  const samples = ["baseline", "sccache"].flatMap((backend) =>
    [1, 2, 3].map((i) =>
      JSON.parse(
        readFileSync(`.release/cache-trial/${backend}-${i}.json`, "utf8"),
      ),
    ),
  );
  const jobs = [];
  for (let page = 1; ; page++) {
    const response = await fetch(
      `${process.env.GITHUB_API_URL}/repos/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}/attempts/${process.env.GITHUB_RUN_ATTEMPT}/jobs?per_page=100&page=${page}`,
      {
        headers: {
          Authorization: `Bearer ${process.env.GH_TOKEN}`,
          Accept: "application/vnd.github+json",
        },
        signal: AbortSignal.timeout(15000),
      },
    );
    if (!response.ok) throw Error(`Jobs API returned ${response.status}`);
    const data = await response.json();
    jobs.push(...data.jobs);
    if (data.jobs.length < 100) break;
  }
  const result = compareSamples(samples, jobs, process.env.CI_SOURCE);
  writeFileSync(
    ".release/cache-trial/decision.json",
    JSON.stringify(result, null, 2) + "\n",
  );
  const report = `## Compilation cache comparison\n\nSource: \`${result.source}\`. Six warm builds; identical inventories (${result.package_count} packages, ${result.test_count} test identities). Both baseline and cached-build tests passed.\n\n| Sample | Complete build job | Compile/lint/doctest command |\n| --- | ---: | ---: |\n${result.rows.map((r) => `| ${r.sample} | ${r.job_seconds.toFixed(1)}s | ${r.build_seconds.toFixed(1)}s |`).join("\n")}\n\nMedian build-job time: baseline ${result.baseline_median_seconds}s; sccache ${result.sccache_median_seconds}s. Improvement: ${(result.improvement * 100).toFixed(2)}%. Decision: ${result.retain_sccache ? "meets the 10% adoption threshold" : "retain rust-cache alone"}.\n`;
  console.log(report);
  appendFileSync(process.env.GITHUB_STEP_SUMMARY, report);
}
