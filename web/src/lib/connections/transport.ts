import { ApiError } from "../api/client.js";
const idPattern = /^[0-9a-f]{32}$/;
export function readConnectionHash(hash: string): { id: string; hash: string } {
  const mark = hash.indexOf("?"),
    query = new URLSearchParams(mark < 0 ? "" : hash.slice(mark + 1));
  const ids = query.getAll("server");
  if (ids.length > 1 || (ids.length === 1 && !idPattern.test(ids[0]!)))
    throw new ApiError("invalid_server_selection");
  query.delete("server");
  return {
    id: ids[0] ?? "local",
    hash:
      (mark < 0 ? hash : hash.slice(0, mark)) + (query.size ? `?${query}` : ""),
  };
}
export function connectionHash(hash: string, id: string) {
  const clean = readConnectionHash(hash).hash;
  if (id === "local") return clean;
  if (!idPattern.test(id)) throw new ApiError("invalid_server_selection");
  return `${clean}${clean.includes("?") ? "&" : "?"}server=${id}`;
}
export function connectionFetch(
  id: string,
  handle: string,
  fetcher: typeof fetch = fetch,
  invalidated?: () => void,
): typeof fetch {
  if (!idPattern.test(id) || !idPattern.test(handle))
    throw new ApiError("invalid_connection");
  const transport: typeof fetch & {
    connectionEvents?: { path: string; handle: string };
  } = async (input, init) => {
    if (
      typeof input !== "string" ||
      !input.startsWith("/v1/") ||
      input.startsWith("/v1/local/")
    )
      throw new ApiError("unsupported_remote_action");
    const headers = new Headers(init?.headers);
    headers.delete("authorization");
    headers.set("x-jj-store-connection-session", handle);
    headers.set("x-jj-store-protocol", "1");
    const response = await fetcher(
      `/v1/local/connections/${id}/api/${input.slice(4)}`,
      {
        ...init,
        headers,
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
      },
    );
    if (
      (response.status === 403 || response.status === 409) &&
      response.headers.get("content-type")?.startsWith("application/json")
    ) {
      try {
        const body = await response.clone().json();
        if (
          body.code === "connection_denied" ||
          body.code === "connection_identity_changed"
        )
          invalidated?.();
      } catch {
        /* Normal API decoding retains the original response. */
      }
    }
    return response;
  };
  transport.connectionEvents = {
    path: `/v1/local/connections/${id}/events`,
    handle,
  };
  return transport;
}
