import { test, expect } from "vitest";
import { collapseRepositoryLogRevisions } from "../../web/src/lib/projects/repository-log-collapse.js";
import { parseRepositoryLogMessage } from "../../web/src/lib/projects/repository-log-message.js";
import { buildRepositoryLogGraph } from "../../web/src/lib/projects/repository-log-graph.js";
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
});
test("git_sha1_ids_remain_complete_in_operation_messages", () => {
  const commit = "87e1cd5db3c0a551cd4a4feedde962ca63368fb9";
  for (const prefix of [
    "create bookmark main pointing to commit ",
    "describe commit ",
  ]) {
    expect(parseRepositoryLogMessage(`${prefix}${commit}`)).toEqual([
      { type: "text", value: prefix },
      { type: "commit", value: commit },
    ]);
  }
  expect(parseRepositoryLogMessage(`(${commit.toUpperCase()})`)).toEqual([
    { type: "text", value: "(" },
    { type: "commit", value: commit },
    { type: "text", value: ")" },
  ]);
});
test("partial_or_embedded_ids_remain_plain_text", () => {
  for (const message of [
    "a".repeat(12),
    "a".repeat(39),
    "a".repeat(41),
    "a".repeat(64),
    "a".repeat(129),
    `prefix${"a".repeat(40)}`,
    `${"a".repeat(40)}suffix`,
    `_${"a".repeat(40)}_`,
  ]) {
    expect(parseRepositoryLogMessage(message)).toEqual([
      { type: "text", value: message },
    ]);
  }
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
