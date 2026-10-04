import { ApiError, decode, responseBytes, type Model } from "./client.js";

export type Bootstrap = Model<"Bootstrap">;

export async function bootstrap(
  fetcher: typeof fetch = (...args) => fetch(...args),
  signal?: AbortSignal,
): Promise<Bootstrap> {
  const response = await fetcher("/v1/bootstrap", {
    credentials: "omit",
    redirect: "error",
    cache: "no-store",
    signal: AbortSignal.any([
      signal ?? new AbortController().signal,
      AbortSignal.timeout(35000),
    ]),
    headers: { "x-jj-store-protocol": "1" },
  });
  if (!response.headers.get("content-type")?.startsWith("application/json")) {
    await response.body?.cancel();
    throw new ApiError("unexpected_response");
  }
  const value = decode(
    new TextDecoder("utf-8", { fatal: true }).decode(
      await responseBytes(response, 1024 * 1024),
    ),
  ) as Bootstrap & { code?: string };
  if (!response.ok) throw new ApiError(value.code ?? "bootstrap_failed");
  return value;
}
