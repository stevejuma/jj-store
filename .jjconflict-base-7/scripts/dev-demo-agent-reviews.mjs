import { createHash } from "node:crypto";

function u64(value) {
  const bytes = Buffer.alloc(8);
  bytes.writeBigUInt64BE(BigInt(value));
  return bytes;
}

export function normalizedAgentId(sourceInstanceId, integration, kind, native) {
  const kindBytes = Buffer.from(kind);
  const nativeBytes = Buffer.from(native);
  return createHash("sha256")
    .update("jj-store/agent-normalized-id/v1\0")
    .update(Buffer.from(sourceInstanceId, "hex"))
    .update(integration)
    .update(u64(kindBytes.length))
    .update(kindBytes)
    .update(u64(nativeBytes.length))
    .update(nativeBytes)
    .digest("hex")
    .slice(0, 32);
}

export function agentContentId(repositoryId, mediaType, bytes) {
  const envelope = {
    format: 1,
    repository: repositoryId,
    kind: "agent_object",
    payload: {
      type: "content",
      value: {
        schema_version: 1,
        payload: { media_type: mediaType, bytes: [...bytes] },
      },
    },
  };
  return createHash("sha256").update(JSON.stringify(envelope)).digest("hex");
}

export async function seedAgentReviews({
  api,
  apply,
  create,
  repositoryId,
  requestId,
}) {
  const sessions = await api("agent/sessions?limit=50");
  const summary = sessions.sessions.find(
    (session) => session.integration === "cursor" && session.tool_count > 0,
  );
  if (!summary)
    throw new Error("development review seed found no Cursor agent tools");
  const detail = await api(`agent/sessions/${summary.session_id}`);
  const transcript = detail.transcripts.find(
    (value) => value.source_format === "cursor-normalized-messages-v1",
  );
  if (!transcript)
    throw new Error("development review seed found no Cursor transcript");
  const editToolId = normalizedAgentId(
    transcript.source_instance_id,
    "cursor",
    "tool",
    "conv_demo/edit_1",
  );
  const editTool = detail.tools.find(
    (value) => value.tool_call_id === editToolId,
  );
  if (!editTool)
    throw new Error("development review seed found no Cursor edit tool");

  const tool = detail.tools[0];
  const agent = await create(
    "agent-artifact",
    {
      target: "agent_artifact",
      value: {
        session_id: detail.session.session_id,
        session_snapshot: {
          kind: "session_snapshot",
          id: detail.session.snapshot_id,
        },
        owner: {
          kind: "tool",
          tool_call_id: tool.tool_call_id,
          snapshot: { kind: "tool_snapshot", id: tool.snapshot_id },
          attempt_id: requestId("agent-tool-attempt"),
        },
      },
    },
    "Please verify the captured tool artifact without replaying it.",
    { agent_root: detail.snapshot.agent_root },
  );
  const moderated = await apply("agent-reply", {
    command: "append_reply",
    thread_id: agent.thread_id,
    expected_workflow_version: 1,
    parent_comment_id: agent.comment_id,
    body: {
      schema_version: 1,
      format: "safe_markdown",
      text: "Raw tool output copied here for triage.",
      sanitizer_version: "review-markdown-v1",
    },
  });
  await apply("agent-hide", {
    command: "hide_comment",
    comment_id: moderated.comment_id,
    expected_revision_id: moderated.comment_revision.id,
    moderation_reason: "Use the sanitized captured artifact instead",
  });

  const selectedAgentOutput = Buffer.from(JSON.stringify('{"success":true}'));
  const contentId = agentContentId(repositoryId, "json", selectedAgentOutput);
  await create(
    "agent-text",
    {
      target: "agent_text",
      value: {
        session_id: detail.session.session_id,
        session_snapshot: {
          kind: "session_snapshot",
          id: detail.session.snapshot_id,
        },
        owner: {
          kind: "tool",
          tool_call_id: editTool.tool_call_id,
          snapshot: { kind: "tool_snapshot", id: editTool.snapshot_id },
          attempt_id: requestId("agent-tool-attempt"),
        },
        block_id: requestId("agent-output-block"),
        block_version: { kind: "content", id: contentId },
        part: "output",
        content: { kind: "content", id: contentId },
        representation_version: "published-text-v1",
        sanitizer_version: transcript.sanitizer_version,
        normalizer_version: transcript.decoder_version,
        source_coverage: "partial",
        text: {
          encoding: "utf8",
          start_byte: "0",
          end_byte: String(selectedAgentOutput.length),
          selected_sha256: createHash("sha256")
            .update(selectedAgentOutput)
            .digest("hex"),
        },
      },
    },
    "The selected sanitized tool output is pinned to its exact immutable bytes.",
    { agent_root: detail.snapshot.agent_root },
  );
}
