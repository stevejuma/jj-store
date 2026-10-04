import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import {
  workCommentRows,
  workMessage,
} from "../src/lib/ui/store-work/work-presentation.js";
const actor = {
  created_by: "original-writer",
  updated_by: "later-writer",
  created_at_millis: 1000,
  updated_at_millis: 2000,
};
const comment = (
  id: string,
  reply_to: string | null = null,
): Model<"WorkCommentV1"> => ({
  schema_version: 1,
  id,
  task: "native-task",
  body: id,
  reply_to,
  version: 1,
  actor,
});

test("native discussion preserves branch order, paged parent identity and tombstone replies", () => {
  const rows = workCommentRows([
    comment("root"),
    comment("other"),
    comment("reply", "root"),
    { ...comment("removed", "reply"), body: null },
    comment("child", "removed"),
    comment("sibling", "root"),
    comment("fragment", "earlier-page"),
  ]);
  assert.deepEqual(
    rows.map((r) => r.comment.id),
    ["root", "reply", "removed", "child", "sibling", "other", "fragment"],
  );
  assert.equal(rows[3]!.comment.reply_to, "removed");
  assert.equal(rows[3]!.indentRem, 6.75);
  assert.equal(rows.at(-1)!.comment.reply_to, "earlier-page");
  assert.equal(rows.at(-1)!.indentRem, 0);
  assert.ok(rows[1]!.guideIndentRems.includes(0));
});
test("discussion caps visual depth without changing native reply relationships", () => {
  const comments = Array.from({ length: 4096 }, (_, index) =>
    comment(String(index), index ? String(index - 1) : null),
  );
  const last = workCommentRows(comments).at(-1)!;
  assert.equal(last.depth, 4095);
  assert.equal(last.indentRem, 13.5);
  assert.equal(last.comment.reply_to, "4094");
});
test("Community message projection preserves creation actor, native identity and tombstone", () => {
  const message = workMessage({
    ...comment("exact-native-id", "exact-parent"),
    version: 2,
    body: null,
  });
  assert.equal(message.id, "exact-native-id");
  assert.equal(message.parentMessageId, "exact-parent");
  assert.equal(message.authorId, "original-writer");
  assert.equal(message.createdAt, "1970-01-01T00:00:01.000Z");
  assert.equal(message.edited, true);
  assert.match(message.content, /Comment removed/);
  assert.equal(message.sourceEvent, undefined);
});
