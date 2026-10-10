import { expect, test } from "vitest";
import { readingDecorations } from "../../../web/src/lib/ui/store-agent-chat-body/markdown-decorations.js";

test("reading history renders hyphen bullets without changing source offsets", () => {
  const text = "Added a license:\n\n- LICENSE\n- Commit: `52c87453`";
  const { decorations } = readingDecorations(text);
  const classes: string[] = [];
  decorations.between(0, text.length, (_from, _to, value) => {
    classes.push(value.spec.class ?? "");
  });
  expect(
    classes.filter((v) => v.includes("cm-formatting-list-bullet")),
  ).toHaveLength(2);
});

test("reading history renders angle-bracket repository file destinations as links", () => {
  const text = "Updated [LICENSE](<$WORKSPACE/repositories/repo/LICENSE:3>)";
  const { decorations } = readingDecorations(text, [], {
    fileLink: (destination: string) => destination.startsWith("$WORKSPACE/"),
  });
  const links: string[] = [];
  decorations.between(0, text.length, (_from, _to, value) => {
    if (value.spec.attributes?.["data-chat-file-link"])
      links.push(value.spec.attributes["data-chat-file-link"]);
  });
  expect(links).toEqual(["$WORKSPACE/repositories/repo/LICENSE:3"]);
});
