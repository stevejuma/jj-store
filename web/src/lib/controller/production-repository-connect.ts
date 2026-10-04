import {
  repositorySource,
  selectedRepository,
  configureRepositoryConnection,
} from "./repository-source.js";
import { ApiError } from "../api/client.js";
import { connectionDeniedMessage, storeErrorMessage } from "./store-error.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
export async function connectRepositorySource(
  options: {
    data: ProductionStoreState;
    history: ReturnType<typeof createStoreRouteHistory>;
    token(): string | null;
    setToken(token: string | null): void;
    source(): StoreDataSource | null;
    setSource(source: StoreDataSource): void;
    nextVersion(): number;
    version(): number;
    nextSession(): number;
    current(ticket: number, source: StoreDataSource): boolean;
    disconnect(message?: string, denied?: boolean, preserve?: boolean): void;
    configure(
      actions: import("../api/client.js").Model<"Capabilities">["typed_mutations"],
    ): void;
    follow(source: StoreDataSource): void;
    connected(): Promise<void>;
    projects: { restore(): Promise<void> };
    reader: { restorePage(): Promise<void> };
    fetcher?: typeof fetch;
    sources?: import("./repository-sources.js").RepositorySources;
  },
  repository: string,
  token: string | null,
) {
  options.disconnect("", false, true);
  options.data.status.connection = "connecting";
  const ticket = options.nextVersion(),
    requestedRoute = options.history.route;
  const credential =
    options.data.accessMode === "authenticated"
      ? (token ?? options.token())
      : null;
  if (options.data.accessMode === "authenticated" && !credential)
    throw new ApiError("bearer_required");
  try {
    const candidate = selectedRepository(
      options.data,
      options.history.route,
      repository,
    );
    const active = options.sources
      ? options.sources.acquire(candidate, credential)
      : repositorySource(
          candidate,
          credential,
          `session-${options.nextSession()}`,
          () => {
            if (options.source() === active)
              options.disconnect(
                connectionDeniedMessage(options.data.accessMode),
                true,
              );
          },
          options.fetcher,
        );
    options.setSource(active);
    const capabilities = await active.capabilities();
    const captured = await active.snapshot();
    if (
      !options.current(ticket, active) ||
      options.history.route !== requestedRoute
    )
      return;
    const selected = configureRepositoryConnection(
      options.data,
      active.repository,
      captured,
      capabilities,
      options.configure,
    );
    if (options.data.accessMode === "authenticated")
      options.setToken(credential);
    options.history.connect(active.repository, selected);
    options.data.status.connection = "connected";
    options.data.status.repositories = "ready";
    options.follow(active);
    await options.connected();
    if (options.history.route.page === "projects")
      await options.projects.restore();
    else if (
      options.history.route.page !== "threads" &&
      options.history.route.snapshotOperationId
    )
      await options.reader.restorePage();
  } catch (cause) {
    if (ticket !== options.version()) return;
    const message = storeErrorMessage(cause);
    options.disconnect(message);
    options.data.status.connection = "failed";
  }
}
