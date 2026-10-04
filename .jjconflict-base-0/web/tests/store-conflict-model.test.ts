import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import {
  buildConflictCommand,
  type StoreConflictResolution,
} from "../src/lib/ui/store-conflicts/model.js";

const textConflict: Model<"ConflictDescriptor"> = {
  path: "src/lib.rs",
  fingerprint: "ab".repeat(64),
  terms: [
    { id: 0, role: "add", label: "left", value: null },
    { id: 1, role: "remove", label: "base", value: null },
    { id: 2, role: "add", label: "right", value: null },
  ],
  editor: {
    status: "text",
    contents: ["left\n", "base\n", "right\n"],
    executable: false,
  },
};

const structuralConflict: Model<"ConflictDescriptor"> = {
  path: "assets/logo.bin",
  fingerprint: "cd".repeat(64),
  terms: [
    { id: 0, role: "add", label: null, value: null },
    { id: 1, role: "remove", label: null, value: null },
  ],
  editor: { status: "unsupported", reason: "structural" },
};

test("atomic conflict commands retain every path and native fingerprint", () => {
  const selections = new Map<string, StoreConflictResolution>([
    ["src/lib.rs", { type: "select_term", term: 2 }],
    ["assets/logo.bin", { type: "delete" }],
  ]);
  assert.deepEqual(
    buildConflictCommand(
      "12".repeat(64),
      [textConflict, structuralConflict],
      selections,
    ),
    {
      type: "resolve_conflicts",
      commit: "12".repeat(64),
      paths: [
        {
          path: "src/lib.rs",
          fingerprint: "ab".repeat(64),
          resolution: { type: "select_term", term: 2 },
        },
        {
          path: "assets/logo.bin",
          fingerprint: "cd".repeat(64),
          resolution: { type: "delete" },
        },
      ],
    },
  );
});

test("atomic conflict commands reject omissions and foreign term indexes", () => {
  assert.throws(
    () =>
      buildConflictCommand(
        "12".repeat(64),
        [textConflict, structuralConflict],
        new Map([["src/lib.rs", { type: "select_term", term: 0 }]]),
      ),
    /Choose a resolution for assets\/logo\.bin/u,
  );
  assert.throws(
    () =>
      buildConflictCommand(
        "12".repeat(64),
        [textConflict],
        new Map([["src/lib.rs", { type: "select_term", term: 9 }]]),
      ),
    /term 9/u,
  );
});

test("uploaded conflict resolution keeps the scoped staged handle", () => {
  const command = buildConflictCommand(
    "12".repeat(64),
    [textConflict],
    new Map([
      [
        "src/lib.rs",
        {
          type: "uploaded_file",
          upload: "ef".repeat(32),
          executable: true,
          copy_from_term: 0,
        },
      ],
    ]),
  );
  assert.deepEqual(command.paths[0]!.resolution, {
    type: "uploaded_file",
    upload: "ef".repeat(32),
    executable: true,
    copy_from_term: 0,
  });
});
