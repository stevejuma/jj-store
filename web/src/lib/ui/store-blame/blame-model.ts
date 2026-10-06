import type { StoreRoute } from "../../controller/contracts.js";
import type { BlameRange } from "../../controller/blame-types.js";
import { storePageRoute } from "../../controller/store-stacks-navigation.js";

export const BLAME_UNAVAILABLE =
  "Blame is not available on this server yet. Source remains available.";
export function sourceLines(contents: string): string[] {
  if (!contents) return [];
  const lines = contents.split("\n");
  if (lines.at(-1) === "") lines.pop();
  return lines;
}
export function blameCommitRoute(
  route: StoreRoute,
  commitId: string,
): StoreRoute {
  return {
    ...storePageRoute(route, "log"),
    workspace: null,
    commitId,
    path: route.path,
  };
}
export interface BlameChunk {
  start: number;
  end: number;
  commitId?: string;
  continued: boolean;
}
/** Split long groups into bounded render chunks, retaining continuation attribution. */
export function blameChunks(
  lineCount: number,
  ranges: readonly BlameRange[],
): BlameChunk[] {
  const groups: { start: number; end: number; commitId?: string }[] = [];
  let cursor = 1;
  for (const range of ranges) {
    if (range.start > cursor) groups.push({ start: cursor, end: range.start });
    const previous = groups.at(-1);
    if (previous?.commitId === range.commitId && previous.end === range.start)
      previous.end = range.end;
    else groups.push({ ...range });
    cursor = range.end;
  }
  if (cursor <= lineCount) groups.push({ start: cursor, end: lineCount + 1 });
  return groups.flatMap((group) => {
    const chunks: BlameChunk[] = [];
    for (let start = group.start; start < group.end; start += 32)
      chunks.push({
        ...group,
        start,
        end: Math.min(start + 32, group.end),
        continued: start !== group.start,
      });
    return chunks;
  });
}
