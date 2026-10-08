import type { Model } from "../api/client.js";
import type { ChatBodyBlock } from "../ui/store-agent-chat-body/model.js";
import type { ActivityRow } from "../ui/store-agent-execution/presentation.js";
import { liveTranscriptTool } from "../ui/community-agent-activity/transcript-tool-adapters.js";
import { richTranscriptToolLabel } from "../ui/community-agent-activity/transcript-tool-rich.js";
import type { ConversationTextSource } from "./source-map.js";

/** Every displayed slice retains its captured string pointer and original offset. */
export function capturedToolProjection(
  row: ActivityRow,
  key: string,
  revision: string,
  captured: Model<"AgentExecutionFeedbackMessage"> | undefined,
  sources: Map<string, ConversationTextSource>,
) {
  const tool = liveTranscriptTool(row);
  const blocks: ChatBodyBlock[] = [];
  for (const part of (captured?.text_parts ?? []).filter(
    (part) =>
      !part.part ||
      /(?:^|\/)(?:text|output|stdout|stderr|content|patch|diff|command|result|newText|oldText)$/.test(
        part.part,
      ),
  )) {
    const files = [
      ...part.text.matchAll(/^\*\*\* (?:Update|Add|Delete) File: (.+)\r?$/gm),
    ];
    const pieces = files.length
      ? files.map((file, i) => {
          const from = file.index! + file[0].length + 1;
          const end =
            files[i + 1]?.index ?? part.text.indexOf("*** End Patch", from);
          const text = part.text
            .slice(from, end < 0 ? undefined : end)
            .trimEnd();
          return { from, text, path: file[1], kind: "patch" as const };
        })
      : [
          {
            from: 0,
            text: part.text,
            path: tool.paths?.length === 1 ? tool.paths[0] : undefined,
            kind: /^(?:diff --git|@@ |--- )/m.test(part.text)
              ? ("patch" as const)
              : ("code" as const),
          },
        ];
    for (const piece of pieces) {
      const id = `${key}/part/${part.part ?? "text"}/${piece.from}`;
      const block: ChatBodyBlock = {
        id,
        revision,
        label:
          piece.path ?? (part.part ? `${row.text} · ${part.part}` : row.text),
        text: piece.text,
        kind: piece.kind,
        path: piece.path,
        commentable: true,
        lineNumbered: tool.kind === "read" && /^\s*\d+\t/m.test(piece.text),
      };
      blocks.push(block);
      sources.set(id, {
        target: captured!.target,
        part: part.part ?? null,
        text: part.text,
        from: piece.from,
        revision,
      });
    }
  }
  if (!blocks.length) {
    const block: ChatBodyBlock = {
      id: `${key}/section`,
      revision,
      kind: "code",
      label: row.text,
      text: JSON.stringify(row.value, null, 2),
      commentable: !!captured,
      wholeSection: true,
      path: "tool.json",
    };
    blocks.push(block);
    if (captured)
      sources.set(block.id, {
        target: captured.target,
        part: null,
        text: block.text,
        from: 0,
        revision,
        wholeSection: true,
      });
  }
  return { blocks, title: richTranscriptToolLabel(tool) ?? row.text };
}
