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
  assert.deepEqual(document.security, [{ repositoryBearer: [] }]);
});

test("native_hex_rejects_short_uppercase_and_untyped_values", () => {
  for (const [name, width] of [
    ["RepositoryId", 32],
    ["CommitId", 128],
    ["ConflictFingerprint", 128],
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
