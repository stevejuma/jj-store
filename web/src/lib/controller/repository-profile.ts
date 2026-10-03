import {
  ApiError,
  fullId,
  pretty,
  type Head,
  type Model,
  type Snapshot,
} from "../api/client.js";
export type RepositoryProfile = "native" | "git_sha1_jj045";
export type LocalHead = Model<"LocalRepositorySnapshot"> & {
  profile: "git_sha1_jj045";
};
export type RepositoryHead = Head | LocalHead;
export type LocalReadSnapshot = Model<"LocalCodeSnapshot"> & {
  profile: "git_sha1_jj045";
  heads: string[];
};
export type CodeSnapshot = Snapshot | LocalReadSnapshot;
export const isLocal = (
  value: RepositoryHead | CodeSnapshot,
): value is LocalHead | LocalReadSnapshot =>
  "profile" in value && value.profile === "git_sha1_jj045";
export const snapshotRepository = (value: RepositoryHead | CodeSnapshot) =>
  "repository" in value ? value.repository : value.repo_id;
export const snapshotOperation = (value: CodeSnapshot) =>
  "operation" in value ? value.operation : value.operation_id;
export function codeId(value: string, profile: RepositoryProfile = "native") {
  return fullId(value, profile === "native" ? 128 : 40);
}
export function validateProfileRoute(
  route: import("./contracts.js").StoreRoute,
  profile: RepositoryProfile,
) {
  for (const id of [route.commitId, route.evolutionCommitId])
    if (id) codeId(id, profile);
}
export function sameCodeSnapshot(a: CodeSnapshot, b: CodeSnapshot) {
  if (isLocal(a) || isLocal(b))
    return (
      isLocal(a) &&
      isLocal(b) &&
      a.repository === b.repository &&
      String(a.generation) === String(b.generation) &&
      "operation" in a &&
      "operation" in b &&
      a.operation === b.operation
    );
  return (
    a.repo_id === b.repo_id &&
    a.epoch === b.epoch &&
    String(a.published_seq) === String(b.published_seq) &&
    String(a.manifest_revision) === String(b.manifest_revision) &&
    a.operation_id === b.operation_id &&
    pretty(a.heads) === pretty(b.heads)
  );
}
export type LocalCapabilities = Model<"LocalRepositoryCapabilities"> & {
  read_api: boolean;
  browsing_windows: boolean;
  events: boolean;
  index_tasks: false;
  typed_mutations: string[];
  conflict_resolution_actions: string[];
  conflict_upload_bytes: 0;
  local_workspace_links: false;
  local_folder_picker: false;
  local_storage_management: false;
  local_sync_management: false;
  local_search_management: false;
};
export type RepositoryCapabilities = Model<"Capabilities"> | LocalCapabilities;
export function validateLocalHead(
  value: Model<"LocalRepositorySnapshot">,
  repository: string,
): LocalHead {
  if (
    value.profile !== "git_sha1_jj045" ||
    value.repository !== repository ||
    value.version !== 1 ||
    BigInt(value.generation) <= 0n ||
    value.heads.length > 256
  )
    throw new ApiError("snapshot_mismatch");
  for (const id of value.heads) fullId(id);
  if (value.metadata_commit) fullId(value.metadata_commit, 40);
  if (value.actor) fullId(value.actor, 32);
  return { ...value, profile: "git_sha1_jj045" };
}

export const unavailableNativeManagement = {
  local_workspace_links: false,
  local_folder_picker: false,
  local_storage_management: false,
  local_sync_management: false,
  local_search_management: false,
} as const;
