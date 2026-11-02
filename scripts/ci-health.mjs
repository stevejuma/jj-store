import assert from "node:assert/strict";
import { appendFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";

// A successful partial/manual run cannot refresh scheduled full validation.
export function nightlyHealth(runs, now = Date.now()) {
  const canonical = runs.filter(
    (run) =>
      run.event === "schedule" &&
      run.head_branch === "main" &&
      run.repository?.full_name === "stevejuma/jj-store" &&
      run.path === ".github/workflows/ci-heavy.yml" &&
      Number.isFinite(Date.parse(run.created_at)) &&
      Date.parse(run.created_at) <= now,
  );
  canonical.sort((a, b) => Date.parse(b.created_at) - Date.parse(a.created_at));
  const attempt = canonical[0];
  const success = canonical.find(
    (run) =>
      run.status === "completed" &&
      run.conclusion === "success" &&
      run.complete === true,
  );
  const age = (run, field) =>
    run && Number.isFinite(Date.parse(run[field]))
      ? (now - Date.parse(run[field])) / 3600000
      : Infinity;
  const issues = [];
  if (age(attempt, "created_at") > 30)
    issues.push("No scheduled attempt within 30 hours");
  if (age(success, "updated_at") > 48)
    issues.push("No complete scheduled success within 48 hours");
  if (attempt?.status === "completed" && attempt.conclusion !== "success")
    issues.push(`Latest scheduled attempt: ${attempt.conclusion}`);
  if (attempt?.conclusion === "success" && attempt.complete !== true)
    issues.push("Latest success has incomplete qualification evidence");
  return {
    healthy: issues.length === 0,
    issues,
    attempt: attempt?.html_url ?? null,
    success: success?.html_url ?? null,
    source: attempt?.head_sha ?? null,
  };
}

async function main() {
  assert.equal(process.env.GITHUB_REPOSITORY, "stevejuma/jj-store");
  const root = `${process.env.GITHUB_API_URL || "https://api.github.com"}/repos/stevejuma/jj-store`;
  const api = async (path) => {
    const response = await fetch(`${root}/${path}`, {
      headers: {
        Authorization: `Bearer ${process.env.GH_TOKEN}`,
        Accept: "application/vnd.github+json",
      },
      signal: AbortSignal.timeout(15000),
    });
    if (!response.ok)
      throw Error(`Nightly evidence unavailable: HTTP ${response.status}`);
    return response.json();
  };
  const runs = [];
  // A bounded freshness window avoids treating old history as current health.
  for (let page = 1; page <= 5; page++) {
    const data = await api(
      `actions/workflows/ci-heavy.yml/runs?event=schedule&branch=main&per_page=100&page=${page}`,
    );
    runs.push(...data.workflow_runs);
    if (data.workflow_runs.length < 100) break;
  }
  for (const run of runs.filter(
    (r) =>
      r.conclusion === "success" &&
      Date.now() - Date.parse(r.updated_at) <= 48 * 3600000,
  )) {
    const jobs = [];
    for (let page = 1; ; page++) {
      const data = await api(
        `actions/runs/${run.id}/attempts/${run.run_attempt}/jobs?per_page=100&page=${page}`,
      );
      jobs.push(...data.jobs);
      if (data.jobs.length < 100) break;
    }
    const gates = jobs.filter(
      (job) => job.name === "Complete qualification gate",
    );
    run.complete = gates.length === 1 && gates[0].conclusion === "success";
  }
  const health = nightlyHealth(runs);
  const summary = [
    "## Nightly validation health",
    "",
    `Source: ${health.source ?? "unknown"}`,
    `Latest attempt: ${health.attempt ?? "missing"}`,
    `Last complete success: ${health.success ?? "missing"}`,
    ...health.issues,
    "",
  ].join("\n");
  console.log(summary);
  if (process.env.GITHUB_STEP_SUMMARY)
    appendFileSync(process.env.GITHUB_STEP_SUMMARY, summary);
  if (!health.healthy) process.exitCode = 1;
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await main();
