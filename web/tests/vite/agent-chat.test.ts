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

// Live chat row projection is shared by the inbox and contextual sidebar.
import { activityRows } from "../../src/lib/ui/store-agent-execution/presentation.js";
test("retains recorded lifecycle clocks and separates original text from context", () => {
  const events = [
    {
      sequence: 1,
      observed_at_millis: 1000,
      committed: true,
      payload: {
        type: "opened",
        requestId: "open-s",
        controllerSessionId: "s",
      },
    },
    {
      sequence: 2,
      observed_at_millis: 2000,
      committed: true,
      payload: {
        type: "prompt",
        requestId: "p",
        text: "Authored",
        context: "<agent_context>facts</agent_context>",
      },
    },
    {
      sequence: 3,
      observed_at_millis: 3000,
      committed: true,
      payload: {
        type: "record",
        requestId: "p",
        record: { status: "completed" },
      },
    },
    {
      sequence: 4,
      observed_at_millis: 4000,
      committed: true,
      payload: {
        type: "opened",
        requestId: "open-s-2",
        controllerSessionId: "s",
      },
    },
  ];
  const rows = activityRows(events);
  expect(rows.map((row) => row.kind)).toEqual(["session", "user", "record"]);
  expect(rows[0].observedAtMillis).toBe(1000);
  expect(rows[2].observedAtMillis).toBe(3000);
  expect(rows[1].text).toBe("Authored");
  expect(rows[1].value.context).toBe("<agent_context>facts</agent_context>");
  const legacy = activityRows([
    {
      ...events[1],
      payload: {
        type: "prompt",
        requestId: "p",
        text: "captured prefix\nAuthored",
        authoredText: "Authored",
        context: "<agent_context>captured prefix</agent_context>",
      },
    },
  ]);
  expect(legacy[0].text).toBe("Authored");
});

test("context delivery updates one authored row without replacing its recorded clock or XML", () => {
  const prompt = {
    type: "prompt",
    requestId: "p",
    text: "Original message",
    userText: "Original message",
    context: "<agent_context>Captured facts</agent_context>",
    sentContext: {
      version: 1,
      delivery: "prepared",
      wireText: "Original message",
      wireContext: "<agent_context>Captured facts</agent_context>",
    },
  };
  const rows = activityRows([
    { sequence: 1, observed_at_millis: 2000, committed: true, payload: prompt },
    {
      sequence: 2,
      observed_at_millis: 4000,
      committed: true,
      payload: {
        ...prompt,
        type: "context_delivery",
        sentContext: { ...prompt.sentContext, delivery: "sent" },
      },
    },
  ]);
  expect(rows).toHaveLength(1);
  expect(rows[0].id).toBe("p/user");
  expect(rows[0].text).toBe("Original message");
  expect(rows[0].observedAtMillis).toBe(2000);
  expect(rows[0].value.context).toBe(prompt.context);
  expect(rows[0].value.sentContext).toEqual({
    ...prompt.sentContext,
    delivery: "sent",
  });
});

test("partial delivery keeps the original managed send and per-recipient receipts for explicit checks", async () => {
  const calls: unknown[] = [];
  const receipt = {
    request_id: "a".repeat(32),
    conversation_id: "c".repeat(32),
    comment_publication: { state: "published", request_id: "b".repeat(32) },
    agents: [
      {
        version: 1,
        request_id: "d".repeat(32),
        session_id: "first",
        state: "completed",
        error: null,
      },
      {
        version: 1,
        request_id: "e".repeat(32),
        session_id: "second",
        state: "failed",
        error: "dispatch_not_sent",
      },
    ],
  };
  const manager = new AgentExecution(
    () => true,
    async (_url, init) => {
      calls.push(JSON.parse(String(init?.body)));
      return Response.json(receipt);
    },
  );
  manager.value = status();
  try {
    manager.chat.showGeneral();
    await expect(
      manager.chat.send(
        "Work",
        ["exact-Helper"],
        undefined,
        receipt.request_id,
      ),
    ).rejects.toThrow("dispatch_not_sent");
    const original = JSON.parse(JSON.stringify(manager.chat.pending));
    expect(original.request_id).toBe(receipt.request_id);
    await expect(manager.chat.send("Work", ["exact-Helper"])).rejects.toThrow(
      /pending/,
    );
    await expect(manager.chat.retry()).rejects.toThrow("dispatch_not_sent");
    expect(calls).toEqual([original, original]);
    expect(manager.chat.receipts[receipt.request_id].agents).toHaveLength(2);
  } finally {
    manager.destroy();
  }
});

