import {
  QueryObserver,
  type QueryClient,
  type FetchQueryOptions,
} from "@tanstack/svelte-query";

/** Each caller owns an observer; only the last release can abort shared transport. */
export async function readQuery<T>(
  client: QueryClient,
  options: FetchQueryOptions<T>,
  signal?: AbortSignal,
): Promise<T> {
  signal?.throwIfAborted();
  const observer = new QueryObserver(client, { ...options, enabled: false });
  const release = observer.subscribe(() => {});
  let stop: (() => void) | undefined;
  try {
    const read = client.fetchQuery(options);
    if (!signal) return await read;
    return await Promise.race([
      read,
      new Promise<never>((_, reject) => {
        stop = () =>
          reject(signal.reason ?? new DOMException("Aborted", "AbortError"));
        signal.addEventListener("abort", stop, { once: true });
        if (signal.aborted) stop();
      }),
    ]);
  } finally {
    if (stop) signal?.removeEventListener("abort", stop);
    release();
    observer.destroy();
  }
}
