import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
import type { RepositoryLogRevision } from "../../web/src/lib/projects/repository-log.js";
import type { RevisionDetail } from "../../web/src/lib/projects/presentation.js";

export function inMemoryStoreDetail(
  route: StoreRoute,
  revisions: readonly RepositoryLogRevision[],
  detail: RevisionDetail,
): RevisionDetail | undefined {
  const selected =
    route.page === "oplog" ? route.inspectedOperationId : route.commitId;
  if (!selected) return undefined;
  return revisions.find((revision) => revision.eventId === selected)?.empty
    ? undefined
    : detail;
}
