import { QueryClient } from "@tanstack/svelte-query";
import { ApiError } from "../api/client.js";

export const STORE_QUERY_STALE_TIME_MS = 30_000;
export const STORE_QUERY_GC_TIME_MS =
  typeof window === "undefined" ? Number.POSITIVE_INFINITY : 10 * 60_000;

export function createStoreQueryClient(): QueryClient {
  return new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: STORE_QUERY_STALE_TIME_MS,
        gcTime: STORE_QUERY_GC_TIME_MS,
        refetchOnWindowFocus: false,
        retry: (failures, error) =>
          !(error instanceof ApiError) && failures < 1,
      },
    },
  });
}
