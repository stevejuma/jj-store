import {
  communityEntityLinkMatches,
  normalizeCommunityEntityHref,
} from "../../community/community-link-normalize.js";

import { parseCommunityEntityLink } from "../../community/community-links.js";

export type CommunityEntityMarkdownSegment = {
  kind: "entity";
  href: string;
  value: string;
};

export function communityMarkdownEntityMatches(value: string) {
  return communityEntityLinkMatches(value);
}

type NativeLinkNode = {
  type: string;
  url?: string;
  data?: { hName?: string; hProperties?: Record<string, unknown> };
  children?: NativeLinkNode[];
};

// Native routes are application links; tell Mira's file-link handler to emit a
// safe anchor so the later owned entity renderer can retain the full identity.
export function remarkNativeEntityLinks() {
  return (tree: NativeLinkNode) => {
    function visit(node: NativeLinkNode) {
      const href =
        node.type === "link" && node.url
          ? normalizeCommunityEntityHref(node.url)
          : undefined;
      if (href)
        node.data = {
          ...node.data,
          hName: "a",
          hProperties: { ...node.data?.hProperties, href },
        };
      node.children?.forEach(visit);
    }
    visit(tree);
  };
}

export function entitySegmentFromHref(
  href: string | undefined,
  label: string,
): CommunityEntityMarkdownSegment | undefined {
  if (!href) return undefined;
  const normalized = normalizeCommunityEntityHref(href);
  if (!normalized || !parseCommunityEntityLink(normalized)) return undefined;
  return {
    kind: "entity",
    href: normalized,
    value: label === href || label === normalized ? normalized : label,
  };
}
