import { ApiError, pretty } from "../api/client.js";
import {
  emptyWorkState,
  workId,
  type WorkController,
  type WorkQuery,
  type WorkTransport,
} from "./work-types.js";
import type { Model } from "../api/client.js";
export function createWorkController(context: {
  source(): WorkTransport;
  route(): { projectId?: string | null; taskId?: string | null };
  select(project: string | null, task: string | null): void;
  changed(): void;
}): WorkController {
  const state = $state(emptyWorkState());
  let version = 0,
    session = 0;
  const pins = new Map<string, string>();
  const message = (cause: unknown) =>
    cause instanceof Error ? cause.message : "Request failed";
  function accept(
    page: Model<"WorkPage">,
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
  async function read(
    query: WorkQuery,
    kind: "projects" | "tasks" | "comments",
    more = false,
  ) {
    const source = context.source(),
      ticket = version;
    const result = await source.page({ ...query, limit: 50 });
    if (ticket !== version || source !== context.source()) return;
    accept(result, kind, more);
  }
  async function restore() {
    const ticket = ++version;
    state.loading = true;
    state.error = "";
    state.project = null;
    state.task = null;
    state.tasks = [];
    state.comments = [];
    state.taskCursor = null;
    state.commentCursor = null;
    try {
      await read({}, "projects");
      if (ticket !== version) return;
      const route = context.route();
      if (route.projectId) await read({ project: route.projectId }, "tasks");
      if (ticket !== version) return;
      if (route.taskId)
        await read(
          { project: route.projectId ?? undefined, task: route.taskId },
          "comments",
        );
    } catch (cause) {
      if (ticket === version) state.error = message(cause);
    } finally {
      if (ticket === version) state.loading = false;
    }
  }
  async function send() {
    const request = state.pending;
    if (!request || state.writing) return;
    const source = context.source(),
      ticket = session;
    state.writing = true;
    state.error = "";
    state.message = "";
    try {
      const result = await source.apply(request);
      if (ticket !== session || source !== context.source()) return;
      if (result.status === "committed") {
        state.pending = null;
        state.message = "Saved. Refresh projects to see changes.";
        context.changed();
      } else
        state.message = "Outcome unknown. Check status or retry this request.";
    } catch (cause) {
      if (ticket !== session) return;
      // Only explicit admission rejections prove that this request did not publish.
      if (
        cause instanceof ApiError &&
        [
          "stale_work",
          "invalid_request",
          "intent_conflict",
          "not_found",
          "access_denied",
        ].includes(cause.code)
      ) {
        state.pending = null;
        state.error = message(cause);
      } else
        state.message = "Outcome unknown. Check status or retry this request.";
    } finally {
      if (ticket === session) state.writing = false;
    }
  }
  return {
    state,
    restore,
    refresh: restore,
    reset() {
      version++;
      session++;
      pins.clear();
      Object.assign(state, emptyWorkState());
    },
    async select(project, task = null) {
      context.select(project, task);
      await restore();
    },
    async more(kind) {
      if (state.loading) return;
      const cursor =
        kind === "projects"
          ? state.projectCursor
          : kind === "tasks"
            ? state.taskCursor
            : state.commentCursor;
      if (!cursor) return;
      const ticket = version;
      state.loading = true;
      try {
        await read(
          {
            cursor,
            ...(kind === "tasks"
              ? { project: state.project!.id }
              : kind === "comments"
                ? { task: state.task!.id, project: state.project!.id }
                : {}),
          },
          kind,
          true,
        );
      } catch (cause) {
        if (ticket === version) state.error = message(cause);
      } finally {
        if (ticket === version) state.loading = false;
      }
    },
    async submit(command) {
      if (state.pending || state.writing) return;
      state.pending = { version: 1, request_id: workId(), command };
      await send();
    },
    retry: send,
    async check() {
      const pending = state.pending,
        ticket = session;
      if (!pending || state.writing) return;
      state.writing = true;
      try {
        const result = await context.source().status(pending.request_id);
        if (ticket !== session) return;
        if (result.status === "committed") {
          state.pending = null;
          state.message = "Saved. Refresh projects to see changes.";
          context.changed();
        } else
          state.message =
            "No committed receipt yet. Retain this request and retry.";
      } catch (cause) {
        if (ticket === session) state.error = message(cause);
      } finally {
        if (ticket === session) state.writing = false;
      }
    },
  };
}
