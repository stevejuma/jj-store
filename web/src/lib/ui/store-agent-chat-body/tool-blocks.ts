import type { ChatBodyBlock, ChatBodyTool } from "./model.js";

/** Raw and formatted projections share the exact caller-owned tool revision. */
export function rawToolBlock(tool: ChatBodyTool): ChatBodyBlock {
  return {
    id: `${tool.id}/raw`,
    revision: tool.revision,
    kind: "code",
    label: `Raw captured data for ${tool.title}`,
    path: "tool.json",
    text: JSON.stringify(tool.raw, null, 2) ?? "null",
  };
}
