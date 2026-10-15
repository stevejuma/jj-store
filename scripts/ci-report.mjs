import { appendFileSync } from "node:fs";
// Fetch after build/test producers finish so post-job cache-save time is visible.
// Observability is best effort and cannot turn a failed required gate green.
try {
  const root = `${process.env.GITHUB_API_URL}/repos/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}/attempts/${process.env.GITHUB_RUN_ATTEMPT}/jobs`;
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
  const duration = (item) =>
    item.started_at && item.completed_at
      ? `${((Date.parse(item.completed_at) - Date.parse(item.started_at)) / 1000).toFixed(1)}s`
      : "pending";
  const lines = [
    "## CI timings",
    "",
    "GitHub job and step wall times include cache restore/save and artifact transfers. Exact cache hits are recorded by each producer; restore logs distinguish fallback hits from misses.",
    "",
    "| Job / step | Duration | Result |",
    "| --- | ---: | --- |",
  ];
  for (const job of jobs) {
    lines.push(
      `| ${job.name.replaceAll("|", "\\|")} | ${duration(job)} | ${job.conclusion ?? job.status} |`,
    );
    for (const step of job.steps ?? [])
      if (
        /cache|artifact|archive|ci-build|ci-tests|Rust tests|test compilation/i.test(
          step.name,
        )
      )
        lines.push(
          `| ↳ ${step.name.replaceAll("|", "\\|")} | ${duration(step)} | ${step.conclusion ?? step.status} |`,
        );
  }
  appendFileSync(process.env.GITHUB_STEP_SUMMARY, lines.join("\n") + "\n");
} catch (error) {
  console.warn(`CI timing report unavailable: ${error.message}`);
}
