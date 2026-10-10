import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import SwaggerParser from "@apidevtools/swagger-parser";
import { schemaValidator, responseValidator } from "./contracts-validation.mjs";

const document = JSON.parse(
  readFileSync(new URL("../schemas/openapi.json", import.meta.url)),
);
const validate = schemaValidator(document);
const schema = (name) => ({ $ref: `#/components/schemas/${name}` });

test("openapi_document_validates_without_external_references", async () => {
  await SwaggerParser.validate(structuredClone(document), {
    resolve: { external: false },
  });
  assert.equal(document.openapi, "3.1.0");
  assert.deepEqual(document.security, []);
  assert.deepEqual(document["x-jj-store-access-policies"], [
    "authenticated",
    "local_trusted",
  ]);
  assert.deepEqual(document.paths["/v1/bootstrap"].get.security, [{}]);
});

test("native_hex_rejects_short_uppercase_and_untyped_values", () => {
  for (const [name, width] of [
    ["RepositoryId", 32],
    ["CommitId", 128],
    ["ConflictFingerprint", 128],
    ["ConflictUploadId", 64],
  ]) {
    validate(schema(name), "ab".repeat(width / 2));
    for (const value of [
      "ab".repeat(width / 2 - 1),
      "AB".repeat(width / 2),
      12,
      [1, 2],
    ]) {
      assert.throws(() => validate(schema(name), value));
    }
  }
});

test("frozen_native_shapes_preserve_nullability_and_tuples", () => {
  const corpus = JSON.parse(
    readFileSync(
      new URL("../fixtures/native-profile/v1-envelopes.json", import.meta.url),
    ),
  );
  for (const [kind, name] of Object.entries({
    commit: "NativeCommit",
    tree: "NativeTree",
    copy: "NativeCopyHistory",
    view: "NativeView",
    operation: "NativeOperation",
  })) {
    const envelope = Buffer.from(corpus[kind], "hex");
    assert.equal(envelope.subarray(0, 4).toString(), "JJST");
    const payload = JSON.parse(envelope.subarray(113).toString());
    validate(schema(name), payload);
    if (kind === "operation") {
      for (const commit_predecessors of [null, {}])
        validate(schema(name), { ...payload, commit_predecessors });
      delete payload.commit_predecessors;
      assert.throws(() => validate(schema(name), payload));
    }
    if (kind === "tree") {
      assert.throws(() =>
        validate(schema(name), {
          entries: payload.entries.map(([name, value]) => [value, name]),
        }),
      );
    }
  }
});

test("tagged_union_and_required_nullable_fields_fail_closed", () => {
  validate(schema("CopyReference"), { type: "placeholder" });
  validate(schema("CopyReference"), { type: "object", id: "00".repeat(64) });
  assert.throws(() => validate(schema("CopyReference"), { type: "object" }));
  assert.throws(() =>
    validate(schema("CopyReference"), { type: "future", id: "00".repeat(64) }),
  );
  validate(schema("NativeTreeValue"), { type: "submodule", id: [1, 2, 3] });
  assert.throws(() =>
    validate(schema("NativeTreeValue"), { type: "submodule", id: "010203" }),
  );
});

test("response_validation_rejects_missing_contracts_and_wrong_media", () => {
  const check = responseValidator(document);
  assert.throws(() =>
    check({
      path: "/v1/repos/{repo}/future",
      method: "get",
      status: 200,
      media: "application/json",
      data: {},
    }),
  );
  assert.throws(() =>
    check({
      path: "/v1/repos/{repo}/events",
      method: "get",
      status: 200,
      media: "application/json",
      data: {},
    }),
  );
  assert.throws(() =>
    check({
      path: "/v1/repos/{repo}/snapshot",
      method: "get",
      status: 200,
      media: "application/json",
      data: {},
    }),
  );
});

