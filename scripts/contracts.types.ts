// Compile-time consumer acceptance. Expected errors must remain errors.
import type { components, operations } from "../web/src/lib/api/generated.js";
type Schemas = components["schemas"];
type LogQuery = operations["log"]["parameters"]["query"];
const log: LogQuery = { at_time_millis: 1790000000000 };
// @ts-expect-error A time anchor is mandatory even with an explicit operation.
const missingTime: LogQuery = { at_op: "00" };
const tree: Schemas["NativeTree"] = {
  entries: [["link", { type: "symlink", id: "00" }]],
};
const invalidTree: Schemas["NativeTree"] = {
  // @ts-expect-error Native entries preserve name/value tuple ordering.
  entries: [[{ type: "symlink", id: "00" }, "link"]],
};
declare const operation: Schemas["NativeOperation"];
const predecessorMap: Record<string, string[]> | null =
  operation.commit_predecessors;
const knownEmpty: Schemas["NativeOperation"] = {
  ...operation,
  commit_predecessors: {},
};
const absent: Schemas["NativeOperation"] = {
  ...operation,
  commit_predecessors: null,
};
const { commit_predecessors: _, ...omitted } = operation;
// @ts-expect-error Absent predecessor data serializes as explicit null, not an omitted field.
const missingPredecessors: Schemas["NativeOperation"] = omitted;

function graphIds(graph: Schemas["GraphNodes"]): string[] {
  switch (graph.graph) {
    case "log":
      return graph.nodes.map((node) => node.commit.id);
    case "operations":
      return graph.nodes.map((node) => node.id);
    case "evolution":
      return graph.nodes.flatMap((node) => node.predecessors);
    default: {
      const exhausted: never = graph;
      return exhausted;
    }
  }
}
function eventPosition(event: Schemas["CommittedEvent"]): number {
  if (event.type === "changed") {
    // @ts-expect-error A change notification does not contain an invented snapshot.
    event.snapshot;
    return event.cursor.position;
  }
  return event.snapshot.published_seq;
}
function taskError(task: Schemas["IndexTask"]): string | undefined {
  if (task.status === "failed") return task.error;
  if (task.status === "ready") return task.index.operation;
  return undefined;
}
void [
  log,
  missingTime,
  tree,
  invalidTree,
  predecessorMap,
  knownEmpty,
  absent,
  missingPredecessors,
  graphIds,
  eventPosition,
  taskError,
];

function operationParent(
  value: Schemas["OperationDiffData"],
): string | undefined {
  if (value.status === "difference") return value.parent;
  if (value.status === "select_parent") return value.parents[0];
  return undefined;
}
void operationParent;

type ReferenceWindow =
  operations["references"]["responses"][200]["content"]["application/json"];
declare const references: ReferenceWindow;
const referenceCursor: string | null = references.next_cursor;
type OperationChangesQuery =
  operations["operation_changes"]["parameters"]["query"];
const operationChanges: OperationChangesQuery = { parent: "00" };
// @ts-expect-error Operation merges never choose an implicit comparison parent.
const missingOperationParent: OperationChangesQuery = {};
function exactComparison(
  value: Schemas["Comparison"],
): [string, string] | undefined {
  if (value.status === "exact") return [value.from, value.to];
  if (value.status === "select") {
    // @ts-expect-error Ambiguous comparisons do not invent one from commit.
    value.from;
  }
  return undefined;
}
type DiffPathsQuery = operations["diff_paths"]["parameters"]["query"];
const diffPaths: DiffPathsQuery = { at_op: "00", from: "00", to: "00" };
// @ts-expect-error A changed-path page is always pinned to an explicit operation.
const unpinnedDiffPaths: DiffPathsQuery = { from: "00", to: "00" };
void [
  referenceCursor,
  operationChanges,
  missingOperationParent,
  exactComparison,
  diffPaths,
  unpinnedDiffPaths,
];

type MutationRequest =
  operations["mutation_apply"]["requestBody"]["content"]["application/json"];
const mutation: MutationRequest = {
  version: 1,
  request_id: "00",
  expected: { epoch: "00", position: 0, heads: ["00"] },
  command: { type: "describe", commit: "00", description: "example" },
};
// @ts-expect-error Caller identity is never an input capability.
const actor: MutationRequest = { ...mutation, principal: "00" };
const shell: MutationRequest = {
  ...mutation,
  // @ts-expect-error No arbitrary CLI or shell endpoint exists.
  command: { type: "shell", args: [] },
};
type MutationOutcome =
  operations["mutation_apply"]["responses"][200]["content"]["application/json"];
function mutationId(value: MutationOutcome): string {
  if (value.status === "committed") return value.receipt.request_id;
  // @ts-expect-error Unknown outcome is not a receipt.
  value.receipt;
  return value.request_id;
}
void [mutation, actor, shell, mutationId];
