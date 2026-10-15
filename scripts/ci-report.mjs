import {
  appendFileSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  existsSync,
  writeFileSync,
} from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { sourceSha } from "./ci-inventory.mjs";

export function secondsBetween(start, end) {
  if (
    !start ||
    !end ||
    !Number.isFinite(Date.parse(start)) ||
    !Number.isFinite(Date.parse(end))
  )
    return null;
  const duration = (Date.parse(end) - Date.parse(start)) / 1000;
  return duration < 0 ? null : duration;
}
export function summarizeJobs(jobs) {
  return jobs.map((job) => ({
    id: job.id,
    name: job.name,
    status: job.status,
    conclusion: job.conclusion,
    runner: job.runner_name,
    labels: job.labels,
    created_at: job.created_at ?? null,
    started_at: job.started_at,
    completed_at: job.completed_at,
    queue_seconds: secondsBetween(job.created_at, job.started_at),
    // GitHub does not always expose dependency-ready timestamps. Never label
    // time since workflow dispatch as runner queueing.
    dependency_wait_seconds: null,
    execution_seconds: secondsBetween(job.started_at, job.completed_at),
    steps: (job.steps || []).map((step) => ({
      name: step.name,
      conclusion: step.conclusion,
      seconds: secondsBetween(step.started_at, step.completed_at),
    })),
  }));
}
export function junitCases(xml) {
  return [
    ...xml.matchAll(/<testcase\b([^>]*?)(?:\/>|>([\s\S]*?)<\/testcase>)/g),
  ].map(([, attrs, body = ""]) => {
    const values = Object.fromEntries(
      [...attrs.matchAll(/([\w:-]+)="([^"]*)"/g)].map(([, k, v]) => [k, v]),
    );
    return {
      name: values.name,
      class: values.classname,
      seconds: values.time === undefined ? null : Number(values.time),
      result: /<(failure|error)\b/.test(body)
        ? "failed"
        : /<skipped\b/.test(body)
          ? "skipped"
          : "passed",
    };
  });
}
export function localResults() {
  const reports = [],
    libtest = [];
  for (const dir of [".release/ci-results", "target/nextest/heavy"]) {
    if (!existsSync(dir)) continue;
    for (const name of readdirSync(dir).filter((name) =>
      name.endsWith(".xml"),
    )) {
      const path = join(dir, name),
        cases = junitCases(readFileSync(path, "utf8"));
      reports.push({
        path,
        counts: {
          total: cases.length,
          passed: cases.filter((t) => t.result === "passed").length,
          failed: cases.filter((t) => t.result === "failed").length,
          skipped: cases.filter((t) => t.result === "skipped").length,
        },
        slowest: cases
          .sort((a, b) => (b.seconds || 0) - (a.seconds || 0))
          .slice(0, 20),
      });
    }
  }
  // Keep native libtest commands unchanged; report explicit summaries without
  // inventing per-test timings for commands that did not emit them.
  for (const name of readdirSync(".release/ci-results").filter((name) =>
    name.endsWith(".log"),
  )) {
    const log = readFileSync(join(".release/ci-results", name), "utf8");
    const matches = [
      ...log.matchAll(
        /test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;/g,
      ),
    ];
    if (matches.length)
      libtest.push({
        path: name,
        counts: matches.reduce(
          (sum, [, p, f, s]) => ({
            passed: sum.passed + Number(p),
            failed: sum.failed + Number(f),
            skipped: sum.skipped + Number(s),
          }),
          { passed: 0, failed: 0, skipped: 0 },
        ),
      });
  }
  const storagePath = ".scratch/rustfs-contract.json";
  const storage = existsSync(storagePath)
    ? JSON.parse(readFileSync(storagePath, "utf8"))
    : null;
  const rustfs = storage
    ? [
        {
          path: storagePath,
          result: storage.result,
          failures: storage.failures,
          counts: {
            total: storage.tests.length,
            passed: storage.tests.filter((t) => t.result === "pass").length,
            failed: storage.tests.filter((t) => t.result === "fail").length,
            skipped: 0,
          },
          tests: storage.tests,
        },
      ]
    : [];
  return {
    junit: reports,
    libtest,
    rustfs,
    test_counts_available: reports.length + libtest.length + rustfs.length > 0,
  };
}
async function main() {
  mkdirSync(".release/ci-results", { recursive: true });
  const report = {
    format: 1,
    source: process.env.CI_SOURCE || sourceSha(),
    run: process.env.GITHUB_RUN_ID,
    attempt: process.env.GITHUB_RUN_ATTEMPT,
    suite: process.env.CI_REPORT_NAME || "standard",
    runner: {
      os: process.env.RUNNER_OS,
      arch: process.env.RUNNER_ARCH,
      name: process.env.RUNNER_NAME,
    },
    producer_run: process.env.CI_PRODUCER_RUN || null,
    producer_attempt: process.env.CI_PRODUCER_ATTEMPT || null,
    results: localResults(),
    jobs: [],
    timing_error: null,
  };
  try {
    const root = `${process.env.GITHUB_API_URL || "https://api.github.com"}/repos/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}/attempts/${process.env.GITHUB_RUN_ATTEMPT}/jobs`;
    const jobs = [];
    for (let page = 1; ; page++) {
      const response = await fetch(`${root}?per_page=100&page=${page}`, {
        headers: {
          Authorization: `Bearer ${process.env.GH_TOKEN}`,
          Accept: "application/vnd.github+json",
        },
        signal: AbortSignal.timeout(15000),
      });
      if (!response.ok) throw Error(`Jobs API returned ${response.status}`);
      const data = await response.json();
      jobs.push(...data.jobs);
      if (data.jobs.length < 100) break;
    }
    report.jobs = summarizeJobs(jobs);
  } catch (error) {
    report.timing_error = error.message;
    console.warn(`CI timing report unavailable: ${error.message}`);
  }
  report.runner_minutes = report.jobs.length
    ? report.jobs.reduce((sum, job) => sum + (job.execution_seconds || 0), 0) /
      60
    : null;
  report.runner_minutes_complete =
    !report.timing_error &&
    report.jobs.length > 0 &&
    report.jobs.every(
      (job) =>
        ["skipped", "cancelled"].includes(job.conclusion) ||
        job.execution_seconds !== null,
    );
  writeFileSync(
    ".release/ci-results/timing.json",
    JSON.stringify(report, null, 2) + "\n",
  );
  const show = (value) => (value === null ? "unknown" : `${value.toFixed(1)}s`);
  const lines = [
    "## CI timings",
    "",
    `Source: \`${report.source}\`. Run/attempt: ${report.run}/${report.attempt}.`,
    "",
    "Execution includes setup, cache and transfers. Queue time is unknown when GitHub omits job creation; dependency waiting is not queue time. Runner minutes sum parallel execution, not elapsed time or cost. Running jobs and post-job cache saves may be incomplete in this snapshot.",
    "",
    "| Job | Execution | Queue | Result |",
    "| --- | ---: | ---: | --- |",
  ];
  for (const job of report.jobs) {
    lines.push(
      `| ${job.name.replaceAll("|", "\\|")} | ${show(job.execution_seconds)} | ${show(job.queue_seconds)} | ${job.conclusion ?? job.status} |`,
    );
    for (const step of job.steps.filter((step) =>
      /cache|artifact|archive|build|compile|test|acceptance|contract|setup/i.test(
        step.name,
      ),
    ))
      lines.push(
        `| ↳ ${step.name.replaceAll("|", "\\|")} | ${show(step.seconds)} | | ${step.conclusion ?? "pending"} |`,
      );
  }
  for (const result of [...report.results.junit, ...report.results.rustfs])
    lines.push(
      "",
      `Tests (${result.path}): ${result.counts.passed} passed, ${result.counts.failed} failed, ${result.counts.skipped} skipped.`,
      ...(result.slowest || result.tests)
        .slice(0, 5)
        .map((t) => `- ${t.name}: ${show(t.seconds)} (${t.result})`),
    );
  if (!report.results.test_counts_available)
    lines.push(
      "",
      "No structured test counts were produced by this job; consult its command logs. Missing reports do not establish a pass.",
    );
  if (process.env.GITHUB_STEP_SUMMARY)
    appendFileSync(process.env.GITHUB_STEP_SUMMARY, lines.join("\n") + "\n");
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await main();
