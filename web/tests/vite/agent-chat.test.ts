import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import { agentMentionToken } from "../../src/lib/community/agent-mentions.js";
import { agentLiveState } from "../../src/lib/controller/agent-live-status.js";
import type { AgentStatus } from "../../src/lib/controller/agent-execution-types.js";
import { chatContext } from "../../src/lib/controller/agent-chat-context.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
const status = (): AgentStatus => ({
  version: 1,
  runtime: "ready",
  revision: 0,
  setups: [
    {
      id: "exact-Helper",
      name: "Helper",
      avatar: "🤖",
      description: "",
      runner: "codex",
      instructions: "",
      preferences: { model: null, effort: null, mode: "ask" },
    },
  ],
  sessions: [],
  runners: [],
  live_agents: [],
  conversations: [],
});
test("opening any scoped chat is draft-only, Send resolves exact tags and retains ambiguous identity", async () => {
  const calls: Record<string, unknown>[] = [];
  const fetcher = vi.fn(async (_input, init) => {
    calls.push(JSON.parse(String(init?.body)));
    throw new TypeError("Disconnected");
  });
  const manager = new AgentExecution(() => true, fetcher);
  manager.value = status();
  try {
    manager.chat.showGeneral("exact-Helper");
    const id = manager.chat.selected;
    expect(fetcher).not.toHaveBeenCalled();
    manager.chat.draft!.text += "Do work";
    await expect(
      manager.chat.send(manager.chat.draft!.text, ["exact-Helper"]),
    ).rejects.toThrow("Disconnected");
    expect(calls[0]).toMatchObject({
      conversation_id: id,
      agent_ids: ["exact-Helper"],
      attachments: [],
      text: "Do work",
    });
    const retained = manager.chat.pending;
    manager.chat.close();
    manager.chat.show({
      scope: "personal",
      home: null,
      thread: null,
      agent_target: null,
      context: "General conversation",
    });
    expect(manager.chat.selected).toBe(id);
    expect(manager.chat.draft!.text).toContain("Do work");
    await expect(manager.chat.retry()).rejects.toThrow();
    expect(calls[1]).toEqual(calls[0]);
    expect(manager.chat.pending?.request_id).toBe(retained?.request_id);
    expect(manager.chat.draft!.text).toContain(
      agentMentionToken("exact-Helper"),
    );
  } finally {
    manager.destroy();
  }
});
test("live avatars use exact IDs and refresh failure is unknown", () => {
  const value = status();
  value.live_agents = [
    {
      agent_id: "exact-Helper",
      state: "attention",
      enabled: true,
      generation: 1,
      error: null,
    },
  ];
  expect(agentLiveState(value, "exact-Helper")).toBe("attention");
  expect(agentLiveState(value, "exact-helper")).toBe("unknown");
  expect(agentLiveState(value, "exact-Helper", true)).toBe("unknown");
});
test("repository chat pins the visible revision and project chat does not use home pins for linked code", () => {
  const state = {
    route: {
      page: "files",
      repository: "code",
      snapshotOperationId: "code-operation",
      commitId: null,
    },
    snapshot: { id: "visible-revision" },
  };
  const controller = { state } as unknown as StoreController;
  expect(chatContext(controller).attachments).toEqual([
    {
      repository: "code",
      target_id: null,
      operation: "code-operation",
      commit: "visible-revision",
    },
  ]);
  Object.assign(state.route, {
    page: "projects",
    projectId: "project",
    workHome: "metadata",
    workRepository: "linked-code",
    snapshotOperationId: "metadata-operation",
  });
  expect(chatContext(controller).attachments).toEqual([
    {
      repository: "linked-code",
      target_id: null,
      operation: null,
      commit: null,
    },
  ]);
});
test("workspace discovery failure does not turn an authoritative idle avatar unknown", async () => {
  const value = status();
  value.live_agents = [
    {
      agent_id: "exact-Helper",
      state: "idle",
      enabled: true,
      generation: 2,
      error: null,
    },
  ];
  let unavailable = true;
  const manager = new AgentExecution(
    () => true,
    async (input) => {
      if (String(input).includes("/targets")) {
        if (unavailable) throw new TypeError("Discovery unavailable");
        return Response.json([]);
      }
      return Response.json(value);
    },
  );
  try {
    await manager.refresh();
    expect(manager.connectionError).toBe("");
    expect(manager.targetError).toContain("workspaces unavailable");
    expect(
      agentLiveState(manager.value, "exact-Helper", !!manager.connectionError),
    ).toBe("idle");
    unavailable = false;
    await manager.refresh();
    expect(manager.targetError).toBe("");
  } finally {
    manager.destroy();
  }
});

