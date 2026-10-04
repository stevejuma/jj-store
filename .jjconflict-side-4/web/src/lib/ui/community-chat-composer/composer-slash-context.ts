import { getContext, setContext } from "svelte";
import type { ComposerSlashCatalog } from "./composer-slash-catalog.js";
import type { ComposerSlashKind } from "./composer-slash.js";
import type { ComposerSlashEntry } from "./composer-slash-session.js";
export interface ComposerSlashSource {
  catalog(): ComposerSlashCatalog;
  search?(
    kind: ComposerSlashKind,
    query: string,
    signal: AbortSignal,
  ): Promise<readonly ComposerSlashEntry[]>;
}
const slashCatalogKey = Symbol("community-composer-slash-catalog");
export function provideCommunityComposerSlashCatalog(
  source: ComposerSlashSource,
) {
  setContext(slashCatalogKey, source);
}
export function useCommunityComposerSlashCatalog():
  ComposerSlashSource | undefined {
  return getContext(slashCatalogKey);
}
