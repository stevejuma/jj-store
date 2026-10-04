import {
  fullId,
  type Client,
  type Envelope,
  type Graph,
} from "../api/client.js";
import type { createHttpQuery } from "./http-query.js";
import type { GraphQuery } from "./native-data-source.js";

export function readHttpGraph(
  client: Client,
  cached: ReturnType<typeof createHttpQuery>,
  kind: "log" | "operations" | "evolution",
  input: GraphQuery,
) {
  const query = {
    at_op: fullId(input.operation),
    cursor: input.cursor,
    limit: input.limit ?? 50,
    ...(kind === "log"
      ? {
          revset: input.revset ?? "all()",
          workspace: input.workspace,
          user_email: input.userEmail ?? "",
          at_time_millis: input.atTimeMillis ?? Date.now(),
        }
      : {}),
    ...(kind === "operations" ? { query: input.operationQuery ?? "" } : {}),
    ...(kind === "evolution" ? { commit: fullId(input.commit ?? "") } : {}),
  };
  return cached<Envelope<Graph>>(
    kind,
    input.operation,
    query,
    (signal) => client.request(kind, query, { signal }),
    input.signal,
  );
}
