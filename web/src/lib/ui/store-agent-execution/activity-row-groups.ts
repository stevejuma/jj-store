// @community-copy src/components/community-agent-activity/model.ts
import type { AgentSession } from "../../controller/agent-execution-types.js";
import type { ActivityRow } from "./presentation.js";

export type ScopedActivityRow = ActivityRow & {
  session: AgentSession;
  key: string;
};
export type ActivityRowGroup = {
  kind: "row" | "tools";
  key: string;
  row: ScopedActivityRow;
  items: ScopedActivityRow[];
};

/** Group adjacent tools without crossing a session, turn or recorded day divider. */
export function groupActivityRows(
  rows: readonly ScopedActivityRow[],
  dayStarts: ReadonlyMap<string, string>,
): ActivityRowGroup[] {
  const groups: ActivityRowGroup[] = [];
  for (const row of rows) {
    const previous = groups.at(-1);
    if (row.kind === "tool") {
      if (
        previous?.kind === "tools" &&
        previous.row.session.id === row.session.id &&
        previous.row.request === row.request &&
        !dayStarts.has(row.key)
      ) {
        previous.items.push(row);
      } else {
        groups.push({
          kind: "tools",
          key: `tools:${row.key}`,
          row,
          items: [row],
        });
      }
    } else {
      groups.push({ kind: "row", key: row.key, row, items: [] });
    }
  }
  return groups;
}
