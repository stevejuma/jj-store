import type { StoreLoadStatus } from "./contracts.js";
export type StoreSearchResultKind = "operation" | "commit" | "file";
export interface StoreSearchResult {
  id: string;
  kind: StoreSearchResultKind;
  title: string;
  description: string;
  author: string;
  timestamp: number;
  operationId: string | null;
  commitId: string | null;
  changeId: string | null;
  scope: string | null;
  path: string | null;
  hasConflict: boolean;
}
export interface StoreSearchState {
  readonly available: boolean;
  readonly query: string;
  readonly results: readonly StoreSearchResult[];
  readonly status: StoreLoadStatus;
  readonly error: string;
  readonly canLoadMore: boolean;
  readonly isStale: boolean;
  readonly degraded: boolean;
  readonly truncated: boolean;
}
