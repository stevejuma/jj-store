import { authenticatedAgentFetch } from "../connections/agent-transport.js";
import { createDirectory } from "./directory.svelte.js";
import { ServerAgentAdministration } from "./server-agent-administration.svelte.js";
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
  active: () => boolean = () => true,
) {
  const directory = createDirectory({
    available: () =>
      active() &&
      data.directorySupported &&
      (data.accessMode === "local_trusted" || token() !== null),
    token,
    denied,
    fetcher,
    freshness,
  });
  const agentAdministration = new ServerAgentAdministration(
    () => active() && data.accessMode === "authenticated" && token() !== null,
    token,
    fetcher,
  );
  const capture = createProductionCaptureManagement(data, fetcher, freshness);
  const execution = new AgentExecution(
    () =>
      active() &&
      data.executionSupported &&
      (data.accessMode === "local_trusted" || token() !== null),
    (input, init) =>
      data.accessMode === "local_trusted"
        ? (fetcher ?? fetch)(input, init)
        : authenticatedAgentFetch(fetcher, token)(input, init),
    (repository) =>
      data.repositories.find((r) => r.id === repository)?.principal ?? "",
    freshness,
    () => data.accessMode === "local_trusted",
  );
  const configuration = createServerSettings({
    available: () =>
      active() && data.accessMode === "local_trusted" && data.settingsSupported,
    client: new LocalClient("", fetcher),
  });
  return {
    capture,
    execution,
    configuration,
    directory,
    agentAdministration,
    destroy() {
      freshness?.destroy();
      capture.destroy();
      execution.destroy();
      configuration.destroy();
      directory.destroy();
      agentAdministration.destroy();
    },
  };
}
