import assert from "node:assert/strict";
import test from "node:test";
import type { Head, Model } from "../src/lib/api/client.js";
import type { StoreDataSource } from "../src/lib/controller/native-data-source.js";
import { createProductionStoreConflicts } from "../src/lib/controller/production-store-conflicts.js";

const head: Head = {
  version: 1,
  repo_id: "ab".repeat(16),
  epoch: "cd".repeat(16),
  published_seq: 7,
  manifest_revision: 9,
  code_state_version: 5,
  heads: ["12".repeat(64)],
};

test("conflict uploads bind bytes to the captured head commit path and fingerprint", async () => {
  const calls: unknown[][] = [];
  const response: Model<"ConflictUpload"> = {
    version: 1,
    id: "34".repeat(32),
    file: "56".repeat(64),
    bytes: 5,
    binding: {
      expected: {
        epoch: head.epoch,
        position: head.published_seq,
        code_state_version: head.code_state_version,
        heads: [...head.heads],
      },
      commit: "78".repeat(64),
      path: "src/lib.rs",
      fingerprint: "9a".repeat(64),
    },
  };
  const source = {
    async uploadConflict(...args: unknown[]) {
      calls.push(args);
      return response;
    },
  } as unknown as StoreDataSource;
  const conflicts = createProductionStoreConflicts({
    source: () => source,
    head: () => head,
    current: (candidate) => candidate === source,
    uploadLimit: () => 8,
  });
  const bytes = new TextEncoder().encode("merge");
  assert.equal(
    await conflicts.upload(
      "78".repeat(64),
      "src/lib.rs",
      "9a".repeat(64),
      bytes,
    ),
    response,
  );
  assert.deepEqual(calls, [
    [head, "78".repeat(64), "src/lib.rs", "9a".repeat(64), bytes],
  ]);
});

test("conflict uploads reject oversized bytes before transport", async () => {
  let contacted = false;
  const source = {
    async uploadConflict() {
      contacted = true;
      throw new Error("unexpected");
    },
  } as unknown as StoreDataSource;
  const conflicts = createProductionStoreConflicts({
    source: () => source,
    head: () => head,
    current: () => true,
    uploadLimit: () => 4,
  });
  await assert.rejects(
    conflicts.upload(
      "78".repeat(64),
      "src/lib.rs",
      "9a".repeat(64),
      new Uint8Array(5),
    ),
    /4 bytes/u,
  );
  assert.equal(contacted, false);
});