test("workspace choices use exact target pins and unresolved Send freezes selections", async () => {
  const manager = new AgentExecution(
    () => true,
    async () => {
      throw new TypeError("Disconnected");
    },
  );
  manager.value = status();
  manager.context.repository = "legacy-only";
  manager.targets = [0, 1].map((n) => ({
    id: `target-${n}`,
    repository: "code",
    name: `Workspace ${n}`,
    folder: `/device/${n}`,
    profile: "local_jj_git",
    operation: "1".repeat(128),
    commit: String(n + 1).repeat(40),
  }));
  try {
    manager.chat.showGeneral();
    manager.chat.attach("code");
    await expect(
      manager.chat.send("Do work", ["exact-Helper"]),
    ).rejects.toThrow("Choose a workspace");
    manager.chat.selectWorkspace("code", "target-1");
    await expect(
      manager.chat.send("Do work", ["exact-Helper"]),
    ).rejects.toThrow("Disconnected");
    const retained = structuredClone(
      JSON.parse(JSON.stringify(manager.chat.pending)),
    );
    manager.chat.selectWorkspace("code", "target-0");
    manager.chat.selectRepository("code", "another");
    manager.chat.removeAttachment("code");
    manager.chat.attach("another");
    expect(manager.chat.draft!.attachments).toEqual(retained.attachments);
    expect(manager.chat.pending).toEqual(retained);
    expect(retained.attachments[0]).toMatchObject({
      repository: "code",
      target_id: "target-1",
      operation: "1".repeat(128),
      commit: "2".repeat(40),
    });
  } finally {
    manager.destroy();
  }
});

test("captured context retains bounded pinned file text and project content", () => {
  const state = {
    route: {
      page: "files",
      repository: "code",
      path: "readme.md",
      commitId: "revision",
    },
    status: { files: "ready" },
    snapshot: { id: "revision" },
    blob: { path: "readme.md", contents: "🐈".repeat(5000) },
  };
  const controller = { state } as unknown as StoreController;
  const context = JSON.parse(chatContext(controller).origin.context);
  expect(
    new TextEncoder().encode(context.file.content).length,
  ).toBeLessThanOrEqual(8192);
  expect(context.file.content).not.toContain("�");
  expect(context.file.truncated).toBe(true);
  state.snapshot.id = "old-revision";
  expect(
    JSON.parse(chatContext(controller).origin.context).file,
  ).toBeUndefined();
});

test("captured task context preserves full native version integers", () => {
  const controller = {
    state: {
      route: {
        page: "projects",
        repository: "home",
        projectId: "project",
        taskId: "task",
      },
    },
    work: {
      state: {
        project: {
          id: "project",
          title: "Project",
          body: "Description",
          version: 9007199254740993n,
        },
        task: {
          id: "task",
          title: "Task",
          body: "Task details",
          version: 9007199254740995n,
        },
      },
    },
  } as unknown as StoreController;
  expect(chatContext(controller).origin.context).toContain(
    '"version": 9007199254740993',
  );
  expect(chatContext(controller).origin.context).toContain(
    '"version": 9007199254740995',
  );
});
