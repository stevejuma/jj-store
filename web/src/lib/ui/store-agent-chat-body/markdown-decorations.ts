import { markdownLanguage } from "@codemirror/lang-markdown";
import { RangeSetBuilder } from "@codemirror/state";
import { Decoration, type DecorationSet } from "@codemirror/view";
import {
  getAtxHeadingMarkerRange,
  getMarkdownLinkTextRange,
  getIndentLineLayout,
} from "@lapismd/mira/codemirror";
import { PathLabel } from "./path-label.js";

/** Fixed reading decorations: selection never reveals Markdown source markers. */
export function readingDecorations(
  text: string,
  ignored: readonly { from: number; to: number }[] = [],
  options: {
    fileLink?: (destination: string) => boolean;
    paths?: readonly (readonly [string, string])[];
  } = {},
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
        name === "ListMark" &&
        node.node.parent?.parent?.name === "BulletList"
      ) {
        const from = text.lastIndexOf("\n", node.from - 1) + 1;
        const end = text.indexOf("\n", node.to);
        const layout = getIndentLineLayout(
          text.slice(from, end < 0 ? undefined : end),
        );
        if (layout?.listKind === "ul" && layout.markerTo !== undefined)
          marks.push(
            {
              from: node.from,
              to: from + layout.markerTo,
              value: Decoration.mark({
                class: `cm-formatting-list cm-formatting-list-ul cm-formatting-list-bullet cm-list-${layout.depth}`,
              }),
            },
            {
              from: node.from,
              to: node.to,
              value: Decoration.mark({ class: "cm-meta" }),
            },
          );
      }
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
        const url = node.node.getChild("URL");
        const destination = url
          ? text.slice(url.from, url.to).replace(/^<|>$/g, "")
          : "";
        const external = /^https?:\/\//i.test(destination);
        const file = !external && options.fileLink?.(destination);
        if (external || file) {
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
                [file ? "data-chat-file-link" : "data-chat-link"]: destination,
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
  for (const [path, label] of [...(options.paths ?? [])].sort(
    (a, b) => b[0].length - a[0].length,
  )) {
    if (!path) continue;
    let from = text.indexOf(path);
    while (from >= 0) {
      const to = from + path.length;
      if (
        !hidden.some((range) => from < range.to && to > range.from) &&
        !marks.some(
          (mark) => mark.value.spec.widget && from < mark.to && to > mark.from,
        )
      )
        marks.push({
          from,
          to,
          value: Decoration.replace({ widget: new PathLabel(label) }),
        });
      from = text.indexOf(path, to);
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
