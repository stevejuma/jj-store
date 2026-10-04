import type {
  RepositoryBlob,
  StoreControllerState,
} from "../controller/contracts.js";
export function describeFileStats(blob: RepositoryBlob): string {
  if (blob.binary) {
    return "Binary file · Size unavailable";
  }
  const contents = blob.contents ?? "";
  const normalized = contents.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const contentWithoutTerminalNewline = normalized.endsWith("\n")
    ? normalized.slice(0, -1)
    : normalized;
  const lines = contentWithoutTerminalNewline
    ? contentWithoutTerminalNewline.split("\n")
    : [];
  const lineCount = lines.length;
  const locCount = lines.filter((line) => line.trim().length > 0).length;
  const byteCount = new TextEncoder().encode(contents).length;
  const lineLabel = lineCount === 1 ? "line" : "lines";
  const byteLabel = byteCount === 1 ? "Byte" : "Bytes";
  return `${lineCount} ${lineLabel} (${locCount} loc) · ${byteCount} ${byteLabel}`;
}

export function fileEditUnavailable(
  view: StoreControllerState,
  blob: RepositoryBlob | undefined,
): string {
  if (!view.authenticated)
    return "Connect with repository write access to edit this file.";
  if (!view.mutation.available.includes("edit_file"))
    return "This repository server does not support file editing.";
  const bookmark = view.bookmarks.find(
    (candidate) => candidate.name === view.route.bookmark,
  );
  if (!view.route.bookmark || !bookmark)
    return "Choose a local bookmark to edit this file.";
  if (
    bookmark.targets.length !== 1 ||
    view.targets.length !== 1 ||
    bookmark.targets[0] !== view.targets[0]
  )
    return "Choose the bookmark's single current target to edit this file.";
  if (!blob || blob.binary || blob.too_large || blob.contents === undefined)
    return "Only bounded UTF-8 text files can be edited.";
  return "";
}
