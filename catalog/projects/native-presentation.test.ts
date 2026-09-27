import { test, expect } from "vitest";
import { collapseRepositoryLogRevisions } from "./repository-log-collapse.js";
import { parseRepositoryLogMessage } from "./repository-log-message.js";
import { buildRepositoryLogGraph } from "./repository-log-graph.js";
import {
  fixtureGraph,
  IDS,
  revisions,
  operations,
} from "../fixtures/repository.js";

test("preserves_native_divergence_and_bookmark_targets", () => {
  const graph = fixtureGraph("conflicted");
  expect(graph.revisions.map((r) => r.eventId)).toContain(IDS.left);
  expect(graph.revisions.map((r) => r.eventId)).toContain(IDS.right);
  expect(graph.revisions.find((r) => r.eventId === IDS.left)?.changeId).toBe(
    graph.revisions.find((r) => r.eventId === IDS.right)?.changeId,
  );
  expect(
    collapseRepositoryLogRevisions([...revisions, revisions[0]!]),
  ).toHaveLength(revisions.length);
  expect(fixtureGraph("retired").revisions).toEqual([]);
  expect(
    fixtureGraph("feature/native-objects").revisions.map((r) => r.eventId),
  ).not.toContain(IDS.right);
});
test("native_ids_remain_complete_in_messages", () => {
  const change = "ab".repeat(16);
  expect(parseRepositoryLogMessage(`describe ${IDS.left} ${change}`)).toEqual([
    { type: "text", value: "describe " },
    { type: "commit", value: IDS.left },
    { type: "text", value: " " },
    { type: "change", value: change },
  ]);
  expect(parseRepositoryLogMessage("a".repeat(40))).toEqual([
    { type: "text", value: "a".repeat(40) },
  ]);
});
test("operation_siblings_keep_separate_graph_lanes", () => {
  const before = JSON.stringify(operations);
  const rows = buildRepositoryLogGraph(
    operations.revisions,
    operations.devices,
  );
  expect(rows[0]?.isTip).toBe(true);
  expect(rows[1]?.isTip).toBe(true);
  expect(rows[0]?.commitLane).not.toBe(rows[1]?.commitLane);
  expect(JSON.stringify(operations)).toBe(before);
});
