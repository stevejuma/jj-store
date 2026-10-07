import { expect, test } from "vitest";
import {
  groupActivityRows,
  type ScopedActivityRow,
} from "../../src/lib/ui/store-agent-execution/activity-row-groups.js";
import { activityDayDividers } from "../../src/lib/ui/store-agent-execution/presentation.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";

const row = (
  id: string,
  kind: ScopedActivityRow["kind"] = "tool",
  request = "turn",
  session = "session",
  observedAtMillis = 1000,
): ScopedActivityRow => ({
  id,
  key: `${session}:${id}`,
  kind,
  request,
  text: id,
  value: { status: "completed" },
  sequence: 1,
  observedAtMillis,
  session: { id: session } as AgentSession,
});
const group = (rows: ScopedActivityRow[]) =>
  groupActivityRows(
    rows,
    activityDayDividers(rows.map((r) => ({ ...r, id: r.key }))),
  );

test("adjacent tools form one stable group without replacing or mutating exact rows", () => {
  const first = row("read"),
    second = row("edit");
  const source = [first, second];
  const groups = group(source);
  expect(groups).toHaveLength(1);
  expect(groups[0].kind).toBe("tools");
  expect(groups[0].items).toEqual(source);
  expect(groups[0].items[0]).toBe(first);
  expect(groups[0].key).toBe(group([first])[0].key);
  expect(source).toEqual([first, second]);
  const update = { ...second, value: { status: "failed" } };
  expect(group([first, update])[0].key).toBe(groups[0].key);
  expect(group([first, update])[0].items[1].value.status).toBe("failed");
});

test.each([
  "user",
  "assistant",
  "plan",
  "permission",
  "record",
  "session",
] as const)("%s splits consecutive tool groups", (kind) => {
  const groups = group([row("read"), row("boundary", kind), row("edit")]);
  expect(groups.map((g) => g.kind)).toEqual(["tools", "row", "tools"]);
  expect(
    groups.filter((g) => g.kind === "tools").map((g) => g.items.length),
  ).toEqual([1, 1]);
});

test("session, turn and recorded day boundaries remain distinct", () => {
  for (const second of [
    row("edit", "tool", "next"),
    row("edit", "tool", "turn", "other"),
    row("edit", "tool", "turn", "session", 86401000),
  ]) {
    expect(group([row("read"), second])).toHaveLength(2);
  }
  expect(group([])).toEqual([]);
});
