import type { ChatBodyBlock } from "../../../web/src/lib/ui/store-agent-chat-body/model.js";

/** Captured apply_patch rows retain hunk context, without invented file coordinates. */
export function capturedPatchBlocks(
  input: string,
  id: string,
  revision: string,
): ChatBodyBlock[] {
  let patch: unknown;
  try {
    patch = JSON.parse(input);
  } catch {
    patch = input;
  }
  if (typeof patch !== "string" || !patch.startsWith("*** Begin Patch"))
    return [];
  const files: { path: string; rows: string[] }[] = [];
  let current: (typeof files)[number] | undefined;
  for (const line of patch.split("\n")) {
    const file = /^\*\*\* (?:Update|Add|Delete) File: (.+)$/.exec(line);
    if (file) {
      current = { path: file[1], rows: [] };
      files.push(current);
    } else if (current && !line.startsWith("*** ")) current.rows.push(line);
  }
  return files.map((file, index) => ({
    id: `${id}/patch/${index}`,
    revision,
    kind: "patch",
    path: file.path,
    label: `Captured patch · ${file.path}`,
    text: file.rows.join("\n").trimEnd(),
  }));
}
