import { ApiError, type Model } from "../../api/client.js";
import type { StoreDataSource } from "../native-data-source.js";
import {
  sameCodeSnapshot,
  type LocalReadSnapshot,
} from "../repository-profile.js";

export async function stackReferencePage(
  source: StoreDataSource,
  pin: LocalReadSnapshot,
  signal: AbortSignal,
  cursor?: string,
) {
  const result = await source.references(pin.operation, cursor, {
    kind: "bookmarks",
    signal,
  });
  signal.throwIfAborted();
  if (
    !sameCodeSnapshot(pin, result.snapshot) ||
    !("references" in result.data) ||
    result.data.version !== 1 ||
    result.data.operation !== pin.operation ||
    result.data.references.length > 50 ||
    (result.next_cursor && result.next_cursor === cursor)
  )
    throw new ApiError("snapshot_mismatch");
  return {
    references: result.data.references as Model<"LocalReferencesReference">[],
    cursor: result.next_cursor ?? undefined,
  };
}

export async function stackReferenceWindow(
  source: StoreDataSource,
  pin: LocalReadSnapshot,
  signal: AbortSignal,
  pages: number,
) {
  const references: Model<"LocalReferencesReference">[] = [],
    seen = new Set<string>();
  let cursor: string | undefined,
    count = 0;
  do {
    const page = await stackReferencePage(source, pin, signal, cursor);
    references.push(...page.references);
    if (cursor) seen.add(cursor);
    cursor = page.cursor;
    if (cursor && seen.has(cursor))
      throw new ApiError("invalid_stack_continuation");
    count++;
  } while (cursor && count < pages);
  return { references, cursor, count, seen };
}
