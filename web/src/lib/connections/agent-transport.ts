import { ApiError } from "../api/client.js";
/** Execution alone has an authenticated API. Device management keeps its own transport. */
export function authenticatedAgentFetch(
  fetcher: typeof fetch = fetch,
  token: () => string | null,
): typeof fetch {
  return (input, init) => {
    if (
      typeof input !== "string" ||
      !input.startsWith("/v1/") ||
      (input.startsWith("/v1/local/") &&
        !/^\/v1\/local\/agents(?:\?|$|\/(?:chat|command|outcome|events|stream|targets|workspace\/inspect|feedback)(?:\?|$))/.test(
          input,
        ))
    )
      throw new ApiError("unsupported_remote_agent_action");
    const credential = token();
    if (!credential) throw new ApiError("agent_execution_denied");
    const headers = new Headers(init?.headers);
    headers.set("authorization", `Bearer ${credential}`);
    return fetcher(input.replace("/v1/local/agents", "/v1/agents"), {
      ...init,
      headers,
    });
  };
}
