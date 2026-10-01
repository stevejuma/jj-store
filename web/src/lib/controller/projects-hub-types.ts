import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { WorkController, WorkTransport } from "./work-types.js";
export type ProjectsTab = "projects" | "tasks" | "reviews" | "repositories";
export type WorkBrowseQuery = {
  kind?: "projects" | "tasks";
  project?: string;
  repository?: string;
  parent?: string;
  state?: "active" | "open" | "in_progress" | "blocked" | "done";
  q?: string;
  cursor?: string;
  limit?: number;
};
export interface WorkBrowseTransport extends WorkTransport {
  directory?(
    signal?: AbortSignal,
  ): Promise<
    import("./projects-repository-types.js").RepositoryDirectorySummary
  >;
  reviews?(
    cursor?: string,
    signal?: AbortSignal,
  ): Promise<Model<"ReviewThreadPage">>;
  browse(
    query: WorkBrowseQuery,
    signal?: AbortSignal,
  ): Promise<Model<"WorkBrowsePage">>;
}
export interface ProjectHome {
  repository: StoreRepositorySummary;
  projects: Model<"WorkProjectSummary">[];
  tasks: Model<"WorkTaskSummary">[];
  snapshot: Model<"WorkReadSnapshot"> | null;
  cursor: string | null;
  total: number;
  loading: boolean;
  error: string;
}
export interface ProjectsHubController {
  readonly repositories: ReturnType<
    typeof import("./projects-repositories.svelte.js").createProjectsRepositories
  >;
  readonly reviews: ReturnType<
    typeof import("./projects-reviews.svelte.js").createProjectsReviews
  >;
  readonly navigation: ReturnType<
    typeof import("./projects-navigation.svelte.js").createProjectsNavigation
  >;
  readonly commands: readonly { home: string; work: WorkController }[];
  readonly scroll: Map<string, number>;
  readonly homes: readonly ProjectHome[];
  readonly loading: boolean;
  readonly error: string;
  readonly detail: WorkController | undefined;
  readonly route: StoreRoute;
  restore(): Promise<void>;
  refresh(): Promise<void>;
  more(home: string): Promise<void>;
  openHub(tab?: ProjectsTab): Promise<void>;
  openProject(
    home: string,
    project: string,
    task?: string | null,
  ): Promise<void>;
  filter(
    fields: Partial<
      Pick<
        StoreRoute,
        "workTab" | "workQuery" | "workState" | "workRepository" | "workHome"
      >
    >,
  ): Promise<void>;
  transport(home: string): WorkBrowseTransport;
  work(home: string): WorkController;
  reset(): void;
}

export interface ProjectsHubContext {
  repositories(): readonly StoreRepositorySummary[];
  route(): StoreRoute;
  push(route: StoreRoute): void;
  source(home: string): WorkBrowseTransport;
  changed(): void;
  activate?(home: string | null): void;
  discover?(): Promise<void>;
}
