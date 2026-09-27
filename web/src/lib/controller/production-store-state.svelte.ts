import type { Model, Snapshot } from "../api/client.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type {
  RepositoryBlob,
  RepositoryChoice,
  RepositorySnapshot,
  StoreControllerStatus,
} from "./contracts.js";
import type { Comparison } from "./production-store-loaders.js";

type MutableStatus = {
  -readonly [K in keyof StoreControllerStatus]: StoreControllerStatus[K];
};

export interface ProductionStoreState {
  heads: string[];
  repositories: { id: string; name: string }[];
  bookmarks: Bookmark[];
  choices: RepositoryChoice[];
  graph: RepositoryLogGraph;
  targets: string[];
  snapshot: RepositorySnapshot | undefined;
  blob: RepositoryBlob | undefined;
  detail: RevisionDetail | undefined;
  comparisonParents: string[];
  conflictCount: number | null;
  unavailableComparisonCount: number;
  status: MutableStatus;
  refsCursor: string | null;
  graphCursor: string | null;
  graphAtTimeMillis: number | null;
  pathsCursor: string | null;
  pathEntries: Model<"Entry">[];
  fileTerms: Map<string, string>;
  comparisons: Map<string, Comparison>;
  neededIndex: Snapshot | null;
  refsPageSnapshot: Snapshot | null;
  graphPageSnapshot: Snapshot | null;
  pathsPageSnapshot: Snapshot | null;
}

export const emptyGraph = (): RepositoryLogGraph => ({
  revisions: [],
  devices: [],
});

export const initialStatus = (): MutableStatus => ({
  connection: "disconnected",
  navigation: "idle",
  repositories: "idle",
  bookmarks: "idle",
  graph: "idle",
  files: "idle",
  detail: "idle",
  live: "disconnected",
  refreshAvailable: false,
  error: "",
});

export function createProductionStoreState(): ProductionStoreState {
  const state = $state({
    heads: [],
    repositories: [],
    bookmarks: [],
    choices: [],
    graph: emptyGraph(),
    targets: [],
    snapshot: undefined,
    blob: undefined,
    detail: undefined,
    comparisonParents: [],
    conflictCount: null,
    unavailableComparisonCount: 0,
    status: initialStatus(),
    refsCursor: null,
    graphCursor: null,
    graphAtTimeMillis: null,
    pathsCursor: null,
    pathEntries: [],
    fileTerms: new Map(),
    comparisons: new Map(),
    neededIndex: null,
    refsPageSnapshot: null,
    graphPageSnapshot: null,
    pathsPageSnapshot: null,
  });
  return state;
}

export function clearProductionStoreView(state: ProductionStoreState) {
  state.bookmarks = [];
  state.choices = [];
  state.graph = emptyGraph();
  state.targets = [];
  state.snapshot = undefined;
  state.blob = undefined;
  state.detail = undefined;
  state.comparisonParents = [];
  state.conflictCount = null;
  state.unavailableComparisonCount = 0;
  state.refsCursor = null;
  state.graphCursor = null;
  state.graphAtTimeMillis = null;
  state.pathsCursor = null;
  state.pathEntries = [];
  state.fileTerms = new Map();
  state.comparisons = new Map();
  state.refsPageSnapshot = null;
  state.graphPageSnapshot = null;
  state.pathsPageSnapshot = null;
}
