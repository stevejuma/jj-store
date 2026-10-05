import { describe, expect, test } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import { agentTimeline } from "../../src/lib/collaboration/agent-timeline.js";
import { groupedAgentActivity } from "../../../catalog/fixtures/agent-tool-groups.js";

describe("native agent transcript grouping", () => {
  test("groups adjacent calls in recorded order without changing native records", () => {
    const source = Object.freeze([...groupedAgentActivity]);
    const before = JSON.stringify(source);
    const rows = agentTimeline(source);
    expect(rows.map((row) => row.type)).toEqual([
      "message",
      "tools",
      "message",
      "tools",
    ]);
    expect(rows[0]).toBe(source[0]);
    expect(rows[2]).toBe(source[4]);
    const group = rows[1];
    expect(group?.type).toBe("tools");
    if (group?.type !== "tools")
      throw new Error("Expected adjacent tool group");
    expect(group.items).toEqual(source.slice(1, 4));
    group.items.forEach((entry, index) =>
      expect(entry).toBe(source[index + 1]),
    );
    expect(group.items.map((entry) => entry.tool.tool_call_id)).toEqual([
      "d5".repeat(16),
      "e1".repeat(16),
      "e2".repeat(16),
    ]);
    expect(JSON.stringify(source)).toBe(before);
  });

  test("never groups across capture gaps, permissions or recorded metadata", () => {
    const tool = groupedAgentActivity[1]!;
    const boundaries: Model<"AgentActivity">[] = [
      {
        type: "capture_gap",
        event_id: "f1".repeat(16),
        sequence: 7,
        missing_from: 3,
        missing_to: 6,
      },
      {
        type: "permission",
        event_id: "f2".repeat(16),
        sequence: 8,
        state: "denied",
        tool_call_id: null,
      },
      {
        type: "permission",
        event_id: "f3".repeat(16),
        sequence: 9,
        state: "not_observed",
        tool_call_id: null,
      },
      {
        type: "file_observed",
        event_id: "f4".repeat(16),
        sequence: 10,
        path: "src/lib.rs",
      },
      {
        type: "session_state",
        event_id: "f5".repeat(16),
        sequence: 11,
        state: "ended",
      },
    ];
    for (const boundary of boundaries) {
      const rows = agentTimeline([tool, boundary, groupedAgentActivity[2]!]);
      expect(rows.map((row) => row.type)).toEqual([
        "tools",
        boundary.type,
        "tools",
      ]);
      expect(rows[1]).toBe(boundary);
    }
  });

  test("keeps group identity stable when an adjacent continuation is appended", () => {
    const before = agentTimeline(groupedAgentActivity.slice(0, 3));
    const after = agentTimeline(groupedAgentActivity.slice(0, 4));
    expect(before[1]?.sequence).toBe(after[1]?.sequence);
    expect(before[1]?.sequence).toBe(groupedAgentActivity[1]?.sequence);
    const largeSequence = 9_007_199_254_740_993n;
    expect(
      agentTimeline([
        { ...groupedAgentActivity[1]!, sequence: largeSequence },
      ])[0]?.sequence,
    ).toBe(largeSequence);
    expect(agentTimeline([])).toEqual([]);
  });
});
