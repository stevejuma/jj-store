import { readQuery } from "./read-query.js";
import type { QueryClient } from "@tanstack/svelte-query";
import { storeQueryKeys, type StoreQueryScope } from "./store-query-keys.js";
export function createHttpQuery(
  queryClient: QueryClient,
  scope: StoreQueryScope,
  active: () => boolean,
) {
  return <T>(
    kind: string,
    snapshot: string | null,
    args: Record<string, string | number | bigint | boolean | undefined>,
    read: (signal: AbortSignal) => Promise<T>,
    signal?: AbortSignal,
  ): Promise<T> => {
    if (!active())
      return Promise.reject(new DOMException("Closed", "AbortError"));
    return readQuery(
      queryClient,
      {
        queryKey: storeQueryKeys.resource(scope, kind, snapshot, args),
        staleTime:
          snapshot &&
          ["file", "commit", "operation", "commit-summary"].includes(kind)
            ? Infinity
            : 30_000,
        queryFn: async ({ signal }) => {
          const value = await read(signal);
          if (!active()) throw new DOMException("Closed", "AbortError");
          return value;
        },
      },
      signal,
    );
  };
}
