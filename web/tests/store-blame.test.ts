import assert from "node:assert/strict";
import test from "node:test";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";
import {
  blameCommitRoute,
  blameChunks,
  sourceLines,
  blameSizeReason,
} from "../src/lib/ui/store-blame/blame-model.js";
const hash = `#/repos/${"ab".repeat(16)}/files?at_op=${"12".repeat(64)}&commit=${"34".repeat(20)}&file=src%2Ftest.ts&directory=src`;
test("file display routes restore and reject invalid contexts", () => {
  for (const mode of ["source", "preview", "blame"]) {
    const parsed = storeRouteFromHash(`${hash}&file_view=${mode}`);
    assert.equal(parsed.kind, "route");
    if (parsed.kind !== "route") return;
    assert.equal(parsed.route.fileDisplay, mode);
    assert.equal(
      storeRouteFromHash(storeRouteToHash(parsed.route)!).kind,
      "route",
    );
  }
  for (const invalid of [
    hash + "&file_view=x",
    hash + "&file_view=blame&file_view=source",
    hash.replace("/files?", "/log?") + "&file_view=blame",
    hash.replace(/&commit=[^&]+/, "") + "&file_view=blame",
    hash.replace(/&file=[^&]+/, "") + "&file_view=blame",
  ])
    assert.equal(storeRouteFromHash(invalid).kind, "invalid");
});
test("blame links keep operation and path but clear incompatible context", () => {
  const parsed = storeRouteFromHash(hash);
  assert.equal(parsed.kind, "route");
  if (parsed.kind !== "route") return;
  const route = blameCommitRoute(
    {
      ...parsed.route,
      bookmark: "main",
      fileDisplay: "blame",
      historyQuery: "all()",
    },
    "56".repeat(20),
  );
  assert.equal(route.page, "log");
  assert.equal(route.path, "src/test.ts");
  assert.equal(route.snapshotOperationId, parsed.route.snapshotOperationId);
  assert.equal(route.commitId, "56".repeat(20));
  assert.equal(route.bookmark, "");
  assert.equal(route.fileDisplay, undefined);
  assert.equal(route.historyQuery, undefined);
  assert.equal(storeRouteFromHash(storeRouteToHash(route)!).kind, "route");
});
test("line counts, unknown origins and bounded continuation groups", () => {
  assert.deepEqual(sourceLines(""), []);
  assert.deepEqual(sourceLines("a\nb\n"), ["a", "b"]);
  assert.deepEqual(sourceLines("a\nb"), ["a", "b"]);
  const rows = blameChunks(100, [
    { start: 1, end: 81, commitId: "a" },
    { start: 81, end: 90, commitId: "a" },
  ]);
  assert.ok(rows.every((row) => row.end - row.start <= 32));
  assert.deepEqual(
    rows.map((row) => [row.start, row.end, row.commitId]),
    [
      [1, 33, "a"],
      [33, 65, "a"],
      [65, 90, "a"],
      [90, 101, undefined],
    ],
  );
  assert.equal(rows[1]!.continued, true);
});

test("blame admission explains UTF-8 byte and line bounds", () => {
  assert.equal(blameSizeReason("a".repeat(256 * 1024)), undefined);
  assert.match(blameSizeReason("é".repeat(128 * 1024 + 1))!, /256 KiB/);
  assert.equal(blameSizeReason("a\n".repeat(32_768)), undefined);
  assert.match(blameSizeReason("a\n".repeat(32_769))!, /32,768 lines/);
});
