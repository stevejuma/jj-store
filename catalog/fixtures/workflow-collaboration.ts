import { createThreadTransport } from "./thread-transport.js";
import type { StoreDataSource } from "../../web/src/lib/controller/native-data-source.js";
import type { Model } from "../../web/src/lib/controller/thread-types.js";
import { home } from "./workflows.js";

/** Independent code and thread reads; no history or conversation view is preloaded. */
export function workflowCollaboration(
  repository: string,
  fail = false,
): StoreDataSource {
  const { transport } = createThreadTransport();
  const operation = "61".repeat(64),
    epoch = "62".repeat(16);
  const snapshot = {
    version: 1,
    repo_id: repository,
    epoch,
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    heads: [operation],
  };
  const reader: Pick<
    StoreDataSource,
    "repository" | "threads" | "close" | "snapshot" | "queryInfo" | "graph"
  > = {
    repository,
    close() {},
    snapshot: async () => {
      if (fail) throw Error("Source lookup failed");
      return snapshot;
    },
    queryInfo: async () => ({
      version: 1,
      jj_version: "0.45.1",
      max_query_bytes: 4096,
      callables: [
        {
          name: "bookmarks",
          signature: "bookmarks(pattern)",
          insert_text: "bookmarks()",
          description: "Commits at bookmarks.",
          kind: "function",
        },
      ],
      aliases: [],
      operators: ["|", "&"],
      operation_fields: [],
      unsupported: [],
    }),
    graph: async () => ({
      snapshot: { ...snapshot, operation_id: operation },
      next_cursor: null,
      data: {
        version: 1,
        operation,
        data: {
          graph: "log",
          nodes: [
            {
              commit: {
                id: "63".repeat(64),
                change_id: "64".repeat(16),
                parents: [],
                description: "Fixture commit",
                author: {
                  name: repository === home ? "Home author" : "Code author",
                  email:
                    repository === home
                      ? "home@example.invalid"
                      : "code@example.invalid",
                  timestamp: { millis: 1, offset_minutes: 0 },
                },
                committer: {
                  name: "Fixture",
                  email: "fixture@example.invalid",
                  timestamp: { millis: 1, offset_minutes: 0 },
                },
                tree_terms: [],
                conflicted: false,
                visible: true,
                bookmarks: [
                  {
                    name: "main",
                    remote: null,
                    added: true,
                    removed: false,
                    conflicted: false,
                  },
                ],
                workspaces: [],
                divergent: false,
              },
              edges: [],
            },
          ],
        },
        next: null,
        stats: {
          object_reads: 1,
          object_bytes: 1,
          work: 1,
          metadata_nodes: 1,
          predicate_file_bytes: 0,
          predicate_lines: 0,
        },
        cache: {
          state: "built",
          evicted: 0,
          retained_entries: 0,
          retained_bytes: 0,
        },
      } satisfies Model<"GraphPage">,
    }),
    threads: {
      ...transport,
      async list(cursor) {
        if (fail) throw Error("Thread lookup failed");
        const page = await transport.list(cursor);
        return {
          ...page,
          snapshot: { ...page.snapshot, repo_id: repository },
          items: {
            type: "threads",
            threads:
              repository === home && page.items.type === "threads"
                ? page.items.threads
                : [],
          },
        };
      },
    },
  };
  return reader as StoreDataSource;
}
