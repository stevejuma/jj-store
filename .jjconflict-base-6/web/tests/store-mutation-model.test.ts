import assert from "node:assert/strict";
import test from "node:test";
import {
  buildMutationCommand,
  type StoreMutationFormValue,
} from "../src/lib/ui/store-mutations/model.js";

const commit = "12".repeat(64);
const otherCommit = "34".repeat(64);
const operation = "56".repeat(64);

function form(
  value: Partial<StoreMutationFormValue> = {},
): StoreMutationFormValue {
  return {
    description: "Updated description",
    workspace: "workspace",
    bookmark: "feature/native",
    target: commit,
    base: operation,
    expectedTargets: [otherCommit, commit],
    retainBookmarks: true,
    restoreDescendants: false,
    ...value,
  };
}

test("native mutation inputs reject abbreviated object identities", () => {
  assert.throws(
    () => buildMutationCommand("describe", form({ target: "1234abcd" })),
    /full native commit ID/u,
  );
  assert.throws(
    () => buildMutationCommand("reconcile", form({ base: "5678abcd" })),
    /full native operation ID/u,
  );
  assert.throws(
    () => buildMutationCommand("new", form({ target: "1234abcd" })),
    /full native parent commit ID/u,
  );
});

test("bookmark commands preserve exact expected target sets", () => {
  assert.deepEqual(buildMutationCommand("bookmark-move", form()), {
    type: "bookmark_move",
    name: "feature/native",
    expected: [otherCommit, commit],
    target: commit,
  });
  assert.deepEqual(buildMutationCommand("bookmark-delete", form()), {
    type: "bookmark_delete",
    name: "feature/native",
    expected: [otherCommit, commit],
  });
});

test("reconcile and abandon retain their native command controls", () => {
  assert.deepEqual(buildMutationCommand("reconcile", form()), {
    type: "reconcile_operations",
    base: operation,
  });
  assert.deepEqual(buildMutationCommand("abandon", form()), {
    type: "abandon",
    commits: [commit],
    retain_bookmarks: true,
    restore_descendants: false,
  });
});
