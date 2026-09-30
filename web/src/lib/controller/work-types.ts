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
export interface WorkTransport {
  page(query: WorkQuery): Promise<Model<"WorkPage">>;
  apply(request: WorkRequest): Promise<Model<"WorkCommandOutcome">>;
  status(id: string): Promise<Model<"WorkCommandStatus">>;
}
export interface WorkState {
  projects: Model<"ProjectV1">[];
  project: Model<"ProjectV1"> | null;
  tasks: Model<"TaskV1">[];
  task: Model<"TaskV1"> | null;
  comments: Model<"WorkCommentV1">[];
  snapshot: Model<"WorkReadSnapshot"> | null;
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
  readonly state: WorkState;
  restore(): Promise<void>;
  refresh(): Promise<void>;
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
