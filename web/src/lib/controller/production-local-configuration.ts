import { createDirectory } from "./directory.svelte.js";
import { LocalClient } from "../api/local-client.js";
import { AgentExecution } from "./agent-execution.svelte.js";
import { createServerSettings } from "./server-settings.svelte.js";
import { createProductionCaptureManagement } from "./production-capture-management.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
export function createLocalConfiguration(
  data: ProductionStoreState,
  fetcher?: typeof fetch,
  freshness?: import("./application-freshness.js").ApplicationFreshness,
  token: () => string | null = () => null,
  denied?: () => void,
) {
  const directory = createDirectory({
    available: () =>
      data.directorySupported &&
      (data.accessMode === "local_trusted" || token() !== null),
    token,
    denied,
    fetcher,
    freshness,
  });
  const capture = createProductionCaptureManagement(data, fetcher, freshness);
  const execution = new AgentExecution(
    () => data.accessMode === "local_trusted" && data.executionSupported,
    fetcher,
    (repository) =>
      data.repositories.find((r) => r.id === repository)?.principal ?? "",
    freshness,
  );
  const configuration = createServerSettings({
    available: () =>
      data.accessMode === "local_trusted" && data.settingsSupported,
    client: new LocalClient("", fetcher),
  });
  return {
    capture,
    execution,
    configuration,
    directory,
    destroy() {
      freshness?.destroy();
      capture.destroy();
      execution.destroy();
      configuration.destroy();
      directory.destroy();
    },
  };
}
