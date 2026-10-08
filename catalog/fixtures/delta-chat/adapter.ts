import type {
  ChatBodyBlock,
  ChatBodyItem,
  ChatBodyTool,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import { byteSlice } from "./offsets.js";
import { capturedPatchBlocks } from "./patches.js";
import { markdownBlocks } from "../../../web/src/lib/ui/store-agent-chat-body/markdown-blocks.js";

interface Span {
  start: number;
  end: number;
}
interface DeltaRecord {
  start: number;
  end?: number;
  id: string;
  kind: string;
  name?: string;
  input?: string;
  output?: string;
  status?: string;
  title?: string;
  createdAt?: number;
  titleEnd?: number;
  contentEnd?: number;
  layout?: Record<string, Span>;
}
export interface DeltaChatFixture {
  version: number;
  source: { title: string; sha256: string };
  buffer: string;
  records: readonly DeltaRecord[];
  draftStart: number;
}
const user = { id: "demo-user", name: "You", handle: "", initials: "YO" };
const agent = { id: "demo-agent", name: "Codex", handle: "", initials: "AI" };

export function deltaChatItems(fixture: DeltaChatFixture): ChatBodyItem[] {
  if (fixture.version !== 1)
    throw new Error("Unsupported Delta comparison fixture");
  const revision = fixture.source.sha256;
  const slice = (span?: Span) =>
    span ? byteSlice(fixture.buffer, span.start, span.end).trim() : "";
  const block = (
    id: string,
    label: string,
    text: string,
    kind: ChatBodyBlock["kind"] = "markdown",
    path?: string,
  ): ChatBodyBlock => ({ id, revision, label, text, kind, path });
  const nodes: { start: number; item: ChatBodyItem; group?: string }[] = [];
  const occupied = fixture.records.flatMap((r) =>
    r.end !== undefined && r.end > r.start
      ? [{ start: r.start, end: r.end }]
      : [],
  );
  for (const record of fixture.records) {
    if (record.kind === "AssistantTurn") continue;
    const content =
      record.end === undefined
        ? ""
        : slice({ start: record.start, end: record.end });
    if (record.kind === "User")
      nodes.push({
        start: record.start,
        item: {
          id: record.id,
          kind: "message",
          role: "user",
          author: user,
          label: "Message from You",
          blocks: markdownBlocks(
            `${record.id}/text`,
            revision,
            "User message",
            content,
          ),
        },
      });
    else if (record.kind === "ToolUse") {
      let input: Record<string, unknown> = {};
      try {
        const parsed: unknown = JSON.parse(record.input ?? "{}");
        if (parsed && typeof parsed === "object")
          input = parsed as Record<string, unknown>;
      } catch {
        /* Retain opaque captured input. */
      }
      const title =
        slice(record.layout?.meta_range) || record.name || "Tool call";
      const blocks =
        record.name === "apply_patch"
          ? capturedPatchBlocks(record.input ?? "", record.id, revision)
          : [];
      if (!blocks.length) {
        const path = typeof input.path === "string" ? input.path : undefined;
        const output = record.output || slice(record.layout?.result_range);
        const command =
          typeof input.command === "string" ? `$ ${input.command}\n\n` : "";
        blocks.push(
          block(
            `${record.id}/output`,
            path ?? `${title} output`,
            `${command}${output || "No output was captured."}`,
            "code",
            path,
          ),
        );
      }
      const tool: ChatBodyTool = {
        id: record.id,
        revision,
        detailPlacement: record.name === "read_file" ? "sidebar" : "inline",
        title,
        status:
          record.status === "Succeeded"
            ? "complete"
            : record.status === "Failed"
              ? "error"
              : "unknown",
        blocks,
        raw: {
          name: record.name,
          input: record.input,
          output: record.output,
          status: record.status,
        },
      };
      const group = slice(record.layout?.action_group_title_range);
      nodes.push({
        start: record.start,
        group,
        item: {
          id: `${record.id}/group`,
          kind: "tools",
          label: group || title,
          blocks: [],
          tools: [tool],
          defaultExpanded: record.name === "apply_patch",
        },
      });
    } else if (record.kind === "Thinking") {
      nodes.push({
        start: record.start,
        item: {
          id: record.id,
          kind: "reasoning",
          label: "Thought",
          blocks: [
            block(
              `${record.id}/text`,
              "Recorded thinking",
              byteSlice(
                fixture.buffer,
                record.titleEnd!,
                record.contentEnd!,
              ).trim(),
            ),
          ],
        },
      });
    } else if (
      record.kind === "ExternalEdits" ||
      record.kind === "AgentMessage"
    ) {
      const handoff = record.kind === "AgentMessage";
      nodes.push({
        start: record.start,
        item: {
          id: record.id,
          kind: handoff ? "handoff" : "context",
          label: handoff
            ? `Agent response · ${record.title}`
            : slice(record.layout?.meta_range) || "External changes",
          blocks: [
            block(
              `${record.id}/text`,
              handoff ? "Returned agent response" : "External change context",
              slice(record.layout?.result_range) || content,
              handoff ? "markdown" : "code",
            ),
          ],
        },
      });
    }
  }
  // Free intervals contain assistant prose. Never merge prose across tools.
  let cursor =
    fixture.records.find((r) => r.kind === "AssistantTurn")?.start ??
    fixture.draftStart;
  for (const span of [
    ...occupied,
    { start: fixture.draftStart, end: fixture.draftStart },
  ].sort((a, b) => a.start - b.start)) {
    if (span.start > cursor) {
      const text = byteSlice(fixture.buffer, cursor, span.start).trim();
      if (text)
        nodes.push({
          start: cursor,
          item: {
            id: `assistant-${cursor}`,
            kind: "message",
            role: "assistant",
            author: agent,
            label: "Message from Codex",
            blocks: markdownBlocks(
              `assistant-${cursor}/text`,
              revision,
              "Assistant message",
              text,
            ),
          },
        });
    }
    cursor = Math.max(cursor, span.end);
  }
  const result: ChatBodyItem[] = [];
  for (const node of nodes.sort((a, b) => a.start - b.start)) {
    const previous = result.at(-1);
    if (
      node.item.kind === "tools" &&
      previous?.kind === "tools" &&
      !node.group &&
      !node.item.defaultExpanded &&
      !previous.defaultExpanded
    ) {
      previous.tools = [...(previous.tools ?? []), ...(node.item.tools ?? [])];
    } else result.push(node.item);
  }
  const userMessage = result.find((item) => item.role === "user");
  const lastMessage = result.filter((item) => item.role === "assistant").at(-1);
  const turnId =
    fixture.records.find((record) => record.kind === "AssistantTurn")?.id ??
    userMessage?.id ??
    "captured-turn";
  const summary = (
    lastMessage?.blocks[0]?.text.split(/\n\s*\n/)[0] ?? fixture.source.title
  )
    .replace(/[*`_]/g, "")
    .replace(/\s+/g, " ")
    .slice(0, 280);
  const turn = {
    id: turnId,
    author: "You",
    agentName: "Codex",
    summary,
    prompt:
      userMessage?.blocks.map((block) => block.text).join("\n\n") ??
      fixture.source.title,
    createdAt: fixture.records.find((record) => record.kind === "User")
      ?.createdAt,
  };
  // This captured main thread has one exchange. Assistant display segments and
  // tool groups share its actual AssistantTurn identity rather than becoming turns.
  for (const item of result) item.turn = turn;
  return result;
}
