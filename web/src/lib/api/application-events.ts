import { ApiError, decode, type Model } from "./client.js";
export type ApplicationEvent = Model<"ApplicationFreshnessEvent">;
export type FreshnessChange = Extract<
  ApplicationEvent,
  { type: "changed" }
>["change"];
export type FreshnessDomain = FreshnessChange["domain"];
const domains: readonly string[] = [
  "repository",
  "code",
  "threads",
  "agents",
  "agent_execution",
  "agent_settings",
  "reviews",
  "work",
  "storage",
  "sync",
  "search",
  "index_tasks",
  "capture",
  "configuration",
  "discovery",
  "profiles",
  "emoji_library",
  "workspaces",
  "conversion",
];
const cursorPattern = /^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]{0,19})$/u;
export function validApplicationCursor(id: string) {
  return (
    cursorPattern.test(id) && BigInt(id.split(":")[3]!) <= 18446744073709551615n
  );
}
export function assertApplicationContinuity(
  previous: string | undefined,
  event: ApplicationEvent,
  id: string,
) {
  if (
    previous &&
    event.type !== "reset" &&
    (id.split(":").slice(0, 3).join(":") !==
      previous.split(":").slice(0, 3).join(":") ||
      (event.type === "changed"
        ? BigInt(id.split(":")[3]!) <= BigInt(previous.split(":")[3]!)
        : BigInt(id.split(":")[3]!) < BigInt(previous.split(":")[3]!)))
  )
    throw new ApiError("invalid_application_cursor");
}
function validateChange(value: FreshnessChange) {
  if (
    !value ||
    typeof value !== "object" ||
    !domains.includes(value.domain) ||
    !(
      value.repository === null ||
      (typeof value.repository === "string" &&
        /^[0-9a-f]{32}$/u.test(value.repository))
    ) ||
    typeof value.witness !== "string" ||
    !value.witness.length ||
    value.witness.length > 128 ||
    (value.identity != null &&
      (typeof value.identity !== "string" || value.identity.length > 128))
  )
    throw new ApiError("invalid_application_event");
}
export function validateApplicationEvent(event: ApplicationEvent, id: string) {
  if (
    !event ||
    event.version !== 1 ||
    !["ready", "changed", "reset"].includes(event.type) ||
    !validApplicationCursor(id)
  )
    throw new ApiError("invalid_application_event");
  if (event.type === "changed") validateChange(event.change);
  else {
    if (
      !Array.isArray(event.witnesses) ||
      event.witnesses.length > 4096 ||
      (event.type === "reset" && typeof event.reason !== "string")
    )
      throw new ApiError("invalid_application_event");
    event.witnesses.forEach(validateChange);
  }
}
export class ApplicationEventParser {
  private buffer = "";
  constructor(private deliver: (event: ApplicationEvent, id: string) => void) {}
  push(text: string) {
    this.buffer += text;
    if (this.buffer.length > 1024 * 1024)
      throw new ApiError("event_frame_limit");
    while (true) {
      const end = /\r?\n\r?\n/u.exec(this.buffer);
      if (!end) return;
      const frame = this.buffer.slice(0, end.index);
      this.buffer = this.buffer.slice(end.index + end[0].length);
      let name = "",
        id = "";
      const lines: string[] = [];
      for (const line of frame.split(/\r?\n/u)) {
        if (line.startsWith("event:")) name = line.slice(6).trim();
        if (line.startsWith("id:")) id = line.slice(3).trim();
        if (line.startsWith("data:"))
          lines.push(line.slice(5).replace(/^ /u, ""));
      }
      if (!lines.length) continue;
      const event = decode(lines.join("\n")) as ApplicationEvent;
      validateApplicationEvent(event, id);
      if (name !== event.type) throw new ApiError("invalid_application_event");
      this.deliver(event, id);
    }
  }
}
/** Header admission has an ordinary deadline; delivery has an independent heartbeat deadline. */
export async function applicationEvents(
  token: string | null,
  after: string | undefined,
  signal: AbortSignal,
  deliver: (event: ApplicationEvent, id: string) => void,
  fetcher: typeof fetch = (...args) => fetch(...args),
) {
  const abort = new AbortController();
  const combined = AbortSignal.any([signal, abort.signal]);
  let timer = setTimeout(
    () =>
      abort.abort(
        new DOMException("Event connection timed out", "TimeoutError"),
      ),
    35000,
  );
  let reader: ReadableStreamDefaultReader<Uint8Array> | undefined;
  try {
    const headers: Record<string, string> = {
      "x-jj-store-protocol": "1",
      accept: "text/event-stream",
    };
    if (token !== null) headers.authorization = `Bearer ${token}`;
    if (after) headers["last-event-id"] = after;
    const response = await fetcher("/v1/events", {
      headers,
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: combined,
    });
    clearTimeout(timer);
    if (
      !response.ok ||
      !response.body ||
      !response.headers.get("content-type")?.startsWith("text/event-stream")
    ) {
      await response.body?.cancel();
      throw new ApiError(
        response.status === 401 || response.status === 403
          ? "access_denied"
          : "event_unavailable",
      );
    }
    const parser = new ApplicationEventParser(deliver);
    const decoder = new TextDecoder("utf-8", { fatal: true });
    reader = response.body.getReader();
    const stop = () => {
      void reader?.cancel(combined.reason).catch(() => {});
    };
    combined.addEventListener("abort", stop, { once: true });
    try {
      while (!combined.aborted) {
        timer = setTimeout(
          () =>
            abort.abort(
              new DOMException("Event heartbeat stalled", "TimeoutError"),
            ),
          45000,
        );
        const { done, value } = await reader.read();
        clearTimeout(timer);
        if (combined.aborted) throw combined.reason;
        if (done) break;
        parser.push(decoder.decode(value, { stream: true }));
      }
    } finally {
      combined.removeEventListener("abort", stop);
    }
  } finally {
    clearTimeout(timer);
    await reader?.cancel().catch(() => {});
    reader?.releaseLock();
  }
}
