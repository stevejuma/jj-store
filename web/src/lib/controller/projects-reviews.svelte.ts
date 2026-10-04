import { ApiError, pretty } from "../api/client.js";
import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";
export interface ProjectReviewHome {
  repository: StoreRepositorySummary;
  threads: Model<"ReviewThreadSummary">[];
  snapshot: import("./repository-read-types.js").ReviewSnapshot | null;
  cursor: string | null;
  loading: boolean;
  unsupported: boolean;
  error: string;
}
export function createProjectsReviews(context: {
  source(home: string): WorkBrowseTransport;
  denied(home: string): void;
}) {
  let homes = $state<ProjectReviewHome[]>([]),
    generation = 0,
    reads = new AbortController();
  const views = new Map<string, ProjectReviewHome[]>();
  const key = (route: StoreRoute) =>
    `${route.projectId ? route.repository : ""}:${route.projectId ?? ""}:${route.workRepository ?? ""}`;
  const copy = () => homes.map((home) => ({ ...home }));
  function save(route: StoreRoute) {
    if (homes.some((home) => home.loading || home.error)) return;
    views.set(key(route), copy());
    if (views.size > 16) views.delete(views.keys().next().value!);
  }
  async function read(home: ProjectReviewHome, ticket: number, more = false) {
    if (home.unsupported) return;
    home.loading = true;
    home.error = "";
    try {
      const source = context.source(home.repository.id);
      if (!source.reviews) throw new Error("Reviews unavailable");
      const page = await source.reviews(
        more ? (home.cursor ?? undefined) : undefined,
        reads.signal,
      );
      if (ticket !== generation) return;
      if (more && pretty(home.snapshot) !== pretty(page.snapshot))
        throw new ApiError("snapshot_mismatch");
      home.threads = more ? [...home.threads, ...page.threads] : page.threads;
      home.snapshot = page.snapshot;
      home.cursor = page.next_cursor;
    } catch (cause) {
      if (ticket !== generation) return;
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      ) {
        context.denied(home.repository.id);
        views.clear();
        home.threads = [];
        home.snapshot = null;
        home.cursor = null;
      }
      home.error =
        cause instanceof Error ? cause.message : "Reviews unavailable";
    } finally {
      if (ticket === generation) home.loading = false;
    }
  }
  return {
    get homes() {
      return homes;
    },
    async load(
      route: StoreRoute,
      repositories: readonly StoreRepositorySummary[],
      refresh = false,
      affected?: ReadonlySet<string>,
    ) {
      reads.abort();
      reads = new AbortController();
      const ticket = ++generation;
      if (refresh) views.clear();
      const cached = views.get(key(route));
      const permitted = repositories.filter(
        (repo) => !route.workRepository || repo.id === route.workRepository,
      );
      if (
        cached &&
        cached.length === permitted.length &&
        cached.every((home) =>
          permitted.some((repo) => repo.id === home.repository.id),
        )
      ) {
        homes = cached.map((home) => ({
          ...home,
          repository: permitted.find((repo) => repo.id === home.repository.id)!,
        }));
        return;
      }
      const counts = new Map(
        homes.map((home) => [home.repository.id, home.threads.length]),
      );
      homes = repositories
        .filter(
          (repo) => !route.workRepository || repo.id === route.workRepository,
        )
        .map((repository) => ({
          repository,
          threads:
            affected && !affected.has(repository.id)
              ? (homes.find((home) => home.repository.id === repository.id)
                  ?.threads ?? [])
              : [],
          snapshot:
            affected && !affected.has(repository.id)
              ? (homes.find((home) => home.repository.id === repository.id)
                  ?.snapshot ?? null)
              : null,
          cursor:
            affected && !affected.has(repository.id)
              ? (homes.find((home) => home.repository.id === repository.id)
                  ?.cursor ?? null)
              : null,
          loading: false,
          unsupported: !context.source(repository.id).reviews,
          error: "",
        }));
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            if (affected && !affected.has(home.repository.id) && home.snapshot)
              continue;
            await read(home, ticket);
            const count = Math.min(counts.get(home.repository.id) ?? 0, 4096),
              seen = new Set<string>();
            while (
              refresh &&
              home.cursor &&
              !home.error &&
              ticket === generation &&
              home.threads.length < count
            ) {
              if (seen.has(home.cursor))
                throw new Error("Repeated review cursor.");
              seen.add(home.cursor);
              await read(home, ticket, true);
            }
          }
        }),
      );
      if (ticket === generation) save(route);
    },
    async more(route: StoreRoute, id: string) {
      const home = homes.find((home) => home.repository.id === id);
      if (home?.cursor && !home.loading) await read(home, generation, true);
      save(route);
    },
    reset() {
      generation++;
      reads.abort();
      homes = [];
      views.clear();
    },
  };
}
