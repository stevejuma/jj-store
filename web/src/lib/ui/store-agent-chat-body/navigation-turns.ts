import type { ChatBodyItem, ChatBodyTurn } from "./model.js";
import { patchLines } from "./patch-lines.js";
export interface ChatNavigationTurn extends ChatBodyTurn {
  firstItemId: string;
  toolCount: number;
  changedFiles: number;
  additions: number;
  deletions: number;
}
/** A turn is a prompt and its response/tool flow, never an individual tool row. */
export function navigationTurns(
  items: readonly ChatBodyItem[],
): ChatNavigationTurn[] {
  const groups: { turn: ChatBodyTurn; items: ChatBodyItem[] }[] = [];
  for (const item of items) {
    const previous = groups.at(-1);
    const turn =
      item.turn ??
      (item.role !== "user" && previous
        ? previous.turn
        : {
            id: item.id,
            author: item.author?.name ?? "You",
            prompt:
              item.role === "user"
                ? item.blocks.map((block) => block.text).join("\n\n")
                : item.label,
            summary:
              item.role === "assistant"
                ? (item.blocks[0]?.text ?? item.label)
                : "",
          });
    if (previous?.turn.id === turn.id) previous.items.push(item);
    else groups.push({ turn, items: [item] });
  }
  return groups.map(({ turn, items: content }) => {
    const tools = content.flatMap((item) => item.tools ?? []);
    const patches = tools.flatMap((tool) =>
      tool.blocks.filter((block) => block.kind === "patch"),
    );
    const lines = patches.flatMap((block) => patchLines(block.text));
    return {
      ...turn,
      firstItemId: content[0].id,
      toolCount: tools.length,
      changedFiles: new Set(patches.map((block) => block.path).filter(Boolean))
        .size,
      additions: lines.filter((line) => line.kind === "addition").length,
      deletions: lines.filter((line) => line.kind === "deletion").length,
    };
  });
}
