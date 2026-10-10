import { test } from "node:test";
import assert from "node:assert/strict";
import {
  groupSessionInboxItems,
  type SessionInboxItem,
} from "../src/lib/ui/community-inbox/model.js";
import { sessionItems } from "../src/lib/ui/store-agent-execution/session-presentation.js";
import type {
  AgentSession,
  AgentStatus,
} from "../src/lib/controller/agent-execution-types.js";

test("repository-backed conversations group by code attachments with their own operation and workspace, never their metadata home", () => {
  const session = {
    id: "chat",
    repository: "personal-home",
    conversation_id: "conversation",
    conversation_title: "Task",
    avatar: "",
    runner: "codex",
    state: "idle",
    connected: true,
    operation: "home-op",
    workspace_name: "home-workspace",
    repository_bindings: [
      {
        repository: "code-a",
        operation: "code-op-a",
        workspace_name: "code-workspace-a",
      },
      {
        repository: "code-b",
        operation: "code-op-b",
        workspace_name: "code-workspace-b",
      },
    ],
  } as AgentSession;
  const status = { sessions: [session], setups: [] } as unknown as AgentStatus;
  const items = sessionItems(status, undefined, false, [
    { id: "code-a", name: "demo/server-only" },
    { id: "code-b", name: "Second codebase" },
  ]);
  assert.deepEqual(
    groupSessionInboxItems(items).map((g) => g.repositoryId),
    ["code-a", "code-b"],
  );
  assert.equal(items[0].repository.name, "demo/server-only");
  assert.equal(
    items[0].repository.href,
    "#/repos/code-a/files?at_op=code-op-a&workspace=code-workspace-a",
  );
  assert.equal(
    items[1].repository.href,
    "#/repos/code-b/files?at_op=code-op-b&workspace=code-workspace-b",
  );
  session.repository_bindings = [];
  assert.equal(sessionItems(status)[0].repository.name, "Personal");
  assert.equal(session.repository, "personal-home");
});

test("repository groups retain service order, distinct identities and exact per-chat links", () => {
  const author = { id: "agent", name: "Agent", initials: "AG", handle: "" };
  const repository = (id: string, operation: string) => ({
    id,
    name: "Same name",
    label: "Open Same name",
    href: `#/repos/${id}/files?at_op=${operation}&workspace=workspace-${operation}`,
  });
  const items: SessionInboxItem[] = [
    {
      id: "one",
      author,
      context: "Codex",
      status: "Offline",
      repository: repository("repo-a", "one"),
    },
    {
      id: "two",
      author,
      context: "Cursor",
      status: "Online",
      repository: repository("repo-b", "two"),
    },
    {
      id: "three",
      author,
      context: "Codex",
      status: "Offline",
      repository: repository("repo-a", "three"),
    },
    { id: "draft", author, context: "New chat", status: "Draft" },
  ];
  const groups = groupSessionInboxItems(items);
  assert.deepEqual(
    groups.map((group) => [
      group.repositoryId,
      group.name,
      group.items.map((item) => item.id),
    ]),
    [
      ["repo-a", "Same name", ["one", "three"]],
      ["repo-b", "Same name", ["two"]],
      [undefined, "No repository", ["draft"]],
    ],
  );
  assert.strictEqual(groups[0].items[1], items[2]);
  assert.equal(
    groups[0].items[1].repository?.href,
    "#/repos/repo-a/files?at_op=three&workspace=workspace-three",
  );
  const renamed = groupSessionInboxItems(
    items.map((item) =>
      item.repository?.id === "repo-a"
        ? { ...item, repository: { ...item.repository, name: "Renamed" } }
        : item,
    ),
  );
  assert.equal(renamed[0].id, groups[0].id);
  assert.equal(renamed[0].name, "Renamed");
  assert.deepEqual(
    items.map((item) => item.id),
    ["one", "two", "three", "draft"],
  );
});

test("empty inboxes do not invent repository groups", () => {
  assert.deepEqual(groupSessionInboxItems([]), []);
});

test("pins form the first expandable group and collapse repository aliases into one canonical row", () => {
  const author = { id: "agent", name: "Agent", initials: "AG", handle: "" };
  const rows: SessionInboxItem[] = [
    {
      id: "shared:a",
      targetId: "home:shared",
      author,
      context: "Thread",
      status: "active",
      repository: { id: "a", name: "A", href: "#/repos/a", label: "Open A" },
    },
    {
      id: "shared:b",
      targetId: "home:shared",
      author,
      context: "Thread",
      status: "active",
      repository: { id: "b", name: "B", href: "#/repos/b", label: "Open B" },
    },
    { id: "other", author, context: "Other", status: "active" },
  ];
  const groups = groupSessionInboxItems(rows, ["home:shared"]);
  assert.equal(groups[0].id, "pinned");
  assert.equal(groups[0].name, "Pinned");
  assert.deepEqual(groups[0].items, [rows[0]]);
  assert.equal(groups[1].id, "unbound");
  assert.strictEqual(groups[0].items[0], rows[0]);
  assert.equal(groupSessionInboxItems(rows).length, 3);
});
