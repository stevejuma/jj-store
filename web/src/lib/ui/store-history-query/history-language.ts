import {
  StreamLanguage,
  HighlightStyle,
  syntaxHighlighting,
} from "@codemirror/language";
import {
  autocompletion,
  acceptCompletion,
  completionKeymap,
  type CompletionContext,
} from "@codemirror/autocomplete";
import { Prec } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import { tags } from "@lezer/highlight";
import type { HistoryQueryInfo } from "../../controller/history-query-types.js";

export type HistoryLanguage = "revset" | "operations";

/** Lexical presentation only. The operation-pinned backend owns validation. */
export function historyLanguage(
  kind: HistoryLanguage,
  info: HistoryQueryInfo | null,
) {
  const aliases = new Set(
    info?.aliases.map((value) => value.split("(")[0]) ?? [],
  );
  return StreamLanguage.define({
    token(stream) {
      if (stream.eatSpace()) return null;
      if (stream.match(/"(?:[^"\\]|\\.)*(?:"|$)|'(?:[^'\\]|\\.)*(?:'|$)/))
        return "string";
      if (kind === "operations" && stream.match(/[a-z_]+(?=:)/))
        return "propertyName";
      if (stream.match(/::|\.\.|[|&~:+-]/)) return "operator";
      if (stream.match(/[(),]/)) return "punctuation";
      if (stream.match(/\d+(?![a-z_])/)) return "number";
      const word = stream.match(/[a-zA-Z_@][a-zA-Z0-9_@/.]*/);
      if (word) {
        const name = (word as RegExpMatchArray)[0];
        if (kind === "operations" && /^(true|false)$/.test(name)) return "bool";
        if (kind === "revset" && /^\s*\(/.test(stream.string.slice(stream.pos)))
          return aliases.has(name) ? "macroName" : "variableName.function";
        return "variableName";
      }
      stream.next();
      return null;
    },
  });
}

export function insideQueryString(text: string): boolean {
  let quote = "",
    escape = false;
  for (const char of text) {
    if (escape) {
      escape = false;
      continue;
    }
    if (quote && char === "\\") {
      escape = true;
      continue;
    }
    if (char === quote) quote = "";
    else if (!quote && (char === '"' || char === "'")) quote = char;
  }
  return !!quote;
}

export function historyCompletion(
  kind: HistoryLanguage,
  info: HistoryQueryInfo | null,
  symbols: readonly string[],
) {
  return (context: CompletionContext) => {
    if (insideQueryString(context.state.doc.sliceString(0, context.pos)))
      return null;
    const word = context.matchBefore(/[a-zA-Z_][a-zA-Z0-9_]*/);
    if (!word && !context.explicit) return null;
    const from = word?.from ?? context.pos;
    if (kind === "operations") {
      const prefix = context.state.doc.sliceString(0, from);
      const field = /(?:^|\s)(snapshot|from):$/.exec(prefix)?.[1];
      const options =
        field === "snapshot"
          ? ["true", "false"]
          : field === "from"
            ? ["@", "@-", "@--"]
            : (info?.operation_fields.map((name) => `${name}:`) ?? []);
      return {
        from,
        options: options.map((label) => ({ label, type: "property" })),
        validFor: /^[\w@+-]*$/,
      };
    }
    const seen = new Set<string>();
    const options = (info?.callables ?? []).map((callable) => ({
      label: callable.signature,
      detail: callable.kind,
      info: callable.description,
      type: "function",
      apply: callable.insert_text,
    }));
    for (const declaration of info?.aliases ?? []) {
      if (!options.some((option) => option.label === declaration))
        options.push({
          label: declaration,
          detail: "standard alias",
          info: "JJ standard alias",
          type: "function",
          apply: declaration,
        });
    }
    for (const symbol of symbols)
      options.push({
        label: symbol,
        detail: "bookmark",
        info: "Bookmark in this snapshot",
        type: "variable",
        apply: symbol,
      });
    for (const operator of info?.operators ?? [])
      options.push({
        label: operator,
        detail: "operator",
        info: "JJ revset operator",
        type: "operator",
        apply: operator,
      });
    return {
      from,
      options: options.filter(
        (option) => !seen.has(option.label) && !!seen.add(option.label),
      ),
      validFor: /^\w*$/,
    };
  };
}

const highlighting = syntaxHighlighting(
  HighlightStyle.define([
    { tag: tags.string, class: "history-query-string" },
    { tag: tags.function(tags.variableName), class: "history-query-function" },
    { tag: tags.macroName, class: "history-query-alias" },
    { tag: tags.propertyName, class: "history-query-field" },
    { tag: [tags.operator, tags.punctuation], class: "history-query-operator" },
    { tag: [tags.number, tags.bool], class: "history-query-literal" },
    { tag: tags.variableName, class: "history-query-symbol" },
  ]),
);

export function historyExtensions(
  kind: HistoryLanguage,
  info: HistoryQueryInfo | null,
  symbols: readonly string[],
  run: () => void,
) {
  return [
    historyLanguage(kind, info),
    highlighting,
    autocompletion({
      override: [historyCompletion(kind, info, symbols)],
      icons: false,
      activateOnTypingDelay: 100,
      interactionDelay: 0,
    }),
    Prec.highest(
      keymap.of([
        { key: "Tab", run: acceptCompletion },
        {
          key: "Mod-Enter",
          run: () => {
            run();
            return true;
          },
        },
        ...completionKeymap,
      ]),
    ),
  ];
}
