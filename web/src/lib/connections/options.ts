import type { HostEnvironment } from "../host/contracts.js";
import type { StoreRouteBrowser } from "../controller/store-route-browser.js";
export interface ServerConnectionOptions {
  host?: HostEnvironment;
  browser?: StoreRouteBrowser;
  storage?: Storage;
  fetcher?: typeof fetch;
}
