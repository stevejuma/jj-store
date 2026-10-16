import { localLink } from "../../appearance/local-link.js";
import { linkContent } from "../../links/site-links.js";
import type { CommunityCustomEmoji } from "../../community/contracts.js";

import type {
  CommunityMemberModel,
  CommunityRoomModel,
} from "../community-workspace/model.js";

import type {
  CommunityMarkdownSegment,
  CommunityMarkdownSyntaxOptions,
} from "./syntax.js";

import { linkSegmentFromHast, segmentToHast } from "./syntax-hast-links.js";

export type CommunityMarkdownHastNode = {
  type: string;
  value?: string;
  tagName?: string;
  properties?: Record<string, unknown>;
  children?: CommunityMarkdownHastNode[];
  position?: { start: { offset?: number } };
};

export type CommunityMarkdownTokenizer = (
  value: string,
  members: readonly CommunityMemberModel[],
  rooms: readonly CommunityRoomModel[],
  emojis: readonly CommunityCustomEmoji[],
  mentionPubkeys: readonly string[],
) => CommunityMarkdownSegment[];

export function applyCommunityMarkdownSyntax(
  tree: CommunityMarkdownHastNode,
  options: CommunityMarkdownSyntaxOptions,
  tokenize: CommunityMarkdownTokenizer,
  standaloneLinks: ReadonlyMap<number, string> = new Map(),
): void {
  transformChildren(tree, options, tokenize, []);
  // Replace the paragraph as a block, never insert a card into prose.
  if (tree.type === "root")
    tree.children = tree.children?.map((paragraph) => {
      const offset = paragraph.position?.start.offset;
      const href =
        offset === undefined ? undefined : standaloneLinks.get(offset);
      const link = paragraph.children?.[0];
      if (
        !href ||
        paragraph.tagName !== "p" ||
        paragraph.children?.length !== 1 ||
        link?.tagName !== "communityexternallink" ||
        href !== link.properties?.href
      )
        return paragraph;
      return {
        ...paragraph,
        tagName: "communitylinkpreview",
        properties: { href, label: linkContent(href) },
      };
    });
}

function transformChildren(
  node: CommunityMarkdownHastNode,
  options: CommunityMarkdownSyntaxOptions,
  tokenize: CommunityMarkdownTokenizer,
  ancestors: string[],
): void {
  if (!Array.isArray(node.children)) return;
  const currentAncestors =
    node.tagName === undefined ? ancestors : [...ancestors, node.tagName];
  const skipText = currentAncestors.some((tag) =>
    ["a", "code", "pre", "script", "style"].includes(tag),
  );

  const nextChildren: CommunityMarkdownHastNode[] = [];
  for (const child of node.children) {
    const mentionLink = linkSegmentFromHast(child, options.members());
    if (mentionLink) {
      nextChildren.push(segmentToHast(mentionLink));
      continue;
    }
    if (!skipText && child.tagName === "a" && !containsMedia(child)) {
      const text = (node: CommunityMarkdownHastNode): string =>
        node.value ?? (node.children ?? []).map(text).join("");
      const link = localLink(child.properties?.href, text(child));
      if (link) {
        nextChildren.push({
          type: "element",
          tagName: "communityexternallink",
          properties: {
            href: link.href,
            label: link.label,
            title: child.properties?.title,
          },
          children: child.children,
        });
        continue;
      }
    }
    if (child.type === "text" && typeof child.value === "string" && !skipText) {
      nextChildren.push(
        ...tokenize(
          child.value,
          options.members(),
          options.rooms(),
          options.emojis(),
          options.mentionPubkeys(),
        ).map(segmentToHast),
      );
      continue;
    }
    transformChildren(child, options, tokenize, currentAncestors);
    nextChildren.push(child);
  }
  node.children = nextChildren;
}

function containsMedia(node: CommunityMarkdownHastNode): boolean {
  return (
    ["img", "audio", "video"].includes(node.tagName ?? "") ||
    (node.children ?? []).some(containsMedia)
  );
}
