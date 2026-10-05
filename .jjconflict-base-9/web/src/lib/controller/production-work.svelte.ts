import {
  acceptWorkPage,
  cacheWorkView,
  reloadWorkPages,
} from "./work-refresh-pages.js";
import { loadWorkCommentFocus } from "./work-comment-focus.js";
import { ApiError, pretty } from "../api/client.js";
import {
  emptyWorkState,
  workId,
  type WorkController,
  type WorkQuery,
  type WorkTransport,
} from "./work-types.js";
import type { Model } from "../api/client.js";
import { workReactionSession } from "./work-reactions.svelte.js";
export function createWorkController(context: {
  repository?(): string;
  principal?(): string;
  source(): WorkTransport;
  route(): {
    projectId?: string | null;
    taskId?: string | null;
    commentId?: string | null;
  };
  select(project: string | null, task: string | null): void;
  changed(): void;
}): WorkController {
  const state = $state(emptyWorkState());
  const reactions = workReactionSession(context);
  let version = 0,
    session = 0;
  let reads = new AbortController();
  let stale = false;
  const cached = new Map<string, ReturnType<typeof cacheWorkView>>();
  const key = (project?: string | null, task?: string | null) =>
    `${project ?? ""}:${task ?? ""}`;
  const pins = new Map<string, string>();
  const message = (cause: unknown) =>
    cause instanceof Error ? cause.message : "Request failed";
  async function read(
    query: WorkQuery,
    kind: "projects" | "tasks" | "comments",
    more = false,
  ) {
    const source = context.source(),
      ticket = version;
    const result = await source.page({ ...query, limit: 50 }, reads.signal);
    if (ticket !== version || source !== context.source()) return;
    acceptWorkPage(state, pins, result, kind, more);
  }
  async function restore(refresh = true) {
    refresh ||= stale;
    const selection = context.route();
    const sameSelection =
      selection.projectId === state.project?.id &&
      (selection.taskId ?? null) === (state.task?.id ?? null);
    const counts = {
      projects: state.projects.length,
      tasks: sameSelection ? state.tasks.length : 0,
      comments: sameSelection ? state.comments.length : 0,
    };
    if (
      !stale &&
      !refresh &&
      state.snapshot &&
      !state.error &&
      !state.loading
    ) {
      cached.set(
        key(state.project?.id, state.task?.id),
        cacheWorkView(state, pins),
      );
      if (cached.size > 16) cached.delete(cached.keys().next().value!);
    }
    reads.abort();
    reads = new AbortController();
    const ticket = ++version;
    state.loading = true;
    state.error = "";
    state.focusCommentId = selection.commentId ?? null;
    const saved =
      !refresh &&
      !selection.commentId &&
      cached.get(key(selection.projectId, selection.taskId));
    if (saved) {
      const { savedPins, ...view } = saved;
      pins.clear();
      for (const [kind, pin] of savedPins) pins.set(kind, pin);
      Object.assign(state, view, { loading: false });
      return;
    }
    if (
      selection.projectId !== state.project?.id ||
      (selection.taskId ?? null) !== (state.task?.id ?? null)
    ) {
      state.project = null;
      state.task = null;
      state.tasks = [];
      state.comments = [];
    }
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
      if (refresh)
        await reloadWorkPages(
          state,
          route,
          counts,
          read,
          () => ticket === version,
        );
      await loadWorkCommentFocus(state, route, read, () => ticket === version);
      if (ticket === version) stale = false;
    } catch (cause) {
      if (ticket === version) {
        if (
          cause instanceof ApiError &&
          ["access_denied", "bearer_required"].includes(cause.code)
        ) {
          reset();
        }
        state.error = message(cause);
      }
    } finally {
      if (ticket === version) state.loading = false;
    }
  }
  function reset() {
    reactions.clear();
    version++;
    session++;
    reads.abort();
    pins.clear();
    cached.clear();
    stale = false;
    Object.assign(state, emptyWorkState());
  }
  function invalidate() {
    reactions.invalidate();
    cached.clear();
    stale = true;
  }
  async function refresh() {
    invalidate();
    const source = context.source(),
      ticket = session;
    await source.refresh?.();
    if (ticket !== session || source !== context.source()) return;
    await restore(true);
  }
  async function committed() {
    state.pending = null;
    state.message = "";
    try {
      await refresh();
    } catch (cause) {
      state.error = message(cause);
    }
    context.changed();
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
        await committed();
      } else
        state.message = "Outcome unknown. Check status or retry this request.";
    } catch (cause) {
      if (ticket !== session) return;
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      ) {
        reset();
        state.error = message(cause);
        return;
      }
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
    get reactions() {
      return reactions.current;
    },
    state,
    restore,
    refresh,
    invalidate,
    reset,
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
        source = context.source(),
        ticket = session;
      if (!pending || state.writing) return;
      state.writing = true;
      state.error = "";
      try {
        const result = await source.status(pending.request_id);
        if (ticket !== session || source !== context.source()) return;
        if (result.status === "committed") {
          await committed();
        } else
          state.message =
            "No committed receipt yet. Retain this request and retry.";
      } catch (cause) {
        if (ticket === session) {
          if (
            cause instanceof ApiError &&
            ["access_denied", "bearer_required"].includes(cause.code)
          )
            reset();
          state.error = message(cause);
        }
      } finally {
        if (ticket === session) state.writing = false;
      }
    },
  };
}
