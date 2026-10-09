import { expect, test } from "vitest";
import { EditorState } from "@codemirror/state";
import { language } from "@codemirror/language";
import { highlightTree } from "@lezer/highlight";
import { miraClassHighlighter } from "@lapismd/mira/codemirror";
import {
  capturedLinePrefixes,
  fileCodeLanguage,
  numberedCodeHighlights,
} from "../../../web/src/lib/ui/store-agent-chat-body/captured-code.js";

test("maps captured gutter labels without renumbering gaps or shifting Unicode offsets", () => {
  const text = "  9\t👩🏽‍💻 café\n 10\t\nUnnumbered note\n 90\tlast";
  const prefixes = capturedLinePrefixes(text);
  expect(prefixes.map((row) => row.number)).toEqual(["9", "10", "90"]);
  expect(prefixes.map(({ from, to }) => text.slice(from, to))).toEqual([
    "  9\t",
    " 10\t",
    " 90\t",
  ]);
  expect(prefixes.at(-1)!.to).toBe(text.indexOf("last"));
  expect(capturedLinePrefixes("123 plain prose\n\tindented code")).toEqual([]);
});

test("highlights numbered Markdown without changing captured source or prefixes", () => {
  const text =
    "     1\t# Heading 👩🏽‍💻\n     2\t\n     3\t[Link](https://example.com)";
  const ranges = numberedCodeHighlights(text, "README.md");
  const heading = ranges.find(
    (r) =>
      r.classes.includes("cm-heading") &&
      text.slice(r.from, r.to).includes("Heading"),
  )!;
  expect(text.slice(heading.from, heading.to)).toContain("Heading");
  expect(heading.from).toBeGreaterThanOrEqual(text.indexOf("#"));
  expect(ranges.some((r) => r.classes.includes("cm-link"))).toBe(true);
  expect(text).toContain("     1\t# Heading 👩🏽‍💻");
  for (const range of ranges)
    expect(text.slice(range.from, range.to)).not.toContain("\t");
});

test("splits multiline syntax around numbered prefixes and keeps UTF-16 anchors", () => {
  const text = "  9\t/* café 👩🏽‍💻\n 10\tcomment */\n 11\tlet value = 42;";
  const ranges = numberedCodeHighlights(text, "example.rs");
  const comments = ranges.filter((r) => r.classes.includes("cm-comment"));
  expect(comments.map((r) => text.slice(r.from, r.to))).toEqual([
    "/* café 👩🏽‍💻",
    "comment */",
  ]);
  const number = ranges.find((r) => r.classes.includes("cm-number"))!;
  expect(text.slice(number.from, number.to)).toBe("42");
  expect(number.from).toBe(text.indexOf("42"));
});

test("leaves unknown formats plain and supports unnumbered lines in a capture", () => {
  expect(numberedCodeHighlights(" 1\tunknown", "output.txt")).toEqual([]);
  const text = " 1\t# Heading\nUnnumbered **bold** footnote.";
  expect(
    numberedCodeHighlights(text, "README.md").some(
      (r) =>
        text.slice(r.from, r.to).includes("bold") &&
        r.classes.includes("cm-strong"),
    ),
  ).toBe(true);
});

test("highlights workspace source languages without shifting source coordinates", () => {
  const tokens = (text: string, path: string) => {
    const grammar = EditorState.create({
      doc: text,
      extensions: fileCodeLanguage(path),
    }).facet(language)!;
    const ranges: { from: number; to: number; classes: string }[] = [];
    highlightTree(
      grammar.parser.parse(text),
      miraClassHighlighter,
      (from, to, classes) => ranges.push({ from, to, classes }),
    );
    return ranges;
  };
  for (const [path, text, keyword] of [
    ["example.ts", "export const greeting = 'café 🦀';", "export"],
    ["example.js", "const value = 42;", "const"],
    ["example.py", "def greet(): return 'café'", "def"],
    ["Example.java", "public class Example {}", "public"],
  ]) {
    const ranges = tokens(text, path);
    expect(
      ranges
        .filter((r) => r.classes.includes("cm-keyword"))
        .map((r) => text.slice(r.from, r.to)),
      path,
    ).toContainEqual(expect.stringContaining(keyword));
  }
  const json = '{"greeting":"café 🦀"}';
  expect(
    tokens(json, "example.json").some(
      (r) =>
        r.classes.includes("cm-string") &&
        json.slice(r.from, r.to).includes("café 🦀"),
    ),
  ).toBe(true);
});
