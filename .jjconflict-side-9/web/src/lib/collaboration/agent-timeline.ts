// @community-copy src/components/community-agent-activity/model.ts
import type { Model } from "../api/client.js";

type Activity = Model<"AgentActivity">;
type ToolActivity = Extract<Activity, { type: "tool" }>;
export type AgentTimelineEntry =
  | Exclude<Activity, ToolActivity>
  | { type: "tools"; sequence: Activity["sequence"]; items: ToolActivity[] };

/** Group adjacent calls without reordering or changing recorded native entries. */
export function agentTimeline(
  activity: readonly Activity[],
): AgentTimelineEntry[] {
  const timeline: AgentTimelineEntry[] = [];
  for (const entry of activity) {
    if (entry.type !== "tool") {
      timeline.push(entry);
      continue;
    }
    const previous = timeline.at(-1);
    if (previous?.type === "tools") previous.items.push(entry);
    else
      timeline.push({
        type: "tools",
        sequence: entry.sequence,
        items: [entry],
      });
  }
  return timeline;
}
