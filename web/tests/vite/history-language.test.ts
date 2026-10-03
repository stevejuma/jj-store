import { expect, test } from "vitest";
import { CompletionContext } from "@codemirror/autocomplete";
import { EditorState } from "@codemirror/state";
import { ensureSyntaxTree } from "@codemirror/language";
import {
  historyCompletion,
  historyLanguage,
  historyExtensions,
  insideQueryString,
} from "../../src/lib/ui/store-history-query/history-language.js";
import type { HistoryQueryInfo } from "../../src/lib/controller/history-query-types.js";

const info: HistoryQueryInfo = {
  version: 1,
  jj_version: "0.45.1",
  max_query_bytes: 4096,
  callables: [
    {
      name: "description",
      signature: "description(pattern)",
      insert_text: "description()",
      description: "Match description",
      kind: "function",
    },
  ],
  aliases: ["mutable()"],
  operators: ["::", "|"],
  operation_fields: ["description", "user", "from", "snapshot"],
  unsupported: [],
};
function complete(
  kind: "revset" | "operations",
  text: string,
  explicit = true,
) {
  const state = EditorState.create({ doc: text });
  return historyCompletion(kind, info, ["main"])(
    new CompletionContext(state, text.length, explicit),
  );
}

test("revset completion includes metadata, standard aliases and snapshot bookmarks", () => {
  const result = complete("revset", "des");
  expect(result?.from).toBe(0);
  expect(result?.options.map((option) => option.label)).toEqual(
    expect.arrayContaining(["description(pattern)", "mutable()", "main", "::"]),
  );
  expect(complete("revset", 'description("des')).toBeNull();
  expect(insideQueryString('description("escaped \\" value')).toBe(true);
});

test("operation completion exposes its own fields and values without revsets", () => {
  expect(
    complete("operations", "us")?.options.map((option) => option.label),
  ).toContain("user:");
  expect(
    complete("operations", "snapshot:f")?.options.map((option) => option.label),
  ).toEqual(["true", "false"]);
  expect(
    complete("operations", "from:")?.options.map((option) => option.label),
  ).toContain("@--");
  expect(complete("operations", 'description:"user:')).toBeNull();
});

test("lexical grammar highlights functions, aliases, operators and quoted values", () => {
  const text = 'mutable() & description("escaped \\" text")';
  const state = EditorState.create({
    doc: text,
    extensions: [historyLanguage("revset", info)],
  });
  const nodes: string[] = [];
  ensureSyntaxTree(state, text.length)?.iterate({
    enter: (node) => {
      nodes.push(node.type.name);
    },
  });
  expect(nodes).toEqual(
    expect.arrayContaining([
      "macroName",
      "operator",
      "string",
      "variableName.function",
    ]),
  );
});

test("history editor preserves pasted multiline revsets", () => {
  const text = 'mutable() &\n description("fix")';
  const state = EditorState.create({
    extensions: historyExtensions("revset", info, [], () => {}),
  });
  expect(
    state.update({ changes: { from: 0, insert: text } }).newDoc.toString(),
  ).toBe(text);
});
