import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";
import { IDS, revisions } from "./repository.js";

type ReviewThread = StoreControllerState["reviewThreads"][number];
type ReviewDetail = NonNullable<StoreControllerState["reviewThread"]>;

const id16 = (byte: string) => byte.repeat(16);
const id32 = (byte: string) => byte.repeat(32);
const relatedRevision = revisions.find(
  (revision) => revision.eventId === IDS.merge,
)!;
const reviewer = id16("e2");
const reviewAnchor = {
  schema_version: 1,
  repository_id: id16("c1"),
  target: {
    target: "revision" as const,
    value: {
      operation_id: IDS.opLeft,
      commit_id: relatedRevision.eventId,
      change_id: relatedRevision.changeId,
    },
  },
  created_context: {
    publication_position: 20,
    code_state_version: 7,
    agent_root: id32("c3"),
  },
  sensitivity: "repository" as const,
};
const summary: ReviewThread = {
  thread_id: id16("e1"),
  snapshot_id: id32("e3"),
  original_anchor_id: id32("e4"),
  anchor: reviewAnchor,
  event_page_root_id: id32("e5"),
  status: "open",
  selected_workflow_action: null,
  thread_version: 1,
  content_version: 1,
  workflow_version: 1,
  comment_count: 3,
  last_event_sequence: 3,
};

export const reviewThreads: ReviewThread[] = [
  summary,
  {
    ...summary,
    thread_id: id16("f1"),
    snapshot_id: id32("f2"),
    status: "resolved",
  },
  {
    ...summary,
    thread_id: id16("18"),
    snapshot_id: id32("19"),
    original_anchor_id: id32("1a"),
    anchor: {
      ...reviewAnchor,
      target: {
        target: "code_artifact",
        value: {
          operation_id: IDS.opLeft,
          commit_id: relatedRevision.eventId,
          change_id: relatedRevision.changeId,
          path: "src/lib.rs",
          object: { kind: "file", id: id32("1b") },
          value_kind: "file",
        },
      },
    },
  },
  {
    ...summary,
    thread_id: id16("2a"),
    snapshot_id: id32("2b"),
    original_anchor_id: id32("2c"),
    comment_count: 2,
    anchor: {
      ...reviewAnchor,
      target: {
        target: "agent_artifact",
        value: {
          session_id: id16("a1"),
          session_snapshot: {
            kind: "session_snapshot",
            id: id32("a2"),
          },
          owner: {
            kind: "tool",
            tool_call_id: id16("d5"),
            snapshot: { kind: "tool_snapshot", id: id32("d6") },
            attempt_id: id16("d5"),
          },
        },
      },
    },
  },
];

function visibleReviewComment(
  byte: string,
  parentByte: string | null,
  author: string,
  createdAtMillis: number,
  text: string,
): ReviewDetail["comments"][number] {
  return {
    comment_id: id16(byte),
    revision_id: id32(byte),
    parent_comment_id: parentByte ? id16(parentByte) : null,
    author,
    actor: author,
    created_at_millis: createdAtMillis,
    previous_revision_id: null,
    content: {
      state: "visible",
      body_id: id32(byte),
      format: "safe_markdown",
      text,
      sanitizer_version: "review-markdown-v1",
    },
  };
}

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
      value: reviewAnchor,
    },
    comments: [
      visibleReviewComment(
        "11",
        null,
        reviewer,
        1_796_000_000_000,
        "Can we preserve the exact operation when this retry races?",
      ),
      visibleReviewComment(
        "14",
        "11",
        id16("16"),
        1_796_000_060_000,
        "Yes. The command keeps the original request identity and precondition.",
      ),
      visibleReviewComment(
        "1c",
        "14",
        reviewer,
        1_796_000_120_000,
        "The nested follow-up stays attached to the reply branch.",
      ),
    ],
    events: [],
  };
}
