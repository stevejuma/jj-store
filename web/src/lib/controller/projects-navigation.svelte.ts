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
    loading = $state(false);
  let visible = false,
    revision = 0;
  const dirty = new Map<string, number>();
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
  let pendingLoad: Promise<void> | null = null;
  async function loadPages(refresh = false, affected?: ReadonlySet<string>) {
    const allowed = new Set(context.repositories().map((repo) => repo.id));
    for (const id of dirty.keys()) if (!allowed.has(id)) dirty.delete(id);
    reads.abort();
    reads = new AbortController();
    const ticket = ++generation;
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
            const id = home.repository.id,
              stamp = dirty.get(id);
            if (
              home.snapshot &&
              ((!refresh && stamp === undefined) ||
                (affected && !affected.has(id)))
            )
              continue;
            await reloadProjectHomeWindow(
              home,
              (more) => read(home, ticket, more),
              refresh,
              () => ticket === generation,
            );
            if (ticket === generation && !home.error && dirty.get(id) === stamp)
              dirty.delete(id);
          }
        }),
      );
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  function load(refresh = false, affected?: ReadonlySet<string>) {
    if (!refresh && pendingLoad) return pendingLoad;
    const request = loadPages(refresh, affected);
    pendingLoad = request;
    void request
      .finally(() => {
        if (pendingLoad === request) pendingLoad = null;
      })
      .catch(() => {});
    return request;
  }
  return {
    get dirty() {
      return dirty.size > 0;
    },
    get visible() {
      return visible;
    },
    setVisible(value: boolean) {
      visible = value;
      if (visible) return load();
      return Promise.resolve();
    },
    invalidate(affected?: ReadonlySet<string>) {
      for (const repo of context.repositories())
        if (!affected || affected.has(repo.id)) dirty.set(repo.id, ++revision);
    },
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
      pendingLoad = null;
      dirty.clear();
      visible = false;
      loading = false;
    },
  };
}
