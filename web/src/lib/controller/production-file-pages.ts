import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { Model, Snapshot } from "../api/client.js";

export interface StoredPathPage {
  path: string;
  recursive: boolean;
  entries: Model<"Entry">[];
  cursor: string | null;
  snapshot: Snapshot;
}

export const pathPageKey = (path: string, recursive: boolean) =>
  `${recursive ? "recursive" : "directory"}:${path}`;

export function storePathPage(
  pages: ReadonlyMap<string, StoredPathPage>,
  page: StoredPathPage,
  more: boolean,
): Map<string, StoredPathPage> {
  const next = new Map(pages);
  const previous = next.get(pathPageKey(page.path, page.recursive));
  next.set(pathPageKey(page.path, page.recursive), {
    ...page,
    entries:
      more && previous ? [...previous.entries, ...page.entries] : page.entries,
  });
  return next;
}

export function visiblePathEntries(
  pages: ReadonlyMap<string, StoredPathPage>,
  mode: FileListingViewMode,
): Model<"Entry">[] {
  const recursive = mode !== "tree";
  const candidates = recursive
    ? [pages.get(pathPageKey("", true))].filter(Boolean)
    : [...pages.values()].filter((page) => !page.recursive);
  const entries = new Map<string, Model<"Entry">>();
  for (const page of candidates)
    for (const entry of page?.entries ?? []) entries.set(entry.path, entry);
  return [...entries.values()];
}

export function nextPathPage(
  pages: ReadonlyMap<string, StoredPathPage>,
  mode: FileListingViewMode,
  directory: string,
): StoredPathPage | undefined {
  if (mode !== "tree") {
    const page = pages.get(pathPageKey("", true));
    return page?.cursor ? page : undefined;
  }
  const selected = pages.get(pathPageKey(directory, false));
  if (selected?.cursor) return selected;
  return [...pages.values()].find((page) => !page.recursive && page.cursor);
}
