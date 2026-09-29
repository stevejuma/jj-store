import { expect, test } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import type {
  StoreDataSource,
  StoreReviewCommandRequest,
} from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreCollaboration } from "../../src/lib/controller/production-store-collaboration.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const id16 = (byte: string) => byte.repeat(16);
const id32 = (byte: string) => byte.repeat(32);
const session: Model<"AgentSessionSummary"> = {
  session_id: id16("11"),
  snapshot_id: id32("12"),
  integration: "codex",
  state: "ended",
  parent_session_id: null,
  coverage: {
    completeness: "complete",
    gaps: 0,
    redacted_fields: 0,
    truncated: false,
  },
  evidence_count: 1,
  event_batch_count: 1,
  transcript_count: 1,
  tool_count: 0,
};
const descriptor: Model<"AgentTranscriptDescriptor"> = {
  transcript_id: id32("13"),
  source_instance_id: id16("14"),
  source_generation: 1,
  source_format: "codex-jsonl",
  decoder_version: "v1",
  sanitizer_version: "v1",
  representation: "sanitized_native",
  logical_byte_length: 8,
  record_count: 1,
  gaps: 0,
};
const thread: Model<"ReviewThreadSummary"> = {
  thread_id: id16("21"),
  snapshot_id: id32("22"),
  original_anchor_id: id32("23"),
  event_page_root_id: id32("24"),
  status: "open",
  selected_workflow_action: null,
  thread_version: 1,
  content_version: 1,
  workflow_version: 3,
  comment_count: 1,
  last_event_sequence: 1,
};
const reviewDetail = (): Model<"ReviewThreadDetail"> => ({
  version: 1,
  snapshot: {
    repo_id: id16("31"),
    epoch: id16("32"),
    published_seq: 4,
    manifest_revision: 5,
    review_root: id32("33"),
    review_state_version: 2,
  },
  thread,
  anchor: {
    object_id: id32("34"),
    value: {
      schema_version: 1,
      repository_id: id16("31"),
      target: { target: "repository" },
      created_context: {
        publication_position: 3,
        code_state_version: 1,
        agent_root: null,
      },
      sensitivity: "repository",
    },
  },
  comments: [],
  events: [],
});

const agentDetail = (): Model<"AgentSessionDetail"> => ({
  version: 1,
  snapshot: {
    repo_id: id16("31"),
    epoch: id16("32"),
    published_seq: 4,
    manifest_revision: 5,
    agent_root: id32("35"),
    agent_state_version: 2,
  },
  session,
  evidence: [],
  event_batches: [],
  transcripts: [descriptor],
  tools: [],
});

