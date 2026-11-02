export function timingSummary(report) {
  const show = (value) => (value === null ? "unknown" : `${value.toFixed(1)}s`);
  const lines = [
    "## CI timings",
    "",
    `Source: \`${report.source}\`. Run/attempt: ${report.run}/${report.attempt}.`,
    "",
    "Execution includes setup, cache and transfers. Scheduler queue measures job creation to start and may include workflow concurrency waits. Runner-only queueing and dependency waiting are unknown without readiness timestamps. Runner minutes sum parallel execution, not elapsed time or cost. Running jobs and post-job cache saves may be incomplete in this snapshot.",
    "",
    "| Job | Execution | Scheduler queue | Result |",
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
  for (const result of [
    ...report.results.junit,
    ...report.results.rustfs,
    ...report.results.tap,
  ])
    lines.push(
      "",
      `Tests (${result.path}): ${result.counts.passed} passed, ${result.counts.failed} failed, ${result.counts.skipped} skipped.`,
      ...(result.slowest || result.tests || [])
        .slice(0, 5)
        .map(
          (test) => `- ${test.name}: ${show(test.seconds)} (${test.result})`,
        ),
    );
  if (!report.results.test_counts_available)
    lines.push(
      "",
      "No structured test counts were produced by this job; consult its command logs. Missing reports do not establish a pass.",
    );
  for (const phase of report.results.cargo)
    lines.push(
      `Cargo target preparation (${phase.path}, ${phase.profile}): ${show(phase.seconds)}. Includes compilation and dependency preparation.`,
    );
  for (const result of report.results.libtest)
    lines.push(
      `Libtest execution (${result.path}): ${show(result.execution_seconds)} across completed test binaries; excludes compilation and harness startup.`,
    );
  return lines.join("\n") + "\n";
}
