import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";

type AgentSession = StoreControllerState["agentSessions"][number];
type AgentDetail = NonNullable<StoreControllerState["agentSession"]>;
type AgentChunk = StoreControllerState["agentTranscript"][number];
type ReviewThread = StoreControllerState["reviewThreads"][number];
type ReviewDetail = NonNullable<StoreControllerState["reviewThread"]>;

const id16 = (byte: string) => byte.repeat(16);
const id32 = (byte: string) => byte.repeat(32);

export const agentSessions: readonly AgentSession[] = [
  {
    session_id: id16("a1"),
    snapshot_id: id32("a2"),
    integration: "codex",
    state: "ended",
    parent_session_id: null,
    coverage: {
      completeness: "complete",
      gaps: 0,
      redacted_fields: 2,
      truncated: false,
    },
    evidence_count: 2,
    event_batch_count: 1,
    transcript_count: 1,
    tool_count: 3,
  },
  {
    session_id: id16("b1"),
    snapshot_id: id32("b2"),
    integration: "open_code",
    state: "active",
    parent_session_id: null,
    coverage: {
      completeness: "partial",
      gaps: 1,
      redacted_fields: 0,
      truncated: false,
    },
    evidence_count: 1,
    event_batch_count: 1,
    transcript_count: 1,
    tool_count: 1,
  },
];

export const agentDetail: AgentDetail = {
  version: 1,
  snapshot: {
    repo_id: id16("c1"),
    epoch: id16("c2"),
    published_seq: 22,
    manifest_revision: 9,
    agent_root: id32("c3"),
    agent_state_version: 4,
  },
  session: agentSessions[0]!,
  evidence: [id32("d1")],
  event_batches: [id32("d2")],
  transcripts: [
    {
      transcript_id: id32("d3"),
      source_instance_id: id16("d4"),
      source_generation: 1,
      source_format: "codex-jsonl",
      decoder_version: "codex-0.156.1/v1",
      sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1",
      representation: "sanitized_native",
      logical_byte_length: 180,
      record_count: 2,
      gaps: 0,
    },
  ],
  tools: [{ tool_call_id: id16("d5"), snapshot_id: id32("d6") }],
};

export const agentTranscript: readonly AgentChunk[] = [
  {
    sequence: 1,
    media_type: "text",
    content: "I inspected the publication boundary and kept the read pinned.",
  },
  {
    sequence: 2,
    media_type: "json",
    content: JSON.stringify({
      tool: "rg",
      status: "completed",
      redacted: true,
    }),
  },
];

const threadId = id16("e1");
const reviewer = id16("e2");
const summary: ReviewThread = {
  thread_id: threadId,
  snapshot_id: id32("e3"),
  original_anchor_id: id32("e4"),
  event_page_root_id: id32("e5"),
  status: "open",
  selected_workflow_action: null,
  thread_version: 1,
  content_version: 1,
  workflow_version: 1,
  comment_count: 2,
  last_event_sequence: 2,
};

export const reviewThreads: ReviewThread[] = [
  summary,
  {
    ...summary,
    thread_id: id16("f1"),
    snapshot_id: id32("f2"),
    status: "resolved",
  },
];

export function reviewDetail(): ReviewDetail {
  return {
    version: 1,
    snapshot: {
      repo_id: id16("c1"),
      epoch: id16("c2"),
      published_seq: 24,
      manifest_revision: 11,
      review_root: id32("f3"),
      review_state_version: 5,
    },
    thread: { ...summary },
    anchor: {
      object_id: summary.original_anchor_id,
      value: {
        schema_version: 1,
        repository_id: id16("c1"),
        target: {
          target: "revision",
          value: {
            operation_id: "01".repeat(64),
            commit_id: "02".repeat(64),
            change_id: id16("03"),
          },
        },
        created_context: {
          publication_position: 20,
          code_state_version: 7,
          agent_root: id32("c3"),
        },
        sensitivity: "repository",
      },
    },
    comments: [
      {
        comment_id: id16("11"),
        revision_id: id32("12"),
        parent_comment_id: null,
        author: reviewer,
        actor: reviewer,
        created_at_millis: 1_796_000_000_000,
        previous_revision_id: null,
        content: {
          state: "visible",
          body_id: id32("13"),
          format: "safe_markdown",
          text: "Can we preserve the exact operation when this retry races?",
          sanitizer_version: "review-markdown-v1",
        },
      },
      {
        comment_id: id16("14"),
        revision_id: id32("15"),
        parent_comment_id: id16("11"),
        author: id16("16"),
        actor: id16("16"),
        created_at_millis: 1_796_000_060_000,
        previous_revision_id: null,
        content: {
          state: "visible",
          body_id: id32("17"),
          format: "safe_markdown",
          text: "Yes. The command keeps the original request identity and precondition.",
          sanitizer_version: "review-markdown-v1",
        },
      },
    ],
    events: [],
  };
}
