import { describe, expect, test } from "vitest";
import {
  maskPromptTags,
  matchPromptElements,
  outsideRanges,
  promptTags,
} from "../../src/lib/ui/store-collaboration/prompt-element-ranges.js";

describe("agent prompt element boundaries", () => {
  test("arbitrary names and quoted greater-than attributes retain exact source", () => {
    const source =
      '<自定义:scope data-version="a > b"><next.one enabled>**body**</next.one></自定义:scope>';
    const tags = promptTags(source),
      elements = matchPromptElements(tags);
    expect(tags.map((t) => t.name)).toEqual([
      "自定义:scope",
      "next.one",
      "next.one",
      "自定义:scope",
    ]);
    expect(elements).toHaveLength(1);
    expect(elements[0]!.open.label).toBe('<自定义:scope data-version="a > b">');
    expect(elements[0]!.children[0]!.open.name).toBe("next.one");
    expect(
      source.slice(
        elements[0]!.children[0]!.open.end,
        elements[0]!.children[0]!.close!.start,
      ),
    ).toBe("**body**");
    expect(elements[0]!.close!.end).toBe(source.length);
  });
  test("sibling and self-closing elements keep native order", () => {
    expect(
      matchPromptElements(promptTags("<a/><b>text</b><c />")).map(
        (e) => e.open.name,
      ),
    ).toEqual(["a", "b", "c"]);
    expect(
      matchPromptElements(promptTags("<same><same>inside</same></same>"))[0]!
        .children,
    ).toHaveLength(1);
  });
  test("mismatched and unfinished sections cannot hide source", () => {
    expect(
      matchPromptElements(promptTags("<outer><inner></outer></inner>")),
    ).toEqual([]);
    expect(
      matchPromptElements(promptTags("<unclosed><nested>text</nested>")),
    ).toEqual([]);
    expect(
      matchPromptElements(
        promptTags("<wrong></different><valid>ok</valid>"),
      ).map((e) => e.open.name),
    ).toEqual(["valid"]);
    expect(matchPromptElements(promptTags("<Case>text</case>"))).toEqual([]);
  });
  test("escaped examples autolinks and comparisons are not prompt tags", () => {
    expect(
      promptTags(
        "\\<example> and <https://example.com> and <a@example.com> and 1 < 2",
      ),
    ).toEqual([]);
  });
  test("code and comment ranges withhold every overlapping tag", () => {
    const source = "<outer><code/><real>ok</real><!-- <comment/> --></outer>";
    const ignored = [
      { start: 7, end: 14 },
      { start: source.indexOf("<!--"), end: source.indexOf("-->") + 3 },
    ];
    expect(
      outsideRanges(promptTags(source), ignored).map((t) => t.name),
    ).toEqual(["outer", "real", "real", "outer"]);
  });
  test("masking keeps UTF-16 offsets and line endings for Markdown code detection", () => {
    const source = '<outer\r\n source="x">**content**</outer>\r\n';
    const mask = maskPromptTags(source, promptTags(source));
    expect(mask.length).toBe(source.length);
    expect([...mask.matchAll(/\r\n/g)].map((m) => m.index)).toEqual(
      [...source.matchAll(/\r\n/g)].map((m) => m.index),
    );
    expect(mask[0]).toBe("x");
    expect(mask).toContain("**content**");
  });
  test("excessive nesting falls back without returning partial hidden subtrees", () => {
    const source = "<any>".repeat(65) + "text" + "</any>".repeat(65);
    expect(matchPromptElements(promptTags(source))).toEqual([]);
  });
});
