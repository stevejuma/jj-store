import assert from "node:assert/strict";

/** Core UI inputs are same-run only and never authorize qualification reuse. */
export function verifyUiProducer(run, jobs, expected, env = process.env) {
  assert.equal(
    expected.run,
    env.GITHUB_RUN_ID,
    "UI inputs must be from this run",
  );
  assert.equal(expected.attempt, env.GITHUB_RUN_ATTEMPT, "UI attempt mismatch");
  assert.equal(String(run.id), expected.run);
  assert.equal(Number(run.run_attempt), Number(expected.attempt));
  assert.equal(run.repository.full_name, expected.repository);
  assert.ok(["pull_request", "push", "workflow_dispatch"].includes(run.event));
  assert.ok(
    [
      ".github/workflows/ci.yml",
      ".github/workflows/ci-ui.yml",
      ".github/workflows/ci-heavy.yml",
      ".github/workflows/publish-release.yml",
    ].includes(run.path),
    "Unapproved UI producer workflow",
  );
  const producers = jobs.filter(
    (job) =>
      job.name === "UI acceptance inputs" ||
      job.name.endsWith("/ UI acceptance inputs"),
  );
  assert.equal(producers.length, 1, "Missing or ambiguous UI producer");
  assert.equal(producers[0].conclusion, "success", "Unsuccessful UI producer");
}
