import { LocalClient } from "../api/local-client.js";
import { HttpCaptureManagement } from "./capture-http.js";
import { createCaptureManagement } from "./capture-management.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export function createProductionCaptureManagement(
  data: ProductionStoreState,
  fetcher?: typeof fetch,
  freshness?: import("./application-freshness.js").ApplicationFreshness,
) {
  return createCaptureManagement({
    freshness,
    transport: new HttpCaptureManagement(new LocalClient("", fetcher)),
    available: () =>
      data.accessMode === "local_trusted" &&
      data.storageReady &&
      data.captureSupported,
  });
}
