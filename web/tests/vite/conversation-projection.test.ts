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

test("committed comments-only user rows retain their canonical batch and turn", () => {
  const input = {
    ...row(""),
    kind: "user" as const,
    value: { sentContext: { commentSubmission: { request_id: "batch" } } },
  };
  const p = agentConversationProjection(
    [input],
    { controller: feedback("", [], "/text") },
    user,
  );
  assert.equal(p.items.length, 1);
  assert.equal(p.items[0].id, input.key);
  assert.equal(p.items[0].commentBatchId, "batch");
  assert.deepEqual(p.items[0].blocks, []);
});

test("execute output uses committed stdout parts instead of the terminal snapshot envelope", () => {
  const output = 'README contents\nexport const greeting = "Hello 🦀";\n';
  const captured = feedback(output).messages[0];
  captured.text_parts = [
    { part: "/content/0/terminalId", text: "exec-terminal" },
    { part: "/rawOutput/formatted_output", text: output },
  ];
  const value = {
    kind: "execute",
    status: "completed",
    rawInput: { command: "cat README.md src/example.ts" },
    rawOutput: { formatted_output: output, exit_code: 0 },
    content: [{ type: "terminal", terminalId: "exec-terminal" }],
    _meta: { terminal_exit: { exit_code: 0, terminal_id: "exec-terminal" } },
  };
  const before = structuredClone(value);
  const sources = new Map<string, ConversationTextSource>();
  const projected = capturedToolProjection(
    { ...row("Execute"), kind: "tool", value },
    "exec",
    "rev",
    captured,
    sources,
  );
  assert.equal(projected.blocks.length, 1);
  const block = projected.blocks[0];
  assert.equal(block.text, output);
  assert.equal(block.wholeSection, undefined);
  assert.equal(block.label, "Output");
  assert.equal(sources.get(block.id)?.part, "/rawOutput/formatted_output");
  const quote = "Hello 🦀",
    from = output.indexOf(quote);
  const source = exactSource(
    block,
    {
      blockId: block.id,
      revision: "rev",
      from,
      to: from + quote.length,
      quote,
    },
    sources.get(block.id)!,
  );
  assert.equal(source.part, "/rawOutput/formatted_output");
  assert.equal(source.quote, quote);
  assert.deepEqual(value, before);
});

test("execute output aliases preserve empty and stderr captures without fabricating terminal text", () => {
  for (const key of ["formatted_output", "aggregated_output", "stdout"]) {
    const captured = feedback("").messages[0];
    captured.text_parts = [
      { part: `/rawOutput/${key}`, text: "" },
      { part: "/rawOutput/stderr", text: "warning\n" },
    ];
    const sources = new Map<string, ConversationTextSource>();
    const projected = capturedToolProjection(
      { ...row("Execute"), kind: "tool", value: { kind: "execute" } },
      key,
      "rev",
      captured,
      sources,
    );
    assert.deepEqual(
      projected.blocks.map((b) => [b.label, b.text]),
      [
        ["Output", ""],
        ["Error output", "warning\n"],
      ],
    );
  }
  const captured = feedback("").messages[0];
  captured.text_parts = [
    { part: "/content/0/terminalId", text: "unknown-terminal" },
  ];
  const projected = capturedToolProjection(
    {
      ...row("Execute"),
      kind: "tool",
      value: {
        kind: "execute",
        content: [{ type: "terminal", terminalId: "unknown-terminal" }],
      },
    },
    "unknown",
    "rev",
    captured,
    new Map(),
  );
  assert.equal(projected.blocks[0].wholeSection, true);
  assert.match(projected.blocks[0].text, /unknown-terminal/);
});

test("terminal-only committed captures show retained execution output with whole-section comments", () => {
  const captured = feedback("").messages[0];
  captured.text_parts = [
    { part: "/0/type", text: "terminal" },
    { part: "/0/terminalId", text: "exec-terminal" },
  ];
  const value = {
    kind: "execute",
    rawInput: { command: "cat README.md" },
    rawOutput: { formatted_output: "Retained README 🦀\n", exit_code: 0 },
    content: [{ type: "terminal", terminalId: "exec-terminal" }],
  };
  const sources = new Map<string, ConversationTextSource>();
  const projected = capturedToolProjection(
    { ...row("Execute"), kind: "tool", value },
    "terminal",
    "rev",
    captured,
    sources,
  );
  const block = projected.blocks[0];
  assert.equal(block.label, "Shell");
  assert.equal(block.text, "$ cat README.md\nRetained README 🦀\n");
  assert.equal(block.wholeSection, true);
  assert.equal(sources.get(block.id)?.wholeSection, true);
  const source = exactSource(
    block,
    {
      blockId: block.id,
      revision: "rev",
      from: 0,
      to: block.text.length,
      quote: block.text,
    },
    sources.get(block.id)!,
  );
  assert.equal(source.selection, null);
  assert.equal(source.part, null);
  assert.equal(source.quote, "");
});

test("recorded tools retain command and output without writable capture targets", () => {
  const activity: Model<"AgentActivity">[] = [
    {
      type: "tool",
      sequence: 1,
      event_id: hex("7"),
      tool: {
        tool_call_id: hex("8"),
        snapshot_id: hex("9", 64),
        turn_id: null,
        native_name: "exec_command",
        kind: "execute",
        state: "succeeded",
        input: { media_type: "json", content: '{"command":"cat README.md"}' },
        output: { media_type: "text", content: "Retained README 🦀" },
        observed_paths: [],
        code: [],
      },
    },
  ];
  const before = structuredClone(activity);
  const projection = recordedConversationProjection(
    activity,
    hex("6"),
    "codex",
    user,
    user,
  );
  const tool = projection.items[0].tools![0];
  assert.match(tool.blocks[0].text, /cat README.md/);
  assert.match(tool.blocks[0].text, /Retained README 🦀/);
  assert.equal(tool.raw.rawInput, activity[0].tool.input!.content);
  assert.equal(tool.raw.rawOutput, activity[0].tool.output!.content);
  assert.equal(projection.sources.size, 0);
  assert.equal(tool.blocks[0].commentable, false);
  assert.deepEqual(activity, before);
});
