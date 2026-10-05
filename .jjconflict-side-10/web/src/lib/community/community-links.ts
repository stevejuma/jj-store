import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../controller/store-route-codec.js";

import type { StoreRoute } from "../controller/contracts.js";

export type CommunityEntityLink = {
  kind: "channel" | "message" | "project" | "repo" | "task";
  route: StoreRoute;
  href: string;
  label?: string;
};

export type CommunityEntityLinkIcon =
  | "message-circle"
  | "hash"
  | "folder-kanban"
  | "folder-git-2"
  | "git-commit-horizontal"
  | "history"
  | "circle-dot";

export type CommunityEntityLinkKindAttr =
  | "message"
  | "channel"
  | "project"
  | "repository"
  | "log"
  | "oplog"
  | "commit"
  | "task";

export function trimLinkCandidate(value: string): string {
  return value.replace(/[.,;!?]+$/u, "");
}

export function communityEntityLinkHref(link: CommunityEntityLink): string {
  return link.href;
}

export function parseCommunityEntityLink(
  value: string,
): CommunityEntityLink | undefined {
  let hash = trimLinkCandidate(value.trim());
  if (/^https?:/u.test(hash)) {
    try {
      hash = new URL(hash).hash;
    } catch {
      return;
    }
  }
  const parsed = storeRouteFromHash(hash);
  if (parsed.kind !== "route") return;
  const route = parsed.route;
  const href = storeRouteToHash(route);
  if (!href) return;
  const kind = route.threadId
    ? route.replyId
      ? "message"
      : "channel"
    : route.taskId
      ? "task"
      : route.projectId
        ? "project"
        : "repo";
  return { kind, route, href };
}
