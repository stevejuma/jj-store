import { expect, test } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type {
  AgentEvent,
  AgentSession,
} from "../../src/lib/controller/agent-execution-types.js";
import type { ChatPromptSubmission } from "../../src/lib/ui/store-agent-chat-body/model.js";
import { agentMentionToken } from "../../src/lib/community/agent-mentions.js";
import {
  authoredPrompt,
  promptDelivery,
} from "../../src/lib/conversations/prompt-delivery.js";

const sessions = ["first", "second"].map(
  (id) => ({ id, title: id }) as AgentSession,
);
const event = (
  sequence: number,
  request: string,
  type: string,
  delivery?: string,
): AgentEvent => ({
  sequence,
  observed_at_millis: null,
  committed: false,
  payload: { requestId: request, type, sentContext: { version: 1, delivery } },
});
const submission = (id: string): ChatPromptSubmission => ({
  id,
  text: "Captured text",
  comments: [
    {
      threadId: "thread",
      commentId: "comment",
      text: "Captured review",
      author: { id: "human", name: "You", initials: "YO", handle: "" },
      anchor: {
        blockId: "source",
        revision: "revision",
        from: 0,
        to: 4,
        quote: "Code",
      },
    },
  ],
});
const receipt = (id: string): Model<"AgentConversationOutcome"> => ({
  request_id: id,
  conversation_id: "conversation",
  comment_publication: null,
  agents: sessions.map((s, i) => ({
    version: 1,
    request_id: `${id}-${i}`,
    session_id: s.id,
    state: "queued",
    error: null,
  })),
});

test("queue captures immutable reviews and advances recipients independently across views", () => {
  const manager = new AgentExecution(() => true);
  try {
    const captured = submission("one");
    const result = manager.promptQueue.submitted(captured, receipt("one"));
    expect(result.publication).toBe("queued");
    expect(result.recoveryRequired).toBe(false);
    captured.text = "Next draft";
    captured.comments[0].text = "Edited draft";
    manager.promptQueue.submitted(submission("two"), receipt("two"));
    const entries = manager.promptQueue.entries(sessions);
    expect(entries.map((e) => e.submission.id)).toEqual(["one", "two"]);
    expect(entries[0].text).toBe("Captured text");
    expect(entries[0].submission.comments[0].text).toBe("Captured review");
    manager.chat.events.first = [event(1, "one-0", "started")];
    expect(
      manager.promptQueue.entries(sessions)[0].recipients.map((r) => r.session),
    ).toEqual(["second"]);
    expect(manager.promptQueue.isQueued("first", "one-0")).toBe(false);
    expect(manager.promptQueue.isQueued("second", "one-1")).toBe(true);
    manager.selected = "first";
    manager.events = [event(2, "two-0", "started")];
    // A newer inbox page cannot erase earlier exact start acknowledgements.
    expect(manager.promptQueue.isQueued("first", "one-0")).toBe(false);
    expect(manager.promptQueue.isQueued("first", "two-0")).toBe(false);
  } finally {
    manager.destroy();
  }
});

test("unknown admission never invents a queued message or resubmits it", () => {
  const manager = new AgentExecution(() => true);
  try {
    const unknown = receipt("uncertain");
    unknown.agents[0].state = "dispatching";
    unknown.agents[1].state = "outcome_unknown";
    const result = manager.promptQueue.submitted(
      submission("uncertain"),
      unknown,
    );
    expect(result.recoveryRequired).toBe(true);
    expect(manager.promptQueue.entries(sessions)).toEqual([]);
    manager.chat.events.first = [
      event(1, "uncertain-0", "context_delivery", "queued"),
    ];
    expect(manager.promptQueue.entries(sessions)[0].recipients).toHaveLength(1);
  } finally {
    manager.destroy();
  }
});