test("mutation_requests_are_strict_and_unknown_outcomes_retain_identity", () => {
  const body =
    document.paths["/v1/repos/{repo}/mutations"].post.requestBody.content[
      "application/json"
    ].schema;
  const request = {
    version: 1,
    request_id: "aa".repeat(16),
    expected: { epoch: "bb".repeat(16), position: 0, heads: ["00".repeat(64)] },
    command: {
      type: "new",
      parents: ["00".repeat(64)],
      description: "",
      workspace: null,
    },
  };
  validate(body, request);
  for (const invalid of [
    { ...request, principal: "cc".repeat(16) },
    { ...request, command: { type: "shell", args: ["new"] } },
    { ...request, command: { ...request.command, ignored: true } },
    { ...request, expected: { ...request.expected, latest: true } },
    { ...request, request_id: "aa" },
  ])
    assert.throws(() => validate(body, invalid));
  const response =
    document.paths["/v1/repos/{repo}/mutations"].post.responses["200"].content[
      "application/json"
    ].schema;
  validate(response, {
    status: "outcome_unknown",
    request_id: request.request_id,
  });
  assert.throws(() => validate(response, { status: "outcome_unknown" }));
  assert.throws(() =>
    validate(response, { status: "committed", request_id: request.request_id }),
  );
});

test("collaboration_commands_reject_actor_injection_and_keep_exact_rich_values", () => {
  const operation = document.paths["/v1/repos/{repo}/reactions/commands"].post;
  const body = operation.requestBody.content["application/json"].schema;
  const request = {
    version: 1,
    request_id: "ab".repeat(16),
    command: {
      type: "set",
      target: { type: "thread", thread_id: "cd".repeat(16) },
      observed: [],
      value: { type: "emoji", sequence: "❤️" },
    },
  };
  validate(body, request);
  assert.throws(() => validate(body, { ...request, actor: "ef".repeat(16) }));
  assert.throws(() =>
    validate(body, {
      ...request,
      command: {
        ...request.command,
        value: { type: "custom", emoji_id: "12".repeat(16) },
      },
    }),
  );
  validate(body, {
    ...request,
    command: {
      ...request.command,
      value: {
        type: "custom",
        emoji_id: "12".repeat(16),
        version: { kind: "custom_emoji_version", id: "34".repeat(32) },
      },
    },
  });
  const raster = document.paths["/v1/repos/{repo}/reaction-assets/{asset}"].get;
  assert.ok(raster.responses[200].content["image/png"]);
  assert.equal(
    raster.parameters.some((p) => p.name === "Range"),
    false,
  );
  assert.deepEqual(Object.keys(raster.responses[200].content).sort(), [
    "image/gif",
    "image/jpeg",
    "image/png",
    "image/webp",
  ]);
  const upload =
    document.paths["/v1/repos/{repo}/reaction-emojis/{emoji_id}"].post;
  assert.ok(upload.requestBody.content["application/octet-stream"]);
  assert.equal(upload.requestBody.content["application/json"], undefined);
  const ids = Object.values(document.paths).flatMap((path) =>
    Object.values(path).map((op) => op.operationId),
  );
  assert.equal(new Set(ids).size, ids.length);
});

test("websocket_freshness_documents_authentication_and_typed_event_frames", () => {
  const op = document.paths["/v1/events/ws"].get;
  assert.ok(op.responses[101]);
  assert.equal(document.paths["/v1/events/ws"].head, undefined);
  validate(op["x-websocket-client-schema"], { version: 1, token: null });
  assert.throws(() =>
    validate(op["x-websocket-client-schema"], { version: 1, command: "write" }),
  );
  const id = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:0`;
  for (const event of [
    { version: 1, type: "ready", witnesses: [] },
    { version: 1, type: "reset", reason: "generation_or_scope", witnesses: [] },
    {
      version: 1,
      type: "changed",
      change: { repository: null, domain: "agent_settings", witness: "one" },
    },
  ])
    validate(op["x-websocket-event-schema"], { id, event });
  validate(op["x-websocket-heartbeat-schema"], {
    version: 1,
    type: "heartbeat",
  });
});
