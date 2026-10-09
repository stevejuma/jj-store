import { EditorState, Text, type Extension } from "@codemirror/state";
import { language, StreamLanguage } from "@codemirror/language";
import {
  javascript,
  typescript,
  json,
} from "@codemirror/legacy-modes/mode/javascript";
import { python } from "@codemirror/legacy-modes/mode/python";
import { java } from "@codemirror/legacy-modes/mode/clike";
import { swift } from "@codemirror/legacy-modes/mode/swift";
import { highlightTree } from "@lezer/highlight";
import { miraClassHighlighter } from "@lapismd/mira/codemirror";
import { repositoryEditorLanguage } from "../community-projects/repository-editor-language.js";

export function capturedCodeLanguage(path?: string): Extension {
  return path?.toLowerCase().endsWith(".swift")
    ? StreamLanguage.define(swift)
    : repositoryEditorLanguage(path ?? "output.txt").extension;
}

export function fileCodeLanguage(path?: string): Extension {
  if (path?.toLowerCase().endsWith(".swift"))
    return StreamLanguage.define(swift);
  const language = repositoryEditorLanguage(path ?? "output.txt");
  // The repository's editable CodeEditor provides these built-in grammars.
  // Mira readers supply their own extensions, including these existing modes.
  if (language.id === "typescript" || language.id === "tsx")
    return StreamLanguage.define(typescript);
  if (language.id === "javascript" || language.id === "jsx")
    return StreamLanguage.define(javascript);
  if (language.id === "json") return StreamLanguage.define(json);
  if (language.id === "python") return StreamLanguage.define(python);
  if (language.id === "java" || path?.toLowerCase().endsWith(".java"))
    return StreamLanguage.define(java);
  return language.extension;
}

/** Prefix ranges stay in captured UTF-16 coordinates, including empty lines. */
export function capturedLinePrefixes(text: string) {
  let offset = 0;
  return text.split("\n").flatMap((row) => {
    const from = offset;
    offset += row.length + 1;
    const match = /^\s*(\d+)\t/.exec(row);
    return match
      ? [{ from, to: from + match[0].length, number: match[1] }]
      : [];
  });
}

/** Parse a numbered read's contents, then map tokens into its unchanged capture. */
export function numberedCodeHighlights(text: string, path?: string) {
  const rows = text.split("\n");
  const original = Text.of(rows);
  const captures = new Map(
    capturedLinePrefixes(text).map((row) => [row.from, row.to - row.from]),
  );
  const prefixes = rows.map(
    (_, i) => captures.get(original.line(i + 1).from) ?? 0,
  );
  const projected = Text.of(rows.map((row, i) => row.slice(prefixes[i])));
  const grammar = EditorState.create({
    doc: projected,
    extensions: capturedCodeLanguage(path),
  }).facet(language);
  const ranges: { from: number; to: number; classes: string }[] = [];
  if (!grammar) return ranges;
  highlightTree(
    grammar.parser.parse(projected.toString()),
    miraClassHighlighter,
    (from, to, classes) => {
      const last = projected.lineAt(Math.max(from, to - 1)).number;
      for (let i = projected.lineAt(from).number; i <= last; i++) {
        const line = projected.line(i);
        const start = Math.max(from, line.from);
        const end = Math.min(to, line.to);
        if (start >= end) continue;
        const offset = original.line(i).from + prefixes[i - 1] - line.from;
        ranges.push({ from: start + offset, to: end + offset, classes });
      }
    },
  );
  return ranges;
}