test("cancel targets the selected queued recipient and retains the next draft", async () => {
  const commands: unknown[] = [];
  let cancelled = false;
  const manager = new AgentExecution(
    () => true,
    async (url, init) => {
      const path = String(url);
      if (path.endsWith("/command")) {
        const body = JSON.parse(String(init?.body));
        commands.push(body);
        cancelled = true;
        return Response.json({
          version: 1,
          request_id: body.request_id,
          session_id: body.session_id,
          state: "accepted",
          error: null,
        });
      }
      if (path.includes("/outcome?"))
        return Response.json({
          version: 1,
          request_id: "one-1",
          session_id: "second",
          state: cancelled ? "cancelled" : "queued",
          error: null,
        });
      return Response.json({
        version: 1,
        runtime: "ready",
        revision: 0,
        sessions,
        setups: [],
        runners: [],
      });
    },
  );
  try {
    manager.promptQueue.submitted(submission("one"), receipt("one"));
    manager.chat.events.second = [
      event(1, "one-1", "context_delivery", "queued"),
    ];
    manager.drafts.first = "Do not replace this draft";
    await manager.promptQueue.cancel("second", "one-1");
    expect(commands).toMatchObject([
      { session_id: "second", action: { action: "cancel", target: "one-1" } },
    ]);
    expect(
      manager.promptQueue.entries(sessions)[0].recipients.map((r) => r.session),
    ).toEqual(["first"]);
    expect(manager.drafts.first).toBe("Do not replace this draft");
    expect(manager.promptQueue.hidePrompt("second", "one-1")).toBe(true);
    await manager.promptQueue.cancel("second", "one-1");
    expect(commands).toHaveLength(1);
  } finally {
    manager.destroy();
  }
});

test("audience tokens and whitespace cannot replace Stop with an empty send", () => {
  expect(authoredPrompt(` ${agentMentionToken("helper")} \n `)).toBe("");
  expect(authoredPrompt(`${agentMentionToken("helper")} Do this next`)).toBe(
    "Do this next",
  );
});
test.each(["started", "outcome_unknown"])(
  "a %s receipt is checked without cancelling or replaying the former queued request",
  async (state) => {
    const calls: string[] = [];
    const manager = new AgentExecution(
      () => true,
      async (url) => {
        calls.push(String(url));
        return Response.json({
          version: 1,
          request_id: "one-0",
          session_id: "first",
          state,
          error: null,
        });
      },
    );
    try {
      manager.promptQueue.submitted(submission("one"), receipt("one"));
      await manager.promptQueue.cancel("first", "one-0");
      expect(calls).toEqual(["/v1/local/agents/outcome?request=one-0"]);
      expect(
        manager.promptQueue
          .entries(sessions)[0]
          .recipients.map((r) => r.session),
      ).toEqual(state === "started" ? ["second"] : ["first", "second"]);
    } finally {
      manager.destroy();
    }
  },
);
test("queue-full is a proven refusal and retains its captured submission", () => {
  const manager = new AgentExecution(() => true);
  try {
    const full = receipt("full");
    full.agents = full.agents.map((a) => ({
      ...a,
      state: "failed",
      error: "queue_full",
    }));
    expect(
      manager.promptQueue.submitted(submission("full"), full).publication,
    ).toBe("refused");
    expect(manager.promptQueue.entries(sessions)).toEqual([]);
    expect(manager.promptQueue.captured.full.submission.text).toBe(
      "Captured text",
    );
  } finally {
    manager.destroy();
  }
});

test("later exact start releases a queue after an unknown status read", async () => {
  const manager = new AgentExecution(
    () => true,
    async () =>
      Response.json({
        version: 1,
        request_id: "one-0",
        session_id: "first",
        state: "outcome_unknown",
        error: null,
      }),
  );
  try {
    manager.promptQueue.submitted(submission("one"), receipt("one"));
    manager.chat.events.first = [
      event(1, "one-0", "context_delivery", "queued"),
    ];
    await manager.promptQueue.review("first", "one-0");
    expect(manager.promptQueue.isPending("first", "one-0")).toBe(true);
    manager.chat.events.first = [
      ...manager.chat.events.first,
      event(2, "one-0", "started"),
    ];
    expect(manager.promptQueue.hidePrompt("first", "one-0")).toBe(false);
    expect(
      manager.promptQueue.entries(sessions)[0].recipients.map((r) => r.session),
    ).toEqual(["second"]);
  } finally {
    manager.destroy();
  }
});

test("late context acknowledgements cannot revive a started or completed request", () => {
  const started = [
    event(1, "one", "started"),
    event(2, "one", "context_delivery", "queued"),
  ];
  expect(promptDelivery(started, "one")).toBe("started");
  expect(
    promptDelivery(
      [
        ...started,
        event(3, "one", "record"),
        event(4, "one", "context_delivery", "sent"),
      ],
      "one",
    ),
  ).toBe("record");
});
