import { ApiError, Client, type Model } from "../api/client.js";
import { HttpThreadsDataSource } from "./http-threads-data-source.js";
import type { StoreRepositorySummary } from "./contracts.js";
import { createStoreQueryClient } from "./store-query-client.js";

export interface RepositoryThreadEntry {
  threads: readonly Model<"ThreadSummaryV1">[];
  cursor: string | null;
  loading: boolean;
  error: string;
  snapshot?: Model<"CollabReadSnapshotV1">;
}
export type RepositoryThreadNavigation = ReturnType<
  typeof createRepositoryThreadNavigation
>;
export function supportsRepositoryThreads(repo: StoreRepositorySummary) {
  return (repo.profile ?? "native") === "native" && !!repo.permission;
}

export function createProductionRepositoryThreadNavigation(
  repositories: () => readonly StoreRepositorySummary[],
  credential: () => string | null,
  fetcher?: typeof fetch,
) {
  return createRepositoryThreadNavigation({
    repositories,
    credential,
    source(repository) {
      const client = new Client(repository, credential(), () => {}, fetcher);
      const queries = createStoreQueryClient();
      let active = true;
      const transport = new HttpThreadsDataSource(
        client,
        queries,
        { repository, profile: "native", session: crypto.randomUUID() },
        () => active,
      );
      return {
        list: (cursor) => transport.list(cursor),
        close() {
          active = false;
          void queries.cancelQueries();
          queries.clear();
          client.close();
        },
      };
    },
  });
}

export function createRepositoryThreadNavigation(context: {
  repositories(): readonly StoreRepositorySummary[];
  credential(): string | null;
  source(repository: string): {
    list(cursor?: string): Promise<Model<"CollabPage">>;
    close(): void;
  };
}) {
  let generation = 0;
  let entries = $state<Record<string, RepositoryThreadEntry>>({});
  const pending = new Map<string, Promise<void>>();
  const readers = new Set<{ close(): void }>();
  const allowed = (id: string) =>
    context
      .repositories()
      .some((r) => r.id === id && supportsRepositoryThreads(r));
  function clear() {
    generation++;
    for (const reader of readers) reader.close();
    readers.clear();
    pending.clear();
    entries = {};
  }
  async function read(repository: string, more = false) {
    if (!allowed(repository)) return;
    if (pending.has(repository)) return pending.get(repository);
    const ticket = generation,
      credential = context.credential();
    const retained = entries[repository];
    if (more && !retained?.cursor) return;
    const valid = () =>
      ticket === generation &&
      credential === context.credential() &&
      allowed(repository);
    const task = Promise.resolve().then(async () => {
      entries[repository] = {
        threads: retained?.threads ?? [],
        cursor: retained?.cursor ?? null,
        loading: true,
        error: "",
      };
      let source: ReturnType<typeof context.source> | undefined;
      try {
        source = context.source(repository);
        readers.add(source);
        let cursor = more ? (retained?.cursor ?? undefined) : undefined;
        const rows = more ? [...(retained?.threads ?? [])] : [];
        const count = more ? 0 : (retained?.threads.length ?? 0);
        let snapshot = more ? retained?.snapshot : undefined;
        for (let page = 0; page < 20; page++) {
          const result = await source.list(cursor);
          if (!valid()) return;
          if (
            result.snapshot.repo_id !== repository ||
            result.items.type !== "threads" ||
            (snapshot &&
              (result.snapshot.thread_root !== snapshot.thread_root ||
                result.snapshot.epoch !== snapshot.epoch))
          )
            throw new ApiError("invalid_thread_page");
          snapshot = result.snapshot;
          rows.push(...result.items.threads);
          cursor = result.next_cursor ?? undefined;
          if (more || !cursor || rows.length >= count) break;
        }
        entries[repository] = {
          threads: [
            ...new Map(rows.map((row) => [row.value.thread_id, row])).values(),
          ],
          cursor: cursor ?? null,
          loading: false,
          error: "",
          snapshot,
        };
      } catch (cause) {
        if (valid())
          entries[repository] = {
            threads:
              cause instanceof ApiError &&
              ["access_denied", "unauthorized"].includes(cause.code)
                ? []
                : (retained?.threads ?? []),
            cursor: null,
            loading: false,
            error:
              cause instanceof Error
                ? cause.message
                : "Could not load repository threads.",
          };
      } finally {
        if (source) {
          source.close();
          readers.delete(source);
        }
        if (pending.get(repository) === task) pending.delete(repository);
      }
    });
    pending.set(repository, task);
    return task;
  }
  async function load(refresh = false) {
    const ticket = generation;
    for (const id of Object.keys(entries)) if (!allowed(id)) delete entries[id];
    const repositories = context
      .repositories()
      .filter(
        (repo) =>
          supportsRepositoryThreads(repo) && (refresh || !entries[repo.id]),
      );
    let index = 0;
    await Promise.all(
      Array.from({ length: Math.min(2, repositories.length) }, async () => {
        while (ticket === generation && index < repositories.length)
          await read(repositories[index++]!.id);
      }),
    );
  }
  return {
    get entries() {
      return entries;
    },
    load,
    read,
    more: (repository: string) => read(repository, true),
    clear,
  };
}
