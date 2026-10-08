import { describe, test, expect } from "vitest";
import reference from "./reference.json";
import { deltaChatItems } from "./adapter.js";
import { utf8ToUtf16, byteSlice } from "./offsets.js";
import { capturedPatchBlocks } from "./patches.js";
import {
  commentAnchor,
  validAnchor,
} from "../../../web/src/lib/ui/store-agent-chat-body/anchors.js";
import { navigationTurns } from "../../../web/src/lib/ui/store-agent-chat-body/navigation-turns.js";
import { markdownBlocks } from "../../../web/src/lib/ui/store-agent-chat-body/markdown-blocks.js";

describe("Delta reference mapping", () => {
  test("preserves all 30 tools and assistant segments between tool groups", () => {
    const items = deltaChatItems(reference);
    expect(items.flatMap((i) => i.tools ?? [])).toHaveLength(30);
    expect(
      items.filter((i) => i.kind === "message" && i.role === "assistant"),
    ).toHaveLength(2);
    expect(items.map((i) => i.id).length).toBe(
      new Set(items.map((i) => i.id)).size,
    );
    expect(items[0].role).toBe("user");
    expect(
      items.flatMap((i) => i.tools ?? []).every((t) => t.status === "complete"),
    ).toBe(true);
    expect(JSON.stringify(reference)).not.toContain("stevejuma");
    expect(items.some((i) => i.kind === "handoff")).toBe(true);
    expect(items.find((i) => i.kind === "handoff")!.blocks[0].kind).toBe(
      "context",
    );
  });
  test("maps the full prompt/response and 30 tools into one actual conversation turn", () => {
    const turns = navigationTurns(deltaChatItems(reference));
    expect(turns).toHaveLength(1);
    expect(turns[0].toolCount).toBe(30);
    expect(turns[0].prompt).toContain("Would like to extend");
    expect(turns[0].summary).toContain("Block screen only");
    expect(turns[0].createdAt).toBeGreaterThan(0);
    expect(turns[0].additions).toBeGreaterThan(0);
    expect(turns[0].firstItemId).toBe("delta-0");
  });
  test("converts Unicode byte offsets and rejects offsets inside characters", () => {
    const text = "aé👩🏽‍💻z";
    const prefix = "aé👩🏽‍💻";
    expect(utf8ToUtf16(text, new TextEncoder().encode(prefix).length)).toBe(
      prefix.length,
    );
    expect(byteSlice("aéz", 1, 3)).toBe("é");
    expect(() => utf8ToUtf16(text, 2)).toThrow();
    expect(() => utf8ToUtf16(text, 999)).toThrow(RangeError);
  });
  test("keeps multi-file patch hunks without synthetic file coordinates", () => {
    const tool = reference.records.find(
      (r) => r.kind === "ToolUse" && r.name === "apply_patch",
    )!;
    const blocks = capturedPatchBlocks(tool.input!, tool.id, "v1");
    expect(blocks).toHaveLength(6);
    expect(blocks[0].path).toBe("Sources/BedtimeCore/Schedule.swift");
    expect(blocks[0].text).toContain("+public enum BedtimeAction");
    expect(blocks[0].text).toContain("@@");
  });
});
describe("immutable comment anchors", () => {
  const block = {
    id: "b",
    revision: "v1",
    kind: "markdown" as const,
    label: "prose",
    text: "First paragraph.\n\nSecond café paragraph.",
  };
  test("uses paragraph or line targets for a collapsed caret", () => {
    expect(commentAnchor(block, { from: 22, to: 22 }).quote).toBe(
      "Second café paragraph.",
    );
    expect(
      commentAnchor({ ...block, kind: "code" }, { from: 3, to: 3 }).quote,
    ).toBe("First paragraph.");
    const empty = { ...block, kind: "code" as const, text: "\nA line" };
    expect(validAnchor(empty, commentAnchor(empty, { from: 0, to: 0 }))).toBe(
      true,
    );
  });
  test("preserves exact ranges and never retargets changed revisions or bytes", () => {
    const anchor = commentAnchor(block, { from: 0, to: 5 });
    expect(anchor.quote).toBe("First");
    expect(validAnchor(block, anchor)).toBe(true);
    expect(validAnchor({ ...block, revision: "v2" }, anchor)).toBe(false);
    expect(validAnchor({ ...block, text: "Different" }, anchor)).toBe(false);
    expect(
      commentAnchor({ ...block, kind: "context" }, { from: 2, to: 5 }).quote,
    ).toBe(block.text);
  });
  test("separates selectable fenced code and Mira-rendered complex blocks", () => {
    const blocks = markdownBlocks(
      "m",
      "v1",
      "Message",
      "Paragraph\n\n```rust\nfn main() {}\n```\n\n---\n\nLast paragraph.",
    );
    expect(blocks.map((b) => b.kind)).toEqual([
      "markdown",
      "code",
      "custom",
      "markdown",
    ]);
    expect(blocks[1].text).toBe("fn main() {}");
  });
});
