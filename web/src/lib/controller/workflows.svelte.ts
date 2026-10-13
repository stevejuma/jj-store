import { workId } from "./work-types.js";
import { ApiError } from "../api/client.js";
import type {
  WorkflowTransport,
  WorkflowCommand,
  WorkflowRequest,
  WorkflowPage,
} from "./workflow-types.js";
export function createWorkflows(transport: WorkflowTransport) {
  const state = $state({
    page: null as WorkflowPage | null,
    pending: null as WorkflowRequest | null,
    loading: false,
    writing: false,
    error: "",
    readError: "",
    message: "",
  });
  let generation = 0;
  async function refresh() {
    const ticket = ++generation;
    state.loading = true;
    try {
      let page = await transport.page({ limit: 256 });
      const target = state.page?.workflows.length ?? 0;
      let workflows = page.workflows;
      while (page.next && workflows.length < target && ticket === generation) {
        const next = await transport.page({
          at: page.position,
          after: page.next,
          limit: 256,
        });
        if (
          next.epoch !== page.epoch ||
          next.root !== page.root ||
          next.position !== page.position ||
          next.repository !== page.repository
        )
          throw Error("Workflow catalog snapshot changed.");
        workflows = [...workflows, ...next.workflows];
        page = next;
      }
      page = { ...page, workflows };
      if (ticket === generation) {
        state.page = page;
        state.readError = "";
      }
    } catch (error) {
      if (ticket === generation)
        state.readError =
          error instanceof Error ? error.message : "Workflows unavailable";
    } finally {
      if (ticket === generation) state.loading = false;
    }
  }
  async function retry() {
    if (!state.pending || state.writing) return;
    state.writing = true;
    try {
      const status = await transport.status(state.pending.request_id);
      const result =
        status.status === "committed"
          ? status
          : status.status === "not_known"
            ? await transport.apply(state.pending)
            : status;
      if (result.status === "committed") {
        state.error = "";
        state.pending = null;
        state.message = "Workflow command saved";
        await refresh();
      } else
        state.message =
          "The outcome is still unknown. Check the original request before continuing.";
    } catch (error) {
      state.error =
        error instanceof Error ? error.message : "Workflow command unavailable";
      // This response is emitted before WorkflowStore publication. Other
      // failures may follow a committed write, so their identity stays pending.
      if (error instanceof ApiError && error.code === "invalid_request") {
        state.pending = null;
        state.message = "";
      }
    } finally {
      state.writing = false;
    }
  }
  return {
    state,
    transport,
    refresh,
    async more() {
      if (!state.page?.next || state.loading) return;
      const ticket = generation,
        previous = state.page;
      state.loading = true;
      try {
        const next = await transport.page({
          at: previous.position,
          after: previous.next!,
          limit: 256,
        });
        if (ticket !== generation) return;
        if (
          next.epoch !== previous.epoch ||
          next.root !== previous.root ||
          next.position !== previous.position ||
          next.repository !== previous.repository
        )
          throw Error("Workflow catalog snapshot changed.");
        state.page = {
          ...next,
          workflows: [...previous.workflows, ...next.workflows],
        };
        state.readError = "";
      } catch (cause) {
        if (ticket === generation)
          state.readError =
            cause instanceof Error ? cause.message : "Workflows unavailable";
      } finally {
        if (ticket === generation) state.loading = false;
      }
    },
    retry,
    async submit(command: WorkflowCommand) {
      if (state.pending) return false;
      state.error = "";
      state.message = "";
      state.pending = { version: 1, request_id: workId(), command };
      await retry();
      return state.pending === null && !state.error;
    },
    stop() {
      generation++;
    },
  };
}
export type WorkflowController = ReturnType<typeof createWorkflows>;
