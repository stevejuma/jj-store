import type { ChatBodyItem } from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
const code = Array.from(
  { length: 60 },
  (_, i) => `export const line${i + 1} = ${i + 1};`,
).join("\n");
export const largeCaptureItems: ChatBodyItem[] = (
  ["code", "patch"] as const
).map((kind) => ({
  id: `large-${kind}-item`,
  kind: "tools",
  label: `Synthetic large ${kind}`,
  defaultExpanded: true,
  blocks: [],
  tools: [
    {
      id: `large-${kind}-tool`,
      revision: "demo-1",
      title: `${kind === "code" ? "Read" : "Apply patch"} large.ts`,
      status: "complete",
      blocks: [
        {
          id: `large-${kind}`,
          revision: "demo-1",
          kind,
          label: `Large captured ${kind}`,
          path: "large.ts",
          text:
            kind === "code"
              ? code
              : code
                  .split("\n")
                  .map((line) => `+${line}`)
                  .join("\n"),
        },
      ],
      raw: { output: code },
    },
  ],
}));
