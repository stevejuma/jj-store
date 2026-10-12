import { fullId, type Client, type Model } from "../api/client.js";
import type { SearchQuery } from "./native-data-source.js";
import type { createHttpQuery } from "./http-query.js";

export function readHttpSearch(
  client: Client,
  queryRead: ReturnType<typeof createHttpQuery>,
  input: SearchQuery,
) {
  const query = {
    q: input.query,
    mode: "visible_commits",
    at_op: fullId(input.operation),
    page_size: input.limit ?? 50,
    cursor: input.cursor,
  };
  return queryRead<Model<"SearchPage">>(
    "search",
    input.operation,
    query,
    (signal) => client.request("search", query, { signal }),
  );
}
