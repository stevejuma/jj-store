import { ApiError, pretty } from "../api/client.js";
import { reloadProjectHomeWindow } from "./projects-home-read.js";
import type { StoreRepositorySummary } from "./contracts.js";
import type { ProjectHome, WorkBrowseTransport } from "./projects-hub-types.js";

/** The sidebar keeps its own unfiltered home snapshots and loaded window. */
export function createProjectsNavigation(context: {
  repositories(): readonly StoreRepositorySummary[];
  transport(home: string): WorkBrowseTransport;
  denied(home: string): void;
}) {
  let homes = $state<ProjectHome[]>([]),
    loading = $state(false),
    loaded = false;
  let generation = 0,
    reads = new AbortController();
  async function read(home: ProjectHome, ticket: number, more = false) {
    home.loading = true;
    home.error = "";
    try {
      const page = await context.transport(home.repository.id).browse(
        {
          kind: "projects",
          limit: 50,
          ...(more && home.cursor ? { cursor: home.cursor } : {}),
        },
        reads.signal,
      );
      if (ticket !== generation) return;
      if (more && pretty(home.snapshot) !== pretty(page.snapshot))
        throw new ApiError("snapshot_mismatch");
      home.projects = more
        ? [...home.projects, ...page.projects]
        : page.projects;
      home.snapshot = page.snapshot;
      home.cursor = page.next_cursor;
      home.total = Number(page.total_matches);
    } catch (cause) {
      if (ticket !== generation) return;
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      ) {
        context.denied(home.repository.id);
        home.projects = [];
        home.snapshot = null;
        home.cursor = null;
      }
      home.error =
        cause instanceof Error ? cause.message : "Projects unavailable";
    } finally {
      if (ticket === generation) home.loading = false;
    }
  }
  async function load(refresh = false) {
    if (!refresh && (loaded || loading)) return;
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation;
    loaded = true;
    loading = true;
    homes = context.repositories().map((repository) => ({
      ...(homes.find((home) => home.repository.id === repository.id) ?? {
        projects: [],
        tasks: [],
        snapshot: null,
        cursor: null,
        total: 0,
        loading: false,
        error: "",
      }),
      repository,
    }));
    let index = 0;
    try {
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            await reloadProjectHomeWindow(
              home,
              (more) => read(home, ticket, more),
              refresh,
              () => ticket === generation,
            );
          }
        }),
      );
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  return {
    get homes() {
      return homes;
    },
    get loading() {
      return loading;
    },
    load,
    async more(id: string) {
      const home = homes.find((home) => home.repository.id === id);
      if (home?.cursor && !home.loading) await read(home, generation, true);
    },
    reset() {
      generation++;
      reads.abort();
      homes = [];
      loaded = false;
      loading = false;
    },
  };
}
