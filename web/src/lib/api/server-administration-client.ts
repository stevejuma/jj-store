import { ApiError, decode, pretty, responseBytes } from "./client.js";
/** Operator and onboarding transport stays bound to its original connection. */
export class ServerAdministrationClient {
  readonly lifetime = new AbortController();
  constructor(
    private available: () => boolean,
    private token: () => string | null,
    private fetcher: typeof fetch = fetch,
  ) {}
  async request<T>(path: string, body?: unknown): Promise<T> {
    const credential = this.token();
    if (this.lifetime.signal.aborted || !this.available() || !credential)
      throw new ApiError("agent_administration_denied");
    const response = await this.fetcher(`/v1/${path}`, {
      method: body === undefined ? "GET" : "POST",
      headers: {
        authorization: `Bearer ${credential}`,
        "x-jj-store-protocol": "1",
        ...(body === undefined ? {} : { "content-type": "application/json" }),
      },
      body: body === undefined ? undefined : pretty(body),
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.any([
        this.lifetime.signal,
        AbortSignal.timeout(35000),
      ]),
    });
    const result = decode(
      new TextDecoder().decode(await responseBytes(response, 1024 * 1024)),
    ) as T & { code?: string };
    if (credential !== this.token() || this.lifetime.signal.aborted)
      throw new ApiError("administration_session_changed");
    if (!response.ok)
      throw new ApiError(result.code ?? "agent_administration_unavailable");
    return result;
  }
  close() {
    this.lifetime.abort();
  }
}
