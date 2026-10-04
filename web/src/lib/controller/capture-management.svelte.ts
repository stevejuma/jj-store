import { captureConfigurationObserver } from "./capture-configuration-observer.js";
import { createCaptureSettings } from "./capture-settings-management.js";
import { ApiError } from "../api/client.js";
import {
  emptyCaptureManagement,
  captureHistorySupported,
  type CaptureIntent,
  type CaptureManagement,
  type CaptureTransport,
} from "./capture-types.js";
import {
  captureId,
  requireCapture,
  sameImport,
  validateCaptureSources,
  validateCaptureStatus,
  validateControlOutcome,
  validateImportOutcome,
  validateImportStatus,
  validateRescan,
} from "./capture-validation.js";

import { createCaptureObservations } from "./capture-observations.js";
import { createCaptureRefresh } from "./capture-refresh.js";
import { captureError, captureRefusals } from "./capture-errors.js";

/** Owned by the installation, never reset by repository or settings navigation. */
export function createCaptureManagement(options: {
  transport: CaptureTransport;
  available: () => boolean;
  requestId?: () => string;
  freshness?: import("./application-freshness.js").ApplicationFreshness;
}): CaptureManagement {
  const state = $state(emptyCaptureManagement());
  let alive = true;
  const available = () => alive && options.available();
  const transport = options.transport;
  const cursors = new Set<string>();
  const configuration = createCaptureSettings({ state, transport, available });
  const configurationObservations = captureConfigurationObserver(
    state,
    configuration.observe,
    options.freshness,
  );
  const ready = () =>
    !state.settings.saving &&
    !state.settings.value?.restart_required &&
    available() &&
    !state.busy &&
    state.status === "ready" &&
    state.value?.enabled &&
    state.value.policy !== null;
  async function run(
    work: () => Promise<void>,
    failed: (cause: unknown) => void,
  ) {
    if (!available() || state.busy || state.settings.saving) return;
    state.busy = true;
    state.commandBusy = true;
    state.error = "";
    state.message = "";
    try {
      await work();
    } catch (cause) {
      if (available()) failed(cause);
    } finally {
      if (alive) {
        state.commandBusy = false;
        state.busy = false;
        automatic.request();
      }
    }
  }
  function newId() {
    const id = options.requestId?.() ?? crypto.randomUUID().replaceAll("-", "");
    requireCapture(captureId(id));
    return id;
  }
  async function sendPending() {
    const intent = state.pending;
    if (!intent || state.settings.value?.restart_required) return;
    await run(
      async () => {
        if (intent.kind === "control") {
          const outcome = await transport.control(intent.command);
          validateControlOutcome(outcome, intent.command);
          if (!available()) return;
          state.controlOutcome = outcome;
          state.value = outcome.status;
          state.status = "ready";
          state.message = `Policy accepted. ${outcome.runtime_requested ? "Worker change requested; status updates automatically." : "No worker change requested."}`;
        } else {
          const outcome = await transport.import(intent.command);
          validateImportOutcome(outcome, intent.command);
          if (!available()) return;
          state.importStatus = {
            version: 1,
            request_id: intent.command.request_id,
            receipt: outcome.receipt,
            progress: outcome.progress,
          };
          state.message =
            "History import accepted. Publication progress updates automatically.";
        }
        state.pending = null;
      },
      (cause) => {
        if (
          !intent.uncertain &&
          cause instanceof ApiError &&
          captureRefusals.has(cause.code)
        ) {
          state.pending = null;
          state.status = "failed";
          state.error = captureError(cause);
        } else {
          intent.uncertain = true;
          state.error =
            "Outcome unknown. Keep this request ID and retry the original request or inspect its receipt.";
        }
      },
    );
  }
  const observations = createCaptureObservations({
    state,
    transport,
    available,
    cursors,
    observeSettings: configurationObservations.observe,
  });
  const automatic = createCaptureRefresh({
    available,
    ...observations,
    freshness: options.freshness,
  });
  async function refresh() {
    await observations.refresh();
  }
  async function moreSources() {
    const current = state.sources,
      cursor = current?.next_cursor;
    if (!cursor || state.sourceState !== "ready") return;
    await run(
      async () => {
        const page = await transport.sources(cursor);
        validateCaptureSources(page);
        requireCapture(
          page.runtime_generation === current.runtime_generation &&
            page.enabled === current.enabled &&
            !cursors.has(cursor),
        );
        const selectors = new Set(current.sources.map((s) => s.selector));
        requireCapture(
          page.sources.every((s) => !selectors.has(s.selector)) &&
            (!page.next_cursor ||
              (page.next_cursor !== cursor && !cursors.has(page.next_cursor))),
        );
        requireCapture(current.sources.length + page.sources.length <= 4096);
        if (!available()) return;
        cursors.add(cursor);
        state.sources = {
          ...page,
          sources: [...current.sources, ...page.sources],
        };
        state.sourcesObservedAt = Date.now();
      },
      () => {
        state.sourceState = "failed";
        state.error =
          "Source coverage changed or is unavailable. Refresh capture to select a new source snapshot.";
      },
    );
  }
  async function control(action: "pause" | "resume") {
    if (!ready() || state.pending) return;
    const policy = state.value!.policy!;
    state.pending = {
      kind: "control",
      uncertain: false,
      command: {
        version: 1,
        request_id: newId(),
        generation: policy.generation,
        expected_revision: policy.revision,
        action,
      },
    };
    await sendPending();
  }
  async function importHistory(selector: string) {
    if (
      !ready() ||
      state.pending ||
      state.value!.policy!.paused ||
      state.sourceState !== "ready"
    )
      return;
    const source = state.sources?.sources.find((s) => s.selector === selector);
    if (
      !source?.awaiting_activity ||
      source.running ||
      !captureHistorySupported(source.profile)
    )
      return;
    const policy = state.value!.policy!;
    state.pending = {
      kind: "import",
      uncertain: false,
      command: {
        version: 1,
        request_id: newId(),
        generation: policy.generation,
        expected_revision: policy.revision,
        selector,
      },
    };
    await sendPending();
  }
  async function rescan() {
    if (!ready() || state.pending || state.value!.policy!.paused) return;
    const policy = state.value!.policy!;
    await run(
      async () => {
        const outcome = await transport.rescan({
          version: 1,
          generation: policy.generation,
          expected_revision: policy.revision,
        });
        validateRescan(outcome);
        if (!available()) return;
        state.value = outcome.status;
        state.sourceState = "idle";
        state.message = outcome.accepted
          ? "Rescan requested. Discovery updates automatically."
          : "Rescan was not accepted. Discovery updates automatically.";
      },
      (cause) => {
        state.status = "failed";
        state.error = `${captureError(cause)} Rescan completion is unconfirmed.`;
      },
    );
  }
  async function inspectImport(id: string) {
    if (!captureId(id)) return;
    await run(
      async () => {
        const result = await transport.inspect(id);
        validateImportStatus(result, id);
        const intent = state.pending;
        if (
          intent?.kind === "import" &&
          intent.command.request_id === id &&
          result.receipt
        )
          requireCapture(sameImport(result.receipt.command, intent.command));
        if (!available()) return;
        state.importStatus = result;
        if (
          intent?.kind === "import" &&
          intent.command.request_id === id &&
          result.receipt
        )
          state.pending = null;
      },
      (cause) => {
        state.error = `${captureError(cause)} Import receipt remains unconfirmed.`;
      },
    );
  }
  return {
    get supported() {
      return available();
    },
    state,
    configure: configuration.configure,
    cancelConfiguration: configuration.cancelConfiguration,
    saveConfiguration: configuration.saveConfiguration,
    refresh,
    setActive: automatic.setActive,
    moreSources,
    control,
    rescan,
    importHistory,
    inspectImport,
    retry: sendPending,
    destroy() {
      configurationObservations.close?.();
      automatic.setActive(false);
      alive = false;
      state.busy = false;
      transport.close();
    },
  };
}
