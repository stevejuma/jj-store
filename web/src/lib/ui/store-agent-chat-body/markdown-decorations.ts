import { markdownLanguage } from "@codemirror/lang-markdown";
import { RangeSetBuilder } from "@codemirror/state";
import { Decoration, type DecorationSet } from "@codemirror/view";
import {
  getAtxHeadingMarkerRange,
  getMarkdownLinkTextRange,
} from "@lapismd/mira/codemirror";

/** Fixed reading decorations: selection never reveals Markdown source markers. */
export function readingDecorations(
  text: string,
  ignored: readonly { from: number; to: number }[] = [],
): {
  decorations: DecorationSet;
  hidden: DecorationSet;
} {
  const marks: { from: number; to: number; value: Decoration }[] = [];
  const hidden: { from: number; to: number }[] = [];
  markdownLanguage.parser.parse(text).iterate({
    enter(node) {
      if (ignored.some((r) => node.from >= r.from && node.to <= r.to))
        return false;
      const name = node.name;
      if (
        [
          "HeaderMark",
          "EmphasisMark",
          "CodeMark",
          "StrikethroughMark",
        ].includes(name)
      ) {
        const heading =
          name === "HeaderMark"
            ? getAtxHeadingMarkerRange(text.slice(node.from), node.from)
            : null;
        hidden.push(heading ?? { from: node.from, to: node.to });
      }
      const className = /^ATXHeading/.test(name)
        ? `chat-heading chat-heading-${name.at(-1)}`
        : name === "StrongEmphasis"
          ? "chat-strong"
          : name === "Emphasis"
            ? "chat-emphasis"
            : name === "InlineCode"
              ? "chat-inline-code"
              : name === "Strikethrough"
                ? "chat-strike"
                : undefined;
      if (className)
        marks.push({
          from: node.from,
          to: node.to,
          value: Decoration.mark({ class: className }),
        });
      if (name === "Link") {
        const source = text.slice(node.from, node.to);
        const link = /^\[([^\]]+)\]\(([^\s)]+)(?:\s+[^)]*)?\)$/.exec(source);
        if (link && /^https?:\/\//i.test(link[2])) {
          const range = getMarkdownLinkTextRange(source, node.from, node.to);
          if (!range) return;
          const { from: start, to: end } = range;
          hidden.push(
            { from: node.from, to: start },
            { from: end, to: node.to },
          );
          marks.push({
            from: start,
            to: end,
            value: Decoration.mark({
              class: "chat-link",
              attributes: {
                role: "link",
                tabindex: "0",
                "data-chat-link": link[2],
              },
            }),
          });
          return false;
        }
      }
    },
  });
  const hiddenBuilder = new RangeSetBuilder<Decoration>();
  for (const range of hidden.sort((a, b) => a.from - b.from)) {
    if (range.to > range.from) {
      const value = Decoration.replace({});
      hiddenBuilder.add(range.from, range.to, value);
      marks.push({ ...range, value });
    }
  }
  return {
    decorations: Decoration.set(
      marks.map((m) => m.value.range(m.from, m.to)),
      true,
    ),
    hidden: hiddenBuilder.finish(),
  };
}
