import type { AgentEvent } from "../../web/src/lib/controller/agent-execution-types.js";
export function eventsResponse(
  permission: boolean,
  retainedError = false,
  longHistory = false,
): {
  version: number;
  next: null;
  events: AgentEvent[];
} {
  return {
    version: 1,
    next: null,
    events: [
      ...(longHistory
        ? Array.from({ length: 40 }, (_, index) => ({
            sequence: index + 1,
            committed: true,
            payload: {
              type: "activity",
              requestId: `history-${index}`,
              update: {
                sessionUpdate: "agent_message_chunk",
                content: {
                  text: `History message ${index}.\n\nThis retained chat should scroll above the composer without being covered by the input.`,
                },
              },
            },
          }))
        : []),
      {
        sequence: 1,
        committed: true,
        payload: { type: "started", requestId: "55".repeat(16) },
      },
      {
        sequence: 2,
        committed: false,
        payload: permission
          ? {
              type: "permission",
              permissionId: "exact-permission",
              requestId: "55".repeat(16),
              request: {
                toolCall: { title: "Edit source" },
                options: [
                  {
                    optionId: "allow-exact",
                    kind: "allow_once",
                    name: "Allow once",
                  },
                  {
                    optionId: "reject-exact",
                    kind: "reject_once",
                    name: "Reject once",
                  },
                ],
              },
            }
          : {
              type: "activity",
              requestId: "55".repeat(16),
              update: {
                sessionUpdate: "agent_message_chunk",
                content: { text: "Ready to help." },
              },
            },
      },
      ...(retainedError
        ? [
            {
              sequence: 3,
              committed: false,
              payload: {
                type: "error",
                requestId: "open-session",
                code: "agent_rejected",
              },
            },
          ]
        : []),
    ],
  };
}
