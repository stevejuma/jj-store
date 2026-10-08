import { expect, test } from "vitest";
import { numberedCodeHighlights } from "../../../web/src/lib/ui/store-agent-chat-body/captured-code.js";

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
