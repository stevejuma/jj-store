import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { verify } from "sigstore";
import {
  integrity,
  packageName,
  repository,
  workflow,
  validateManifest,
  validatePackage,
} from "./conversations-npm-artifact.mjs";
import {
  admitQualifiedRun,
  validateInvocation,
} from "./conversations-npm-plan.mjs";
import {
  provenanceStatement,
  verifySigner,
} from "./conversations-provenance.mjs";
import { validateRegistry } from "./conversations-npm.mjs";

const source = "a".repeat(40),
  bytes = Buffer.from("immutable qualified client");
const expected = { source, run: "123", attempt: "1" };
const manifest = {
  schema: 1,
  repository,
  workflow,
  name: packageName,
  version: "0.1.0",
  ...expected,
  tarball: "lapismd-jj-conversations-0.1.0.tgz",
  integrity: integrity(bytes),
  size: bytes.length,
};
const env = {
  GITHUB_REPOSITORY: repository,
  GITHUB_REF: "refs/heads/main",
  GITHUB_SHA: source,
  GITHUB_EVENT_NAME: "workflow_dispatch",
  GITHUB_WORKFLOW_REF: `${repository}/${workflow}@refs/heads/main`,
  RUNNER_ENVIRONMENT: "github-hosted",
  GITHUB_RUN_ID: "123",
  GITHUB_RUN_ATTEMPT: "1",
  GITHUB_REPOSITORY_ID: "1419706251",
  GITHUB_REPOSITORY_OWNER_ID: "46745434",
  INPUT_MODE: "attest",
};

test("Conversations bootstrap remains main-only and scoped to one explicit mode", () => {
  assert.equal(validateInvocation(env, "0.1.0"), "attest");
  for (const patch of [
    { GITHUB_REF: "refs/heads/feature" },
    { GITHUB_REPOSITORY: "fork/jj-store" },
    { INPUT_MODE: "unknown" },
    { INPUT_QUALIFIED_RUN: "123" },
    { INPUT_MODE: "verify" },
  ])
    assert.throws(() => validateInvocation({ ...env, ...patch }, "0.1.0"));
  assert.equal(
    validateInvocation(
      {
        ...env,
        GITHUB_EVENT_NAME: "push",
        GITHUB_REF: "refs/tags/jj-conversations-v0.1.0",
      },
      "0.1.0",
    ),
    "publish",
  );
  assert.throws(() =>
    validateInvocation(
      { ...env, GITHUB_EVENT_NAME: "push", GITHUB_REF: "refs/tags/v0.1.0" },
      "0.1.0",
    ),
  );
});

test("artifact reuse refuses incomplete heavy qualification, skipped package gates and forks", () => {
  const run = {
    repository: { full_name: repository },
    head_repository: { full_name: repository },
    path: workflow,
    event: "workflow_dispatch",
    head_branch: "main",
    status: "completed",
    conclusion: "success",
    head_sha: source,
    id: 123,
    run_attempt: 1,
  };
  const jobs = [
    {
      name: "Complete source qualification / Complete qualification gate",
      conclusion: "success",
    },
    { name: "Validate exact client artifact", conclusion: "success" },
  ];
  assert.deepEqual(admitQualifiedRun(run, jobs), expected);
  for (const patch of [
    { conclusion: "failure" },
    { event: "pull_request" },
    { head_branch: "feature" },
    { path: ".github/workflows/other.yml" },
    { head_repository: { full_name: "fork/jj-store" } },
    { head_sha: "not-a-sha" },
  ])
    assert.throws(() => admitQualifiedRun({ ...run, ...patch }, jobs));
  assert.throws(() => admitQualifiedRun(run, jobs.slice(1)));
  assert.throws(() =>
    admitQualifiedRun(run, [jobs[0], { ...jobs[1], conclusion: "skipped" }]),
  );
});

