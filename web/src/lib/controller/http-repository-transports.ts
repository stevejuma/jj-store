import { HttpSearchSettings } from "./search-settings.js";
import {
  fullId,
  type Envelope,
  type Client,
  type Model,
} from "../api/client.js";
import { HttpStorageSettings } from "./storage-settings.js";
import { HttpSyncSettings } from "./sync-settings.js";
import { HttpWorkspaceLinks } from "./http-workspace-links.js";

/** Repository transports share the repository's admitted client. */
export class HttpRepositoryTransports {
  readonly searchSettings: HttpSearchSettings;
  readonly storage: HttpStorageSettings;
  readonly sync: HttpSyncSettings;
  readonly localWorkspaces: HttpWorkspaceLinks;
  constructor(private client: Client) {
    this.searchSettings = new HttpSearchSettings(client);
    this.storage = new HttpStorageSettings(client);
    this.sync = new HttpSyncSettings(client);
    this.localWorkspaces = new HttpWorkspaceLinks(client);
  }
  async queryInfo(operation: string) {
    const value = await this.client.request<
      Envelope<import("./history-query-types.js").HistoryQueryInfo>
    >("query-info", { at_op: fullId(operation) });
    if (value.snapshot.operation_id !== operation)
      throw new Error("Query metadata snapshot mismatch");
    return value.data;
  }
  searchStatus(_fresh = false) {
    return this.client.request<Model<"SearchStatus">>("search/status");
  }
  events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: Parameters<Client["events"]>[2],
  ) {
    return this.client.events(after, signal, deliver);
  }
}
