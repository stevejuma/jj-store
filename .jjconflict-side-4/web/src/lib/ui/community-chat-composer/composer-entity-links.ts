import { EditorView } from "@codemirror/view";

import {
  communityEntityLinkMatches,
  normalizeCommunityEntityHref,
} from "../../community/community-link-normalize.js";

import { parseCommunityEntityLink } from "../../community/community-links.js";

import type { CommunityEntityLink } from "../../community/community-links.js";

export function communityComposerEntityMatches(value: string): Array<{
  from: number;
  to: number;
  href: string;
  link: CommunityEntityLink;
}> {
  const matches: Array<{
    from: number;
    to: number;
    href: string;
    link: CommunityEntityLink;
  }> = [];
  for (const match of communityEntityLinkMatches(value)) {
    const link = parseCommunityEntityLink(match.href);
    if (!link) continue;
    const before = value.slice(0, match.start);
    const label = /\[([^\]\n]*)\]\($/u.exec(before);
    const from = label ? match.start - label[0].length : match.start;
    const to = label && value[match.end] === ")" ? match.end + 1 : match.end;
    matches.push({
      from,
      to,
      href: match.href,
      link: label ? { ...link, label: label[1] } : link,
    });
  }
  return matches;
}

export function normalizedCommunityPasteHref(
  clipboard: string,
): string | undefined {
  const trimmed = clipboard.trim();
  if (trimmed === "" || trimmed.includes("\n")) return undefined;
  const href = normalizeCommunityEntityHref(trimmed);
  return href && href !== trimmed ? href : undefined;
}

export function communityComposerPasteHandler() {
  return EditorView.domEventHandlers({
    paste(event, view) {
      const text = event.clipboardData?.getData("text/plain") ?? "";
      const href = normalizedCommunityPasteHref(text);
      if (!href) return false;
      event.preventDefault();
      view.dispatch(view.state.replaceSelection(href));
      return true;
    },
  });
}
