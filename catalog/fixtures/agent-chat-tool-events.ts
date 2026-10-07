import type { AgentEvent } from "../../web/src/lib/controller/agent-execution-types.js";
/** Optional shared-row regression data; default execution fixtures stay unchanged. */
export function appendChatToolEvents(events: AgentEvent[], grouped = false) {
  events.unshift({
    sequence: 0,
    committed: true,
    observed_at_millis: 1791323999000,
    payload: { type: "opened", requestId: "open-first" },
  });
  events.splice(4, 0, {
    sequence: 7,
    committed: true,
    observed_at_millis: 1791324002500,
    payload: {
      type: "activity",
      requestId: "55".repeat(16),
      toolSnapshot: {
        toolCallId: "read",
        title: "Read captured.ts",
        kind: "read",
        status: "completed",
        locations: [{ path: "src/captured.ts" }],
        rawOutput: {
          formatted_output: "export const captured = 42;",
        },
      },
    },
  });
  if (grouped)
    events.splice(5, 0, {
      sequence: 8,
      committed: true,
      observed_at_millis: 1791324002600,
      payload: {
        type: "activity",
        requestId: "55".repeat(16),
        toolSnapshot: {
          toolCallId: "edit",
          title: "Edit captured.ts",
          kind: "edit",
          status: "completed",
          content: [
            {
              type: "diff",
              path: "src/captured.ts",
              oldText: "export const captured = 41;",
              newText: "export const captured = 42;",
            },
          ],
        },
      },
    });
  events[1]!.payload = {
    ...(events[1]!.payload as object),
    context:
      "<agent_context>\n<repository_context>\n```text\nPinned repository facts\n```\n</repository_context>\n</agent_context>",
  };
  events.forEach((event, index) => (event.sequence = index + 1));
}
