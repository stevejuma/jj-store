import { Client, pretty, type Model } from "../api/client.js";
import type { StoreLoadStatus } from "./contracts.js";
import type { operations } from "../api/generated.js";
export type SyncAction = NonNullable<
  operations["local_sync_control"]["requestBody"]
>["content"]["application/json"]["action"];
export interface SyncTransport {
  status(): Promise<Model<"LocalSyncStatus">>;
  control(action: SyncAction): Promise<Model<"LocalSyncStatus">>;
}
export interface SyncSettings {
  value: Model<"LocalSyncStatus"> | null;
  state: StoreLoadStatus;
  error: string;
  changing: boolean;
}
export const emptySyncSettings = (): SyncSettings => ({
  value: null,
  state: "idle",
  error: "",
  changing: false,
});
export class HttpSyncSettings implements SyncTransport {
  constructor(private client: Client) {}
  status() {
    return this.client.request<Model<"LocalSyncStatus">>("local/sync");
  }
  control(action: SyncAction) {
    return this.client.request<Model<"LocalSyncStatus">>(
      "local/sync/control",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty({ version: 1, action }),
      },
    );
  }
}
