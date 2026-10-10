import type {
  AgentStatus,
  AgentEvent,
} from "../../web/src/lib/controller/agent-execution-types.js";
import type { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
import {
  fixtureSession,
  fixtureJsonResponse as json,
} from "./agent-execution-session.js";
export const changeRepository = "aa".repeat(16);
export const changeOperation = "11".repeat(64);
export const changeBase = "22".repeat(20);
export const changeCommit = "33".repeat(20);

export function stageFleetConversation(
  manager: AgentExecution,
  status: AgentStatus,
  changes: boolean,
  unread: boolean,
) {
  manager.chat.showGeneral("agent-0");
  const completed = completedFleetConversation(
    status,
    manager.chat.selected,
    manager.chat.draft!.origin,
    changes,
    unread,
  );
  manager.value = completed.status;
  manager.chat.events[completed.session.id] = completed.visibleEvents;
  return completed;
}

export function fleetChangesResponse(init?: RequestInit) {
  const body = JSON.parse(String(init?.body));
  const oldTurn = body.operation === "55".repeat(64);
  const operation = oldTurn ? "55".repeat(64) : changeOperation;
  const from = oldTurn ? "66".repeat(20) : changeBase;
  const to = oldTurn ? "77".repeat(20) : changeCommit;
  if (body.request.kind === "commit_summary")
    return json({
      snapshot: {
        version: 1,
        repository: changeRepository,
        profile: "git_sha1_jj045",
        operation,
        generation: 1,
        heads: [operation],
      },
      data: {
        kind: "commit_summary",
        data: {
          key: { kind: "commit", id: to },
          data: { kind: "commit", data: { change_id: "88".repeat(16) } },
        },
      },
      next_cursor: null,
    });
  const diff = body.request.kind === "diff";
  const change = (file: string) => ({
    path: file,
    before: [],
    after: [],
    sources: [],
    content: diff
      ? {
          status: "text",
          hunks: [
            {
              kind: "different",
              before: oldTurn ? "earlier\n" : "old\n",
              after: oldTurn ? "historic\n" : "new\nmore\n",
              before_start: 0,
              after_start: 0,
            },
          ],
        }
      : { status: "unavailable", reason: "not_requested" },
  });
  return json({
    snapshot: {
      version: 1,
      repository: changeRepository,
      profile: "git_sha1_jj045",
      operation,
      generation: 1,
      heads: [operation],
    },
    data: {
      kind: body.request.kind,
      data: {
        version: 1,
        from,
        to,
        path: body.request.query.path,
        changes: diff
          ? [change(body.request.query.path)]
          : Array.from({ length: 20 }, (_, n) =>
              change(`src/example-${n + 1}.ts`),
            ),
      },
    },
    next_cursor: null,
  });
}

export function completedFleetConversation(
  status: AgentStatus,
  conversation: string,
  origin: AgentStatus["conversations"][number]["origin"],
  completedChanges: boolean,
  unread = false,
) {
  const session = fixtureSession({
    repository: changeRepository,
    operation: changeOperation,
    commit: changeCommit,
    status: "Online",
    permission: false,
    workspacePath: "/device/chat",
    lastActivityAt: Date.now(),
    preferences: status.setups[0]!.preferences,
    modeReady: true,
    advertisedModels: false,
  });
  session.setup_id = "agent-0";
  session.title = "Helper";
  session.avatar = "🤖";
  session.conversation_id = conversation;
  session.published_sequence = 6;
  session.repository_bindings = completedChanges
    ? [
        {
          repository: changeRepository,
          target_id: "workspace",
          profile: "local_jj_git",
          workspace: "agent-helper",
          selected_operation: changeOperation,
          base_commit: changeBase,
          operation: changeOperation,
          commit: changeCommit,
        },
      ]
    : [];
  const nextStatus: AgentStatus = {
    ...status,
    sessions: [session],
    live_agents: [
      {
        agent_id: "agent-0",
        state: "idle",
        enabled: true,
        generation: 1,
        error: null,
        restart_required: false,
        inference_host: null,
      },
    ],
    conversations: [
      {
        id: conversation,
        origin,
        home: changeRepository,
        discussion_thread: null,
        metadata_auto_eligible: false,
        participants: [session.id],
      },
    ],
  };
  const events: AgentEvent[] = [
    {
      sequence: 1,
      observed_at_millis: Date.now(),
      committed: true,
      payload: {
        type: "record",
        requestId: "turn",
        record: { status: "completed" },
      },
    },
    {
      sequence: 2,
      observed_at_millis: Date.now(),
      committed: true,
      payload: {
        type: "repository_snapshot",
        requestId: "turn",
        repository: changeRepository,
        state: "completed",
        operation: changeOperation,
        commit: changeCommit,
      },
    },
  ];
  if (completedChanges) {
    const boundary = (
      sequence: number,
      type: string,
      requestId: string,
      operation: string,
      commit: string,
    ): AgentEvent => ({
      sequence,
      observed_at_millis: Date.now(),
      committed: true,
      payload: {
        type,
        requestId,
        repository: changeRepository,
        state: "completed",
        operation,
        commit,
      },
    });
    events.unshift(
      boundary(0, "repository_turn_start", "turn", changeOperation, changeBase),
    );
    events.unshift(
      boundary(
        -3,
        "repository_turn_start",
        "earlier-turn",
        "55".repeat(64),
        "66".repeat(20),
      ),
      {
        sequence: -2,
        observed_at_millis: Date.now(),
        committed: true,
        payload: {
          type: "record",
          requestId: "earlier-turn",
          record: { status: "completed" },
        },
      },
      boundary(
        -1,
        "repository_snapshot",
        "earlier-turn",
        "55".repeat(64),
        "77".repeat(20),
      ),
    );
    events.forEach((event, index) => {
      event.sequence = index + 1;
    });
  }
  const visibleEvents = events;
  if (unread) {
    session.last_sequence = 3;
    return {
      status: nextStatus,
      session,
      visibleEvents,
      events: [
        ...events,
        {
          sequence: 3,
          committed: false,
          observed_at_millis: Date.now(),
          payload: { type: "ready" },
        },
      ],
    };
  }
  return { status: nextStatus, session, events, visibleEvents };
}

export function fleetTargetsResponse(repository: string, multiple: boolean) {
  return json(
    multiple
      ? [0, 1].map((n) => ({
          id: `workspace-${n}`,
          repository,
          name: n ? "Feature workspace" : "Original workspace",
          folder: `/device/workspace-${n}`,
          profile: "local_jj_git",
          operation: String(n + 1).repeat(128),
          commit: String(n + 1).repeat(40),
        }))
      : [],
  );
}

export function createFleetChangesResponder(failure: boolean) {
  let attempts = 0;
  return (init?: RequestInit) => {
    const body = JSON.parse(String(init?.body));
    if (
      failure &&
      body.operation === changeOperation &&
      body.request.kind === "diff_paths" &&
      attempts++ === 0
    )
      return new Response(JSON.stringify({ code: "unavailable" }), {
        status: 503,
        headers: { "content-type": "application/json" },
      });
    return fleetChangesResponse(init);
  };
}
