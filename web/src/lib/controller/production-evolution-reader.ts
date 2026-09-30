import { fullId, type Snapshot } from "../api/client.js";
import { displayable } from "../graph.js";
import type { StoreEvolutionState } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import { presentGraph } from "./native-presentation.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

export type ProductionEvolutionState = {
  -readonly [K in keyof StoreEvolutionState]: StoreEvolutionState[K];
} & { cursor: string | null; snapshot?: Snapshot; seenCursors: Set<string> };

export const emptyEvolution = (): ProductionEvolutionState => ({
  graph: { revisions: [], devices: [] },
  status: "idle",
  error: "",
  canLoadMore: false,
  predecessorsUnknown: false,
  cursor: null,
  seenCursors: new Set(),
});

export function createProductionEvolutionReader(context: {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  operation(): string | null;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
}) {
  let request = 0;
  return {
    async loadEvolution(more = false) {
      const commit = context.history.route.evolutionCommitId;
      const operation = context.operation();
      if (!commit || !operation) return;
      if (!more) context.data.evolution = emptyEvolution();
      const state = context.data.evolution;
      const source = context.source(),
        ticket = context.version(),
        sequence = ++request;
      const current = () =>
        sequence === request &&
        context.current(ticket, source) &&
        context.history.route.evolutionCommitId === commit &&
        context.data.evolution === state;
      state.status = more ? "refreshing" : "loading";
      state.error = "";
      try {
        const cursor = more ? (state.cursor ?? undefined) : undefined;
        if (cursor && state.seenCursors.has(cursor))
          throw new Error("Repeated evolution cursor.");
        const value = await source.graph("evolution", {
          operation: fullId(operation),
          commit: fullId(commit),
          cursor,
          limit: 50,
        });
        if (!current()) return;
        assertPinned(value.snapshot, source, operation);
        assertSameSnapshot(value.snapshot, more ? state.snapshot : undefined);
        if (value.data.data.graph !== "evolution" || !displayable(value.data))
          throw new Error("Invalid or oversized evolution graph.");
        const graph = presentGraph(value.data, []);
        const revisions = new Map(
          (more ? state.graph.revisions : []).map((revision) => [
            revision.eventId,
            revision,
          ]),
        );
        for (const revision of graph.revisions)
          revisions.set(revision.eventId, revision);
        if (revisions.size > 1000)
          throw new Error("Evolution display limit reached.");
        if (
          value.next_cursor &&
          (value.next_cursor === cursor ||
            state.seenCursors.has(value.next_cursor))
        )
          throw new Error("Repeated evolution cursor.");
        if (cursor) state.seenCursors.add(cursor);
        state.graph = { revisions: [...revisions.values()], devices: [] };
        state.predecessorsUnknown ||= value.data.data.nodes.some(
          (node) => !node.predecessors_known,
        );
        state.cursor = value.next_cursor;
        state.canLoadMore = Boolean(state.cursor);
        state.snapshot = value.snapshot;
        state.status = revisions.size ? "ready" : "empty";
      } catch (cause) {
        if (!current()) return;
        state.status = "failed";
        state.error = storeErrorMessage(cause);
      }
    },
  };
}
