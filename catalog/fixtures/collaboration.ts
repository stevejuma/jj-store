import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";
import { IDS, NOW, revisions, sampleDetail } from "./repository.js";

type AgentSession = StoreControllerState["agentSessions"][number];
type AgentDetail = NonNullable<StoreControllerState["agentSession"]>;
type AgentChunk = StoreControllerState["agentTranscript"][number];
export { reviewDetail, reviewThreads } from "./collaboration-review.js";

const id16 = (byte: string) => byte.repeat(16);
const id32 = (byte: string) => byte.repeat(32);
const relatedRevision = revisions.find(
  (revision) => revision.eventId === IDS.merge,
)!;

export const agentSessions: readonly AgentSession[] = [
  {
    session_id: id16("a1"),
    snapshot_id: id32("a2"),
    integration: "codex",
    state: "ended",
    parent_session_id: null,
    first_activity_at_millis: (NOW - 480) * 1_000,
    last_activity_at_millis: (NOW - 180) * 1_000,
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
    first_activity_at_millis: (NOW - 120) * 1_000,
    last_activity_at_millis: (NOW - 30) * 1_000,
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

const agentAssociation: AgentDetail["code"][number] = {
  association_id: id16("a3"),
  object_id: id32("a4"),
  session_id: agentSessions[0]!.session_id,
  tool_call_ids: [id16("d5")],
  relation: "validated_revision",
  before_operations: [IDS.opRoot],
  after_operations: [IDS.opLeft],
  before_commits: [IDS.left],
  after_commits: [relatedRevision.eventId],
  changes: [relatedRevision.changeId],
  paths: ["src/lib.rs"],
  created_at_millis: 1_796_000_000_000,
};

export const agentRelations: StoreControllerState["agentRelations"] = [
  {
    kind: "association",
    object_id: agentAssociation.object_id,
    association: agentAssociation,
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
  activity: [
    {
      type: "message",
      sequence: 1,
      event_id: id16("41"),
      turn_id: id16("42"),
      message_id: id16("43"),
      role: "user",
      content: {
        media_type: "text",
        content: "Update the native parser and keep its review anchors exact.",
      },
    },
    {
      type: "tool",
      sequence: 2,
      event_id: id16("44"),
      tool: {
        tool_call_id: id16("d5"),
        snapshot_id: id32("d6"),
        turn_id: id16("42"),
        native_name: "apply_patch",
        kind: "edit",
        state: "succeeded",
        input: {
          media_type: "json",
          content: JSON.stringify({
            path: "src/lib.rs",
            old_string: "fn old() {}",
            new_string: "fn main() {}",
          }),
        },
        output: {
          media_type: "json",
          content: JSON.stringify({ success: true }),
        },
        observed_paths: ["src/lib.rs"],
        code: [agentAssociation],
      },
    },
    {
      type: "message",
      sequence: 3,
      event_id: id16("45"),
      turn_id: id16("42"),
      message_id: id16("46"),
      role: "assistant",
      content: {
        media_type: "text",
        content: "Updated the parser and verified the focused contract.",
      },
    },
  ],
  activity_truncated: false,
  code: [agentAssociation],
  code_truncated: false,
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

export const reviewPatch = {
  path: "src/lib.rs",
  patch: sampleDetail.patches["src/lib.rs"]!,
};
