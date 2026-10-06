import { cppLanguage } from "@codemirror/lang-cpp";
import { rustLanguage } from "@codemirror/lang-rust";
import { highlightTree, tagHighlighter, tags } from "@lezer/highlight";

export type RepositorySourceToken = {
  type: string;
  start: number;
  end: number;
};

// Map the editor's grammar tags to Design Core's public syntax token types.
// Other languages continue to use Design Core's built-in tokenizer.
const syntax = tagHighlighter([
  { tag: [tags.keyword, tags.meta], class: "keyword" },
  { tag: [tags.string, tags.escape], class: "string" },
  { tag: tags.comment, class: "comment" },
  { tag: tags.number, class: "number" },
  { tag: tags.bool, class: "constant" },
  { tag: tags.typeName, class: "type" },
  { tag: tags.function(tags.variableName), class: "function" },
  { tag: tags.propertyName, class: "property" },
  { tag: tags.operator, class: "operator" },
  { tag: tags.punctuation, class: "punctuation" },
]);

export function repositorySourceTokenizer(language: string) {
  return language === "rust" || language === "c" || language === "cpp"
    ? tokenizeRepositorySource
    : undefined;
}

/** Public CodeBlock extension, shared by recorded/draft Source and Blame. */
export function tokenizeRepositorySource(
  code: string,
  language: string,
): RepositorySourceToken[] {
  const grammar =
    language === "rust"
      ? rustLanguage
      : language === "c" || language === "cpp"
        ? cppLanguage
        : undefined;
  if (!grammar) return [];
  const tokens: RepositorySourceToken[] = [];
  highlightTree(grammar.parser.parse(code), syntax, (from, to, type) => {
    // Design Core's flat-to-line adapter expects tokens within a single line.
    // Parse the whole file first so multiline strings/comments keep context.
    let start = from;
    while (start < to) {
      const newline = code.indexOf("\n", start);
      const end = newline < 0 ? to : Math.min(to, newline);
      if (start < end) tokens.push({ type, start, end });
      start = end + 1;
    }
  });
  return tokens;
}
