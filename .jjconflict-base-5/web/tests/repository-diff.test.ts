import assert from "node:assert/strict";
import test from "node:test";
import {
  unavailableDiffMessage,
  type DiffUnavailableReason,
} from "../src/lib/projects/repository-diff.js";

test("only confirmed binary content uses a binary label", () => {
  const reasons: DiffUnavailableReason[] = [
    "binary",
    "complexity",
    "too_large",
    "conflict",
    "multiple_sources",
    "structural",
    "not_requested",
  ];
  for (const reason of reasons) {
    const result = unavailableDiffMessage(reason);
    assert.equal(result.title.includes("Binary"), reason === "binary");
    assert.ok(result.description);
  }
  assert.match(
    unavailableDiffMessage("complexity").description,
    /diff work limit/u,
  );
  assert.match(unavailableDiffMessage("too_large").description, /size limit/u);
  assert.equal(unavailableDiffMessage().title, "Text diff unavailable");
  assert.match(unavailableDiffMessage().description, /No text diff/u);
});
