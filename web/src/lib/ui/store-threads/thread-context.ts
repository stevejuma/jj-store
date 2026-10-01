import type { Model } from "../../api/client.js";
import type { StoreRoute } from "../../controller/contracts.js";
import { storeRouteToHash } from "../../controller/store-route-codec.js";
import {
  parseCommunityEntityLink,
  type CommunityEntityLink,
} from "../../community/community-links.js";
export function baselineLinks(
  baseline: Model<"ThreadCodeAnchorV1"> | null,
  route: StoreRoute,
): { label: string; link: CommunityEntityLink }[] {
  if (!baseline) return [];
  const rows: { label: string; route: StoreRoute }[] = [];
  if (baseline.profile === "native") {
    for (const operation of baseline.anchor.operations)
      rows.push({
        label: `Operation ${operation.slice(0, 12)}`,
        route: {
          ...route,
          page: "oplog",
          threadId: null,
          replyId: null,
          snapshotOperationId: operation,
          inspectedOperationId: operation,
        },
      });
    for (const commit of baseline.anchor.commits)
      rows.push({
        label: `Commit ${commit.slice(0, 12)}`,
        route: {
          ...route,
          page: "log",
          threadId: null,
          replyId: null,
          commitId: commit,
        },
      });
  } else {
    for (const evidence of baseline.anchor.operations)
      rows.push({
        label: `Operation ${evidence.operation.slice(0, 12)}`,
        route: {
          ...route,
          repository: evidence.repository,
          page: "oplog",
          threadId: null,
          replyId: null,
          snapshotOperationId: evidence.operation,
          inspectedOperationId: evidence.operation,
        },
      });
    for (const commit of baseline.anchor.commits)
      if (commit.commit.profile === "native_v1")
        rows.push({
          label: `Commit ${commit.commit.id.slice(0, 12)}`,
          route: {
            ...route,
            repository: commit.evidence.repository,
            page: "log",
            threadId: null,
            replyId: null,
            snapshotOperationId: commit.evidence.operation,
            commitId: commit.commit.id,
          },
        });
  }
  return rows.flatMap((row) => {
    const hash = storeRouteToHash(row.route),
      link = hash ? parseCommunityEntityLink(hash) : undefined;
    return link ? [{ label: row.label, link }] : [];
  });
}
export function activityLinks(
  events: readonly Model<"ThreadEventItemV1">[],
  route: StoreRoute,
) {
  const latest = new Map<
    string,
    NonNullable<Model<"ThreadEventItemV1">["link"]>
  >();
  for (const event of events)
    if (event.link && !latest.has(event.link.id))
      latest.set(event.link.id, event.link);
  return [...latest.values()]
    .filter((l) => l.active)
    .flatMap((l) => {
      const target = l.target;
      if (target.type === "code") return baselineLinks(target.anchor, route);
      const hash = storeRouteToHash({
        ...route,
        threadId: null,
        replyId: null,
        page: target.type === "session" ? "agents" : "reviews",
        agentSessionId: target.type === "session" ? target.session_id : null,
        reviewThreadId: target.type === "review" ? target.review_id : null,
      });
      const link = hash ? parseCommunityEntityLink(hash) : undefined;
      return link
        ? [
            {
              label:
                target.type === "session"
                  ? `Agent session ${target.session_id.slice(0, 12)}`
                  : `Review ${target.review_id.slice(0, 12)}`,
              link,
            },
          ]
        : [];
    });
}
