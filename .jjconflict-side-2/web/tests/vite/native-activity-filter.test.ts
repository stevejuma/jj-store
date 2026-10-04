import { expect, test } from "vitest";
import { filterNativeActivity } from "../../src/lib/controller/native-activity-filter.js";
import type { NativeActivityCommit } from "../../src/lib/controller/native-activity-types.js";
import { activityCommit } from "../../../catalog/fixtures/native-activity.js";

const commit = () => activityCommit("11".repeat(16));
test("all activity keeps discovery lazy and agent filters never load session bodies", async () => {
  let reads = 0,
    bodies = 0;
  const associated = commit();
  associated.agents = {
    sessions: [],
    error: "",
    discover: async () => {
      reads++;
      return {
        sessions: [],
        error: "",
        relatedSessionIds: ["22".repeat(16)],
        load: async () => {
          bodies++;
          return { sessions: [], error: "" };
        },
      };
    },
  };
  const plain = { ...commit(), agents: { sessions: [], error: "" } };
  expect(
    (await filterNativeActivity([associated, plain], "all")).rows,
  ).toHaveLength(2);
  expect(reads).toBe(0);
  const result = await filterNativeActivity([associated, plain], "agent");
  expect(result.rows).toHaveLength(1);
  expect(result.rows[0]!.agents.relatedSessionIds).toHaveLength(1);
  expect(bodies).toBe(0);
  expect(
    (await filterNativeActivity([associated, plain], "unassisted")).rows,
  ).toEqual([plain]);
});
test("association classification bounds readers and excludes unknown facts from both categories", async () => {
  let active = 0,
    maximum = 0;
  const rows: NativeActivityCommit[] = Array.from(
    { length: 5 },
    (_, index) => ({
      ...commit(),
      agents: {
        sessions: [],
        error: "",
        discover: async () => {
          active++;
          maximum = Math.max(maximum, active);
          await new Promise((resolve) => setTimeout(resolve, 1));
          active--;
          if (index === 0) throw new Error("Denied");
          return { sessions: [], error: index === 1 ? "Unavailable" : "" };
        },
      },
    }),
  );
  for (const filter of ["agent", "unassisted"] as const) {
    const result = await filterNativeActivity(rows, filter);
    expect(result.unavailable).toBe(2);
    expect(result.rows).toHaveLength(filter === "agent" ? 0 : 3);
  }
  expect(maximum).toBe(2);
});
test("cancelled activity classification stops queued reads and discards delayed results", async () => {
  let current = true,
    reads = 0;
  const rows = Array.from({ length: 6 }, () => ({
    ...commit(),
    agents: {
      sessions: [],
      error: "",
      discover: async () => {
        reads++;
        current = false;
        return {
          sessions: [],
          error: "",
          relatedSessionIds: ["22".repeat(16)],
        };
      },
    },
  }));
  const result = await filterNativeActivity(rows, "agent", () => current);
  expect(result.rows).toHaveLength(0);
  expect(reads).toBeLessThanOrEqual(2);
});
