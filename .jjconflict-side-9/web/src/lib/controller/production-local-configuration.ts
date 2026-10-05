import { LocalClient } from "../api/local-client.js";
import { createServerSettings } from "./server-settings.svelte.js";
import { createProductionCaptureManagement } from "./production-capture-management.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
export function createLocalConfiguration(
  data: ProductionStoreState,
  fetcher?: typeof fetch,
  freshness?: import("./application-freshness.js").ApplicationFreshness,
) {
  const capture = createProductionCaptureManagement(data, fetcher, freshness);
  const configuration = createServerSettings({
    available: () =>
      data.accessMode === "local_trusted" && data.settingsSupported,
    client: new LocalClient("", fetcher),
  });
  return {
    capture,
    configuration,
    destroy() {
      freshness?.destroy();
      capture.destroy();
      configuration.destroy();
    },
  };
}
