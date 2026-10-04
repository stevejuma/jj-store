import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { WorkController, WorkTransport } from "./work-types.js";
export type ProjectsTab =
  "projects" | "tasks" | "reviews" | "repositories" | "activity" | "agents";
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
  agents?(
    cursor?: string,
    signal?: AbortSignal,
  ): Promise<import("./repository-read-types.js").AgentPage>;
  activity?(
    query: import("./native-activity-types.js").NativeActivityQuery,
    signal?: AbortSignal,
  ): Promise<import("./native-activity-types.js").NativeActivityPage>;
  directory?(
    signal?: AbortSignal,
  ): Promise<
    import("./projects-repository-types.js").RepositoryDirectorySummary
  >;
  reviews?(
    cursor?: string,
    signal?: AbortSignal,
  ): Promise<import("./repository-read-types.js").ReviewPage>;
  browse(
    query: WorkBrowseQuery,
    signal?: AbortSignal,
  ): Promise<import("./work-types.js").WorkBrowsePage>;
}
export interface ProjectHome {
  repository: StoreRepositorySummary;
  projects: Model<"WorkProjectSummary">[];
  tasks: Model<"WorkTaskSummary">[];
  snapshot: import("./work-types.js").WorkSnapshot | null;
  cursor: string | null;
  total: number;
  loading: boolean;
  error: string;
}
export interface ProjectsHubController {
  readonly agents: ReturnType<
    typeof import("./projects-agents.svelte.js").createProjectsAgents
  >;
  readonly activity: ReturnType<
    typeof import("./projects-activity.svelte.js").createProjectsActivity
  >;
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
  refresh(
    changes?: readonly import("../api/application-events.js").FreshnessChange[],
  ): Promise<void>;
  invalidate?(
    changes: readonly import("../api/application-events.js").FreshnessChange[],
  ): void;
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
        | "workTab"
        | "workQuery"
        | "workState"
        | "workRepository"
        | "workHome"
        | "workActivity"
      >
    >,
  ): Promise<void>;
  transport(home: string): WorkBrowseTransport;
  work(home: string): WorkController;
  reset(): void;
}

export interface ProjectsHubContext {
  agentResponse?(home: string): ProjectAgentResponse;
  repositories(): readonly StoreRepositorySummary[];
  route(): StoreRoute;
  push(route: StoreRoute): void;
  source(home: string): WorkBrowseTransport;
  changed(): void;
  activate?(home: string | null): void;
  discover?(): Promise<void>;
}
export interface ProjectAgentResponse {
  controller: import("./contracts.js").StoreController;
  load(session: string): Promise<void>;
  reset(): void;
}
