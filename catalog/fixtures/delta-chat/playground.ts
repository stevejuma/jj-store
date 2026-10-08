import type {
  ChatBodyBlock,
  ChatBodyItem,
} from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
import { markdownBlocks } from "../../../web/src/lib/ui/store-agent-chat-body/markdown-blocks.js";
const agent = {
  id: "demo-agent",
  name: "Demo agent",
  handle: "",
  initials: "AI",
};
const demoTurn = {
  id: "demo-turn",
  author: "You",
  agentName: "Demo agent",
  prompt:
    "Explain how inline discussions can work across messages and tool results.",
  summary:
    "Use stable identities for inline discussions across prose, tool output and custom views.",
};
const block = (
  id: string,
  kind: ChatBodyBlock["kind"],
  label: string,
  text: string,
  path?: string,
): ChatBodyBlock => ({ id, revision: "demo-1", kind, label, text, path });
export const playgroundItems: ChatBodyItem[] = [
  {
    id: "prose",
    turn: demoTurn,
    kind: "message",
    role: "assistant",
    author: agent,
    label: "Synthetic assistant message",
    blocks: markdownBlocks(
      "prose",
      "demo-1",
      "Assistant explanation",
      "## A small change\n\nUse **stable identities** for comments, including emoji 👩🏽‍💻 and accented text café.\n\nKeep the original content readable while a discussion is open.",
    ),
  },
  {
    id: "tool-group",
    turn: demoTurn,
    kind: "tools",
    label: "Read a file, ran a command",
    blocks: [],
    tools: [
      {
        id: "read",
        revision: "demo-1",
        title: "Read example.ts",
        detailPlacement: "sidebar",
        status: "complete",
        raw: { path: "src/example.ts", output: "export const answer = 42;" },
        blocks: [
          block(
            "read/code",
            "code",
            "Captured example.ts",
            "export const answer = 42;\nexport const message = 'Hello';",
            "example.ts",
          ),
        ],
      },
      {
        id: "command",
        revision: "demo-1",
        title: "Run tests",
        status: "complete",
        raw: { command: "pnpm test", output: "7 tests passed" },
        blocks: [
          block(
            "command/output",
            "code",
            "Captured test output",
            "$ pnpm test\n\n7 tests passed",
            "output.sh",
          ),
        ],
      },
    ],
  },
  {
    id: "patch-group",
    turn: demoTurn,
    kind: "tools",
    label: "Apply patch",
    defaultExpanded: true,
    blocks: [],
    tools: [
      {
        id: "patch",
        revision: "demo-1",
        title: "Apply patch example.ts",
        status: "complete",
        raw: {
          patch: "-export const answer = 41;\n+export const answer = 42;",
        },
        blocks: [
          block(
            "patch/code",
            "patch",
            "Captured patch example.ts",
            "@@ captured hunk\n-export const answer = 41;\n+export const answer = 42;",
            "example.ts",
          ),
          block(
            "patch/test",
            "patch",
            "Captured patch example.test.ts",
            "@@ captured hunk\n+expect(answer).toBe(42);",
            "example.test.ts",
          ),
        ],
      },
    ],
  },
  {
    id: "custom",
    turn: demoTurn,
    kind: "context",
    label: "Rendered decision table",
    defaultExpanded: true,
    blocks: [
      block(
        "custom/table",
        "custom",
        "Decision table",
        "| Topic | Choice |\n| --- | --- |\n| Comments | Inline discussion |\n| Storage | Story memory |",
      ),
    ],
  },
  {
    id: "demo-context",
    turn: demoTurn,
    kind: "handoff",
    label: "Agent response · Synthetic review",
    blocks: [
      block(
        "demo-context/xml",
        "context",
        "Returned synthetic response",
        '<subagent-completion from="Demo review" status="completed">\nNo regressions found.\n\n**Policy:** Captured demo only.\n</subagent-completion>',
      ),
    ],
  },
];
export const appendedItem: ChatBodyItem = {
  id: "appended",
  turn: {
    ...demoTurn,
    id: "demo-turn-2",
    prompt: "Append another captured response.",
    summary: "The appended response preserves existing comment targets.",
  },
  kind: "message",
  role: "assistant",
  author: agent,
  label: "Appended response",
  blocks: [
    block(
      "appended/text",
      "markdown",
      "Appended response",
      "The new captured response arrived. Existing comments retain their original targets.",
    ),
  ],
};
export const failedItem: ChatBodyItem = {
  id: "failure",
  turn: demoTurn,
  kind: "tools",
  label: "Failed read",
  defaultExpanded: true,
  blocks: [],
  tools: [
    {
      id: "failed",
      revision: "demo-1",
      title: "Read restricted.ts",
      status: "error",
      raw: { error: "Permission denied", truncated: true },
      blocks: [
        block(
          "failed/text",
          "code",
          "Unavailable output",
          "Permission denied. Only this error was captured; file contents are unavailable.",
        ),
      ],
    },
  ],
};
