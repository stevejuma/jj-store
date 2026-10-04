import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
export type WorkRequest = Exact<
  NonNullable<
    operations["work_apply"]["requestBody"]
  >["content"]["application/json"]
>;
export type WorkCommand = WorkRequest["command"];
export type WorkQuery = {
  project?: string;
  task?: string;
  cursor?: string;
  limit?: number;
};
export type WorkSnapshot =
  Model<"WorkReadSnapshot"> | Model<"LocalWorkSnapshot">;
export type WorkPage = Model<"WorkPage"> | Model<"LocalWorkPage">;
export type WorkBrowsePage =
  Model<"WorkBrowsePage"> | Model<"LocalWorkBrowsePage">;
export type WorkActivityPage =
  Model<"WorkActivityPage"> | Model<"LocalWorkActivityPage">;
export type WorkOutcome =
  Model<"WorkCommandOutcome"> | Model<"LocalWorkOutcome">;
export type WorkStatus = Model<"WorkCommandStatus"> | Model<"LocalWorkOutcome">;
export interface WorkTransport {
  readonly reactions?: import("./thread-types.js").ThreadTransport | undefined;
  refresh?(): Promise<void>;
  page(query: WorkQuery, signal?: AbortSignal): Promise<WorkPage>;
  apply(request: WorkRequest): Promise<WorkOutcome>;
  status(id: string): Promise<WorkStatus>;
}
export interface WorkState {
  focusCommentId?: string | null;
  drafts?: Record<string, WorkDraft>;
  projects: Model<"ProjectV1">[];
  project: Model<"ProjectV1"> | null;
  tasks: Model<"TaskV1">[];
  task: Model<"TaskV1"> | null;
  comments: Model<"WorkCommentV1">[];
  snapshot: WorkSnapshot | null;
  projectCursor: string | null;
  taskCursor: string | null;
  commentCursor: string | null;
  loading: boolean;
  error: string;
  message: string;
  pending: WorkRequest | null;
  writing: boolean;
}
export interface WorkController {
  readonly reactions?:
    import("./thread-types.js").ReactionsController | undefined;
  readonly state: WorkState;
  restore(refresh?: boolean): Promise<void>;
  refresh(): Promise<void>;
  invalidate?(): void;
  reset(): void;
  select(project: string | null, task?: string | null): Promise<void>;
  more(kind: "projects" | "tasks" | "comments"): Promise<void>;
  submit(command: WorkCommand): Promise<void>;
  check(): Promise<void>;
  retry(): Promise<void>;
}
export function workId(): string {
  return [...crypto.getRandomValues(new Uint8Array(16))]
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}
export const emptyWorkState = (): WorkState => ({
  drafts: undefined,
  focusCommentId: null,
  projects: [],
  project: null,
  tasks: [],
  task: null,
  comments: [],
  snapshot: null,
  projectCursor: null,
  taskCursor: null,
  commentCursor: null,
  loading: false,
  error: "",
  message: "",
  pending: null,
  writing: false,
});

export interface WorkDraft {
  title: string;
  body: string;
  repository: string;
  parent: string;
  status: "open" | "in_progress" | "blocked" | "done";
  reply: string | null;
  edit: string | null;
  editContext?:
    | {
        body: string;
        commentVersion: number | bigint;
        taskVersion: number | bigint;
      }
    | undefined;
}
