import assert from "node:assert/strict";
import test from "node:test";
import { sanitizeCapturedObservation } from "./agent-capture-sanitizer.mjs";

const secret = "synthetic-agent-secret-4711";
const input = {
  integration: "codex",
  sourceKind: "hook",
  sourceInstanceId: "source-1",
  sourceSequence: 7,
  occurredAtMillis: 1_790_000_000_000,
  payload: {
    session_id: "native-session",
    message: `token=${secret}`,
    path: "/private/workspace/src/main.rs",
  },
};

test("sanitizes before returning a capture-ready observation", async () => {
  const result = await sanitizeCapturedObservation(input, {
    secrets: [secret],
    roots: [
      {
        kind: "workspace",
        path: "/private/workspace",
        id: "workspace-1",
        repositoryId: "repo-1",
      },
    ],
  });

  assert.equal(result.status, "sanitized");
  assert.equal(result.value.integration, "codex");
  assert.equal(result.value.sourceSequence, 7);
  assert.match(result.value.sanitizerVersion, /^agent-redaction\/1\//);
  assert.equal(
    result.value.sanitizationSummary.policy,
    result.value.sanitizerVersion,
  );
  assert.equal(Object.hasOwn(result.value, "unexpected"), false);
  assert.equal(JSON.stringify(result).includes(secret), false);
  assert.equal(JSON.stringify(result).includes("/private/workspace"), false);
  assert.deepEqual(input.payload.message, `token=${secret}`);
});

test("fails closed without touching hostile or unsupported capture input", async () => {
  let accessed = false;
  const hostile = Object.defineProperty({}, "payload", {
    enumerable: true,
    get() {
      accessed = true;
      return secret;
    },
  });
  assert.deepEqual(await sanitizeCapturedObservation(hostile), {
    status: "omitted",
    reason: "invalid_input",
  });
  assert.equal(accessed, false);

  assert.deepEqual(
    await sanitizeCapturedObservation({ ...input, integration: "unknown" }),
    { status: "omitted", reason: "invalid_input" },
  );
});

test("accepts only the initial qualified integration identifiers", async () => {
  for (const integration of ["codex", "cursor", "opencode"]) {
    const result = await sanitizeCapturedObservation({
      ...input,
      integration,
      unexpected: "not part of the capture DTO",
    });
    assert.equal(result.status, "sanitized");
    assert.equal(result.value.integration, integration);
    assert.equal(Object.hasOwn(result.value, "unexpected"), false);
  }
});
