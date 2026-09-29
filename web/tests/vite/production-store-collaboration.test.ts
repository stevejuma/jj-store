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
  first_activity_at_millis: 1_725_000_000_000,
  last_activity_at_millis: 1_725_000_125_000,
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
  anchor: {
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
  event_page_root_id: id32("24"),
  status: "open",
  selected_workflow_action: null,
  thread_version: 1,
  content_version: 1,
  workflow_version: 3,
  comment_count: 1,
  last_event_sequence: 1,
};
const commit = "61".repeat(64);
const change = id16("62");
const revisionThread: Model<"ReviewThreadSummary"> = {
  ...thread,
  thread_id: id16("25"),
  anchor: {
    ...thread.anchor,
    target: {
      target: "revision",
      value: {
        operation_id: "60".repeat(64),
        commit_id: commit,
        change_id: change,
      },
    },
  },
};
const codeThread: Model<"ReviewThreadSummary"> = {
  ...revisionThread,
  thread_id: id16("26"),
  anchor: {
    ...revisionThread.anchor,
    target: {
      target: "code_artifact",
      value: {
        operation_id: "60".repeat(64),
        commit_id: commit,
        change_id: change,
        path: "src/lib.rs",
        object: { kind: "file", id: id32("63") },
        value_kind: "file",
      },
    },
  },
};
const association: Model<"AgentCodeAssociation"> = {
  association_id: id16("64"),
  object_id: id32("65"),
  session_id: session.session_id,
  tool_call_ids: [],
  relation: "validated_revision",
  before_operations: [],
  after_operations: ["60".repeat(64)],
  before_commits: [],
  after_commits: [commit],
  changes: [change],
  paths: ["src/lib.rs"],
  created_at_millis: 1_796_000_000_000,
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
  activity: [],
  activity_truncated: false,
  code: [],
  code_truncated: false,
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
    async agentRelations(commit: string) {
      return {
        version: 1,
        snapshot: agentDetail().snapshot,
        subject: { subject: "revision" as const, commit_id: commit },
        relations: [],
        next_cursor: null,
      };
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

test("revision context follows paged exact agent and review associations", async () => {
  const { controller, data, history, source } = harness();
  source.agentRelations = async (_commit, cursor) => ({
    version: 1,
    snapshot: agentDetail().snapshot,
    subject: { subject: "revision", commit_id: commit },
    relations: cursor
      ? [{ kind: "association", object_id: association.object_id, association }]
      : [{ kind: "checkpoint", object_id: id32("66"), association: null }],
    next_cursor: cursor ? null : "agent-context-next",
  });
  source.reviewThreads = async (cursor) => ({
    version: 1,
    snapshot: reviewDetail().snapshot,
    threads: cursor ? [revisionThread] : [thread],
    next_cursor: cursor ? null : "review-context-next",
  });

  await controller.loadRevisionContext(commit);

  expect(data.agentRelations.at(-1)?.association).toEqual(association);
  expect(history.route.agentSessionId).toBe(session.session_id);
  expect(history.route.reviewThreadId).toBe(revisionThread.thread_id);
  expect(data.agentSession?.session.session_id).toBe(session.session_id);
  expect(data.reviewThreads).toEqual([thread, revisionThread]);
});

test("path context continues review pages to the exact code anchor", async () => {
  const { controller, data, history, source } = harness();
  data.reviewThreads = [revisionThread];
  data.reviewCursor = "path-context-next";
  source.reviewThreads = async (cursor) => ({
    version: 1,
    snapshot: reviewDetail().snapshot,
    threads: cursor === "path-context-next" ? [codeThread] : [],
    next_cursor: null,
  });

  await controller.loadPathReview(commit, "src/lib.rs");

  expect(history.route.reviewThreadId).toBe(codeThread.thread_id);
  expect(data.reviewThreads).toEqual([revisionThread, codeThread]);
});
