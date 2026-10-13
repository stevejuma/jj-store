import type { StoreDataSource } from "../../web/src/lib/controller/native-data-source.js";
import { workflowCollaboration } from "./workflow-collaboration.js";
export const runOperation = "61".repeat(64),
  previousOperation = "65".repeat(64),
  runCommit = "63".repeat(64),
  previousCommit = "66".repeat(64);
export type SourceFailure = "operations" | "commits" | "bookmarks";

export function workflowRunReader(
  repository: string,
  failure?: SourceFailure,
): StoreDataSource {
  const base = workflowCollaboration(repository);
  let failed = false;
  function check(kind: SourceFailure) {
    if (failure === kind && !failed) {
      failed = true;
      throw Error(`${kind} lookup failed`);
    }
  }
  return {
    ...base,
    async graph(kind, query) {
      check(kind === "operations" ? "operations" : "commits");
      const page = await base.graph("log", query);
      const operation = query.operation;
      if (kind === "operations")
        return {
          ...page,
          data: {
            ...page.data,
            data: {
              graph: "operations",
              nodes: [runOperation, previousOperation].map((id, index) => ({
                id,
                parents: [],
                view: "67".repeat(64),
                description: index ? "Previous source" : "Publish source",
                user: "Fixture user",
                host: "fixture",
                start: { millis: 1, offset_minutes: 0 },
                end: { millis: 2, offset_minutes: 0 },
                is_snapshot: false,
                workspace: null,
                attributes: {},
                predecessor_data_present: true,
              })),
            },
          },
        };
      if (page.data.data.graph !== "log")
        throw Error("Unexpected fixture graph");
      return {
        ...page,
        snapshot: { ...page.snapshot, operation_id: operation },
        data: {
          ...page.data,
          operation,
          data: {
            graph: "log",
            nodes: page.data.data.nodes.map((node) => ({
              ...node,
              commit: {
                ...node.commit,
                id:
                  operation === previousOperation ? previousCommit : runCommit,
                description:
                  operation === previousOperation
                    ? "Previous commit"
                    : "Fixture commit",
              },
            })),
          },
        },
      };
    },
    async references(operation) {
      check("bookmarks");
      const page = await base.graph("log", { operation });
      if (page.data.data.graph !== "log")
        throw Error("Unexpected fixture graph");
      const commit =
        operation === previousOperation ? previousCommit : runCommit;
      const source = page.data.data.nodes[0]!.commit;
      const target = {
        commit,
        change: source.change_id,
        parents: [],
        description: source.description,
        author: source.author,
        committer: source.committer,
        conflicted: false,
        workspaces: [],
      };
      return {
        snapshot: { ...page.snapshot, operation_id: operation },
        previous_cursor: null,
        next_cursor: null,
        data: {
          version: 1,
          operation,
          previous: null,
          next: null,
          stats: page.data.stats,
          references: [
            {
              kind: "bookmark",
              name: "main",
              terms: [commit],
              conflicted: false,
              workspaces: [],
              targets: [target],
            },
            {
              kind: "bookmark",
              name: "ambiguous",
              terms: [commit, null],
              conflicted: true,
              workspaces: [],
              targets: [target],
            },
          ],
        },
      };
    },
  };
}
