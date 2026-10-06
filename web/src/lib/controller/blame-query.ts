import type { QueryClient } from "@tanstack/svelte-query";
import { readQuery } from "./read-query.js";
import type {
  BlameController,
  BlameProgress,
  BlameSelection,
} from "./blame-types.js";

/** Reuses application query leases; the host must supply an authorized, qualified reader. */
export function createBlameQuery(options: {
  client: QueryClient;
  /** Includes backend, registration/epoch, authorization lifetime and algorithm version. */
  scope: string;
  active(): boolean;
  read(
    selection: BlameSelection,
    continuation: string | null,
    signal: AbortSignal,
  ): Promise<BlameProgress>;
}): BlameController {
  const prefix = ["file-blame", options.scope];
  options.client.setQueryDefaults(prefix, {
    gcTime: 600_000,
    staleTime: Infinity,
    retry: false,
  });
  return {
    observe(selection, publish) {
      const abort = new AbortController();
      const key = [...prefix, selection];
      const progressKey = [...key, "progress"];
      let progress = options.active()
        ? options.client.getQueryData<BlameProgress>(progressKey)
        : undefined;
      async function run() {
        try {
          if (!options.active())
            throw new Error("Blame source is no longer available.");
          publish({
            status: progress?.complete ? "ready" : "loading",
            ...(progress ? { progress } : {}),
          });
          while (!progress?.complete) {
            const continuation = progress?.continuation ?? null;
            const next = await readQuery(
              options.client,
              {
                queryKey: [...key, "batch", continuation],
                staleTime: Infinity,
                // Only the latest cumulative progress is retained after observers release.
                gcTime: 0,
                retry: false,
                queryFn: async ({ signal }) => {
                  if (!options.active())
                    throw new Error("Blame source is no longer available.");
                  const result = await options.read(
                    selection,
                    continuation,
                    signal,
                  );
                  signal.throwIfAborted();
                  if (!options.active())
                    throw new Error("Blame source is no longer available.");
                  validateProgress(result, selection, continuation);
                  return result;
                },
              },
              abort.signal,
            );
            abort.signal.throwIfAborted();
            if (!options.active())
              throw new Error("Blame source is no longer available.");
            progress = next;
            options.client.setQueryData(progressKey, progress);
            publish({
              status: progress.complete ? "ready" : "loading",
              progress,
            });
          }
        } catch (error) {
          if (!abort.signal.aborted)
            publish({
              status: "failed",
              ...(options.active() && progress ? { progress } : {}),
              error:
                error instanceof Error
                  ? error.message
                  : "Unable to load Blame.",
            });
        }
      }
      void run();
      return () => abort.abort();
    },
  };
}
function validateProgress(
  progress: BlameProgress,
  selection: BlameSelection,
  previous: string | null,
) {
  if (
    progress.lineCount !== selection.lineCount ||
    (progress.complete
      ? progress.continuation !== null
      : !progress.continuation || progress.continuation === previous)
  )
    throw new Error("Invalid Blame progress for the selected file.");
  let end = 1;
  for (const range of progress.ranges) {
    const commit = progress.commits[range.commitId];
    if (
      !Number.isSafeInteger(range.start) ||
      !Number.isSafeInteger(range.end) ||
      range.start < end ||
      range.start >= range.end ||
      range.end > progress.lineCount + 1 ||
      !commit ||
      (commit.commitId ?? commit.eventId) !== range.commitId ||
      !/^(?:[0-9a-f]{40}|[0-9a-f]{128})$/u.test(range.commitId)
    )
      throw new Error("Invalid Blame origin for the selected file.");
    end = range.end;
  }
}
