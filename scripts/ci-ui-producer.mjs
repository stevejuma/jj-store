import assert from "node:assert/strict";
import { isConversationsProducer } from "./ci-conversations-producer.mjs";

export function isScheduledProducer(run) {
  return (
    run.event === "schedule" &&
    run.repository?.full_name === "stevejuma/jj-store" &&
    run.path === ".github/workflows/ci-heavy.yml" &&
    run.head_branch === "main"
  );
}

/** Core UI inputs are same-run only and never authorize qualification reuse. */
export function verifyUiSource(run, expected, env = process.env) {
  assert.equal(
    expected.run,
    env.GITHUB_RUN_ID,
    "UI inputs must be from this run",
  );
  assert.equal(expected.attempt, env.GITHUB_RUN_ATTEMPT, "UI attempt mismatch");
  assert.equal(String(run.id), expected.run);
  assert.equal(Number(run.run_attempt), Number(expected.attempt));
  assert.equal(run.repository.full_name, expected.repository);
  if (run.event === "schedule")
    assert.equal(run.head_sha, expected.source, "Scheduled source mismatch");
  assert.ok(
    ["pull_request", "push", "workflow_dispatch"].includes(run.event) ||
      isScheduledProducer(run),
  );
  assert.ok(
    [
      ".github/workflows/ci.yml",
      ".github/workflows/ci-ui.yml",
      ".github/workflows/ci-heavy.yml",
      ".github/workflows/publish-release.yml",
    ].includes(run.path) || isConversationsProducer(run),
    "Unapproved UI producer workflow",
  );
}

export function verifyUiProducer(
  run,
  jobs,
  expected,
  env = process.env,
  role = "ui",
) {
  verifyUiSource(run, expected, env);
  assert.ok(["ui", "ui-catalog"].includes(role), "Unknown UI producer role");
  const name = role === "ui" ? "UI acceptance inputs" : "Static catalog inputs";
  const producers = jobs.filter(
    (job) => job.name === name || job.name.endsWith(`/ ${name}`),
  );
  assert.equal(producers.length, 1, "Missing or ambiguous UI producer");
  assert.equal(producers[0].conclusion, "success", "Unsuccessful UI producer");
}