test("dispatching retains the exact request until an agent acknowledgement", async () => {
  const calls: unknown[] = [];
  let state = "dispatching";
  const id = "a".repeat(32);
  const manager = new AgentExecution(
    () => true,
    async (url, init) => {
      if (String(url).endsWith("/agents/chatgpt"))
        return Response.json({ supported: false, accounts: [], attempts: [] });
      if (!String(url).endsWith("/agents/chat")) return Response.json(status());
      calls.push(JSON.parse(String(init?.body)));
      return Response.json({
        request_id: id,
        conversation_id: "c".repeat(32),
        agents: [
          {
            version: 1,
            request_id: "d".repeat(32),
            session_id: "first",
            state,
            error: null,
          },
        ],
      });
    },
  );
  manager.value = status();
  try {
    manager.chat.showGeneral();
    await manager.chat.send("Work", ["exact-Helper"], undefined, id);
    expect(manager.chat.pending?.request_id).toBe(id);
    const original = JSON.parse(JSON.stringify(manager.chat.pending));
    await expect(manager.chat.send("Again", ["exact-Helper"])).rejects.toThrow(
      /pending/,
    );
    state = "started";
    await manager.chat.retry();
    expect(calls).toEqual([original, original]);
    expect(manager.chat.pending).toBeNull();
  } finally {
    manager.destroy();
  }
});

test("live acknowledgement refreshes the retained receipt without dispatching again", async () => {
  const request = "a".repeat(32),
    child = "b".repeat(32);
  const calls: string[] = [];
  const manager = new AgentExecution(
    () => true,
    async (url) => {
      calls.push(String(url));
      if (String(url).includes("/events?"))
        return Response.json({
          version: 1,
          events: [
            {
              sequence: 1,
              payload: { type: "started", requestId: child },
              committed: false,
            },
          ],
          next: null,
        });
      if (String(url).includes("/outcome?"))
        return Response.json({
          version: 1,
          request_id: child,
          session_id: "first",
          state: "started",
          error: null,
        });
      return Response.json({
        messages: [],
        profile: null,
        write_unavailable: null,
      });
    },
  );
  manager.value = status();
  manager.chat.open = true;
  manager.chat.selected = "chat";
  manager.value.sessions = [
    {
      id: "first",
      conversation_id: manager.chat.selected,
      repository: "repo",
      setup_id: "exact-Helper",
      state: "running",
    } as AgentStatus["sessions"][number],
  ];
  manager.chat.pending = {
    version: 1,
    request_id: request,
    conversation_id: manager.chat.selected,
    origin: {
      scope: "personal",
      home: null,
      thread: null,
      agent_target: null,
      context: "General conversation",
    },
    attachments: [],
    agent_ids: ["exact-Helper"],
    text: "Work",
    max_duration_ms: 1000,
  };
  manager.chat.receipts[request] = {
    request_id: request,
    conversation_id: manager.chat.selected,
    comment_publication: null,
    agents: [
      {
        version: 1,
        request_id: child,
        session_id: "first",
        state: "dispatching",
        error: null,
      },
    ],
  };
  try {
    await manager.chat.refresh();
    expect(manager.chat.pending).toBeNull();
    expect(manager.chat.receipts[request].agents[0].state).toBe("started");
    expect(calls.some((url) => url.endsWith("/agents/chat"))).toBe(false);
  } finally {
    manager.destroy();
  }
});
