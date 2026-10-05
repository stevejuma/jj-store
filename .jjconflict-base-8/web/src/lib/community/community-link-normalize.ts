import {
  communityEntityLinkHref,
  parseCommunityEntityLink,
  trimLinkCandidate,
  type CommunityEntityLink,
  type CommunityEntityLinkIcon,
  type CommunityEntityLinkKindAttr,
} from "./community-links.js";

export function normalizeCommunityEntityHref(
  value: string,
): string | undefined {
  const link = parseCommunityEntityLink(value);
  return link ? communityEntityLinkHref(link) : undefined;
}

export function communityEntityLinkMatches(
  content: string,
): Array<{ value: string; href: string; start: number; end: number }> {
  const matches: Array<{
    value: string;
    href: string;
    start: number;
    end: number;
  }> = [];
  for (const match of content.matchAll(
    /(?:https?:\/\/[^\s<>]*?)?#\/(?:repos|projects)[^\s<>)]*/gu,
  )) {
    const value = trimLinkCandidate(match[0]);
    const href = normalizeCommunityEntityHref(value);
    if (href)
      matches.push({
        value,
        href,
        start: match.index,
        end: match.index + value.length,
      });
  }
  return matches;
}

export function communityEntityLinkPresentation(link: CommunityEntityLink): {
  linkKind: CommunityEntityLinkKindAttr;
  icon: CommunityEntityLinkIcon;
  fallbackLabel: string;
} {
  const route = link.route;
  if (link.kind === "channel")
    return { linkKind: "channel", icon: "hash", fallbackLabel: "Thread" };
  if (link.kind === "message")
    return {
      linkKind: "message",
      icon: "message-circle",
      fallbackLabel: "Message",
    };
  if (link.kind === "project")
    return {
      linkKind: "project",
      icon: "folder-kanban",
      fallbackLabel: "Project",
    };
  if (link.kind === "task")
    return { linkKind: "task", icon: "circle-dot", fallbackLabel: "Task" };
  if (route.commitId)
    return {
      linkKind: "commit",
      icon: "git-commit-horizontal",
      fallbackLabel: route.commitId.slice(0, 12),
    };
  if (route.page === "log" || route.page === "oplog")
    return {
      linkKind: route.page,
      icon: "history",
      fallbackLabel: route.page === "log" ? "JJ Log" : "Operation Log",
    };
  return {
    linkKind: "repository",
    icon: "folder-git-2",
    fallbackLabel: "Repository",
  };
}
