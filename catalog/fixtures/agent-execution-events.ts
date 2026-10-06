import type { AgentEvent } from "../../web/src/lib/controller/agent-execution-types.js";
export function eventsResponse(
  permission: boolean,
  retainedError = false,
  longHistory = false,
  feedback?: "completed" | "working",
): {
  version: number;
  next: null;
  events: AgentEvent[];
} {
  if (feedback)
    return { version: 1, next: null, events: feedbackEvents(feedback) };
  return {
    version: 1,
    next: null,
    events: [
      ...(longHistory
        ? Array.from({ length: 40 }, (_, index) => ({
            sequence: index + 1,
            observed_at_millis: null,
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
        observed_at_millis: null,
        committed: true,
        payload: { type: "started", requestId: "55".repeat(16) },
      },
      {
        sequence: 2,
        observed_at_millis: null,
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
              observed_at_millis: null,
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

function feedbackEvents(feedback: "completed" | "working"): AgentEvent[] {
  const requestId = "55".repeat(16);
  return [
    {
      sequence: 1,
      observed_at_millis: 1791324000000,
      committed: true,
      payload: { type: "prompt", requestId, text: "Show the retained time" },
    },
    {
      sequence: 2,
      observed_at_millis: 1791324001000,
      committed: true,
      payload: {
        type: "activity",
        requestId,
        update: {
          sessionUpdate: "agent_message_chunk",
          content: { text: "Ready " },
        },
      },
    },
    {
      sequence: 3,
      observed_at_millis: 1791324002000,
      committed: true,
      payload: {
        type: "activity",
        requestId,
        update: {
          sessionUpdate: "agent_message_chunk",
          content: { text: "to help." },
        },
      },
    },
    {
      sequence: 4,
      observed_at_millis: null,
      committed: true,
      payload: {
        type: "activity",
        requestId: "legacy",
        update: {
          sessionUpdate: "agent_message_chunk",
          content: { text: "Legacy message without timing." },
        },
      },
    },
    {
      sequence: 5,
      observed_at_millis: null,
      committed: true,
      payload: { type: "started", requestId },
    },
    ...(feedback === "completed"
      ? [
          {
            sequence: 6,
            observed_at_millis: 1791324003000,
            committed: true,
            payload: {
              type: "record",
              requestId,
              record: { status: "completed" },
            },
          },
        ]
      : []),
  ];
}
