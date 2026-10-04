import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import type { StoreControllerState } from "../src/lib/controller/contracts.js";
import type { RepositoryLogRevision } from "../src/lib/projects/repository-log.js";
import { agentAvatarUrl } from "../src/lib/collaboration/agent-avatar.js";
import {
  agentMember,
  agentToolDuration,
  agentMessageTimestamp,
  transcriptMessage,
  agentSessionTiming,
  anchorMatchesOperation,
  anchorMatchesPath,
  anchorOperationIds,
  formatAgentSessionDuration,
  reviewCommentRows,
  reviewPatchMatchesLine,
} from "../src/lib/collaboration/presentation.js";

test("tool durations preserve subsecond precision and omit unknown or unsafe values", () => {
  assert.equal(agentToolDuration(0), "0ms");
  assert.equal(agentToolDuration(250n), "250ms");
  assert.equal(agentToolDuration(1250), "1.25s");
  assert.equal(agentToolDuration(125000), "2m 5s");
  for (const value of [
    null,
    undefined,
    -1,
    NaN,
    Infinity,
    1.5,
    9007199254740993n,
  ])
    assert.equal(agentToolDuration(value), undefined);
});
import { revisionCollaboration } from "../src/lib/collaboration/revision-collaboration.js";

function summary(
  first_activity_at_millis: number | null,
  last_activity_at_millis: number | null,
): Model<"AgentSessionSummary"> {
  return {
    session_id: "11".repeat(16),
    snapshot_id: "22".repeat(32),
    integration: "codex",
    state: "ended",
    parent_session_id: null,
    first_activity_at_millis,
    last_activity_at_millis,
    coverage: {
      completeness: "complete",
      gaps: 0,
      redacted_fields: 0,
      truncated: false,
    },
    evidence_count: 1,
    event_batch_count: 1,
    transcript_count: 1,
    tool_count: 1,
  };
}

test("agent session timing uses captured first and last activity", () => {
  assert.deepEqual(
    agentSessionTiming(summary(1_700_000_000_000, 1_700_000_125_000)),
    {
      startedAt: 1_700_000_000,
      durationMs: 125_000,
      durationLabel: "2m 5s",
    },
  );
  assert.equal(agentSessionTiming(summary(null, null)), undefined);
  assert.equal(
    agentSessionTiming(summary(1_700_000_125_000, 1_700_000_000_000)),
    undefined,
  );
});

test("agent session durations stay compact", () => {
  assert.equal(formatAgentSessionDuration(0), "0s");
  assert.equal(formatAgentSessionDuration(2_000), "2s");
  assert.equal(formatAgentSessionDuration(3_600_000), "1h");
  assert.equal(formatAgentSessionDuration(90_000_000), "1d 1h");
});

test("supported agents use bundled artwork and agent avatar shape metadata", () => {
  assert.equal(agentAvatarUrl("codex"), "/agent-icons/codex.svg");
  assert.equal(agentAvatarUrl("cursor"), "/agent-icons/cursor.svg");
  assert.equal(agentAvatarUrl("open_code"), "/agent-icons/opencode.svg");
  assert.equal(agentAvatarUrl("owned_controller"), undefined);
  assert.deepEqual(
    {
      avatarUrl: agentMember(summary(1, 2)).avatarUrl,
      kind: agentMember(summary(1, 2)).kind,
    },
    { avatarUrl: "/agent-icons/codex.svg", kind: "agent" },
  );
});

function comment(id: string, parent: string | null): Model<"ReviewComment"> {
  return {
    comment_id: id.repeat(32),
    revision_id: `${id}0`.repeat(32),
    parent_comment_id: parent?.repeat(32) ?? null,
    author: "aa".repeat(16),
    actor: "aa".repeat(16),
    created_at_millis: 1_700_000_000_000,
    previous_revision_id: null,
    content: {
      state: "visible",
      body_id: `${id}1`.repeat(32),
      format: "safe_markdown",
      text: id,
      sanitizer_version: "review-markdown-v1",
    },
  };
}

test("review replies follow Community branch order and bounded indentation", () => {
  const comments = [
    comment("1", null),
    comment("2", "1"),
    comment("4", "1"),
    comment("3", "2"),
  ];
  const rows = reviewCommentRows(comments);

  assert.deepEqual(
    rows.map(
      (row) =>
        row.comment.content.state === "visible" && row.comment.content.text,
    ),
    ["1", "2", "3", "4"],
  );
  assert.deepEqual(
    rows.map((row) => row.depth),
    [0, 1, 2, 1],
  );
  assert.equal(rows[1]?.indentRem, 0);
  assert.equal(rows[1]?.connectsToChildren, true);
  assert.equal(rows[2]?.indentRem, 2.25);
  assert.equal(rows[2]?.connectorIndentRem, 0);
});

