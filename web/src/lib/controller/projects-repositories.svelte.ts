import { ApiError } from "../api/client.js";
import type { StoreRepositorySummary } from "./contracts.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";
import type { RepositoryDirectorySummary } from "./projects-repository-types.js";

export interface RepositoryDirectoryEntry {
  repository: StoreRepositorySummary;
  summary: RepositoryDirectorySummary | null;
  loading: boolean;
  error: string;
}
interface Read {
  entry: RepositoryDirectoryEntry;
  ticket: number;
  signal: AbortController;
  done: Promise<void>;
  settle(): void;
}

export function createProjectsRepositories(context: {
  source(id: string): WorkBrowseTransport;
}) {
  let entries = $state<RepositoryDirectoryEntry[]>([]),
    generation = 0;
  const visible = new Map<string, Set<symbol>>(),
    attempted = new Set<string>(),
    stale = new Set<string>(),
    pending = new Map<string, Read>(),
    queued = new Map<string, Read>(),
    active = new Set<Read>();
  const current = (read: Read) =>
    read.ticket === generation &&
    pending.get(read.entry.repository.id) === read &&
    !read.signal.signal.aborted;
  function cancel(id: string) {
    const read = pending.get(id);
    if (!read) return;
    pending.delete(id);
    queued.delete(id);
    attempted.delete(id);
    read.signal.abort();
    read.entry.loading = false;
    if (!active.has(read)) read.settle();
  }
  async function run(read: Read) {
    const id = read.entry.repository.id;
    let failed = false;
    try {
      const source = context.source(id);
      if (source.directory) {
        const summary = await source.directory(read.signal.signal);
        if (current(read)) {
          read.entry.summary = summary;
          read.entry.error = "";
          stale.delete(id);
        }
      }
    } catch (cause) {
      failed = true;
      if (current(read)) {
        if (
          cause instanceof ApiError &&
          ["access_denied", "bearer_required"].includes(cause.code)
        )
          read.entry.summary = null;
        read.entry.error =
          cause instanceof Error
            ? cause.message
            : "Repository details unavailable";
      }
    } finally {
      if (current(read)) {
        read.entry.loading = false;
        pending.delete(id);
      }
      // A directory may fan out to independent metadata reads. A rejected
      // sibling must not leave the other transports running after this job.
      if (failed) read.signal.abort();
      active.delete(read);
      read.settle();
      queueMicrotask(pump);
    }
  }
  function pump() {
    while (active.size < 2 && queued.size) {
      const [id, read] = queued.entries().next().value!;
      queued.delete(id);
      if (!current(read)) {
        read.settle();
        continue;
      }
      active.add(read);
      void run(read);
    }
  }
  function schedule(id: string) {
    const entry = entries.find((entry) => entry.repository.id === id);
    if (
      !entry ||
      !visible.has(id) ||
      pending.has(id) ||
      (!stale.has(id) && (entry.summary || attempted.has(id)))
    )
      return;
    let settle!: () => void;
    const read: Read = {
      entry,
      ticket: generation,
      signal: new AbortController(),
      done: new Promise<void>((resolve) => {
        settle = resolve;
      }),
      settle: () => settle(),
    };
    attempted.add(id);
    entry.loading = true;
    pending.set(id, read);
    queued.set(id, read);
    pump();
  }
  return {
    get visibleIds() {
      return [...visible.keys()];
    },
    get entries() {
      return entries;
    },
    /** A viewport owns this lease; repeated rows may observe the same repository. */
    watch(id: string) {
      const token = Symbol(id),
        leases = visible.get(id) ?? new Set<symbol>();
      leases.add(token);
      visible.set(id, leases);
      schedule(id);
      let released = false;
      return () => {
        if (released) return;
        released = true;
        const leases = visible.get(id);
        if (!leases?.delete(token)) return;
        if (!leases.size) {
          visible.delete(id);
          cancel(id);
        }
      };
    },
    async load(
      repositories: readonly StoreRepositorySummary[],
      refresh = false,
      affected?: ReadonlySet<string>,
    ) {
      generation++;
      for (const id of [...pending.keys()]) cancel(id);
      const previous = new Map(
        entries.map((entry) => [entry.repository.id, entry]),
      );
      entries = repositories.map((repository) => {
        const prior = previous.get(repository.id);
        if (
          refresh &&
          (!affected || affected.has(repository.id)) &&
          (prior?.summary || attempted.has(repository.id))
        )
          stale.add(repository.id);
        return {
          repository,
          summary: prior?.summary ?? null,
          loading: false,
          error: prior?.error ?? "",
        };
      });
      const ids = new Set(repositories.map((repository) => repository.id));
      for (const set of [attempted, stale])
        for (const id of set) if (!ids.has(id)) set.delete(id);
      for (const id of visible.keys()) if (!ids.has(id)) visible.delete(id);
      for (const id of visible.keys()) schedule(id);
      await Promise.all([...pending.values()].map((read) => read.done));
    },
    reset() {
      generation++;
      for (const id of [...pending.keys()]) cancel(id);
      visible.clear();
      attempted.clear();
      stale.clear();
      entries = [];
    },
  };
}
