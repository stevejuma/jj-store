export interface PromptTag {
  start: number;
  end: number;
  name: string;
  label: string;
  closing: boolean;
  selfClosing: boolean;
}
export interface PromptElement {
  open: PromptTag;
  close?: PromptTag;
  children: PromptElement[];
}
export interface SourceRange {
  start: number;
  end: number;
}

/** Names and attributes are syntax, never a harness registry or executable HTML. */
export function promptTags(source: string): PromptTag[] {
  const pattern =
    /<(\/?)([\p{L}_:][\p{L}\p{N}_.:-]*)(?=[\s/>])((?:[^<>"']|"[^"]*"|'[^']*')*)>/gu;
  const tags: PromptTag[] = [];
  for (const match of source.matchAll(pattern)) {
    const tail = match[3]!,
      closing = !!match[1];
    if (closing ? !!tail.trim() : !!tail && !/^\s|^\/$/.test(tail)) continue;
    let slashes = 0;
    for (let at = match.index - 1; source[at] === "\\"; at--) slashes++;
    if (slashes % 2) continue;
    tags.push({
      start: match.index,
      end: match.index + match[0].length,
      name: match[2]!,
      label: match[0],
      closing,
      selfClosing: !closing && tail.trimEnd().endsWith("/"),
    });
  }
  return tags;
}

export function maskPromptTags(source: string, tags: readonly PromptTag[]) {
  let at = 0;
  const parts: string[] = [];
  for (const tag of tags) {
    parts.push(
      source.slice(at, tag.start),
      "x" + tag.label.slice(1).replace(/[^\r\n]/g, " "),
    );
    at = tag.end;
  }
  return parts.join("") + source.slice(at);
}

/** A mismatched subtree never hides text under a guessed closing boundary. */
export function matchPromptElements(
  tags: readonly PromptTag[],
): PromptElement[] {
  const roots: PromptElement[] = [],
    stack: PromptElement[] = [];
  const accept = (element: PromptElement) =>
    (stack.at(-1)?.children ?? roots).push(element);
  for (const tag of tags) {
    if (tag.closing) {
      if (stack.at(-1)?.open.name !== tag.name) {
        stack.length = 0;
        continue;
      }
      const element = stack.pop()!;
      element.close = tag;
      accept(element);
    } else if (tag.selfClosing) accept({ open: tag, children: [] });
    else {
      if (stack.length >= 64) return [];
      stack.push({ open: tag, children: [] });
    }
  }
  return roots;
}

export function outsideRanges(
  tags: readonly PromptTag[],
  ignored: readonly SourceRange[],
) {
  let at = 0;
  return tags.filter((tag) => {
    while (ignored[at] && ignored[at]!.end <= tag.start) at++;
    return !ignored[at] || tag.end <= ignored[at]!.start;
  });
}
