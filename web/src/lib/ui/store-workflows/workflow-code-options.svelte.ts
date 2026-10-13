import { untrack } from "svelte";
import type { StoreDataSource } from "../../controller/native-data-source.js";
import type { HistoryQueryInfo } from "../../controller/history-query-types.js";
import {
  snapshotOperation,
  snapshotRepository,
} from "../../controller/repository-profile.js";

/** Suggestions are read from the chosen source, never the active history view. */
export function createWorkflowCodeOptions(
  source: () => StoreDataSource | undefined,
) {
  let authors = $state<{ id: string; name: string }[]>([]),
    info = $state<HistoryQueryInfo | null>(null),
    symbols = $state<string[]>([]),
    cursor = $state<string | null>(null),
    loading = $state(false),
    error = $state("");
  let generation = 0,
    operation = "",
    atTimeMillis = 0;
  async function read(more = false) {
    const reader = source(),
      ticket = generation;
    if (!reader || loading || (more && !cursor)) return;
    const current = () => ticket === generation && reader === source();
    loading = true;
    try {
      if (!more) {
        const head = await reader.snapshot(true);
        if (!current()) return;
        if (
          ("repository" in head ? head.repository : head.repo_id) !==
          reader.repository
        )
          throw Error("Source repository changed. Retry to refresh authors.");
        operation = head.heads[0] ?? "";
        atTimeMillis = Date.now();
        if (!operation)
          throw Error("No published operation is available for this source.");
        const metadata = await reader.queryInfo?.(operation);
        if (!current()) return;
        info = metadata ?? null;
      }
      const page = await reader.graph("log", {
        operation,
        atTimeMillis,
        limit: 50,
        cursor: more ? (cursor ?? undefined) : undefined,
      });
      if (!current()) return;
      if (
        snapshotRepository(page.snapshot) !== reader.repository ||
        snapshotOperation(page.snapshot) !== operation ||
        page.data.data.graph !== "log"
      )
        throw Error("Author history changed. Retry to refresh it.");
      authors = [
        ...new Map(
          [
            ...(more ? authors : []),
            ...page.data.data.nodes.flatMap(({ commit }) =>
              commit.author.email
                ? [
                    {
                      id: commit.author.email,
                      name: commit.author.name || commit.author.email,
                    },
                  ]
                : [],
            ),
          ].map((a) => [a.id, a]),
        ).values(),
      ];
      symbols = [
        ...new Set([
          ...(more ? symbols : []),
          ...page.data.data.nodes.flatMap(({ commit }) =>
            commit.bookmarks.map((b) => b.name),
          ),
        ]),
      ];
      cursor = page.next_cursor;
      error = "";
    } catch (cause) {
      if (ticket === generation && reader === source())
        error =
          cause instanceof Error
            ? cause.message
            : "Could not load source authors and query syntax.";
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  $effect(() => {
    const reader = source();
    untrack(() => {
      generation++;
      authors = [];
      info = null;
      symbols = [];
      cursor = null;
      loading = false;
      error = "";
      if (reader) void read();
    });
    return () => {
      generation++;
    };
  });
  return {
    get authors() {
      return authors;
    },
    get info() {
      return info;
    },
    get symbols() {
      return symbols;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    get hasMore() {
      return !!cursor;
    },
    refresh: () => read(),
    more: () => read(true),
  };
}
