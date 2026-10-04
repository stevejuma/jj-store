import { Client, pretty, type Model } from "../api/client.js";
import type { StoreLoadStatus } from "./contracts.js";
import type { operations } from "../api/generated.js";
export type StorageCommand = NonNullable<
  operations["local_storage_command"]["requestBody"]
>["content"]["application/json"];
export type StorageAction = StorageCommand["action"];
export type StorageOperation = Model<"StorageOperation">;
export interface StorageTransport {
  overview(): Promise<Model<"StorageOverview">>;
  command(command: StorageCommand): Promise<StorageOperation>;
  operation(id: string): Promise<StorageOperation>;
}
export interface StorageSettings {
  value: Model<"StorageOverview"> | null;
  operation: StorageOperation | null;
  pending: StorageCommand | null;
  state: StoreLoadStatus;
  error: string;
  changing: boolean;
}
export const emptyStorageSettings = (): StorageSettings => ({
  value: null,
  operation: null,
  pending: null,
  state: "idle",
  error: "",
  changing: false,
});
export class HttpStorageSettings implements StorageTransport {
  constructor(private client: Client) {}
  overview() {
    return this.client.request<Model<"StorageOverview">>("local/storage");
  }
  operation(id: string) {
    return this.client.request<StorageOperation>(
      `local/storage/operations/${id}`,
    );
  }
  command(command: StorageCommand) {
    return this.client.request<StorageOperation>(
      "local/storage/operations",
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(command),
      },
    );
  }
}
