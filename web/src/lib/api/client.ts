import { parse, stringify } from "lossless-json";
import type { components } from "./generated";

/** Preserve native integers without silently narrowing them to IEEE-754. */
export type Exact<T> = T extends number
  ? number | bigint
  : T extends readonly (infer U)[]
    ? Exact<U>[]
    : T extends object
      ? { [K in keyof T]: Exact<T[K]> }
      : T;
export type Model<K extends keyof components["schemas"]> = Exact<
  components["schemas"][K]
>;
export type Snapshot = Model<"ReadSnapshot">;
export type Head = Model<"HeadSnapshot">;
export type Envelope<T> = {
  snapshot: Snapshot;
  data: T;
  next_cursor: string | null;
};
export type Graph = Model<"GraphPage">;
export type Commit = Model<"CommitDetail">;
export type Tree = Model<"TreePage">;
export type Task = Model<"IndexTask">;
export type Event = Model<"CommittedEvent">;
export const pretty = (value: unknown) => stringify(value, null, 2) ?? "";
export function decode(text: string): unknown {
  return parse(text, null, (raw) => {
    if (!/^-?(0|[1-9][0-9]*)$/.test(raw))
      throw new Error("Non-integer API number");
    const n = Number(raw);
    return Number.isSafeInteger(n) ? n : BigInt(raw);
  });
}
export class ApiError extends Error {
  constructor(
    readonly code: string,
    readonly snapshot?: Snapshot,
    readonly heads?: Head,
  ) {
    super(code.replaceAll("_", " "));
  }
}
export function fullId(value: string, width = 128): string {
  if (!new RegExp(`^[0-9a-f]{${width}}$`).test(value))
    throw new ApiError("invalid_full_id");
  return value;
}
async function bytes(response: Response, limit: number): Promise<Uint8Array> {
  const reader = response.body?.getReader();
  if (!reader) return new Uint8Array();
  const chunks: Uint8Array[] = [];
  let length = 0;
  try {
    while (true) {
      const next = await reader.read();
      if (next.done) break;
      length += next.value.byteLength;
      if (length > limit) throw new ApiError("response_limit");
      chunks.push(next.value);
    }
    const result = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) {
      result.set(chunk, offset);
      offset += chunk.length;
    }
    return result;
  } finally {
    await reader.cancel().catch(() => {});
    reader.releaseLock();
  }
}
export class Client {
  #token: string;
  #closed = new AbortController();
  constructor(
    readonly repository: string,
    token: string,
    readonly denied: () => void,
    private fetcher: typeof fetch = (...args) => fetch(...args),
  ) {
    fullId(repository, 32);
    if (!token.trim() || /[\r\n]/.test(token) || token.length > 4096)
      throw new ApiError("invalid_credential");
    this.#token = token.trim();
  }
  close() {
    this.#token = "";
    this.#closed.abort();
  }
  async #fetch(
    path: string,
    query: Record<string, string | number | bigint | boolean | undefined>,
    init: RequestInit,
  ) {
    if (this.#closed.signal.aborted)
      throw new DOMException("Closed", "AbortError");
    const url = `/v1/repos/${this.repository}/${path}`;
    const params = new URLSearchParams();
    for (const [key, value] of Object.entries(query))
      if (value !== undefined) params.set(key, String(value));
    const response = await this.fetcher(
      `${url}${params.size ? `?${params}` : ""}`,
      {
        ...init,
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        signal: AbortSignal.any([
          this.#closed.signal,
          init.signal ?? new AbortController().signal,
          AbortSignal.timeout(35000),
        ]),
        headers: { ...init.headers, authorization: `Bearer ${this.#token}` },
      },
    );
    if (response.status === 401 || response.status === 403) {
      await response.body?.cancel();
      this.close();
      this.denied();
      throw new ApiError("access_denied");
    }
    return response;
  }
  async request<T>(
    path: string,
    query: Record<string, string | number | bigint | boolean | undefined> = {},
    init: RequestInit = {},
  ): Promise<T> {
    const response = await this.#fetch(path, query, init);
    if (!response.headers.get("content-type")?.startsWith("application/json")) {
      await response.body?.cancel();
      throw new ApiError("unexpected_response");
    }
    const value = decode(
      new TextDecoder("utf-8", { fatal: true }).decode(
        await bytes(response, 16 * 1024 * 1024),
      ),
    ) as T;
    if (!response.ok) {
      const error = value as {
        code?: string;
        snapshot?: Snapshot;
        heads?: Head;
      };
      throw new ApiError(
        typeof error.code === "string" ? error.code : "request_failed",
        error.snapshot,
        error.heads,
      );
    }
    return value;
  }
  async file(id: string, operation: string, signal: AbortSignal) {
    let response = await this.#fetch(
      `files/${fullId(id)}`,
      { at_op: fullId(operation) },
      { signal, headers: { Range: "bytes=0-262143" } },
    );
    if (
      response.status === 416 &&
      response.headers.get("content-range") === "bytes */0"
    ) {
      await response.body?.cancel();
      response = await this.#fetch(
        `files/${fullId(id)}`,
        { at_op: fullId(operation) },
        { signal },
      );
    }
    if (!response.ok) {
      await response.body?.cancel();
      throw new ApiError("file_unavailable");
    }
    if (
      response.headers.get("x-jj-store-operation") !== operation ||
      response.headers.get("x-jj-store-repository") !== this.repository
    ) {
      await response.body?.cancel();
      throw new ApiError("snapshot_mismatch");
    }
    const data = await bytes(response, 262144);
    let text: string | undefined;
    try {
      const decoded = new TextDecoder("utf-8", { fatal: true }).decode(data);
      if (!decoded.includes("\0")) text = decoded;
    } catch {
      /* binary */
    }
    return {
      text,
      bytes: data.length,
      range: response.headers.get("content-range"),
      hex: [...data.slice(0, 256)]
        .map((b) => b.toString(16).padStart(2, "0"))
        .join(" "),
    };
  }
  async events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: (event: Event, id: string) => void,
  ) {
    const response = await this.#fetch(
      "events",
      { after },
      { signal, headers: { Accept: "text/event-stream" } },
    );
    if (
      !response.ok ||
      !response.headers.get("content-type")?.startsWith("text/event-stream")
    ) {
      await response.body?.cancel();
      throw new ApiError("event_stream_unavailable");
    }
    const reader = response.body!.getReader();
    const parser = new EventParser(this.repository, deliver);
    const decoder = new TextDecoder("utf-8", { fatal: true });
    try {
      while (true) {
        const value = await reader.read();
        if (value.done) break;
        parser.push(decoder.decode(value.value, { stream: true }));
      }
      parser.push(decoder.decode());
      // An incomplete final frame deliberately does not advance the resume cursor.
    } finally {
      await reader.cancel().catch(() => {});
      reader.releaseLock();
    }
  }
}
/** A bounded streaming parser. Only a fully terminated frame can advance its ID. */
export class EventParser {
  #buffer = "";
  constructor(
    private repository: string,
    private deliver: (event: Event, id: string) => void,
  ) {}
  push(text: string) {
    this.#buffer += text;
    if (this.#buffer.length > 128 * 1024)
      throw new ApiError("event_frame_limit");
    while (true) {
      const match = /\r?\n\r?\n/.exec(this.#buffer);
      if (!match) return;
      const frame = this.#buffer.slice(0, match.index);
      this.#buffer = this.#buffer.slice(match.index + match[0].length);
      let name = "",
        id = "";
      const data: string[] = [];
      for (const line of frame.split(/\r?\n/)) {
        if (line.startsWith(":")) continue;
        const colon = line.indexOf(":");
        const field = colon < 0 ? line : line.slice(0, colon);
        const value = colon < 0 ? "" : line.slice(colon + 1).replace(/^ /, "");
        if (field === "event") name = value;
        if (field === "id") id = value;
        if (field === "data") data.push(value);
      }
      if (!data.length) continue;
      const event = decode(data.join("\n")) as Event;
      if (
        !event ||
        event.version !== 1 ||
        !event.cursor ||
        !["ready", "changed", "reset"].includes(event.type)
      )
        throw new ApiError("invalid_event");
      const cursor = event.cursor;
      if (
        cursor.repository !== this.repository ||
        !/^[0-9a-f]{32}$/.test(cursor.epoch) ||
        !/^(0|[1-9][0-9]*)$/.test(String(cursor.position)) ||
        BigInt(cursor.position) > 18446744073709551615n ||
        id !== `v1:${this.repository}:${cursor.epoch}:${cursor.position}` ||
        name !== (event.type === "changed" ? "repository-changed" : event.type)
      )
        throw new ApiError("invalid_event_cursor");
      this.deliver(event, id);
    }
  }
}
