import { describe, expect, it } from "vitest";
import {
  liveTranscriptTool,
  recordedTranscriptTool,
} from "../../src/lib/ui/community-agent-activity/transcript-tool-adapters.js";
import { richTranscriptToolDetail } from "../../src/lib/ui/community-agent-activity/transcript-tool-rich.js";
import type { ActivityRow } from "../../src/lib/ui/store-agent-execution/presentation.js";
import { groupedAgentActivity } from "../../../catalog/fixtures/agent-tool-groups.js";
const row = (value: Record<string, unknown>): ActivityRow => ({
  id: "turn/tool/1",
  kind: "tool",
  text: String(value.title ?? "Tool"),
  value,
  request: "turn",
  sequence: 1,
});
const rich = (value: Record<string, unknown>) =>
  richTranscriptToolDetail(liveTranscriptTool(row(value)));

describe("retained agent tool rich views", () => {
  it("shows ACP read contents rather than the exec envelope", () => {
    const payload = {
      kind: "read",
      name: "exec_command",
      locations: [{ path: "src/example.ts" }],
      rawOutput: {
        exit_code: 0,
        formatted_output: "export const answer = 42;\n",
      },
    };
    const before = structuredClone(payload);
    expect(rich(payload)).toEqual({
      kind: "file-read",
      path: "src/example.ts",
      content: "export const answer = 42;\n",
    });
    expect(payload).toEqual(before);
  });
  it("retains empty files and avoids assigning multi-file output to one path", () => {
    expect(
      rich({
        kind: "read",
        locations: [{ path: "empty.ts" }],
        rawOutput: { formatted_output: "" },
      }),
    ).toEqual({ kind: "file-read", path: "empty.ts", content: "" });
    expect(
      rich({
        kind: "read",
        locations: [{ path: "a.ts" }, { path: "b.rs" }],
        rawOutput: { formatted_output: "combined" },
      }),
    ).toEqual({ kind: "file-read", content: "combined" });
  });
  it("keeps captured JSON file bytes rather than treating them as an envelope", () => {
    const contents = '{\n  "text": "literal JSON value"\n}\n';
    expect(
      rich({
        kind: "read",
        locations: [{ path: "data.json" }],
        rawOutput: contents,
      }),
    ).toEqual({ kind: "file-read", path: "data.json", content: contents });
  });
  it("uses ACP content diff entries and explicit null preimages", () => {
    const detail = rich({
      kind: "edit",
      title: "Editing files",
      content: [
        {
          type: "diff",
          path: "LICENSE",
          oldText: null,
          newText: "MIT License\n",
        },
      ],
    });
    expect(detail).toMatchObject({
      kind: "file-edit",
      files: [
        {
          path: "LICENSE",
          oldText: null,
          newText: "MIT License\n",
          partial: false,
          additions: 1,
          deletions: 0,
        },
      ],
    });
  });
  it("keeps deletion and empty replacement separate from missing preimages", () => {
    expect(
      rich({
        kind: "edit",
        content: [{ path: "a.ts", oldText: "old\n", newText: "" }],
      }),
    ).toMatchObject({
      kind: "file-edit",
      files: [{ oldText: "old\n", newText: "", partial: false }],
    });
    expect(
      rich({ kind: "edit", content: [{ path: "a.ts", newText: "new\n" }] }),
    ).toMatchObject({ kind: "file-edit", files: [{ partial: true }] });
  });
  it("preserves an old-only partial capture and minimal changed-line stats", () => {
    expect(
      rich({
        kind: "edit",
        content: [{ path: "old.ts", oldText: "retained old contents" }],
      }),
    ).toMatchObject({
      kind: "file-edit",
      files: [{ partial: true, capturedText: "retained old contents" }],
    });
    expect(
      rich({
        kind: "edit",
        content: [
          { path: "a.ts", oldText: "same\nold\n", newText: "same\nnew\n" },
        ],
      }),
    ).toMatchObject({
      kind: "file-edit",
      files: [{ additions: 1, deletions: 1 }],
    });
  });
  it("unwraps command output and stderr, and recognizes search matches", () => {
    expect(
      rich({
        kind: "execute",
        rawInput: { command: "pwd" },
        rawOutput: { formatted_output: "/workspace\n", stderr: "warning" },
      }),
    ).toEqual({
      kind: "shell",
      command: "pwd",
      output: "/workspace\n",
      error: "warning",
    });
    expect(
      rich({
        kind: "search",
        rawInput: { command: 'rg "answer" src' },
        rawOutput: { formatted_output: "src/a.ts:3:export const answer = 42;" },
      }),
    ).toMatchObject({
      kind: "search",
      query: "answer",
      matches: [
        { path: "src/a.ts", line: 3, text: "export const answer = 42;" },
      ],
    });
  });
  it("preserves unsupported payloads and failed tools without inventing file output", () => {
    expect(
      rich({
        kind: "read",
        locations: [{ path: "a.ts" }],
        status: "failed",
        rawOutput: { formatted_output: "denied" },
      }),
    ).toBeUndefined();
    expect(
      rich({
        kind: "other",
        content: [{ type: "image", url: "https://invalid/image" }],
      }),
    ).toBeUndefined();
    expect(
      liveTranscriptTool(
        row({ kind: "read", status: "failed", rawOutput: "denied" }),
      ).error,
    ).toBe("denied");
  });
  it("adapts captured read paths and exact retained patches without mutating records", () => {
    const example = groupedAgentActivity.find((a) => a.type === "tool")!;
    if (example.type !== "tool") throw Error("fixture tool missing");
    const tool = {
      ...example.tool,
      kind: "read" as const,
      input: { media_type: "json", content: '{"file_path":"src/a.ts"}' },
      output: { media_type: "text", content: "const a = 1;\n" },
      code: [],
      observed_paths: [],
      state: "succeeded" as const,
    };
    expect(richTranscriptToolDetail(recordedTranscriptTool(tool))).toEqual({
      kind: "file-read",
      path: "src/a.ts",
      content: "const a = 1;\n",
    });
    expect(tool.kind).toBe("read");
  });
});