test("operation collaboration includes explicit code comparison context", () => {
  const primaryOperation = "60".repeat(64);
  const beforeOperation = "61".repeat(64);
  const afterOperation = "62".repeat(64);
  const anchor: Model<"ArtifactAnchorV1"> = {
    schema_version: 1,
    repository_id: "31".repeat(16),
    target: {
      target: "code_text",
      value: {
        operation_id: primaryOperation,
        commit_id: "63".repeat(64),
        change_id: "64".repeat(16),
        path: "src/lib.rs",
        file_id: "65".repeat(64),
        value_kind: "file",
        text: {
          encoding: "utf8",
          start_byte: "0",
          end_byte: "4",
          selected_sha256: "66".repeat(32),
        },
        diff_context: {
          before_operation_id: beforeOperation,
          before_commit_id: "67".repeat(64),
          before_path: "src/lib.rs",
          after_operation_id: afterOperation,
          after_commit_id: "68".repeat(64),
          after_path: "src/lib.rs",
          side: "after",
          comparison: "explicit_pair",
        },
      },
    },
    created_context: {
      publication_position: 1,
      code_state_version: 1,
      agent_root: null,
    },
    sensitivity: "repository",
  };
  assert.deepEqual(anchorOperationIds(anchor), [
    primaryOperation,
    beforeOperation,
    afterOperation,
  ]);
  assert.equal(anchorMatchesOperation(anchor, beforeOperation), true);
  assert.equal(anchorMatchesPath(anchor, "src/lib.rs"), true);
  assert.equal(
    reviewPatchMatchesLine(
      {
        path: "src/lib.rs",
        patch: "@@ -1 +1 @@",
        lineNumber: 4,
        side: "before",
      },
      {
        path: "src/lib.rs",
        lineNumber: 4,
        variant: "removed",
        text: "old",
      },
    ),
    true,
  );

  const state = {
    agentSessions: [summary(1, 2)],
    agentRelations: [
      {
        kind: "association",
        object_id: "69".repeat(32),
        association: {
          association_id: "6a".repeat(16),
          object_id: "69".repeat(32),
          session_id: "11".repeat(16),
          tool_call_ids: [],
          relation: "validated_revision",
          before_operations: [beforeOperation],
          after_operations: [],
          before_commits: [],
          after_commits: [],
          changes: [],
          paths: ["src/lib.rs"],
          created_at_millis: 1,
        },
      },
    ],
    reviewThreads: [
      {
        thread_id: "6b".repeat(16),
        snapshot_id: "6c".repeat(32),
        original_anchor_id: "6d".repeat(32),
        anchor,
        event_page_root_id: "6e".repeat(32),
        status: "open",
        selected_workflow_action: null,
        thread_version: 1,
        content_version: 1,
        workflow_version: 1,
        comment_count: 2,
        last_event_sequence: 2,
      },
    ],
  } as unknown as StoreControllerState;
  const revision = {
    eventId: beforeOperation,
    commitId: beforeOperation,
    changeId: "70".repeat(16),
  } as RepositoryLogRevision;

  assert.deepEqual(revisionCollaboration(revision, state), {
    members: [agentMember(summary(1, 2))],
    replyCount: 2,
  });
});

test("agent messages use captured times without inferring from sequence or session", () => {
  const time = agentMessageTimestamp(1_704_164_702_456);
  assert.equal(time.createdAt, "2024-01-02T03:05:02.456Z");
  assert.equal(
    time.timestamp,
    new Date(1_704_164_702_456).toLocaleTimeString(undefined, {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }),
  );
  assert.deepEqual(agentMessageTimestamp(1_704_164_702_456n), time);
  for (const unknown of [null, undefined, NaN, Infinity, 9e15]) {
    assert.deepEqual(agentMessageTimestamp(unknown), {
      timestamp: "Time unknown",
    });
  }
  const chunk = {
    sequence: 99,
    media_type: "text" as const,
    content: "**Recorded**",
  };
  assert.equal(
    transcriptMessage(chunk, summary(1, 2)).timestamp,
    "Time unknown",
  );
  const message = transcriptMessage(
    { ...chunk, occurred_at_millis: 1_704_164_702_456 },
    summary(1, 2),
  );
  assert.equal(message.createdAt, time.createdAt);
  assert.equal(message.content, "**Recorded**");
});
