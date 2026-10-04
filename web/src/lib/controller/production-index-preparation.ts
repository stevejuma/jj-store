import { snapshotOperation } from "./repository-profile.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

type Task = Awaited<ReturnType<StoreDataSource["prepareIndex"]>>;
type Active = {
  abort: AbortController;
  task: Task | null;
  cancel: boolean;
  wake: () => void;
};

/** One exact task, one request at a time. Cancel is acknowledged by the server. */
export function createIndexPreparation(options: {
  data: ProductionStoreState;
  source(): StoreDataSource;
  operation(): string | null;
  version(): number;
  signal?(): AbortSignal;
  current(ticket: number, source: StoreDataSource): boolean;
  restore(): Promise<void>;
}) {
  const { data } = options;
  let active: Active | null = null;
  function clear() {
    const old = active;
    active = null;
    old?.abort.abort();
    old?.wake();
    data.indexPreparation = "idle";
    data.indexTaskStatus = null;
    data.indexPreparationError = "";
  }
  async function cancelIndex() {
    if (!active || active.cancel || active.task?.status !== "running") return;
    active.cancel = true;
    data.indexTaskStatus = "canceling";
    active.wake();
  }
  async function prepareIndex() {
    const operation =
      (data.neededIndex ? snapshotOperation(data.neededIndex) : null) ??
      options.operation();
    if (active || !operation || !data.capabilities?.index_tasks) return;
    const source = options.source(),
      ticket = options.version();
    const run: Active = {
      abort: new AbortController(),
      task: null,
      cancel: false,
      wake: () => {},
    };
    active = run;
    const navigation = options.signal?.();
    const stop = () => {
      if (active === run) clear();
    };
    navigation?.addEventListener("abort", stop, { once: true });
    const current = () =>
      active === run &&
      !run.abort.signal.aborted &&
      options.current(ticket, source);
    const wait = (delay: number) =>
      new Promise<void>((resolve) => {
        const timer = setTimeout(done, delay);
        function done() {
          clearTimeout(timer);
          run.wake = () => {};
          resolve();
        }
        run.wake = done;
      });
    data.indexPreparation = "loading";
    data.indexTaskStatus = null;
    data.indexPreparationError = "";
    try {
      if (navigation?.aborted || !current()) return;
      run.task = await source.prepareIndex(operation, run.abort.signal);
      const taskId = run.task.id;
      let polls = 0;
      while (current()) {
        if (
          run.task.id !== taskId ||
          ("operation" in run.task.snapshot
            ? run.task.snapshot.operation
            : run.task.snapshot.operation_id) !== operation
        )
          throw new Error(
            "Graph preparation returned a different task or operation.",
          );
        data.indexTaskStatus =
          run.cancel && run.task.status === "running"
            ? "canceling"
            : run.task.status;
        if (!["running", "canceling"].includes(run.task.status)) break;
        if (!run.cancel)
          await wait(Math.min(5000, 500 * 2 ** Math.min(polls, 4)));
        if (!current()) return;
        if (run.cancel) {
          try {
            run.task = await source.cancelIndexTask(taskId, run.abort.signal);
            if (current()) data.indexPreparationError = "";
          } catch (cause) {
            if (!current()) return;
            data.indexPreparationError = `Cancellation is unconfirmed: ${storeErrorMessage(cause)}`;
          } finally {
            run.cancel = false;
          }
          // A failed cancel remains observable and may be retried explicitly.
        } else {
          run.task = await source.indexTask(taskId, run.abort.signal);
          polls++;
        }
      }
      if (!current()) return;
      if (run.task.status === "canceled") {
        data.indexPreparation = "idle";
        data.indexPreparationError = "";
      } else if (run.task.status !== "ready") {
        data.indexPreparation = "failed";
        data.indexPreparationError = "Graph index preparation failed.";
      } else {
        data.neededIndex = null;
        data.status.error = "";
        data.indexPreparation = "ready";
        data.indexPreparationError = "";
        // Detach before restore invalidates its read scope.
        active = null;
        navigation?.removeEventListener("abort", stop);
        await options.restore();
      }
    } catch (cause) {
      if (!current()) return;
      data.indexPreparation = "failed";
      data.indexPreparationError = storeErrorMessage(cause);
    } finally {
      navigation?.removeEventListener("abort", stop);
      if (active === run) {
        if (!current()) clear();
        else active = null;
      }
    }
  }
  return { clear, prepareIndex, cancelIndex };
}
