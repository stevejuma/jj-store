import { ApiError } from "../../api/client.js";
import { isLocal } from "../repository-profile.js";
import type { LocalReadSnapshot } from "../repository-profile.js";
import type { localStackReferences } from "./local-references.js";
import type { WorkspaceSnapshot, WorkspaceTargetBranch } from "./types.js";

/** No named combination, applied flags, physical edits or mutation capabilities are inferred. */
export function productionStackSnapshot(
  projection: ReturnType<typeof localStackReferences>,
  pin: LocalReadSnapshot,
  folder: string,
  target: WorkspaceTargetBranch | null,
  revision: number,
  nextCursor?: string,
): WorkspaceSnapshot {
  return {
    repository: pin.repository,
    folder,
    profile: "git_sha1_jj045",
    availability: "ready",
    revision,
    readKey: JSON.stringify([
      pin.repository,
      String(pin.generation),
      pin.operation,
      target ? [target.name, target.commit] : null,
    ]),
    base: target?.name ?? null,
    target,
    targetOptions: projection.targets,
    active: null,
    workspaces: [],
    stacks: projection.stacks
      .filter((stack) => stack.name !== target?.name)
      .map((stack) => ({ ...stack, base: target?.name ?? "" })),
    bookmarks: projection.bookmarks.filter(
      (bookmark) => bookmark.name !== target?.name,
    ),
    branchesNextCursor: nextCursor,
    branchUnavailableReasons: projection.unavailable,
    conflicts: [],
    capabilities: [],
    canUndo: false,
  };
}

export function verifiedLocalStackPin(
  source: import("../native-data-source.js").StoreDataSource,
  binding: import("../../api/client.js").Model<"LocalBinding">,
  captured: import("../repository-profile.js").RepositoryHead,
  operation: string,
): LocalReadSnapshot {
  if (
    source.profile !== "git_sha1_jj045" ||
    !isLocal(captured) ||
    captured.repository !== source.repository ||
    binding.repository !== source.repository ||
    binding.availability !== "ready" ||
    !binding.path ||
    String(captured.generation) !== String(binding.generation)
  )
    throw new ApiError("snapshot_mismatch");
  return {
    version: 1,
    profile: "git_sha1_jj045",
    repository: captured.repository,
    generation: captured.generation,
    operation,
    heads: [...captured.heads],
  };
}
