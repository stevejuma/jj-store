import { pretty } from "../api/client.js";
import { snapshotOperation, sameCodeSnapshot } from "./repository-profile.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import type {
  StoreDataSource,
  IndexPreparationKind,
  KindPreparationTask,
} from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

type Task =
  Awaited<ReturnType<StoreDataSource["prepareIndex"]>> | KindPreparationTask;
type Active = {
  abort: AbortController;
  task: Task | null;
  cancel: boolean;
  selection: KindPreparationTask["snapshot"] | null;
  coreSelection: string | null;
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
  freshness?: import("./application-freshness.js").ApplicationFreshness;
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
    data.indexPreparationKind = "graph";
    data.indexProgress = null;
    data.indexCancellationUnconfirmed = false;
    data.indexPreparationError = "";
  }
  async function cancelIndex() {
    if (!active || active.cancel || active.task?.status !== "running") return;
    active.cancel = true;
    data.indexTaskStatus = "canceling";
    active.wake();
  }
  const prepareIndex = () => prepare("graph");
  const prepareKindIndex = (kind: "operations" | "copies") => prepare(kind);
  const prepareCodeIndex = () => prepare("graph", true);
  async function prepare(kind: IndexPreparationKind, core = false) {
    const operation =
      (data.neededIndex ? snapshotOperation(data.neededIndex) : null) ??
      options.operation();
    if (active || !operation || !data.capabilities?.index_tasks) return;
    const source = options.source(),
      ticket = options.version();
    if (
      kind !== "graph" &&
      (source.profile !== "native" ||
        !source.prepareKindIndex ||
        !source.kindIndexTask ||
        !source.cancelKindIndexTask)
    )
      return;
    const run: Active = {
      abort: new AbortController(),
      task: null,
      cancel: false,
      selection: null,
      coreSelection: null,
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
    let observed = options.freshness?.journal.revision ?? 0;
    const wait = (delay: number) => {
      if (options.freshness) {
        const waiting = new AbortController(),
          freshness = options.freshness;
        run.wake = () => waiting.abort();
        return freshness.journal
          .wait({
            after: observed,
            matches: (change) =>
              change.repository === source.repository &&
              change.domain === "index_tasks" &&
              change.identity === run.task?.id,
            // Kind windows emit no SSE hint; bounded status observations carry progress.
            healthy: () => kind === "graph" && freshness.healthy,
            listen: (listener) => freshness.listen(listener),
            signal: AbortSignal.any([run.abort.signal, waiting.signal]),
            delay,
          })
          .finally(() => {
            run.wake = () => {};
          });
      }
      return new Promise<void>((resolve) => {
        const timer = setTimeout(done, delay);
        function done() {
          clearTimeout(timer);
          run.wake = () => {};
          resolve();
        }
        run.wake = done;
      });
    };
    data.indexPreparationKind = kind;
    data.indexProgress = null;
    data.indexCancellationUnconfirmed = false;
    data.indexPreparation = "loading";
    data.indexTaskStatus = null;
    data.indexPreparationError = "";
    try {
      if (navigation?.aborted || !current()) return;
      run.task =
        kind === "graph"
          ? await (core ? source.prepareCodeIndex! : source.prepareIndex).call(
              source,
              operation,
              run.abort.signal,
            )
          : await source.prepareKindIndex!(operation, kind, run.abort.signal);
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
        if (core) {
          const snapshot = run.task.snapshot;
          if (
            run.task.version !== 1 ||
            ("repository" in snapshot
              ? snapshot.repository
              : snapshot.repo_id) !== source.repository ||
            (run.coreSelection !== null &&
              run.coreSelection !== pretty(snapshot))
          )
            throw new Error(
              "History preparation returned a different selection.",
            );
          run.coreSelection ??= pretty(snapshot);
        }
        if (kind !== "graph") {
          if (
            !("kind" in run.task) ||
            run.task.kind !== kind ||
            run.task.version !== 1 ||
            run.task.snapshot.repo_id !== source.repository ||
            (run.selection &&
              !sameCodeSnapshot(run.selection, run.task.snapshot))
          )
            throw new Error(
              "Inventory preparation returned a different selection.",
            );
          run.selection ??= run.task.snapshot;
          const progress = run.task.progress;
          const counters = [
            progress.matching_ids,
            progress.stats.windows,
            progress.stats.scanned,
            progress.stats.descriptor_work,
            progress.stats.record_reads,
            progress.stats.record_bytes,
          ];
          if (
            typeof progress.complete !== "boolean" ||
            counters.some(
              (count) => !Number.isSafeInteger(count) || count < 0,
            ) ||
            progress.matching_ids > progress.stats.scanned ||
            progress.stats.scanned > progress.stats.descriptor_work ||
            (run.task.status === "ready" && !progress.complete) ||
            (data.indexProgress &&
              (progress.matching_ids < data.indexProgress.matching_ids ||
                Object.entries(progress.stats).some(
                  ([key, count]) =>
                    count <
                    data.indexProgress!.stats[
                      key as keyof typeof progress.stats
                    ],
                )))
          )
            throw new Error("Inventory preparation returned invalid progress.");
          data.indexProgress = progress;
        }
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
            run.task =
              kind === "graph"
                ? await source.cancelIndexTask(taskId, run.abort.signal)
                : await source.cancelKindIndexTask!(taskId, run.abort.signal);
            if (current()) {
              data.indexPreparationError = "";
              data.indexCancellationUnconfirmed = false;
            }
          } catch (cause) {
            if (!current()) return;
            data.indexPreparationError = `Cancellation is unconfirmed: ${storeErrorMessage(cause)}`;
            data.indexTaskStatus = run.task.status;
            data.indexCancellationUnconfirmed = true;
          } finally {
            run.cancel = false;
          }
          // A failed cancel remains observable and may be retried explicitly.
        } else {
          observed = options.freshness?.journal.revision ?? observed;
          run.task =
            kind === "graph"
              ? await source.indexTask(taskId, run.abort.signal)
              : await source.kindIndexTask!(taskId, run.abort.signal);
          polls++;
        }
      }
      if (!current()) return;
      data.indexCancellationUnconfirmed = false;
      if (run.task.status === "canceled") {
        data.indexPreparation = "idle";
        data.indexPreparationError = "";
      } else if (run.task.status !== "ready") {
        data.indexPreparation = "failed";
        data.indexPreparationError =
          kind === "graph"
            ? "Graph index preparation failed."
            : "Inventory preparation failed.";
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
      data.indexCancellationUnconfirmed = false;
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
  return {
    clear,
    prepareIndex,
    prepareCodeIndex,
    prepareKindIndex,
    cancelIndex,
  };
}
