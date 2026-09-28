import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { Snapshot } from "../api/client.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type {
  RepositoryBlob,
  RepositoryChoice,
  RepositorySnapshot,
  StoreControllerStatus,
} from "./contracts.js";
import type { Comparison } from "./production-store-loaders.js";
import type { InspectionContinuation } from "./production-inspection-pages.js";
import type { StoredPathPage } from "./production-file-pages.js";

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
  fileViewMode: FileListingViewMode;
  pathPages: Map<string, StoredPathPage>;
  knownDirectories: Set<string>;
  collapsedDirectoryPaths: Set<string>;
  fileTerms: Map<string, string>;
  comparisons: Map<string, Comparison>;
  inspectionContinuation: InspectionContinuation | null;
  neededIndex: Snapshot | null;
  refsPageSnapshot: Snapshot | null;
  graphPageSnapshot: Snapshot | null;
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
  const state: ProductionStoreState = $state({
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
    fileViewMode: "tree",
    pathPages: new Map(),
    knownDirectories: new Set(),
    collapsedDirectoryPaths: new Set(),
    fileTerms: new Map(),
    comparisons: new Map(),
    inspectionContinuation: null,
    neededIndex: null,
    refsPageSnapshot: null,
    graphPageSnapshot: null,
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
  state.fileViewMode = "tree";
  state.pathPages = new Map();
  state.knownDirectories = new Set();
  state.collapsedDirectoryPaths = new Set();
  state.fileTerms = new Map();
  state.comparisons = new Map();
  state.inspectionContinuation = null;
  state.refsPageSnapshot = null;
  state.graphPageSnapshot = null;
}
