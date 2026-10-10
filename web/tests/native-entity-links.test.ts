import assert from "node:assert/strict";
import test from "node:test";
import { parseCommunityEntityLink } from "../src/lib/community/community-links.js";
import { communityEntityLinkMatches } from "../src/lib/community/community-link-normalize.js";
import { composerSlashAutocompleteItems } from "../src/lib/ui/community-chat-composer/composer-slash-catalog.js";
import { emptyComposerSlashSession } from "../src/lib/ui/community-chat-composer/composer-slash-session.js";
import { composerSlashInsert } from "../src/lib/ui/community-chat-composer/composer-slash-insert.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";
const repository = "ab".repeat(16),
  thread = "cd".repeat(16),
  message = "ef".repeat(16);
test("Thread references normalize legacy channels while Review thread identity remains distinct", () => {
  const link = parseCommunityEntityLink(
    `#/repos/${repository}/threads?channel=${thread}&reply=${message}`,
  )!;
  assert.equal(link.kind, "message");
  assert.match(link.href, new RegExp(`thread=${thread}`));
  assert.doesNotMatch(link.href, /channel=/);
  const review = parseCommunityEntityLink(
    `#/repos/${repository}/reviews?at_op=${"12".repeat(64)}&thread=${thread}`,
  )!;
  assert.equal(review.route.reviewThreadId, thread);
  assert.equal(review.route.threadId, undefined);
  assert.equal(
    storeRouteFromHash(
      `#/repos/${repository}/threads?thread=${thread}&channel=${thread}`,
    ).kind,
    "invalid",
  );
});
test("slash picker inserts useful native titles and keeps complete IDs in Markdown routes", () => {
  const href = `#/repos/${repository}/threads?thread=${thread}`;
  const catalog = {
    ...emptyComposerSlashSession(),
    threads: [{ id: thread, label: "Planning", href, detail: "Repository" }],
  };
  assert.equal(
    composerSlashAutocompleteItems("channel/plan", catalog)[0]!.label,
    "Planning",
  );
  const text = composerSlashInsert(
    composerSlashAutocompleteItems("threads/plan", catalog)[0]!,
  );
  assert.equal(text, `[Planning](${href}) `);
  assert.equal(communityEntityLinkMatches(text)[0]!.href, href);
  assert.ok(
    composerSlashAutocompleteItems("", catalog).some(
      (row) => row.label === "Threads",
    ),
  );
  assert.ok(
    !composerSlashAutocompleteItems("", catalog).some(
      (row) => row.label === "Channels",
    ),
  );
});
test("native comments, commits and operations cross-link without remapping identity", () => {
  const project = "11".repeat(16),
    task = "22".repeat(16),
    comment = "33".repeat(16);
  const hash = `#/projects?home=${repository}&project=${project}&task=${task}&comment=${comment}`;
  const link = parseCommunityEntityLink(hash)!;
  assert.equal(link.route.commentId, comment);
  assert.equal(link.route.taskId, task);
  assert.equal(storeRouteFromHash(storeRouteToHash(link.route)!).kind, "route");
  assert.equal(parseCommunityEntityLink("javascript:alert(1)"), undefined);
});
test("canonical Thread and reply links retain exact identity in composer chips", () => {
  for (const href of [
    `#/threads/${repository}/${thread}`,
    `#/threads/${repository}/${thread}?reply=${message}`,
  ]) {
    const text = `[Planning](${href})`;
    assert.deepEqual(communityEntityLinkMatches(text), [
      { value: href, href, start: 11, end: 11 + href.length },
    ]);
    const link = parseCommunityEntityLink(href)!;
    assert.equal(link.route.repository, repository);
    assert.equal(link.route.threadId, thread);
    assert.equal(link.kind, href.includes("reply=") ? "message" : "channel");
  }
});
