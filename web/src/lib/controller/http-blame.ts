import type { QueryClient } from "@tanstack/svelte-query";
import { ApiError, fullId, type Client, type Model } from "../api/client.js";
import { createBlameQuery } from "./blame-query.js";
import type { BlameSelection } from "./blame-types.js";

export function createHttpBlame(options: {
  client: QueryClient;
  scope: string;
  active(): boolean;
  request(
    selection: BlameSelection,
    continuation: string | null,
    signal: AbortSignal,
  ): Promise<Model<"BlamePage">>;
}) {
  return createBlameQuery({
    ...options,
    async read(selection, continuation, signal, preparing) {
      let reset = false;
      for (;;) {
        signal.throwIfAborted();
        try {
          const page = await options.request(selection, continuation, signal);
          if (
            page.version !== 1 ||
            page.repository !== selection.repository ||
            page.operation !== selection.operation ||
            page.commit !== selection.commit ||
            page.path !== selection.path
          )
            throw new ApiError("snapshot_mismatch");
          preparing(false);
          return {
            lineCount: Number(page.line_count),
            ranges: page.ranges.map((r) => ({
              start: Number(r.start),
              end: Number(r.end),
              commitId: r.commit_id,
            })),
            commits: Object.fromEntries(
              Object.entries(page.commits).map(([id, c]) => [
                id,
                {
                  eventId: c.id,
                  commitId: c.id,
                  changeId: c.change_id,
                  message: c.description,
                  author: c.author.name,
                  authorEmail: c.author.email,
                  createdAt: Math.floor(
                    Number(c.committer.timestamp.millis) / 1000,
                  ),
                  committedAtMs: Number(c.committer.timestamp.millis),
                },
              ]),
            ),
            complete: page.complete,
            continuation: page.continuation,
          };
        } catch (error) {
          if (
            error instanceof ApiError &&
            error.code === "blame_reset" &&
            !reset
          ) {
            continuation = null;
            reset = true;
            continue;
          }
          if (
            !(error instanceof ApiError) ||
            !["index_rebuild_required", "busy"].includes(error.code)
          )
            throw error;
          preparing(error.code === "index_rebuild_required");
          await new Promise<void>((resolve, reject) => {
            const done = () => {
              signal.removeEventListener("abort", abort);
              resolve();
            };
            const timer = setTimeout(done, 1000);
            const abort = () => {
              clearTimeout(timer);
              signal.removeEventListener("abort", abort);
              reject(signal.reason);
            };
            signal.addEventListener("abort", abort, { once: true });
            if (signal.aborted) abort();
          });
        }
      }
    },
  });
}

function nativeBlameRequest(client: Client) {
  return (
    selection: BlameSelection,
    continuation: string | null,
    signal: AbortSignal,
  ) =>
    client.request<Model<"BlamePage">>(
      `commits/${fullId(selection.commit)}/blame`,
      {
        at_op: fullId(selection.operation),
        path: selection.path,
        continuation: continuation ?? undefined,
      },
      { signal },
    );
}

export function createNativeBlame(
  client: Client,
  queries: QueryClient,
  scope: () => string,
  active: () => boolean,
) {
  let currentScope: string | undefined;
  let provider: ReturnType<typeof createHttpBlame> | undefined;
  return () => {
    const selectedScope = scope();
    if (!provider || currentScope !== selectedScope) {
      currentScope = selectedScope;
      provider = createHttpBlame({
        client: queries,
        scope: selectedScope,
        active: () => active() && scope() === selectedScope,
        request: nativeBlameRequest(client),
      });
    }
    return provider;
  };
}
