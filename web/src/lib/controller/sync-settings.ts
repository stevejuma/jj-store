import { Client, pretty, type Model } from "../api/client.js";
import type { StoreLoadStatus } from "./contracts.js";
import type { operations } from "../api/generated.js";
export type SyncAction = NonNullable<
  operations["local_sync_control"]["requestBody"]
>["content"]["application/json"]["action"];
export interface SyncTransport {
  capture(signal?: AbortSignal): Promise<Model<"SyncBarrier">>;
  wait(
    barrier: Model<"SyncBarrier">,
    signal?: AbortSignal,
  ): Promise<Model<"LocalSyncWait">>;
  status(): Promise<Model<"LocalSyncStatus">>;
  control(action: SyncAction): Promise<Model<"LocalSyncStatus">>;
}
export interface SyncSettings {
  value: Model<"LocalSyncStatus"> | null;
  state: StoreLoadStatus;
  error: string;
  changing: boolean;
  wait: {
    barrier: Model<"SyncBarrier"> | null;
    state:
      | "idle"
      | "capturing"
      | "waiting"
      | "pending"
      | "reached"
      | "stopped"
      | "invalid"
      | "failed";
    error: string;
  };
}
export const emptySyncSettings = (): SyncSettings => ({
  value: null,
  state: "idle",
  error: "",
  changing: false,
  wait: { barrier: null, state: "idle", error: "" },
});
export class HttpSyncSettings implements SyncTransport {
  constructor(private client: Client) {}
  capture(signal?: AbortSignal) {
    return this.client.request<Model<"SyncBarrier">>(
      "local/sync/barrier",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: "{}",
        signal,
      },
    );
  }
  wait(barrier: Model<"SyncBarrier">, signal?: AbortSignal) {
    return this.client.request<Model<"LocalSyncWait">>(
      "local/sync/wait",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty({ version: 1, barrier, timeout_millis: 1000 }),
        signal,
      },
    );
  }
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
