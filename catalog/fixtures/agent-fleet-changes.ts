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
              before: "old\n",
              after: "new\nmore\n",
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
      operation: changeOperation,
      generation: 1,
      heads: [changeOperation],
    },
    data: {
      kind: body.request.kind,
      data: {
        version: 1,
        from: changeBase,
        to: changeCommit,
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
  session.published_sequence = 2;
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
