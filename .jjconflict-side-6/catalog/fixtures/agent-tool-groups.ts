import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";
import { agentDetail } from "./collaboration.js";
type Activity = NonNullable<
  StoreControllerState["agentSession"]
>["activity"][number];
type Tool = Extract<Activity, { type: "tool" }>["tool"];

const original = agentDetail.activity.find((entry) => entry.type === "tool")!;
const tool = (byte: string, name: string, state: Tool["state"]): Tool => ({
  ...original.tool,
  tool_call_id: byte.repeat(16),
  snapshot_id: byte.repeat(32),
  native_name: name,
  kind: "execute",
  state,
  input: { media_type: "text", content: "cargo test" },
  output: { media_type: "text", content: "Permission denied by the fixture" },
  code: [],
  observed_paths: [],
});

export const groupedAgentActivity: readonly Activity[] = [
  agentDetail.activity[0]!,
  {
    ...original,
    tool: { ...original.tool, state: "unknown", duration_millis: 1250 },
  },
  {
    type: "tool",
    sequence: 3,
    event_id: "e1".repeat(16),
    tool: { ...tool("e1", "Bash", "failed"), duration_millis: 250 },
  },
  {
    type: "tool",
    sequence: 4,
    event_id: "e2".repeat(16),
    tool: { ...tool("e2", "Read", "succeeded"), duration_millis: 0 },
  },
  { ...agentDetail.activity[2]!, sequence: 5 },
  {
    type: "tool",
    sequence: 6,
    event_id: "e3".repeat(16),
    tool: tool("e3", "Search", "running"),
  },
];
