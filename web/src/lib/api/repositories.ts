import { ApiError, decode, responseBytes, type Model } from "./client.js";

export type RepositoryPage = Model<"RepositoryPage">;
export type RepositorySummary = Model<"RepositorySummary">;

async function page(
  token: string | null,
  cursor: string | null,
  fetcher: typeof fetch,
  signal: AbortSignal,
): Promise<RepositoryPage> {
  const query = new URLSearchParams({ limit: "100" });
  if (cursor) query.set("cursor", cursor);
  const headers: Record<string, string> = { "x-jj-store-protocol": "1" };
  if (token) headers.authorization = `Bearer ${token}`;
  const response = await fetcher(`/v1/repositories?${query}`, {
    credentials: "omit",
    redirect: "error",
    cache: "no-store",
    signal,
    headers,
  });
  if (!response.headers.get("content-type")?.startsWith("application/json")) {
    await response.body?.cancel();
    throw new ApiError("unexpected_response");
  }
  const value = decode(
    new TextDecoder("utf-8", { fatal: true }).decode(
      await responseBytes(response, 1024 * 1024),
    ),
  ) as RepositoryPage & { code?: string };
  if (!response.ok)
    throw new ApiError(value.code ?? "repository_discovery_failed");
  return value;
}

export async function repositories(
  token: string | null,
  fetcher: typeof fetch = (...args) => fetch(...args),
  signal?: AbortSignal,
): Promise<RepositorySummary[]> {
  const controller = new AbortController();
  const combined = AbortSignal.any([
    signal ?? new AbortController().signal,
    controller.signal,
    AbortSignal.timeout(35000),
  ]);
  const result: RepositorySummary[] = [];
  let cursor: string | null = null;
  try {
    do {
      const current = await page(token, cursor, fetcher, combined);
      result.push(...current.repositories);
      cursor = current.next_cursor ?? null;
    } while (cursor);
    return result;
  } finally {
    controller.abort();
  }
}
