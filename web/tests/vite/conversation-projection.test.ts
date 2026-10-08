import assert from "node:assert/strict";
import { test } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import { agentConversationProjection } from "../../src/lib/conversations/agent-adapter.js";
import { recordedConversationProjection } from "../../src/lib/conversations/recorded-adapter.js";
import { capturedToolProjection } from "../../src/lib/conversations/tool-adapter.js";
import {
  mergeReplies,
  answerId,
} from "../../src/lib/conversations/discussions.js";
import {
  exactSource,
  type ConversationTextSource,
} from "../../src/lib/conversations/source-map.js";
import type { ActivityRow } from "../../src/lib/ui/store-agent-execution/presentation.js";
const hex = (c: string, n = 32) => c.repeat(n);
const user = { id: hex("1"), name: "You", handle: "", initials: "YO" };
const target = (
  c: string,
): Extract<Model<"ReactionTargetV1">, { type: "agent_message" }> => ({
  type: "agent_message",
  session_id: hex(c),
  message_id: hex("2"),
  event_id: hex("3"),
  batch: { kind: "event_batch", id: hex("4", 64) },
  content: { kind: "content", id: hex(c, 64) },
});
const human = (
  c: string,
): Extract<Model<"ReactionTargetV1">, { type: "human_message" }> => ({
  type: "human_message",
  thread_id: hex("a"),
  message_id: hex(c),
  revision: { kind: "human_message_revision", id: hex(c, 64) },
});
function selector(
  text: string,
  quote: string,
  t = target("5"),
  part: string | null = null,
): Model<"ConversationSourceV1"> {
  const block = {
    id: "b",
    revision: t.content.id,
    kind: "markdown" as const,
    text,
    label: "Message",
  };
  const from = text.indexOf(quote);
  return exactSource(
    block,
    {
      blockId: "b",
      revision: block.revision,
      from,
      to: from + quote.length,
      quote,
    },
    { target: t, part, text, from: 0, revision: block.revision },
  );
}
const session = {
  id: "controller",
  setup_id: null,
  title: "Helper",
  avatar: "",
};
function row(
  text: string,
): ActivityRow & { session: typeof session; key: string } {
  return {
    id: "r/assistant",
    key: "controller:r/assistant",
    session,
    kind: "assistant",
    request: "r",
    sequence: 1,
    sourceFrom: 0,
    text,
    value: {},
  };
}
function feedback(
  text: string,
  answers: Model<"CommentReplyLinkV1">[] = [],
  part: string | null = null,
): Model<"AgentExecutionFeedback"> {
  return {
    messages: [
      {
        row_id: "r/assistant",
        target: target("5"),
        threads: [],
        replies: [],
        text_parts: [{ text, part }],
        answers,
      },
    ],
    profile: null,
    write_unavailable: null,
  };
}
test("several validated answers retain ordinary prose in one turn and shared comments use one destination", () => {
  const text =
    'Intro 🦀.\n<comment-reply discussion="D1" comments="C1 C2">Shared answer.</comment-reply>\nBetween.\n<comment-reply discussion="D2" comments="C3">Second answer.</comment-reply>\nAfter.';
  const answers: Model<"CommentReplyLinkV1">[] = [
    {
      prompt: target("6"),
      answer: selector(text, "Shared answer."),
      comments: [human("b"), human("c")],
    },
    {
      prompt: target("6"),
      answer: selector(text, "Second answer."),
      comments: [human("d")],
    },
  ];
  const p = agentConversationProjection(
    [row(text)],
    { controller: feedback(text, answers) },
    user,
  );
  assert.equal(new Set(p.items.map((i) => i.turn?.id)).size, 1);
  assert.equal(p.items.filter((i) => i.replyContext).length, 2);
  assert.deepEqual(
    p.items.find((i) => i.id === answerId(answers[0]))?.replyContext
      ?.commentIds,
    [`${hex("b")}:${hex("b", 64)}`, `${hex("c")}:${hex("c", 64)}`],
  );
  assert.equal(
    p.items
      .flatMap((i) => i.blocks)
      .map((b) => b.text)
      .join("\n"),
    "Intro 🦀.\nShared answer.\nBetween.\nSecond answer.\nAfter.",
  );
  for (const block of p.items.flatMap((i) => i.blocks))
    assert.equal(
      p.sources
        .get(block.id)!
        .text.slice(
          p.sources.get(block.id)!.from,
          p.sources.get(block.id)!.from + block.text.length,
        ),
      block.text,
    );
});
test("foreign or malformed unvalidated tags stay readable and changed captures disable selection", () => {
  const text =
    '<comment-reply discussion="foreign" comments="invented">Visible</comment-reply>';
  const p = agentConversationProjection(
    [row(text)],
    { controller: feedback(text) },
    user,
  );
  assert.equal(p.items[0].blocks[0].text, text);
  assert.equal(p.items[0].replyContext, undefined);
  const changed = agentConversationProjection(
    [row(text)],
    { controller: feedback("Different") },
    user,
  );
  assert.equal(changed.sources.size, 0);
  assert.equal(changed.items[0].blocks[0].commentable, false);
});
test("multiple agents merge immutable comment revisions while preserving separate answer links", () => {
  const ref = human("b"),
    source = selector("Hello", "Hello");
  const reply: Model<"AgentExecutionFeedbackReply"> = {
    thread_id: ref.thread_id,
    thread_revision: null,
    revision: ref.revision,
    source,
    historical: false,
    batches: [hex("e")],
    message: {
      schema_version: 1,
      message_id: ref.message_id,
      thread_id: ref.thread_id,
      actor: user.id,
      accepted_at_millis: 1,
      previous: null,
      body: "Question",
    },
    answers: [
      { prompt: target("6"), answer: selector("One", "One"), comments: [ref] },
    ],
  };
  const other = {
    ...reply,
    batches: [hex("f")],
    answers: [
      {
        prompt: target("7"),
        answer: selector("Two", "Two", target("8")),
        comments: [ref],
      },
    ],
  };
  const merged = mergeReplies([reply, other, reply]);
  assert.equal(merged.length, 1);
  assert.equal(merged[0].answers.length, 2);
  assert.equal(merged[0].batches.length, 2);
  const edited = { ...reply, revision: human("c").revision, answers: [] };
  assert.equal(mergeReplies([reply, edited]).length, 2);
});
test("patch file and numbered-read slices preserve exact captured pointers and Unicode offsets", () => {
  const text =
    "*** Begin Patch\n*** Update File: a.ts\n@@\n-old\n+🦀\n*** Add File: b.ts\n+é\n*** End Patch";
  const captured = feedback(text).messages[0];
  captured.text_parts[0].part = "/patch";
  const sources = new Map<string, ConversationTextSource>();
  const tool = {
    ...row("Edit"),
    kind: "tool" as const,
    value: { kind: "edit", title: "Edit" },
  };
  const p = capturedToolProjection(tool, "tool", "rev", captured, sources);
  assert.equal(p.blocks.length, 2);
  assert.deepEqual(
    p.blocks.map((b) => b.path),
    ["a.ts", "b.ts"],
  );
  for (const block of p.blocks) {
    const source = exactSource(
      block,
      {
        blockId: block.id,
        revision: block.revision,
        from: 0,
        to: block.text.length,
        quote: block.text,
      },
      sources.get(block.id)!,
    );
    assert.equal(source.part, "/patch");
    assert.equal(
      new TextDecoder().decode(
        new TextEncoder()
          .encode(text)
          .slice(
            Number(source.selection!.start_byte),
            Number(source.selection!.end_byte),
          ),
      ),
      block.text,
    );
  }
  captured.text_parts = [
    { part: "/text", text: "1\tconst 🦀 = 1;\n2\treturn 🦀;" },
  ];
  const read = capturedToolProjection(
    { ...tool, value: { kind: "read" } },
    "read",
    "rev",
    captured,
    sources,
  );
  assert.equal(read.blocks[0].lineNumbered, true);
});
test("recorded rich captures without a plain-text mapping support whole sections without inventing ranges", () => {
  const t = target("5");
  const p = recordedConversationProjection(
    [
      {
        type: "message",
        sequence: 1,
        event_id: t.event_id,
        message_id: t.message_id,
        turn_id: null,
        role: "assistant",
        content: {
          media_type: "json",
          content: '{"text":"Hello"}',
          sanitizer_version: "test",
        } as Model<"AgentContent">,
        reaction_target: t,
      },
    ],
    t.session_id,
    "codex",
    { ...user, name: "Helper" },
    user,
  );
  const block = p.items[0].blocks[0];
  assert.equal(block.wholeSection, true);
  const source = exactSource(
    block,
    {
      blockId: block.id,
      revision: block.revision,
      from: 0,
      to: block.text.length,
      quote: block.text,
    },
    p.sources.get(block.id)!,
  );
  assert.equal(source.selection, null);
});
test("shared answers repeated in feedback and spanning tool boundaries have stable unique fragments", () => {
  const text =
    '<comment-reply discussion="D1" comments="C1 C2">First half. Second half.</comment-reply>';
  const answer: Model<"CommentReplyLinkV1"> = {
    prompt: target("6"),
    answer: selector(text, "First half. Second half."),
    comments: [human("b"), human("c")],
  };
  const split = text.indexOf("Second");
  const first = { ...row(text.slice(0, split)), sourceRowId: "r/assistant" };
  const second = {
    ...row(text.slice(split)),
    id: "r/assistant/segment/3",
    key: "controller:r/assistant/segment/3",
    sourceRowId: "r/assistant",
    sourceFrom: split,
    sequence: 3,
  };
  const tool = {
    ...row("Read"),
    id: "r/tool",
    key: "controller:r/tool",
    kind: "tool" as const,
    sequence: 2,
    value: { title: "Read", kind: "read" },
  };
  const p = agentConversationProjection(
    [first, tool, second],
    { controller: feedback(text, [answer, answer]) },
    user,
  );
  assert.equal(p.items.filter((i) => i.replyContext).length, 2);
  assert.equal(new Set(p.items.map((i) => i.id)).size, p.items.length);
  assert.equal(p.items[0].id, answerId(answer));
  assert.equal(p.items[1].kind, "tools");
  assert.match(p.items[2].id, /:fragment:/);
  assert.deepEqual(
    p.items.flatMap((i) => i.blocks).map((b) => b.text),
    ["First half.", "Second half."],
  );
});
