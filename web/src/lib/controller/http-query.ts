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
  ): Promise<T> => {
    if (!active())
      return Promise.reject(new DOMException("Closed", "AbortError"));
    return queryClient.fetchQuery({
      queryKey: storeQueryKeys.resource(scope, kind, snapshot, args),
      queryFn: async ({ signal }) => {
        const value = await read(signal);
        if (!active()) throw new DOMException("Closed", "AbortError");
        return value;
      },
    });
  };
}
