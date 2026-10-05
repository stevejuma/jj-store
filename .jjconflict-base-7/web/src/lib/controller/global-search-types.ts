import type { StoreRoute } from "./contracts.js";
import type { Model } from "../api/client.js";
export type GlobalSearchScope = {
  label: string;
  repository?: string;
  section?: "all" | "threads" | "projects" | "tasks" | "code";
  thread?: string;
  project?: string;
  task?: string;
};
export interface GlobalSearchResult {
  id: string;
  kind: Model<"ContentKind"> | "repository" | "operation" | "commit" | "file";
  title: string;
  description: string;
  source: string;
  timestamp: number;
  route: StoreRoute;
  revision?: string;
}
export type GlobalSearchController = ReturnType<
  typeof import("./global-search.svelte.js").createGlobalSearch
>;