function harness(
  outcome: "committed" | "unknown" = "committed",
  current: () => boolean = () => true,
) {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  const commands: StoreReviewCommandRequest[] = [];
  const source = {
    repository: id16("31"),
    async agentSessions(cursor?: string) {
      return {
        version: 1,
        snapshot: {
          repo_id: id16("31"),
          epoch: id16("32"),
          published_seq: 4,
          manifest_revision: 5,
          agent_root: id32("35"),
          agent_state_version: 2,
        },
        sessions: [session],
        next_cursor: cursor ? null : "agent-next",
      };
    },
    async agentSession() {
      return agentDetail();
    },
    async agentTranscript() {
      return {
        version: 1,
        snapshot: {
          repo_id: id16("31"),
          epoch: id16("32"),
          published_seq: 4,
          manifest_revision: 5,
          agent_root: id32("35"),
          agent_state_version: 2,
        },
        session_id: session.session_id,
        transcript_id: descriptor.transcript_id,
        source_instance_id: descriptor.source_instance_id,
        source_generation: 1,
        chunks: [{ sequence: 1, media_type: "text" as const, content: "Done" }],
        next_cursor: null,
      };
    },
    async reviewThreads() {
      return {
        version: 1,
        snapshot: reviewDetail().snapshot,
        threads: [thread],
        next_cursor: null,
      };
    },
    async reviewThread() {
      return reviewDetail();
    },
    async applyReviewCommand(request: StoreReviewCommandRequest) {
      commands.push(request);
      if (outcome === "unknown")
        return {
          status: "outcome_unknown" as const,
          request_id: request.request_id,
        };
      return {
        status: "committed" as const,
        receipt: {
          version: 1,
          request_id: request.request_id,
          publication_id: id16("41"),
          request_digest: id32("42"),
          accepted: {
            epoch: id16("32"),
            position: 5,
            code_state_version: 1,
            heads: ["43".repeat(64)],
          },
          result: {
            schema_version: 1,
            thread_id: thread.thread_id,
            comment_id: id16("44"),
            action_id: null,
            thread_snapshot: {
              kind: "thread_snapshot" as const,
              id: id32("45"),
            },
            comment_revision: null,
            event: { kind: "thread_event" as const, id: id32("46") },
          },
        },
      };
    },
    async reviewCommandStatus(requestId: string) {
      return { status: "outcome_unknown" as const, request_id: requestId };
    },
  } as StoreDataSource;
  const controller = createProductionStoreCollaboration({
    data,
    history,
    source: () => source,
    version: () => 1,
    current,
    report: (cause) => {
      throw cause;
    },
    requestId: () => id16("51"),
  });
  return { controller, data, history, commands, source };
}

test("agent sessions load a selected transcript without mixing identities", async () => {
  const { controller, data, history } = harness();
  await controller.loadAgents();
  await controller.selectAgentSession(session.session_id);
  expect(data.agentSessions).toEqual([session]);
  expect(data.agentSession?.session.session_id).toBe(session.session_id);
  expect(data.agentTranscript[0]?.content).toBe("Done");
  expect(history.route.agentSessionId).toBe(session.session_id);
  expect(data.status.transcript).toBe("ready");
});

test("review replies send exact workflow preconditions and refresh detail", async () => {
  const { controller, commands, data } = harness();
  await controller.loadReviews();
  await controller.selectReviewThread(thread.thread_id);
  await controller.reply("Please preserve the pinned root");
  expect(commands[0]).toMatchObject({
    request_id: id16("51"),
    command: {
      command: "append_reply",
      thread_id: thread.thread_id,
      expected_workflow_version: 3,
      body: { text: "Please preserve the pinned root" },
    },
  });
  expect(data.status.reviewAction).toBe("ready");
  expect(data.reviewActionRequestId).toBeNull();
});

test("unknown review outcomes retain the durable request identity", async () => {
  const { controller, data } = harness("unknown");
  await controller.loadReviews();
  await controller.selectReviewThread(thread.thread_id);
  await controller.setResolved(true);
  expect(data.status.reviewAction).toBe("unavailable");
  expect(data.reviewActionRequestId).toBe(id16("51"));
  expect(data.reviewActionMessage).toContain("outcome is unknown");
});

test("late collaboration details cannot repopulate an invalidated session", async () => {
  let active = true;
  let resolveAgent!: (value: Model<"AgentSessionDetail">) => void;
  let resolveReview!: (value: Model<"ReviewThreadDetail">) => void;
  const { controller, data, source } = harness("committed", () => active);
  source.agentSession = () =>
    new Promise((resolve) => {
      resolveAgent = resolve;
    });
  source.reviewThread = () =>
    new Promise((resolve) => {
      resolveReview = resolve;
    });

  const agentRead = controller.loadAgent(session.session_id);
  active = false;
  resolveAgent(agentDetail());
  await agentRead;
  expect(data.agentSession).toBeUndefined();

  active = true;
  const reviewRead = controller.loadReview(thread.thread_id);
  active = false;
  resolveReview(reviewDetail());
  await reviewRead;
  expect(data.reviewThread).toBeUndefined();
});
