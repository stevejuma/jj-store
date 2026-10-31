import { appendFileSync } from "node:fs";
import { join } from "node:path";
import {
  output,
  packageDirectory,
  readJson,
  repository,
  validatePackage,
  workflow,
} from "./conversations-npm-artifact.mjs";

export function admitQualifiedRun(run, jobs) {
  if (
    run.repository?.full_name !== repository ||
    run.head_repository?.full_name !== repository ||
    run.path !== workflow ||
    run.event !== "workflow_dispatch" ||
    run.head_branch !== "main" ||
    run.status !== "completed" ||
    run.conclusion !== "success" ||
    !/^[a-f0-9]{40}$/.test(run.head_sha ?? "") ||
    !Number.isSafeInteger(run.id) ||
    !Number.isSafeInteger(run.run_attempt) ||
    run.id <= 0 ||
    run.run_attempt <= 0 ||
    !jobs.some(
      (job) =>
        job.name.endsWith("Complete qualification gate") &&
        job.conclusion === "success",
    ) ||
    !jobs.some(
      (job) =>
        job.name === "Validate exact client artifact" &&
        job.conclusion === "success",
    )
  )
    throw Error(
      "Artifact reuse requires a successful canonical complete qualification and package gate",
    );
  return {
    source: run.head_sha,
    run: String(run.id),
    attempt: String(run.run_attempt),
  };
}

export function validateInvocation(env, version) {
  if (
    env.GITHUB_REPOSITORY !== repository ||
    !/^[a-f0-9]{40}$/.test(env.GITHUB_SHA ?? "")
  )
    throw Error(
      "Conversations publication requires canonical immutable source",
    );
  if (env.GITHUB_EVENT_NAME === "workflow_dispatch") {
    if (
      env.GITHUB_REF !== "refs/heads/main" ||
      !["attest", "publish", "verify"].includes(env.INPUT_MODE)
    )
      throw Error(
        "Manual Conversations releases require main and an explicit mode",
      );
    if (env.INPUT_MODE === "attest" && env.INPUT_QUALIFIED_RUN)
      throw Error(
        "Detached provenance requires a fresh complete main qualification",
      );
    if (
      env.INPUT_MODE === "verify" &&
      !/^\d+$/.test(env.INPUT_QUALIFIED_RUN ?? "")
    )
      throw Error("Read-only verification requires a qualified artifact run");
    return env.INPUT_MODE;
  }
  if (
    env.GITHUB_EVENT_NAME !== "push" ||
    env.GITHUB_REF !== `refs/tags/jj-conversations-v${version}`
  )
    throw Error(
      "Conversations release tag differs from the exact package version",
    );
  return "publish";
}

export function plan(root = process.cwd(), env = process.env) {
  const pkg = readJson(join(root, packageDirectory, "package.json"));
  validatePackage(pkg);
  const mode = validateInvocation(env, pkg.version);
  let expected = {
    source: env.GITHUB_SHA,
    run: env.GITHUB_RUN_ID,
    attempt: env.GITHUB_RUN_ATTEMPT,
  };
  const reused = Boolean(env.INPUT_QUALIFIED_RUN);
  if (reused) {
    if (!/^\d+$/.test(env.INPUT_QUALIFIED_RUN))
      throw Error("Invalid qualification run identity");
    const run = JSON.parse(
      output("gh", [
        "api",
        `repos/${repository}/actions/runs/${env.INPUT_QUALIFIED_RUN}`,
      ]),
    );
    const pages = JSON.parse(
      output("gh", [
        "api",
        "--paginate",
        "--slurp",
        `repos/${repository}/actions/runs/${run.id}/attempts/${run.run_attempt}/jobs?per_page=100`,
      ]),
    );
    expected = admitQualifiedRun(
      run,
      pages.flatMap((page) => page.jobs),
    );
    if (mode === "publish" && expected.source !== env.GITHUB_SHA)
      throw Error(
        "Publication reuse requires the current immutable main source",
      );
  }
  output(
    "git",
    ["merge-base", "--is-ancestor", expected.source, "origin/main"],
    { cwd: root },
  );
  if (
    !/^\d+$/.test(expected.run ?? "") ||
    !/^\d+$/.test(expected.attempt ?? "")
  )
    throw Error("Missing qualification run/attempt identity");
  const result = {
    ...expected,
    mode,
    reused: String(reused),
    artifact_name: `jj-conversations-npm-${expected.source}-${expected.attempt}`,
  };
  if (env.GITHUB_OUTPUT)
    appendFileSync(
      env.GITHUB_OUTPUT,
      Object.entries(result)
        .map(([key, value]) => `${key}=${value}\n`)
        .join(""),
    );
  console.log(JSON.stringify(result, null, 2));
  return result;
}
