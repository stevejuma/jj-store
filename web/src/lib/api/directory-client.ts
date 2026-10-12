import type { paths } from "./generated.js";
import type { Exact } from "./client.js";
export type DirectoryRequest = Exact<
  NonNullable<
    paths["/v1/directory/commands"]["post"]["requestBody"]
  >["content"]["application/json"]
>;
import { ApiError, decode, pretty, responseBytes } from "./client.js";
/** Installation transport shares admission, memory-only credentials and cancellation. */
export class DirectoryClient {
  readonly lifetime = new AbortController();
  constructor(
    private token: () => string | null,
    private fetcher: typeof fetch = (...args) => fetch(...args),
  ) {}
  async fetch(
    path: string,
    body?: unknown,
    bytes?: Uint8Array<ArrayBuffer>,
  ): Promise<Response> {
    const token = this.token();
    const headers: Record<string, string> = { "x-jj-store-protocol": "1" };
    if (token) headers.authorization = `Bearer ${token}`;
    if (body !== undefined) headers["content-type"] = "application/json";
    if (bytes) headers["content-type"] = "application/octet-stream";
    const response = await this.fetcher(`/v1/directory/${path}`, {
      method: body === undefined && !bytes ? "GET" : "POST",
      headers,
      body: bytes ?? (body === undefined ? undefined : pretty(body)),
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.any([
        this.lifetime.signal,
        AbortSignal.timeout(35_000),
      ]),
    });
    if (token !== this.token()) {
      await response.body?.cancel();
      throw new ApiError("directory_session_changed");
    }
    return response;
  }
  async request<T>(
    path: string,
    body?: unknown,
    bytes?: Uint8Array<ArrayBuffer>,
  ): Promise<T> {
    const token = this.token();
    const response = await this.fetch(path, body, bytes);
    const value = decode(
      new TextDecoder().decode(await responseBytes(response, 16 * 1024 * 1024)),
    ) as T & { code?: string };
    if (token !== this.token()) throw new ApiError("directory_session_changed");
    if (!response.ok) throw new ApiError(value.code ?? "directory_unavailable");
    return value;
  }
  close() {
    this.lifetime.abort();
  }
}
