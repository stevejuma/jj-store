import { xmlLanguage } from "@codemirror/lang-xml";
import { highlightTree, tagHighlighter, tags } from "@lezer/highlight";

type TokenKind = "name" | "attribute" | "string" | "punctuation";
export interface PromptTagToken {
  text: string;
  kind?: TokenKind;
}

const syntax = tagHighlighter([
  { tag: tags.tagName, class: "name" },
  { tag: tags.attributeName, class: "attribute" },
  { tag: tags.attributeValue, class: "string" },
  { tag: tags.angleBracket, class: "punctuation" },
]);

/** Highlight syntax as escaped text spans, never executable source HTML. */
export function promptTagTokens(label: string): PromptTagToken[] {
  const tokens: PromptTagToken[] = [];
  let at = 0;
  highlightTree(xmlLanguage.parser.parse(label), syntax, (from, to, kind) => {
    if (from > at) tokens.push({ text: label.slice(at, from) });
    tokens.push({ text: label.slice(from, to), kind: kind as TokenKind });
    at = to;
  });
  if (at < label.length) tokens.push({ text: label.slice(at) });
  return tokens;
}
