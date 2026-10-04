import type {
  CommunityComposerAutocompleteItem,
  CommunityComposerAutocompleteQuery,
} from "./autocomplete.js";
import {
  communityMemberAutocompleteItems,
  communityRoomAutocompleteItems,
  communityEmojiAutocompleteItems,
} from "./autocomplete.js";
import { composerSlashAutocompleteItems } from "./composer-slash-catalog.js";
import { parseComposerSlashQuery } from "./composer-slash.js";
import type { ComposerSlashSource } from "./composer-slash-context.js";
import type { ComposerSlashCatalog } from "./composer-slash-catalog.js";
import type { ComposerProps } from "./props.js";
export function createComposerAutocomplete(options: {
  props(): Pick<ComposerProps, "members" | "rooms" | "communityEmojis">;
  catalog(): ComposerSlashCatalog;
  source?: ComposerSlashSource;
  accept(
    query: CommunityComposerAutocompleteQuery | undefined,
    items: CommunityComposerAutocompleteItem[],
  ): void;
}) {
  let generation = 0,
    request = new AbortController();
  function clear() {
    generation++;
    request.abort();
    options.accept(undefined, []);
  }
  async function update(next: CommunityComposerAutocompleteQuery | null) {
    request.abort();
    request = new AbortController();
    const signal = request.signal,
      ticket = ++generation;
    if (!next) {
      clear();
      return;
    }
    const props = options.props(),
      catalog = options.catalog();
    const found =
      next.kind === "member"
        ? communityMemberAutocompleteItems(props.members ?? [], next.query)
        : next.kind === "room"
          ? communityRoomAutocompleteItems(props.rooms ?? [], next.query)
          : next.kind === "slash"
            ? composerSlashAutocompleteItems(next.query, catalog)
            : await communityEmojiAutocompleteItems(
                props.communityEmojis ?? [],
                next.query,
              );
    if (ticket === generation) options.accept(next, found);
    if (next.kind !== "slash" || !options.source?.search) return;
    const parsed = parseComposerSlashQuery(next.query);
    if (parsed.stage !== "entry" || !parsed.kind) return;
    try {
      await new Promise<void>((resolve) => {
        const timer = setTimeout(done, 160);
        function done() {
          clearTimeout(timer);
          signal.removeEventListener("abort", done);
          resolve();
        }
        signal.addEventListener("abort", done, { once: true });
      });
      if (ticket !== generation || signal.aborted) return;
      const remote = await options.source.search(
        parsed.kind,
        parsed.filter,
        signal,
      );
      if (ticket !== generation || signal.aborted) return;
      const rows = [
        ...new Map(
          [...catalog[parsed.kind], ...remote].map((row) => [row.href, row]),
        ).values(),
      ];
      options.accept(
        next,
        composerSlashAutocompleteItems(next.query, {
          ...catalog,
          [parsed.kind]: rows,
        }),
      );
    } catch {
      /* Keep authorized local references on unavailable source search. */
    }
  }
  return { clear, update };
}
