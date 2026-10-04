import {
  maskPromptTags,
  matchPromptElements,
  outsideRanges,
  promptTags,
  type PromptElement,
  type SourceRange,
} from "./prompt-element-ranges.js";

type Point = { offset: number; line: number; column: number };
export interface PromptMarkdownNode {
  type: string;
  value?: string;
  children?: PromptMarkdownNode[];
  position?: { start: Point; end: Point };
  data?: { hName?: string; hProperties?: Record<string, unknown> };
}
type Parser = (source: string, file: unknown) => PromptMarkdownNode;
export const PROMPT_ELEMENT_MARKER = "jj-store:prompt-element:";

function codeRanges(root: PromptMarkdownNode): SourceRange[] {
  const ranges: SourceRange[] = [];
  function visit(node: PromptMarkdownNode) {
    if (
      node.position &&
      (node.type === "code" ||
        node.type === "inlineCode" ||
        (node.type === "html" && /^<(?:!|\?)/.test(node.value ?? "")))
    ) {
      ranges.push({
        start: node.position.start.offset,
        end: node.position.end.offset,
      });
    } else node.children?.forEach(visit);
  }
  visit(root);
  return ranges.sort((a, b) => a.start - b.start);
}

/** Wrap only the parser: Mira's complete remark transforms still run afterwards. */
export function remarkPromptElements(this: { parser?: unknown }) {
  if (typeof this.parser !== "function")
    throw Error("Prompt elements require a Markdown parser");
  const parse = this.parser as Parser;
  this.parser = (source: string, file: unknown) => {
    const candidates = promptTags(source);
    if (!candidates.length) return parse(source, file);
    const root = parse(maskPromptTags(source, candidates), file);
    const tags = outsideRanges(candidates, codeRanges(root));
    if (!tags.length) return parse(source, file);
    const elements = matchPromptElements(tags);
    const lineStarts = [0];
    for (let i = 0; i < source.length; i++)
      if (source[i] === "\n") lineStarts.push(i + 1);
    const point = (offset: number): Point => {
      let lo = 0,
        hi = lineStarts.length;
      while (lo + 1 < hi) {
        const mid = (lo + hi) >>> 1;
        if (lineStarts[mid]! <= offset) lo = mid;
        else hi = mid;
      }
      return { offset, line: lo + 1, column: offset - lineStarts[lo]! + 1 };
    };
    // Resolve document-wide Markdown references even when definitions cross sections.
    const definitions = (root.children ?? [])
      .filter(
        (n) =>
          ["definition", "footnoteDefinition"].includes(n.type) && n.position,
      )
      .map((n) =>
        source.slice(n.position!.start.offset, n.position!.end.offset),
      )
      .join("\n\n");
    function markdown(start: number, end: number) {
      const edits = tags.filter((t) => t.start >= start && t.end <= end);
      let at = start;
      const parts: string[] = [],
        insertions: number[] = [];
      for (const tag of edits) {
        parts.push(source.slice(at, tag.start), "\\");
        insertions.push(tag.start - start + insertions.length);
        at = tag.start;
      }
      parts.push(source.slice(at, end));
      const text = parts.join("");
      const parsed = parse(
        text + (definitions ? "\n\n" + definitions : ""),
        file,
      );
      const children = (parsed.children ?? []).filter(
        (n) => !n.position || n.position.start.offset < text.length,
      );
      const offset = (local: number) => {
        let lo = 0,
          hi = insertions.length;
        while (lo < hi) {
          const mid = (lo + hi) >>> 1;
          if (insertions[mid]! < local) lo = mid + 1;
          else hi = mid;
        }
        return start + local - lo;
      };
      function relocate(node: PromptMarkdownNode) {
        if (node.position)
          node.position = {
            start: point(offset(node.position.start.offset)),
            end: point(offset(node.position.end.offset)),
          };
        node.children?.forEach(relocate);
      }
      children.forEach(relocate);
      return children;
    }
    function build(
      start: number,
      end: number,
      sections: readonly PromptElement[],
    ): PromptMarkdownNode[] {
      const children: PromptMarkdownNode[] = [];
      let at = start;
      for (const section of sections) {
        children.push(...markdown(at, section.open.start));
        const stop = section.close?.end ?? section.open.end;
        children.push({
          type: "agentPromptElement",
          data: {
            hName: "div",
            hProperties: {
              title: PROMPT_ELEMENT_MARKER + section.open.label,
            },
          },
          children: section.close
            ? build(section.open.end, section.close.start, section.children)
            : [],
          position: { start: point(section.open.start), end: point(stop) },
        });
        at = stop;
      }
      children.push(...markdown(at, end));
      return children;
    }
    root.children = build(0, source.length, elements);
    return root;
  };
}
