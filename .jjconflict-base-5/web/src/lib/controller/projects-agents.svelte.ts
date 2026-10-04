import { ApiError, pretty } from "../api/client.js";
import type { Model } from "../api/client.js";
import type { StoreRepositorySummary } from "./contracts.js";
import type { WorkBrowseTransport } from "./projects-hub-types.js";
import type { AgentSnapshot } from "./repository-read-types.js";
import { createProjectsAgentSelection } from "./projects-agent-selection.svelte.js";
import type { StoreRoute } from "./contracts.js";
import type { ProjectAgentResponse } from "./projects-hub-types.js";
export interface ProjectAgentHome {
  repository: StoreRepositorySummary;
  sessions: Model<"AgentSessionSummary">[];
  snapshot: AgentSnapshot | null;
  cursor: string | null;
  loading: boolean;
  unsupported: boolean;
  error: string;
}
/** Independent repository snapshots; project metadata never grants AgentStore access. */
export function createProjectsAgents(context: {
  source(home: string): WorkBrowseTransport;
  denied(home: string): void;
  route(): StoreRoute;
  push(route: StoreRoute): void;
  response?(home: string): ProjectAgentResponse;
}) {
  let homes = $state<ProjectAgentHome[]>([]),
    generation = 0;
  let reads = new AbortController(),
    scope = "";
  const selection = createProjectsAgentSelection({
    ...context,
    homes: () => homes,
  });
  async function read(home: ProjectAgentHome, ticket: number, more = false) {
    home.loading = true;
    home.error = "";
    try {
      const source = context.source(home.repository.id);
      if (!source.agents) {
        home.unsupported = true;
        return;
      }
      const page = await source.agents(
        more ? (home.cursor ?? undefined) : undefined,
        reads.signal,
      );
      if (ticket !== generation) return;
      if (
        page.snapshot.repo_id !== home.repository.id ||
        (more && pretty(home.snapshot) !== pretty(page.snapshot))
      )
        throw new ApiError("snapshot_mismatch");
      home.sessions = more
        ? [...home.sessions, ...page.sessions]
        : page.sessions;
      home.snapshot = page.snapshot;
      home.cursor = page.next_cursor;
    } catch (cause) {
      if (ticket !== generation) return;
      if (
        cause instanceof ApiError &&
        cause.code === "unsupported_local_capability"
      ) {
        home.unsupported = true;
        home.sessions = [];
        home.snapshot = null;
        home.cursor = null;
        return;
      }
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      ) {
        context.denied(home.repository.id);
        if (context.route().workAgentRepository === home.repository.id)
          selection.clearSelection();
        home.sessions = [];
        home.snapshot = null;
        home.cursor = null;
      }
      home.error =
        cause instanceof Error ? cause.message : "Agent sessions unavailable";
    } finally {
      if (ticket === generation) home.loading = false;
    }
  }
  return {
    ...selection,
    get response() {
      return selection.response;
    },
    get selectionError() {
      return selection.selectionError;
    },
    get homes() {
      return homes;
    },
    prepare(key: string) {
      if (key !== scope) {
        generation++;
        reads.abort();
        homes = [];
        selection.clearSelection();
        scope = key;
      }
    },
    async load(
      repositories: readonly StoreRepositorySummary[],
      refresh = false,
      affected?: ReadonlySet<string>,
    ) {
      reads.abort();
      reads = new AbortController();
      const ticket = ++generation;
      const counts = new Map(
        homes.map((home) => [
          home.repository.id,
          Math.min(home.sessions.length, 4096),
        ]),
      );
      homes = repositories.map((repository) => ({
        ...(homes.find((home) => home.repository.id === repository.id) ?? {
          sessions: [],
          snapshot: null,
          cursor: null,
          loading: false,
          unsupported: false,
          error: "",
        }),
        repository,
      }));
      let index = 0;
      await Promise.all(
        Array.from({ length: Math.min(2, homes.length) }, async () => {
          while (index < homes.length && ticket === generation) {
            const home = homes[index++]!;
            if (affected && !affected.has(home.repository.id) && home.snapshot)
              continue;
            if (home.snapshot && !refresh && !home.error) continue;
            await read(home, ticket);
            const seen = new Set<string>();
            while (
              refresh &&
              home.cursor &&
              !home.error &&
              ticket === generation &&
              home.sessions.length < (counts.get(home.repository.id) ?? 0)
            ) {
              if (seen.has(home.cursor)) {
                home.error = "Repeated agent cursor.";
                break;
              }
              seen.add(home.cursor);
              await read(home, ticket, true);
            }
          }
        }),
      );
    },
    async more(id: string) {
      const home = homes.find((home) => home.repository.id === id);
      if (home?.cursor && !home.loading) await read(home, generation, true);
    },
    reset() {
      generation++;
      reads.abort();
      homes = [];
      scope = "";
      selection.resetSelections();
    },
  };
}
