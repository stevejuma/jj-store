import { linkContent } from "../../links/site-links.js";
import { localLink } from "../../appearance/local-link.js";

type Node = {
  type: string;
  url?: string;
  children?: Node[];
  position?: { start: { offset?: number }; end: { offset?: number } };
};

/** Remember source paragraph offsets across Mira's safe HTML sanitization. */
export function remarkStandaloneLinks(options: {
  source: () => string;
  paragraphs: Map<number, string>;
}) {
  return (tree: Node) => {
    options.paragraphs.clear();
    const source = options.source();
    for (const paragraph of tree.children ?? []) {
      if (paragraph.type !== "paragraph" || paragraph.children?.length !== 1)
        continue;
      const link = paragraph.children[0]!;
      const start = link.position?.start.offset;
      const end = link.position?.end.offset;
      const offset = paragraph.position?.start.offset;
      if (
        link.type !== "link" ||
        !link.url ||
        start === undefined ||
        end === undefined ||
        offset === undefined
      )
        continue;
      const authored = source.slice(start, end);
      if (
        (authored === link.url || authored === `<${link.url}>`) &&
        linkContent(link.url)
      )
        options.paragraphs.set(offset, localLink(link.url, "")!.href);
    }
  };
}
