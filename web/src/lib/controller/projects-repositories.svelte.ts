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
export function createProjectsRepositories(context: {
  source(id: string): WorkBrowseTransport;
}) {
  let entries = $state<RepositoryDirectoryEntry[]>([]),
    generation = 0;
  let reads = new AbortController();
  return {
    get entries() {
      return entries;
    },
    async load(
      repositories: readonly StoreRepositorySummary[],
      refresh = false,
    ) {
      reads.abort();
      reads = new AbortController();
      const ticket = ++generation;
      entries = repositories.map((repository) => ({
        repository,
        summary:
          entries.find((entry) => entry.repository.id === repository.id)
            ?.summary ?? null,
        loading: false,
        error: "",
      }));
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, entries.length) }, async () => {
          while (index < entries.length && ticket === generation) {
            const entry = entries[index++]!;
            if (entry.summary && !refresh) continue;
            entry.loading = true;
            try {
              const source = context.source(entry.repository.id);
              if (!source.directory) continue;
              const summary = await source.directory(reads.signal);
              if (ticket === generation) entry.summary = summary;
            } catch (cause) {
              if (ticket !== generation) return;
              if (
                cause instanceof ApiError &&
                ["access_denied", "bearer_required"].includes(cause.code)
              )
                entry.summary = null;
              entry.error =
                cause instanceof Error
                  ? cause.message
                  : "Repository details unavailable";
            } finally {
              if (ticket === generation) entry.loading = false;
            }
          }
        }),
      );
    },
    reset() {
      generation++;
      reads.abort();
      entries = [];
    },
  };
}
