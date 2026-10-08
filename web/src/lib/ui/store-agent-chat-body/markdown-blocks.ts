import { markdownLanguage } from "@codemirror/lang-markdown";
import type { ChatBodyBlock } from "./model.js";

/** Native text stays selectable; Mira reading views own complex rendered blocks. */
export function markdownBlocks(
  id: string,
  revision: string,
  label: string,
  text: string,
): ChatBodyBlock[] {
  if (!text.trim()) return [];
  const result: ChatBodyBlock[] = [];
  const cursor = markdownLanguage.parser.parse(text).cursor();
  if (!cursor.firstChild())
    return [{ id, revision, label, text, kind: "markdown" }];
  let start = 0;
  const emit = (end: number) => {
    const source = text.slice(start, end).trim();
    if (source)
      result.push({
        id: `${id}/${start}`,
        revision,
        label,
        text: source,
        kind: "markdown",
        sourceFrom:
          start +
          text.slice(start, end).length -
          text.slice(start, end).trimStart().length,
      });
  };
  do {
    const custom = ["Table", "HTMLBlock", "HorizontalRule"].includes(
      cursor.name,
    );
    const fenced = cursor.name === "FencedCode";
    if (custom || fenced) {
      emit(cursor.from);
      const source = text.slice(cursor.from, cursor.to);
      const match = fenced
        ? /^(`{3,}|~{3,})([^\n]*)\n([\s\S]*?)\n\1\s*$/.exec(source)
        : null;
      const lang = match?.[2].trim().split(/\s/)[0] ?? "";
      const extension =
        (
          {
            bash: "sh",
            shell: "sh",
            typescript: "ts",
            javascript: "js",
            python: "py",
          } as Record<string, string>
        )[lang] ?? lang;
      result.push({
        id: `${id}/${cursor.from}`,
        revision,
        label: fenced
          ? `${label} code${lang ? ` · ${lang}` : ""}`
          : `${label} rendered section`,
        kind: fenced && match ? "code" : "custom",
        text: match?.[3] ?? source,
        sourceFrom: cursor.from + (match ? source.indexOf("\n") + 1 : 0),
        path: fenced ? `captured.${extension || "txt"}` : undefined,
      });
      start = cursor.to;
    }
  } while (cursor.nextSibling());
  emit(text.length);
  return result.length
    ? result
    : [{ id, revision, label, text, kind: "markdown" }];
}
