import type {
  ChatBodyItem,
  ChatCommentThread,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import { demoAgent, demoSender } from "./next-message.svelte.js";

/** Synthetic review overlay; the captured Delta transcript remains untouched. */
export function referenceDiscussions(captured: readonly ChatBodyItem[]) {
  const batchId = "demo-reference-review";
  const examples = [
    {
      id: "action",
      blockId: "delta-19/patch/0",
      quote: "case screenBlock",
      comments: [
        "Demo review: keep screen blocking separate from shutdown so the action stays explicit.",
        "Demo review: also keep the existing shutdown default for schedules saved before this option existed.",
      ],
      reply:
        "Simulated reply: the captured patch adds a distinct screenBlock action and retains .shutdown when decoding an older schedule. No agent was contacted.",
    },
    {
      id: "readme",
      blockId: "delta-11/output",
      quote: "Receive configurable warnings (15 and 5 minutes by default).",
      comments: [
        "Demo review: clarify that the warning countdown applies to both bedtime actions. Please add an example to the README.",
      ],
    },
    {
      id: "permissions",
      blockId: "assistant-63803/text/0",
      quote: "Screen-only mode does **not** require Automation permission.",
      comments: [
        "Demo review: keep this permission difference visible in the settings explanation as well as the summary.",
      ],
      reply:
        "Simulated reply: this discussion is linked to the permission note above. The settings wording would be the next follow-up; this example does not change the app.",
    },
  ];
  const blocks = captured.flatMap((item) => [
    ...item.blocks,
    ...(item.tools ?? []).flatMap((tool) => tool.blocks),
  ]);
  const threads: ChatCommentThread[] = examples.map((example) => {
    const block = blocks.find((block) => block.id === example.blockId);
    const from = block?.text.indexOf(example.quote) ?? -1;
    if (!block || from < 0 || block.text.indexOf(example.quote, from + 1) >= 0)
      throw new Error(`Demo discussion source changed: ${example.id}`);
    const id = `demo-review:${example.id}`;
    const replyId = `${id}/reply`;
    const comments: ChatCommentThread["comments"] = example.comments.map(
      (text, index) => ({
        id: `${id}/comment/${index}`,
        author: demoSender,
        text,
        batchId,
        status: example.reply ? "replied" : "sent",
        replyItemId: example.reply ? replyId : undefined,
      }),
    );
    if (example.reply)
      comments.push({
        id: replyId,
        author: demoAgent,
        text: example.reply,
        status: "sent",
        replyTo: comments[0].id,
      });
    return {
      id,
      label: block.label,
      sourceKind: block.kind,
      anchor: {
        blockId: block.id,
        revision: block.revision,
        from,
        to: from + example.quote.length,
        quote: example.quote,
      },
      comments,
    };
  });
  const turn = {
    id: batchId,
    author: demoSender.name,
    agentName: demoAgent.name,
    prompt: "Demo review comments",
    summary:
      "Simulated follow-up on the action, documentation and permissions.",
  };
  const history: ChatBodyItem[] = [
    {
      id: batchId,
      kind: "message",
      role: "user",
      author: demoSender,
      label: "Demo review comments from You",
      commentBatchId: batchId,
      blocks: [],
      turn,
    },
    ...threads.flatMap((thread) =>
      thread.comments
        .filter((comment) => comment.replyTo)
        .map((reply) => ({
          id: reply.id,
          kind: "message" as const,
          role: "assistant" as const,
          author: demoAgent,
          label: "Simulated review reply",
          replyContext: {
            threadId: thread.id,
            commentIds: thread.comments
              .filter((c) => !c.replyTo)
              .map((c) => c.id),
          },
          blocks: [
            {
              id: `${reply.id}/text`,
              revision: "demo-review-1",
              kind: "markdown" as const,
              label: "Simulated review reply",
              text: reply.text,
            },
          ],
          turn,
        })),
    ),
  ];
  return { items: [...captured, ...history], threads };
}
