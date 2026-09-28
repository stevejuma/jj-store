import assert from "node:assert/strict";
import test from "node:test";
import type { Head, Model } from "../src/lib/api/client.js";
import type { StoreDataSource } from "../src/lib/controller/native-data-source.js";
import { createProductionStoreMutations } from "../src/lib/controller/production-store-mutations.js";
import { initialStoreMutationState } from "../src/lib/controller/store-mutation-types.js";
import type {
  StoreMutationOutcome,
  StoreMutationPreview,
  StoreMutationRequest,
  StoreMutationStatusResponse,
} from "../src/lib/controller/store-mutation-types.js";

const head: Head = {
  version: 1,
  repo_id: "ab".repeat(16),
  epoch: "cd".repeat(16),
  published_seq: 7,
  manifest_revision: 9,
  code_state_version: 5,
  heads: ["12".repeat(64)],
};

function prepared(request: StoreMutationRequest): Model<"Prepared"> {
  return {
    version: 1,
    operation: "34".repeat(64),
    created: [],
    rewrites: [],
    bookmarks: {},
    workspaces: {},
    stats: {
      metadata_nodes: 1,
      predicate_file_bytes: 0,
      predicate_lines: 0,
      object_reads: 2,
      object_bytes: 3,
      work: 4,
    },
    staged_objects: 4,
    staged_bytes: 5,
  };
}

function fixture() {
  const requests: StoreMutationRequest[] = [];
  let previewResponse: StoreMutationPreview | Promise<StoreMutationPreview>;
  let applyResponse: StoreMutationOutcome;
  let statusResponse: StoreMutationStatusResponse;
  let current = true;
  let committed = 0;
  const source = {
    repository: head.repo_id,
    async previewMutation(request: StoreMutationRequest) {
      requests.push(request);
      return previewResponse;
    },
    async applyMutation(request: StoreMutationRequest) {
      requests.push(request);
      return applyResponse;
    },
    async mutationStatus(requestId: string) {
      assert.equal(requestId, requests[0]?.request_id);
      return statusResponse;
    },
  } as StoreDataSource;
  const state = initialStoreMutationState();
  const mutation = createProductionStoreMutations({
    state,
    source: () => source,
    head: () => head,
    current: () => current,
    committed: async () => {
      committed += 1;
    },
  });
  mutation.configure(["describe", "abandon"]);
  const command = {
    type: "describe" as const,
    commit: "56".repeat(64),
    description: "Describe the native change",
  };
  const makeReceipt = (request: StoreMutationRequest) => ({
    version: 1,
    request_id: request.request_id,
    publication_id: "78".repeat(16),
    request_digest: "9a".repeat(32),
    accepted: {
      epoch: head.epoch,
      position: 8,
      code_state_version: 6,
      heads: ["34".repeat(64)],
    },
    result: prepared(request),
  });
  return {
    mutation: { ...mutation, state },
    command,
    requests,
    setPreview(value: StoreMutationPreview | Promise<StoreMutationPreview>) {
      previewResponse = value;
    },
    setApply(value: StoreMutationOutcome) {
      applyResponse = value;
    },
    setStatus(value: StoreMutationStatusResponse) {
      statusResponse = value;
    },
    makeReceipt,
    setCurrent(value: boolean) {
      current = value;
    },
    committed: () => committed,
  };
}

test("preview and apply retain one exact request through commitment", async () => {
  const f = fixture();
  f.setPreview({
    status: "prepared",
    preview: {
      version: 1,
      request_id: "",
      request_digest: "ab".repeat(32),
      expected: {
        epoch: head.epoch,
        position: 7,
        code_state_version: head.code_state_version,
        heads: [...head.heads],
      },
      result: prepared({} as StoreMutationRequest),
    },
  });
  await f.mutation.preview(f.command);
  assert.equal(f.mutation.state.status, "prepared");
  const request = f.requests[0]!;
  assert.match(request.request_id, /^[0-9a-f]{32}$/u);
  assert.deepEqual(request.expected, {
    epoch: head.epoch,
    position: head.published_seq,
    code_state_version: head.code_state_version,
    heads: head.heads,
  });
  f.setApply({ status: "committed", receipt: f.makeReceipt(request) });
  await f.mutation.apply();
  assert.equal(f.requests[1], request);
  assert.equal(f.mutation.state.status, "committed");
  assert.equal(f.committed(), 1);
});

test("unknown outcomes keep the request identity for status recovery", async () => {
  const f = fixture();
  f.setPreview({
    status: "prepared",
    preview: {
      version: 1,
      request_id: "",
      request_digest: "ab".repeat(32),
      expected: {
        epoch: head.epoch,
        position: 7,
        code_state_version: head.code_state_version,
        heads: [...head.heads],
      },
      result: prepared({} as StoreMutationRequest),
    },
  });
  await f.mutation.preview(f.command);
  const request = f.requests[0]!;
  f.setApply({ status: "outcome_unknown", request_id: request.request_id });
  await f.mutation.apply();
  assert.equal(f.mutation.state.status, "unknown");
  f.setStatus({ status: "committed", receipt: f.makeReceipt(request) });
  await f.mutation.checkStatus();
  assert.equal(f.mutation.state.status, "committed");
  assert.equal(f.committed(), 1);
});

test("late preview responses cannot repopulate a departed session", async () => {
  const f = fixture();
  let resolve!: (value: StoreMutationPreview) => void;
  f.setPreview(new Promise((done) => (resolve = done)));
  const pending = f.mutation.preview(f.command);
  f.setCurrent(false);
  resolve({
    status: "prepared",
    preview: {
      version: 1,
      request_id: f.requests[0]!.request_id,
      request_digest: "ab".repeat(32),
      expected: {
        epoch: head.epoch,
        position: 7,
        code_state_version: head.code_state_version,
        heads: [...head.heads],
      },
      result: prepared(f.requests[0]!),
    },
  });
  await pending;
  assert.equal(f.mutation.state.status, "previewing");
  assert.equal(f.mutation.state.result, null);
});
