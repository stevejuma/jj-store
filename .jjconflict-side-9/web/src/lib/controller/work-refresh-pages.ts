import { ApiError, pretty } from "../api/client.js";
import type { WorkQuery, WorkState } from "./work-types.js";
type Kind = "projects" | "tasks" | "comments";
export async function reloadWorkPages(
  state: WorkState,
  route: { projectId?: string | null; taskId?: string | null },
  counts: Record<Kind, number>,
  read: (query: WorkQuery, kind: Kind, more: boolean) => Promise<void>,
  current: () => boolean,
) {
  for (const kind of ["projects", "tasks", "comments"] as const) {
    const query =
      kind === "projects"
        ? {}
        : kind === "tasks"
          ? { project: route.projectId ?? undefined }
          : {
              project: route.projectId ?? undefined,
              task: route.taskId ?? undefined,
            };
    if (
      (kind === "tasks" && !route.projectId) ||
      (kind === "comments" && !route.taskId)
    )
      continue;
    const cursorKey =
      kind === "projects"
        ? "projectCursor"
        : kind === "tasks"
          ? "taskCursor"
          : "commentCursor";
    const seen = new Set<string>();
    while (
      state[cursorKey] &&
      state[kind].length < Math.min(counts[kind], 4096) &&
      current()
    ) {
      const cursor = state[cursorKey]!;
      if (seen.has(cursor)) throw new ApiError("repeated_cursor");
      seen.add(cursor);
      await read({ ...query, cursor }, kind, true);
    }
  }
}

export function cacheWorkView(
  state: WorkState,
  pins: ReadonlyMap<string, string>,
) {
  const {
    projects,
    project,
    tasks,
    task,
    comments,
    snapshot,
    projectCursor,
    taskCursor,
    commentCursor,
  } = state;
  return {
    savedPins: [...pins],
    projects,
    project,
    tasks,
    task,
    comments,
    snapshot,
    projectCursor,
    taskCursor,
    commentCursor,
  };
}

export function acceptWorkPage(
  state: WorkState,
  pins: Map<string, string>,
  page: import("./work-types.js").WorkPage,
  kind: "projects" | "tasks" | "comments",
  more: boolean,
) {
  const key = pretty(page.snapshot);
  if (more && pins.get(kind) !== key) throw new ApiError("snapshot_mismatch");
  pins.set(kind, key);
  state.snapshot = page.snapshot;
  if (kind === "projects") {
    state.projects = more
      ? [...state.projects, ...page.projects]
      : page.projects;
    state.projectCursor = page.next_cursor;
  }
  if (kind === "tasks") {
    state.project = page.projects[0] ?? null;
    state.tasks = more ? [...state.tasks, ...page.tasks] : page.tasks;
    state.taskCursor = page.next_cursor;
  }
  if (kind === "comments") {
    state.project = page.projects[0] ?? null;
    state.task = page.tasks[0] ?? null;
    state.comments = more
      ? [...state.comments, ...page.comments]
      : page.comments;
    state.commentCursor = page.next_cursor;
  }
}
