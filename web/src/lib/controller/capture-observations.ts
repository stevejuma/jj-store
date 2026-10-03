import type {
  CaptureManagementState,
  CaptureTransport,
} from "./capture-types.js";
import {
  requireCapture,
  sameImport,
  validateCaptureStatus,
  validateCaptureSources,
  validateImportStatus,
} from "./capture-validation.js";
import { captureError } from "./capture-errors.js";

/** Ordinary observations have their own cancellation; commands retain installation ownership. */
export function createCaptureObservations(options: {
  state: CaptureManagementState;
  transport: CaptureTransport;
  available(): boolean;
  cursors: Set<string>;
}) {
  const { state, transport, cursors } = options;
  let controller: AbortController | null = null;
  let reading: Promise<boolean> | null = null;
  const current = (read: AbortController) =>
    controller === read && !read.signal.aborted && options.available();
  async function observe(read: AbortController) {
    state.busy = true;
    if (!state.value) state.status = "loading";
    try {
      const value = await transport.status(read.signal);
      validateCaptureStatus(value);
      if (!current(read)) return false;
      state.value = value;
      state.status = "ready";
      const retain = Math.max(1, state.sources?.sources.length ?? 0);
      try {
        let page = await transport.sources(undefined, read.signal);
        validateCaptureSources(page);
        requireCapture(page.enabled === value.enabled);
        const visited = new Set<string>();
        const selectors = new Set(page.sources.map((s) => s.selector));
        while (page.next_cursor && page.sources.length < retain) {
          const cursor = page.next_cursor;
          requireCapture(!visited.has(cursor));
          visited.add(cursor);
          const next = await transport.sources(cursor, read.signal);
          validateCaptureSources(next);
          requireCapture(
            next.runtime_generation === page.runtime_generation &&
              next.enabled === page.enabled &&
              next.sources.every((s) => !selectors.has(s.selector)),
          );
          for (const source of next.sources) selectors.add(source.selector);
          requireCapture(page.sources.length + next.sources.length <= 4096);
          page = { ...next, sources: [...page.sources, ...next.sources] };
        }
        requireCapture(!page.next_cursor || !visited.has(page.next_cursor));
        if (!current(read)) return false;
        state.sources = page;
        state.sourceState = "ready";
        cursors.clear();
        for (const cursor of visited) cursors.add(cursor);
      } catch (cause) {
        if (!current(read)) return false;
        state.sourceState = "failed";
        if (!state.pending)
          state.error = `${captureError(cause)} Source coverage could not be refreshed.`;
      }
      const id =
        state.pending?.kind === "import"
          ? state.pending.command.request_id
          : state.importStatus?.request_id;
      if (id) {
        const result = await transport.inspect(id, read.signal);
        validateImportStatus(result, id);
        const pending = state.pending;
        if (pending?.kind === "import" && result.receipt)
          requireCapture(sameImport(result.receipt.command, pending.command));
        if (!current(read)) return false;
        const previous =
          state.importStatus?.request_id === id
            ? state.importStatus.receipt
            : null;
        if (previous && result.receipt)
          requireCapture(
            sameImport(previous.command, result.receipt.command) &&
              previous.complete_byte_offset ===
                result.receipt.complete_byte_offset &&
              previous.complete_records === result.receipt.complete_records,
          );
        state.importStatus = { ...result, receipt: result.receipt ?? previous };
        if (result.receipt && pending?.kind === "import") state.error = "";
        if (pending?.kind === "import" && result.receipt) state.pending = null;
      }
      return state.sourceState === "ready";
    } catch (cause) {
      if (!current(read)) return false;
      state.status = "failed";
      if (!state.pending) state.error = captureError(cause);
      return false;
    } finally {
      if (controller === read) {
        controller = null;
        reading = null;
        state.busy = false;
      }
    }
  }
  return {
    refresh() {
      if (reading) return reading;
      if (!options.available() || state.busy) return Promise.resolve(false);
      controller = new AbortController();
      reading = observe(controller);
      return reading;
    },
    cancel() {
      if (!controller) return;
      controller.abort();
      controller = null;
      reading = null;
      state.busy = false;
      if (state.status === "loading")
        state.status = state.value ? "ready" : "idle";
    },
  };
}
