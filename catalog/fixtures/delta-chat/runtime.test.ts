import { expect, test } from "vitest";
import { DemoRuntime } from "./runtime.svelte.js";
import { DemoNextMessage, demoSender } from "./next-message.svelte.js";
import { ChatBodyComments } from "../../../web/src/lib/ui/store-agent-chat-body/comments.svelte.js";
import type { ChatBodyBlock } from "../../../web/src/lib/ui/store-agent-chat-body/model.js";

test("simulated replies retain the harness and model selected at delivery", async () => {
  const runtime = new DemoRuntime();
  const next = new DemoNextMessage(() => runtime.snapshot());
  const block: ChatBodyBlock = {
    id: "block",
    revision: "1",
    label: "Source",
    kind: "markdown",
    text: "Keep this source.",
  };
  const state = new ChatBodyComments(() => ({
    items: [
      { id: "source", kind: "message", label: "Source", blocks: [block] },
    ],
    author: demoSender,
    onSend: (submission) => next.accept(submission),
  }));
  runtime.agentId = "demo-opencode";
  runtime.model = "gpt-6-astra";
  runtime.thinking = "high";
  state.open(block, { from: 0, to: 4 }, "Please review");
  expect(await state.send()).toBe(true);
  runtime.agentId = "demo-codex";
  runtime.model = "gpt-6.1-sol";
  next.completeReplies(state);
  expect(next.messages[0].turn).toMatchObject({
    agentName: "OpenCode",
    modelLabel: "gpt-6-astra",
  });
  expect(
    next.messages.find((item) => item.role === "assistant")?.author,
  ).toMatchObject({ name: "OpenCode", avatarUrl: "/agent-icons/opencode.svg" });
});
