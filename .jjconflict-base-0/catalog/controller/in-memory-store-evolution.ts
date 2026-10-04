import type {
  StoreEvolutionState,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { RepositoryLogRevision } from "../../web/src/lib/projects/repository-log.js";

/** Explicit rewrite fixtures; commit-parent ancestry never supplies evolution edges. */
export function inMemoryStoreEvolution(
  route: StoreRoute,
  revisions: RepositoryLogRevision[],
): StoreEvolutionState {
  const selected = revisions.find(
    (revision) => revision.eventId === route.commitId,
  );
  const first = "ec".repeat(64),
    second = "ed".repeat(64);
  const history = selected
    ? [
        { ...selected, parents: [first] },
        {
          ...selected,
          eventId: first,
          commitId: first,
          message: "Earlier description of this change",
          parents: [second],
          createdAt: selected.createdAt - 60,
          bookmarks: [],
          workspaces: [],
        },
        {
          ...selected,
          eventId: second,
          commitId: second,
          message: "Initial version of this change",
          parents: [],
          createdAt: selected.createdAt - 120,
          bookmarks: [],
          workspaces: [],
        },
      ]
    : [];
  const index = history.findIndex(
    (revision) => revision.eventId === route.evolutionCommitId,
  );
  return {
    graph: { revisions: index >= 0 ? history.slice(index) : [], devices: [] },
    status: route.evolutionCommitId ? "ready" : "idle",
    error: "",
    canLoadMore: false,
    predecessorsUnknown: false,
  };
}
