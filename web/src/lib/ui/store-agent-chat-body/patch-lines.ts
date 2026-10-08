export type PatchLineKind = "addition" | "deletion" | "hunk" | "context";

/** Classify one retained file capture without inventing source coordinates. */
export function patchLines(text: string) {
  let inHunk = false;
  let from = 0;
  return text.split("\n").map((text) => {
    if (text.startsWith("diff --git ")) inHunk = false;
    const header =
      !inHunk && (text.startsWith("--- ") || text.startsWith("+++ "));
    const kind: PatchLineKind = text.startsWith("@@")
      ? "hunk"
      : header
        ? "context"
        : text.startsWith("+")
          ? "addition"
          : text.startsWith("-")
            ? "deletion"
            : "context";
    if (kind === "hunk") inHunk = true;
    const line = { from, text, kind };
    from += text.length + 1;
    return line;
  });
}