test("only the public contained Conversations client can enter the npm artifact", () => {
  const pkg = JSON.parse(
    readFileSync(
      new URL("../packages/jj-conversations/package.json", import.meta.url),
      "utf8",
    ),
  );
  assert.doesNotThrow(() => validatePackage(pkg));
  for (const patch of [
    { name: "@lapismd/jj-store" },
    { private: true },
    { publishConfig: { access: "restricted" } },
    { dependencies: { server: "1.0.0" } },
    { devDependencies: { server: "workspace:*" } },
  ])
    assert.throws(() => validatePackage({ ...pkg, ...patch }));
});

test("artifact bytes retain exact source, producing run and attempt across publication reuse", () => {
  assert.doesNotThrow(() => validateManifest(manifest, expected, bytes));
  for (const patch of [
    { source: "b".repeat(40) },
    { run: "124" },
    { attempt: "2" },
    { tarball: "../client.tgz" },
    { size: 0 },
    { integrity: integrity(Buffer.from("other")) },
  ])
    assert.throws(() =>
      validateManifest({ ...manifest, ...patch }, expected, bytes),
    );
  assert.throws(() =>
    validateManifest(manifest, expected, Buffer.from("tampered")),
  );
});

test("npm provenance signs one PURL/digest and the exact main workflow source", () => {
  const statement = provenanceStatement(manifest, env);
  assert.deepEqual(statement.predicate.buildDefinition.internalParameters, {
    github: {
      event_name: "workflow_dispatch",
      repository_id: "1419706251",
      repository_owner_id: "46745434",
    },
  });
  assert.deepEqual(statement.subject, [
    {
      name: "pkg:npm/%40lapismd/jj-conversations@0.1.0",
      digest: {
        sha512: Buffer.from(manifest.integrity.slice(7), "base64").toString(
          "hex",
        ),
      },
    },
  ]);
  assert.equal(
    statement.predicate.buildDefinition.resolvedDependencies[0].digest
      .gitCommit,
    source,
  );
  for (const patch of [
    { GITHUB_REF: "refs/tags/jj-conversations-v0.1.0" },
    { GITHUB_SHA: "b".repeat(40) },
    { GITHUB_WORKFLOW_REF: "other" },
    { RUNNER_ENVIRONMENT: "self-hosted" },
    { GITHUB_EVENT_NAME: "push" },
    { GITHUB_REPOSITORY_ID: "0" },
    { GITHUB_REPOSITORY_OWNER_ID: "0" },
  ])
    assert.throws(() => provenanceStatement(manifest, { ...env, ...patch }));
});

test("downloaded npm package must retain immutable bytes and provenance", () => {
  const metadata = {
    name: packageName,
    version: "0.1.0",
    dist: { integrity: manifest.integrity, attestations: { provenance: {} } },
  };
  assert.doesNotThrow(() => validateRegistry(manifest, metadata, bytes));
  assert.throws(() =>
    validateRegistry(manifest, metadata, Buffer.from("changed")),
  );
  assert.throws(() =>
    validateRegistry(
      manifest,
      { ...metadata, dist: { integrity: manifest.integrity } },
      bytes,
    ),
  );
  assert.throws(() =>
    validateRegistry(manifest, { ...metadata, version: "0.2.0" }, bytes),
  );
});

test("real Sigstore verification rejects a valid foreign workflow and payload tampering", async () => {
  const bundle = JSON.parse(
    readFileSync(
      new URL("./fixtures/design-core-0.7.1-provenance.json", import.meta.url),
      "utf8",
    ),
  );
  const signer = await verify(bundle);
  assert.equal(
    signer.identity.subjectAlternativeName,
    "https://github.com/lapismd/design-core/.github/workflows/release.yml@refs/heads/main",
  );
  await assert.rejects(
    verifySigner(bundle),
    /certificate identity|signer does not match/,
  );
  await assert.rejects(
    verifySigner(bundle, async () => signer),
    /signer does not match/,
  );
  const tampered = structuredClone(bundle);
  tampered.dsseEnvelope.payload = Buffer.from("{}").toString("base64");
  await assert.rejects(verifySigner(tampered));
});
