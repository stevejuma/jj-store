import { EditorState, Text, type Extension } from "@codemirror/state";
import { language, StreamLanguage } from "@codemirror/language";
import { swift } from "@codemirror/legacy-modes/mode/swift";
import { highlightTree } from "@lezer/highlight";
import { miraClassHighlighter } from "@lapismd/mira/codemirror";
import { repositoryEditorLanguage } from "../community-projects/repository-editor-language.js";

export function capturedCodeLanguage(path?: string): Extension {
  return path?.toLowerCase().endsWith(".swift")
    ? StreamLanguage.define(swift)
    : repositoryEditorLanguage(path ?? "output.txt").extension;
}

/** Parse a numbered read's contents, then map tokens into its unchanged capture. */
export function numberedCodeHighlights(text: string, path?: string) {
  const rows = text.split("\n");
  const prefixes = rows.map((row) => /^\s*\d+\t/.exec(row)?.[0].length ?? 0);
  const original = Text.of(rows);
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
