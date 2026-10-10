import { ApiError, decode } from "./client.js";
import {
  applicationEvents,
  validateApplicationEvent,
  type ApplicationEvent,
} from "./application-events.js";

/** Injected HTTP fixtures retain their seam; all browser/worker paths use WebSocket. */
export function applicationEventTransport(
  token: string | null,
  after: string | undefined,
  signal: AbortSignal,
  deliver: (event: ApplicationEvent, id: string) => void,
  fetcher?: typeof fetch,
) {
  return fetcher
    ? applicationEvents(token, after, signal, deliver, fetcher)
    : websocketApplicationEvents(token, after, signal, deliver);
}

export function websocketApplicationEvents(
  token: string | null,
  after: string | undefined,
  signal: AbortSignal,
  deliver: (event: ApplicationEvent, id: string) => void,
): Promise<void> {
  if (signal.aborted) return Promise.reject(signal.reason);
  return new Promise((resolve, reject) => {
    const url = new URL("/v1/events/ws", location.href);
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    const socket = new WebSocket(url, "jj-store-events-v1");
    let finished = false,
      received = false;
    let timer = setTimeout(
      () => finish(new ApiError("event_unavailable")),
      35000,
    );
    const finish = (cause?: unknown) => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      signal.removeEventListener("abort", abort);
      socket.onopen = socket.onmessage = socket.onerror = socket.onclose = null;
      socket.close();
      if (cause) reject(cause);
      else resolve();
    };
    const abort = () => finish(signal.reason);
    signal.addEventListener("abort", abort, { once: true });
    socket.onopen = () => {
      if (socket.protocol !== "jj-store-events-v1" || socket.url !== url.href)
        return finish(new ApiError("invalid_application_event"));
      // Never put credentials into URLs, subprotocols, storage or diagnostics.
      socket.send(JSON.stringify({ version: 1, token, after }));
    };
    socket.onmessage = ({ data }) => {
      try {
        if (typeof data !== "string" || data.length > 1024 * 1024)
          throw new ApiError("event_frame_limit");
        const frame = decode(data) as {
          version?: number;
          type?: string;
          event: ApplicationEvent;
          id: string;
        };
        if (frame?.type === "heartbeat" && frame.version === 1 && received) {
          // Heartbeats renew liveness without invalidating cached data.
        } else {
          validateApplicationEvent(frame?.event, frame?.id);
          deliver(frame.event, frame.id);
          received = true;
        }
        clearTimeout(timer);
        timer = setTimeout(
          () => finish(new ApiError("event_unavailable")),
          45000,
        );
      } catch (cause) {
        finish(cause);
      }
    };
    socket.onerror = () => finish(new ApiError("event_unavailable"));
    socket.onclose = ({ code }) =>
      finish(
        code === 1000 && received
          ? undefined
          : new ApiError(
              code === 4403
                ? "access_denied"
                : code === 4400
                  ? "invalid_application_event"
                  : "event_unavailable",
            ),
      );
    if (signal.aborted) abort();
  });
}
