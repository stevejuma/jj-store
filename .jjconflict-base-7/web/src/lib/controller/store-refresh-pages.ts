import { nextPathPage } from "./production-file-pages.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreRoute } from "./contracts.js";
import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
export function storeLoadedWindows(data: ProductionStoreState) {
  return {
    references: data.choices.length,
    graph: data.graph.revisions.length,
    files: data.snapshot?.files.length ?? 0,
    detail: data.detail?.files.length ?? 0,
    agents: data.agentSessions.length,
    reviews: data.reviewThreads.length,
    transcript: data.agentTranscript.length,
  };
}
export async function reloadStoreWindows(
  data: ProductionStoreState,
  route: StoreRoute,
  counts: ReturnType<typeof storeLoadedWindows>,
  reader: ReturnType<typeof createProductionStoreReader>,
  current: () => boolean,
) {
  const windows: [
    () => number,
    number,
    () => unknown,
    () => Promise<unknown>,
  ][] = [
    [
      () => data.choices.length,
      counts.references,
      () => data.refsCursor,
      () => reader.loadReferences(true),
    ],
    [
      () => data.graph.revisions.length,
      counts.graph,
      () => data.graphCursor,
      () => reader.loadGraph(true),
    ],
    [
      () => data.snapshot?.files.length ?? 0,
      counts.files,
      () =>
        data.snapshot &&
        nextPathPage(data.pathPages, data.fileViewMode, route.directory),
      () => reader.loadMoreFiles(),
    ],
    [
      () => data.detail?.files.length ?? 0,
      counts.detail,
      () => data.inspectionContinuation,
      () => reader.loadMoreDetail(),
    ],
    [
      () => data.agentSessions.length,
      counts.agents,
      () => data.agentCursor,
      () => reader.loadAgents(true),
    ],
    [
      () => data.reviewThreads.length,
      counts.reviews,
      () => data.reviewCursor,
      () => reader.loadReviews(true),
    ],
    [
      () => data.agentTranscript.length,
      counts.transcript,
      () => data.transcriptCursor,
      () => reader.loadTranscript(true),
    ],
  ];
  for (const [size, retained, more, read] of windows) {
    let previous = -1;
    while (
      current() &&
      more() &&
      size() < Math.min(retained, 1000) &&
      size() !== previous
    ) {
      previous = size();
      await read();
    }
  }
}
