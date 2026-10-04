import { Client, fullId, type Model } from "../api/client.js";
import type { ReferenceQuery } from "./native-data-source.js";
import type { createHttpQuery } from "./http-query.js";

export function httpReferences(
  client: Client,
  read: ReturnType<typeof createHttpQuery>,
  operation: string,
  cursor?: string,
  input: ReferenceQuery = {},
) {
  const query = {
    at_op: fullId(operation),
    cursor,
    limit: 50,
    kind: input.kind ?? "bookmarks",
    exact: input.exact,
    q: input.query ?? "",
  };
  if (input.signal)
    return client.request<Model<"Window">>("references", query, {
      signal: input.signal,
    });
  return read<Model<"Window">>("references", operation, query, (signal) =>
    client.request("references", query, { signal }),
  );
}
