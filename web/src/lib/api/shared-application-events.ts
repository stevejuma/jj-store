import { applicationEventTransport } from "./application-events-websocket.js";
import { ApiError } from "./api-error.js";
import { type ApplicationEvent } from "./application-events.js";

/** Share one WebSocket across app tabs with the same credential. */
export async function sharedApplicationEvents(
  token: string | null,
  after: string | undefined,
  signal: AbortSignal,
  deliver: (event: ApplicationEvent, id: string) => void,
  fetcher?: typeof fetch,
) {
  // Injected transports and browsers without SharedWorker retain the direct path.
  if (
    fetcher ||
    typeof SharedWorker === "undefined" ||
    typeof window === "undefined"
  )
    return applicationEventTransport(token, after, signal, deliver, fetcher);
  if (signal.aborted) return Promise.reject(signal.reason);
  let worker: SharedWorker;
  try {
    // The URL identifies the worker. A fixed name would reject a new asset URL
    // while an older deployment is open, forcing each new tab into a direct socket.
    worker = new SharedWorker(
      new URL("./application-events.worker.ts", import.meta.url),
      { type: "module" },
    );
  } catch {
    return applicationEventTransport(token, after, signal, deliver);
  }
  return new Promise<void>((resolve, reject) => {
    let finished = false;
    const timer = setTimeout(
      () => finish(new ApiError("event_unavailable")),
      35000,
    );
    const finish = (cause?: unknown) => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      signal.removeEventListener("abort", abort);
      window.removeEventListener("pagehide", leave);
      worker.port.postMessage({ type: "unsubscribe" });
      worker.port.close();
      worker.onerror = null;
      if (cause) reject(cause);
      else resolve();
    };
    const abort = () => finish(signal.reason);
    const leave = () => finish();
    signal.addEventListener("abort", abort, { once: true });
    window.addEventListener("pagehide", leave, { once: true });
    worker.onerror = () =>
      finish(new ApiError("shared_event_worker_unavailable"));
    worker.port.onmessageerror = () =>
      finish(new ApiError("invalid_application_event"));
    worker.port.onmessage = ({ data }) => {
      if (finished) return;
      try {
        if (data.type === "event") {
          clearTimeout(timer);
          deliver(data.value, data.id);
        } else if (data.type === "end")
          finish(data.error ? new ApiError(data.error) : undefined);
        else finish(new ApiError("invalid_application_event"));
      } catch (cause) {
        finish(cause);
      }
    };
    worker.port.start();
    worker.port.postMessage({ type: "subscribe", token, after });
    if (signal.aborted) abort();
  }).catch((cause) => {
    if (
      cause instanceof ApiError &&
      cause.code === "shared_event_worker_unavailable" &&
      !signal.aborted
    )
      return applicationEventTransport(token, after, signal, deliver);
    throw cause;
  });
}
