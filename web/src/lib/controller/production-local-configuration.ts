import { LocalClient } from "../api/local-client.js";
import { createServerSettings } from "./server-settings.svelte.js";
import { createProductionCaptureManagement } from "./production-capture-management.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
export function createLocalConfiguration(
  data: ProductionStoreState,
  fetcher?: typeof fetch,
) {
  const capture = createProductionCaptureManagement(data, fetcher);
  const configuration = createServerSettings({
    available: () =>
      data.accessMode === "local_trusted" && data.settingsSupported,
    client: new LocalClient("", fetcher),
  });
  return {
    capture,
    configuration,
    destroy() {
      capture.destroy();
      configuration.destroy();
    },
  };
}
