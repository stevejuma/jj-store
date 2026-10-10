import { test } from "node:test";
import assert from "node:assert/strict";
import {
  filterInboxItems,
  emptyInboxFilters,
  inboxFilterOptions,
} from "../src/lib/ui/community-inbox/inbox-filters.js";
import type { SessionInboxItem } from "../src/lib/ui/community-inbox/model.js";
const author = { id: "user", name: "Reviewer", initials: "RE", handle: "" };
const row = (
  id: string,
  repo?: string,
  agents: string[] = [],
  status = "active",
): SessionInboxItem => ({
  id,
  targetId: id.split(":")[0],
  author,
  label: `Title ${id}`,
  context: "Conversation",
  status,
  agents: agents.map((id) => ({ id, name: "Same agent name" })),
  repository: repo
    ? {
        id: repo,
        name: "Same repository name",
        label: "Open repository",
        href: `#/repos/${repo}/files?at_op=exact`,
      }
    : undefined,
});
const items = [
  row("shared:a", "a", ["one", "two"]),
  row("shared:b", "b", ["one", "two"]),
  row("other", "a", ["two"], "archived"),
  row("unbound"),
];
test("facet values use exact identities and OR within AND across groups without replacing canonical aliases", () => {
  const result = filterInboxItems(items, "TITLE shared", {
    ...emptyInboxFilters(),
    repositories: ["a", "b"],
    agents: ["one", "foreign"],
  });
  assert.deepEqual(result, items.slice(0, 2));
  assert.strictEqual(result[0], items[0]);
  assert.equal(result[0].targetId, result[1].targetId);
  assert.equal(result[0].repository?.href, "#/repos/a/files?at_op=exact");
  assert.equal(
    filterInboxItems(items, "", {
      ...emptyInboxFilters(),
      repositories: ["a"],
      agents: ["one"],
      statuses: ["archived"],
    }).length,
    0,
  );
  assert.deepEqual(
    inboxFilterOptions(items, "agents").map((o) => o.id),
    ["one", "two"],
  );
  assert.deepEqual(
    inboxFilterOptions(items, "repositories")
      .map((o) => o.id)
      .sort(),
    ["", "a", "b"],
  );
});
test("unbound, unavailable selections and new loaded pages retain explicit filtering", () => {
  const filters = { ...emptyInboxFilters(), repositories: [""] };
  assert.deepEqual(filterInboxItems(items, "", filters), [items[3]]);
  const selected = { ...emptyInboxFilters(), agents: ["missing"] };
  assert.deepEqual(filterInboxItems(items, "", selected), []);
  const added = row("added", "a", ["missing"]);
  assert.deepEqual(filterInboxItems([...items, added], "", selected), [added]);
  assert.deepEqual(selected.agents, ["missing"]);
});
test("loaded status views respect lifecycle and current service presence", () => {
  assert.deepEqual(
    filterInboxItems(items, "", emptyInboxFilters(), "archived"),
    [items[2]],
  );
  assert.deepEqual(filterInboxItems(items, "", emptyInboxFilters(), "active"), [
    items[0],
    items[1],
    items[3],
  ]);
  const sessions = [
    row("online", undefined, [], "Online"),
    row("offline", undefined, [], "Offline"),
    row("working", undefined, [], "Working"),
  ];
  assert.deepEqual(
    filterInboxItems(sessions, "", emptyInboxFilters(), "connected").map(
      (i) => i.id,
    ),
    ["online", "working"],
  );
});
