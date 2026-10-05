const RUST_KEYWORDS = new Set([
  "as",
  "async",
  "await",
  "break",
  "const",
  "continue",
  "crate",
  "dyn",
  "else",
  "enum",
  "extern",
  "false",
  "fn",
  "for",
  "if",
  "impl",
  "in",
  "let",
  "loop",
  "match",
  "mod",
  "move",
  "mut",
  "pub",
  "ref",
  "return",
  "self",
  "Self",
  "static",
  "struct",
  "super",
  "trait",
  "true",
  "type",
  "unsafe",
  "use",
  "where",
  "while",
]);

const RUST_TOKEN_PATTERN =
  /("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')|(\/\/[^\n]*|\/\*[\s\S]*?\*\/)|(\b[A-Za-z_]\w*!?\b)|(\b\d(?:[\d_]*)(?:\.\d[\d_]*)?\b)|([{}()[\];,.])|([<>+\-*/%=&|!?:]+)/g;

export type RepositorySourceToken = {
  type: string;
  start: number;
  end: number;
};

export function tokenizeRepositorySource(
  code: string,
  language: string,
): RepositorySourceToken[] {
  if (language !== "rust") {
    return [];
  }
  const tokens: RepositorySourceToken[] = [];
  const pattern = new RegExp(RUST_TOKEN_PATTERN.source, "g");
  let cursor = 0;
  for (let match = pattern.exec(code); match; match = pattern.exec(code)) {
    const value = match[0];
    const start = match.index;
    const end = start + value.length;
    pushPlainTokens(tokens, code, cursor, start);
    if (match[1]) {
      tokens.push({ type: "string", start, end });
    } else if (match[2]) {
      tokens.push({ type: "comment", start, end });
    } else if (match[3]) {
      tokens.push({
        type: RUST_KEYWORDS.has(value)
          ? "keyword"
          : isFunctionLike(code, end)
            ? "function"
            : "variable",
        start,
        end,
      });
    } else if (match[4]) {
      tokens.push({ type: "number", start, end });
    } else if (match[5]) {
      tokens.push({ type: "punctuation", start, end });
    } else {
      tokens.push({ type: "operator", start, end });
    }
    cursor = end;
  }
  pushPlainTokens(tokens, code, cursor, code.length);
  return tokens;
}

function isFunctionLike(code: string, end: number): boolean {
  for (let index = end; index < code.length; index++) {
    const character = code[index];
    if (character === "(") {
      return true;
    }
    if (character !== " " && character !== "\t") {
      return false;
    }
  }
  return false;
}

function pushPlainTokens(
  tokens: RepositorySourceToken[],
  code: string,
  start: number,
  end: number,
) {
  let runStart: number | null = null;
  for (let index = start; index < end; index++) {
    if (code[index] === "\n") {
      if (runStart !== null) {
        tokens.push({ type: "variable", start: runStart, end: index });
        runStart = null;
      }
    } else if (runStart === null) {
      runStart = index;
    }
  }
  if (runStart !== null) {
    tokens.push({ type: "variable", start: runStart, end });
  }
}
