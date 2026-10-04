import { validateProfileRoute } from "./repository-profile.js";
import { ApiError } from "../api/client.js";
import type { StoreRepositorySummary } from "./contracts.js";
import { HttpStoreDataSource } from "./http-store-data-source.js";
import { HttpLocalStoreDataSource } from "./http-local-store-data-source.js";
import { createStoreQueryClient } from "./store-query-client.js";
import type { StoreDataSource } from "./native-data-source.js";

export function repositorySource(
  repository: StoreRepositorySummary,
  token: string | null,
  session: string,
  denied: () => void,
  fetcher?: typeof fetch,
  queries?: import("@tanstack/svelte-query").QueryClient,
): StoreDataSource {
  if (repository.availability === "unavailable")
    throw new ApiError("local_folder_unavailable");
  if (repository.availability === "identity_changed")
    throw new ApiError("local_identity_changed");
  if (repository.profile === "git_sha1_jj045")
    return new HttpLocalStoreDataSource(
      repository.id,
      fetcher,
      queries,
      session,
    );
  if (repository.profile && repository.profile !== "native")
    throw new ApiError("unsupported_repository_profile");
  return new HttpStoreDataSource(
    repository.id,
    token,
    queries ?? createStoreQueryClient(),
    session,
    denied,
    fetcher,
    !queries,
  );
}

export function captureRepositoryActor(
  data: import("./production-store-state.svelte.js").ProductionStoreState,
  repository: string,
  head: import("./repository-profile.js").RepositoryHead,
) {
  if ("actor" in head && head.actor)
    data.repositories = data.repositories.map((r) =>
      r.id === repository ? { ...r, principal: head.actor! } : r,
    );
}
export function validateRepositoryRoute(
  data: import("./production-store-state.svelte.js").ProductionStoreState,
  route: import("./contracts.js").StoreRoute,
  source: StoreDataSource,
) {
  try {
    validateProfileRoute(route, source.profile ?? "native");
    return true;
  } catch (cause) {
    data.status.navigation = "failed";
    data.status.error =
      cause instanceof Error ? cause.message : "Invalid profile route";
    return false;
  }
}

export function configureRepositoryConnection(
  data: import("./production-store-state.svelte.js").ProductionStoreState,
  repository: string,
  captured: import("./repository-profile.js").RepositoryHead,
  capabilities: import("./repository-profile.js").RepositoryCapabilities,
  configure: (kinds: string[]) => void,
) {
  data.heads = [...captured.heads];
  data.head = captured;
  captureRepositoryActor(data, repository, captured);
  data.capabilities = capabilities;
  configure(capabilities.typed_mutations);
  data.conflictResolutionActions = capabilities.conflict_resolution_actions;
  data.conflictUploadBytes = capabilities.conflict_upload_bytes;
  data.searchAvailable = capabilities.search;
  return data.heads.length === 1 ? data.heads[0]! : null;
}

export function selectedRepository(
  data: import("./production-store-state.svelte.js").ProductionStoreState,
  route: import("./contracts.js").StoreRoute,
  repository: string,
) {
  const candidate = data.repositories.find((r) => r.id === repository.trim());
  if (!candidate) throw new ApiError("access_denied");
  if (route.repository === candidate.id)
    validateProfileRoute(route, candidate.profile ?? "native");
  return candidate;
}
