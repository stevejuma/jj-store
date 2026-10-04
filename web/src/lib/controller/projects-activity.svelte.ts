import { snapshotOperation } from "./repository-profile.js";
import { ApiError } from "../api/client.js";
import type { StoreRepositorySummary } from "./contracts.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";
import type { NativeActivityPage } from "./native-activity-types.js";
import { assertSameSnapshot } from "./production-snapshot.js";

export interface ProjectsActivityHome {
  repository: StoreRepositorySummary;
  page: NativeActivityPage | null;
  loading: boolean;
  error: string;
}
export function createProjectsActivity(context: {
  source(id: string): WorkBrowseTransport;
}) {
  let homes = $state<ProjectsActivityHome[]>([]),
    generation = 0;
  let reads = new AbortController();
  async function read(
    home: ProjectsActivityHome,
    refresh: boolean,
    ticket: number,
    more = false,
    operation?: string,
  ) {
    home.loading = true;
    home.error = "";
    try {
      const source = context.source(home.repository.id);
      if (!source.activity)
        throw new Error("Repository activity is unavailable.");
      const previous = home.page;
      const page = await source.activity(
        more && previous
          ? {
              operation: snapshotOperation(previous.snapshot),
              cursor: previous.cursor ?? undefined,
              atTimeMillis: previous.atTimeMillis,
              agentSnapshot: previous.agentSnapshot,
            }
          : { refresh, ...(operation ? { operation } : {}) },
        reads.signal,
      );
      if (ticket !== generation) return;
      if (more && previous) {
        assertSameSnapshot(page.snapshot, previous.snapshot);
        if (
          JSON.stringify(page.agentSnapshot) !==
          JSON.stringify(previous.agentSnapshot)
        )
          throw new Error("Agent context snapshot changed. Refresh to retry.");
        if (page.cursor && page.cursor === previous.cursor)
          throw new Error("Repeated activity cursor.");
        const known = new Set(
          previous.commits.map((row) => row.revision.eventId),
        );
        page.commits = [
          ...previous.commits,
          ...page.commits.filter((row) => !known.has(row.revision.eventId)),
        ];
        if (page.commits.length > 1000)
          throw new Error("Activity exceeds the browser read bound.");
      }
      home.page = page;
    } catch (cause) {
      if (ticket !== generation) return;
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      )
        home.page = null;
      home.error =
        cause instanceof Error ? cause.message : "Activity unavailable";
    } finally {
      if (ticket === generation) home.loading = false;
    }
  }
  return {
    get homes() {
      return homes;
    },
    async load(
      repositories: readonly StoreRepositorySummary[],
      refresh = false,
      operations: Readonly<Record<string, string>> = {},
      affected?: ReadonlySet<string>,
    ) {
      reads.abort();
      reads = new AbortController();
      const ticket = ++generation;
      homes = repositories.map((repository) => ({
        repository,
        page:
          homes.find(
            (h) =>
              h.repository.id === repository.id &&
              (!operations[repository.id] ||
                (h.page &&
                  snapshotOperation(h.page.snapshot) ===
                    operations[repository.id])),
          )?.page ?? null,
        loading: false,
        error: "",
      }));
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            if (affected && !affected.has(home.repository.id) && home.page)
              continue;
            const count = Math.min(home.page?.commits.length ?? 0, 1000);
            if (!home.page || refresh)
              await read(
                home,
                refresh,
                ticket,
                false,
                operations[home.repository.id],
              );
            const seen = new Set<string>();
            while (
              refresh &&
              home.page?.cursor &&
              !home.error &&
              ticket === generation &&
              home.page.commits.length < count
            ) {
              if (seen.has(home.page.cursor))
                throw new Error("Repeated activity cursor.");
              seen.add(home.page.cursor);
              await read(home, false, ticket, true);
            }
          }
        }),
      );
    },
    async more(id: string) {
      const home = homes.find((h) => h.repository.id === id);
      if (home?.page?.cursor && !home.loading)
        await read(home, false, generation, true);
    },
    reset() {
      generation++;
      reads.abort();
      homes = [];
    },
  };
}
