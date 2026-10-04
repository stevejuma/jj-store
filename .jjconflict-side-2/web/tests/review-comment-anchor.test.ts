import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import type { StoreDataSource } from "../src/lib/controller/native-data-source.js";
import type { ProductionStoreState } from "../src/lib/controller/production-store-state.svelte.js";
import { createCodeLineReviewAnchor } from "../src/lib/controller/review-comment-anchor.js";

const repository = "ab".repeat(16);
const beforeOperation = "11".repeat(64);
const afterOperation = "22".repeat(64);
const beforeCommit = "33".repeat(64);
const afterCommit = "44".repeat(64);
const beforeFile = "55".repeat(32);
const afterFile = "66".repeat(32);

function file(id: string): Model<"NativeTreeValue"> {
  return { type: "file", id, executable: false, copy: { type: "placeholder" } };
}

function state(): ProductionStoreState {
  return {
    comparisons: new Map([
      [
        "src/lib.rs",
        {
          from: beforeCommit,
          to: afterCommit,
          beforeOperation,
          afterOperation,
        },
      ],
    ]),
    detail: { files: [], patches: {} },
    graph: { revisions: [], devices: [] },
    agentSession: undefined,
  } as unknown as ProductionStoreState;
}

function source(options?: {
  before?: (Model<"NativeTreeValue"> | null)[];
  after?: (Model<"NativeTreeValue"> | null)[];
  beforeText?: string;
  afterText?: string;
}): StoreDataSource {
  return {
    repository,
    async diff(operation, from, to, path) {
      return {
        snapshot: {
          version: 1,
          repo_id: repository,
          epoch: "77".repeat(16),
          published_seq: 9,
          manifest_revision: 4,
          code_state_version: 3,
          operation_id: operation,
          heads: [operation],
        },
        data: {
          version: 1,
          from,
          to,
          path,
          changes: [
            {
              path,
              before: options?.before ?? [file(beforeFile)],
              after: options?.after ?? [file(afterFile)],
              sources: [],
              content: {
                status: "text" as const,
                hunks: [
                  {
                    kind: "different" as const,
                    before_start: 1,
                    after_start: 1,
                    before: options?.beforeText ?? "old\nsecond\n",
                    after: options?.afterText ?? "alpha\nβeta\nlast\n",
                  },
                ],
              },
            },
          ],
          stats: {
            visited_objects: 1,
            visited_bytes: 1,
            elapsed_millis: 1,
            cache_hits: 0,
          },
        },
        next_cursor: null,
      };
    },
    async file(id, operation) {
      if (id === beforeFile && operation === beforeOperation)
        return { text: options?.beforeText ?? "old\nsecond\n", bytes: 11 };
      if (id === afterFile && operation === afterOperation)
        return { text: options?.afterText ?? "alpha\nβeta\nlast\n", bytes: 17 };
      throw new Error("unexpected file identity");
    },
  } as StoreDataSource;
}

test("after-line anchors preserve UTF-8 offsets, digest and operation pair", async () => {
  const anchor = await createCodeLineReviewAnchor(source(), state(), {
    path: "src/lib.rs",
    lineNumber: 2,
    variant: "added",
    text: "βeta",
  });
  assert.equal(anchor?.target.target, "code_text");
  if (anchor?.target.target !== "code_text") return;
  assert.deepEqual(anchor.target.value.text, {
    encoding: "utf8",
    start_byte: "6",
    end_byte: "11",
    selected_sha256: createHash("sha256").update("βeta").digest("hex"),
  });
  assert.equal(anchor.target.value.file_id, afterFile);
  assert.equal(anchor.target.value.operation_id, afterOperation);
  assert.deepEqual(anchor.target.value.diff_context, {
    before_operation_id: beforeOperation,
    before_commit_id: beforeCommit,
    before_path: "src/lib.rs",
    after_operation_id: afterOperation,
    after_commit_id: afterCommit,
    after_path: "src/lib.rs",
    side: "after",
    comparison: "explicit_pair",
  });
});

test("removed-line anchors resolve the before file and operation", async () => {
  const anchor = await createCodeLineReviewAnchor(source(), state(), {
    path: "src/lib.rs",
    lineNumber: 1,
    variant: "removed",
    text: "old",
  });
  assert.equal(anchor?.target.target, "code_text");
  if (anchor?.target.target !== "code_text") return;
  assert.equal(anchor.target.value.file_id, beforeFile);
  assert.equal(anchor.target.value.operation_id, beforeOperation);
  assert.equal(anchor.target.value.text.start_byte, "0");
  assert.equal(anchor.target.value.text.end_byte, "3");
  assert.equal(anchor.target.value.diff_context?.side, "before");
});

test("line anchoring fails closed for stale text, conflicts and final blank lines", async () => {
  const context = {
    path: "src/lib.rs",
    lineNumber: 2,
    variant: "added" as const,
    text: "stale",
  };
  assert.equal(
    await createCodeLineReviewAnchor(source(), state(), context),
    undefined,
  );
  assert.equal(
    await createCodeLineReviewAnchor(
      source({ after: [file(afterFile), file("88".repeat(32))] }),
      state(),
      { ...context, text: "βeta" },
    ),
    undefined,
  );
  assert.equal(
    await createCodeLineReviewAnchor(source({ afterText: "one\n" }), state(), {
      ...context,
      lineNumber: 2,
      text: "",
    }),
    undefined,
  );
});
