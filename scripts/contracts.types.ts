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
