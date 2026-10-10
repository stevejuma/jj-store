import {
  ApiError,
  decode,
  fullId,
  pretty,
  responseBytes,
  type Model,
} from "./client.js";
/** LocalTrusted transport; credentials and native repository URLs never enter this client. */
export class LocalClient {
  readonly closed = new AbortController();
  constructor(
    readonly repository: string,
    readonly fetcher: typeof fetch = (...args) => fetch(...args),
  ) {
    if (repository) fullId(repository, 32);
  }
  close() {
    this.closed.abort();
  }
  async fetch(
    path: string,
    init: RequestInit = {},
    timeout: number | null = 35000,
  ) {
    this.closed.signal.throwIfAborted();
    const response = await this.fetcher(
      `/v1/local/${this.repository ? `repos/${this.repository}/` : ""}${path}`,
      {
        ...init,
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        signal: AbortSignal.any([
          this.closed.signal,
          init.signal ?? new AbortController().signal,
          ...(timeout === null ? [] : [AbortSignal.timeout(timeout)]),
        ]),
        headers: { ...init.headers, "x-jj-store-protocol": "1" },
      },
    );
    this.closed.signal.throwIfAborted();
    return response;
  }
  async request<T>(
    path: string,
    body?: unknown,
    signal?: AbortSignal,
    method?: "POST",
  ): Promise<T> {
    const response = await this.fetch(
      path,
      body === undefined
        ? { signal, method }
        : {
            signal,
            method: "POST",
            headers: { "content-type": "application/json" },
            body: pretty(body),
          },
    );
    if (!response.headers.get("content-type")?.startsWith("application/json")) {
      await response.body?.cancel();
      throw new ApiError("unexpected_response");
    }
    const value = decode(
      new TextDecoder("utf-8", { fatal: true }).decode(
        await responseBytes(response, 16 * 1024 * 1024),
      ),
    ) as T & {
      code?: string;
      diagnostic?: { kind: string; message: string };
      local_snapshot?: Model<"LocalCodeSnapshot">;
    };
    this.closed.signal.throwIfAborted();
    if (!response.ok)
      throw new ApiError(
        value.code ?? "local_request_failed",
        undefined,
        undefined,
        value.diagnostic,
        value.local_snapshot,
      );
    return value;
  }
}
