import { expect, test, vi } from "vitest";
import { useConversationReactions } from "../../src/lib/conversations/reactions.svelte.js";
import type { ThreadsController } from "../../src/lib/controller/thread-types.js";
import type { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type {
  AgentConversationRow,
  ConversationProjection,
} from "../../src/lib/conversations/agent-adapter.js";
test("linked answer reactions use the capture's repository reader and original row identity", async () => {
  const target = {
    type: "agent_message",
    session_id: "session",
    message_id: "message",
    content: { kind: "message", id: "captured" },
  };
  const home = {
    toggle: vi.fn(),
    reactionModels: vi.fn(),
  } as unknown as ThreadsController;
  const remote = {
    toggle: vi.fn(),
    reactionModels: vi.fn(() => [{ key: "reaction" }]),
  } as unknown as ThreadsController;
  const message = vi.fn(() => ({ target })),
    reader = vi.fn(() => remote);
  const manager = {
    discussions: { message, reader },
  } as unknown as AgentExecution;
  const projection = {
    items: [{ id: "answer:exact", blocks: [] }],
    targets: new Map([
      ["answer:exact", { sessionId: "session", rowId: "captured-row" }],
    ]),
  } as unknown as ConversationProjection;
  const rows = [
    { session: { id: "session", repository: "capture-home" } },
  ] as unknown as AgentConversationRow[];
  const feedback = useConversationReactions(() => ({
    threads: home,
    targets: new Map(),
    projection,
    rows,
    manager,
  }));
  expect(feedback.models["answer:exact"]).toEqual([{ key: "reaction" }]);
  const emoji = { shortcode: "custom", url: "/emoji" };
  await feedback.toggle("answer:exact", "id", emoji);
  expect(message).toHaveBeenCalledWith("session", "captured-row");
  expect(reader).toHaveBeenCalledWith("capture-home");
  expect(remote.toggle).toHaveBeenCalledWith(target, "id", emoji);
  expect(home.toggle).not.toHaveBeenCalled();
});
