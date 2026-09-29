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
const operation = "60".repeat(64);
const beforeOperation = "67".repeat(64);
const beforeCommit = "68".repeat(64);
const revisionThread: Model<"ReviewThreadSummary"> = {
  ...thread,
  thread_id: id16("25"),
  anchor: {
    ...thread.anchor,
    target: {
      target: "revision",
      value: {
        operation_id: operation,
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
        operation_id: operation,
        commit_id: commit,
        change_id: change,
        path: "src/lib.rs",
        object: { kind: "file", id: id32("63") },
        value_kind: "file",
      },
    },
  },
};
const codeTextThread: Model<"ReviewThreadSummary"> = {
  ...revisionThread,
  thread_id: id16("27"),
  anchor: {
    ...revisionThread.anchor,
    target: {
      target: "code_text",
      value: {
        operation_id: operation,
        commit_id: commit,
        change_id: change,
        path: "src/lib.rs",
        file_id: id32("69"),
        value_kind: "file",
        text: {
          encoding: "utf8",
          start_byte: "4",
          end_byte: "12",
          selected_sha256: id32("6a"),
        },
        diff_context: {
          before_operation_id: beforeOperation,
          before_commit_id: beforeCommit,
          before_path: "src/lib.rs",
          after_operation_id: operation,
          after_commit_id: commit,
          after_path: "src/lib.rs",
          side: "after",
          comparison: "explicit_pair",
        },
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
  after_operations: [operation],
  before_commits: [],
  after_commits: [commit],
  changes: [change],
  paths: ["src/lib.rs"],
  created_at_millis: 1_796_000_000_000,
};
const toolCallId = id16("71");
const toolSnapshotId = id32("72");
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

const reviewDetailFor = (
  selected: Model<"ReviewThreadSummary">,
): Model<"ReviewThreadDetail"> => ({
  ...reviewDetail(),
  thread: selected,
  anchor: {
    object_id: selected.original_anchor_id,
    value: selected.anchor,
  },
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
  tools: [{ tool_call_id: toolCallId, snapshot_id: toolSnapshotId }],
  activity: [
    {
      type: "tool",
      sequence: 1,
      event_id: id16("73"),
      tool: {
        tool_call_id: toolCallId,
        snapshot_id: toolSnapshotId,
        turn_id: null,
        native_name: "shell",
        kind: "execute",
        state: "succeeded",
        input: null,
        output: null,
        observed_paths: [],
        code: [],
      },
    },
  ],
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
    async agentRelations(subject: { commit: string } | { operation: string }) {
      return {
        version: 1,
        snapshot: agentDetail().snapshot,
        subject: {
          subject: "revision" as const,
          commit_id: "commit" in subject ? subject.commit : commit,
        },
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

test("diff line comments create an exact code-text thread and select its receipt", async () => {
  const { controller, data, commands, history, source } = harness();
  const fileId = id32("74");
  data.comparisons = new Map([
    [
      "src/lib.rs",
      {
        from: beforeCommit,
        to: commit,
        beforeOperation,
        afterOperation: operation,
      },
    ],
  ]);
  source.diff = async () => ({
    snapshot: {
      version: 1,
      repo_id: source.repository,
      epoch: id16("32"),
      published_seq: 4,
      manifest_revision: 5,
      code_state_version: 2,
      heads: [operation],
      operation_id: operation,
    },
    data: {
      version: 1,
      from: beforeCommit,
      to: commit,
      path: "src/lib.rs",
      changes: [
        {
          path: "src/lib.rs",
          before: [],
          after: [
            {
              type: "file",
              id: fileId,
              executable: false,
              copy: { type: "placeholder" },
            },
          ],
          sources: [],
          content: {
            status: "text",
            hunks: [
              {
                kind: "different",
                before_start: 1,
                after_start: 1,
                before: "",
                after: "hello\n",
              },
            ],
          },
        },
      ],
      stats: {
        visited_objects: 1,
        visited_bytes: 1,
        elapsed_millis: 1,
        cache_hits: 0,
      },
    },
    next_cursor: null,
  });
  source.file = async () => ({ text: "hello\n", bytes: 6 });

  await controller.commentOnDiffLine(
    {
      path: "src/lib.rs",
      lineNumber: 1,
      variant: "added",
      text: "hello",
    },
    "Please keep this line",
  );

  expect(commands[0]).toMatchObject({
    command: {
      command: "create_thread",
      anchor: {
        target: {
          target: "code_text",
          value: {
            operation_id: operation,
            commit_id: commit,
            file_id: fileId,
            diff_context: {
              before_operation_id: beforeOperation,
              after_operation_id: operation,
              side: "after",
            },
          },
        },
      },
      body: { text: "Please keep this line" },
    },
  });
  expect(history.route.reviewThreadId).toBe(thread.thread_id);
});

test("tool comments create an exact agent-artifact thread", async () => {
  const { controller, commands } = harness();
  await controller.loadAgent(session.session_id);
  await controller.commentOnAgentTool(toolCallId, "Explain this call");
  expect(commands[0]).toMatchObject({
    command: {
      command: "create_thread",
      anchor: {
        target: {
          target: "agent_artifact",
          value: {
            session_id: session.session_id,
            session_snapshot: {
              kind: "session_snapshot",
              id: session.snapshot_id,
            },
            owner: {
              kind: "tool",
              tool_call_id: toolCallId,
              snapshot: { kind: "tool_snapshot", id: toolSnapshotId },
              attempt_id: toolCallId,
            },
          },
        },
      },
      body: { text: "Explain this call" },
    },
  });
});

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
  source.agentRelations = async (_subject, cursor) => ({
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

test("operation context follows associations and every exact operation anchor", async () => {
  const { controller, data, history, source } = harness();
  const subjects: ({ commit: string } | { operation: string })[] = [];
  source.agentRelations = async (subject) => {
    subjects.push(subject);
    return {
      version: 1,
      snapshot: agentDetail().snapshot,
      subject: { subject: "operation", operation_id: operation },
      relations: [
        { kind: "association", object_id: association.object_id, association },
      ],
      next_cursor: null,
    };
  };
  source.reviewThreads = async () => ({
    version: 1,
    snapshot: reviewDetail().snapshot,
    threads: [codeTextThread],
    next_cursor: null,
  });
  source.reviewThread = async () => reviewDetailFor(codeTextThread);
  source.diff = async () => {
    throw new Error("Patch unavailable in this context test");
  };

  await controller.loadOperationContext(operation);

  expect(subjects).toEqual([{ operation }]);
  expect(data.agentRelations.at(-1)?.association).toEqual(association);
  expect(history.route.agentSessionId).toBe(session.session_id);
  expect(history.route.reviewThreadId).toBe(codeTextThread.thread_id);
  expect(data.reviewThread?.thread.thread_id).toBe(codeTextThread.thread_id);
});

test("operation path context keeps the exact anchored review selected", async () => {
  const { controller, data, history, source } = harness();
  data.reviewThreads = [revisionThread, codeThread];
  source.reviewThread = async () => reviewDetailFor(codeThread);
  source.commit = async () => {
    throw new Error("Patch unavailable in this context test");
  };

  await controller.loadOperationPathReview(operation, "src/lib.rs");

  expect(history.route.reviewThreadId).toBe(codeThread.thread_id);
  expect(data.reviewThread?.thread.thread_id).toBe(codeThread.thread_id);
});

test("code text review loads its exact pinned comparison pair", async () => {
  const { controller, data, source } = harness();
  const calls: [string, string, string, string][] = [];
  source.reviewThread = async () => reviewDetailFor(codeTextThread);
  source.commit = async () => {
    throw new Error("The explicit diff pair must not read commit parents");
  };
  source.diff = async (selectedOperation, from, to, path) => {
    calls.push([selectedOperation, from, to, path]);
    return {
      snapshot: {
        version: 1,
        repo_id: source.repository,
        epoch: id16("32"),
        published_seq: 4,
        manifest_revision: 5,
        code_state_version: 2,
        heads: [selectedOperation],
        operation_id: selectedOperation,
      },
      data: {
        version: 1,
        from,
        to,
        path,
        changes: [
          {
            path,
            before: [],
            after: [],
            sources: [],
            content: {
              status: "text",
              hunks: [
                {
                  kind: "different",
                  before_start: 1,
                  after_start: 1,
                  before: "old\n",
                  after: "new\n",
                },
              ],
            },
          },
        ],
        stats: {
          metadata_nodes: 1,
          predicate_file_bytes: 0,
          predicate_lines: 0,
          object_reads: 1,
          object_bytes: 8,
          work: 1,
        },
      },
      next_cursor: null,
    };
  };
  source.file = async () => ({ text: "old\nnew line\n", bytes: 13 });

  await controller.loadReview(codeTextThread.thread_id);

  expect(calls).toEqual([[operation, beforeCommit, commit, "src/lib.rs"]]);
  expect(data.reviewPatch?.path).toBe("src/lib.rs");
  expect(data.reviewPatch?.patch).toContain("-old");
  expect(data.reviewPatch?.lineNumber).toBe(2);
  expect(data.reviewPatch?.side).toBe("after");
  expect(data.status.reviewAction).toBe("ready");
});

test("a review patch failure preserves the loaded conversation", async () => {
  const { controller, data, source } = harness();
  source.reviewThread = async () => reviewDetailFor(codeTextThread);
  source.diff = async () => {
    throw new Error("Diff was pruned");
  };

  await controller.loadReview(codeTextThread.thread_id);

  expect(data.reviewThread?.thread.thread_id).toBe(codeTextThread.thread_id);
  expect(data.reviewPatch).toBeUndefined();
  expect(data.status.reviewAction).toBe("ready");
});

test("a stale review patch failure cannot clear newer state", async () => {
  let active = true;
  let rejectPatch!: (cause: unknown) => void;
  const { controller, data, source } = harness("committed", () => active);
  source.reviewThread = async () => reviewDetailFor(codeTextThread);
  source.diff = () =>
    new Promise((_resolve, reject) => {
      rejectPatch = reject;
    });

  const read = controller.loadReview(codeTextThread.thread_id);
  await Promise.resolve();
  await Promise.resolve();
  active = false;
  data.reviewPatch = { path: "newer.rs", patch: "newer" };
  rejectPatch(new Error("Late patch failure"));
  await read;

  expect(data.reviewPatch).toEqual({ path: "newer.rs", patch: "newer" });
});
