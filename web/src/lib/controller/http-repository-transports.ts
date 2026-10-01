import { HttpSearchSettings } from "./search-settings.js";
import type { Client } from "../api/client.js";
import { HttpSyncSettings } from "./sync-settings.js";
import { HttpWorkspaceLinks } from "./http-workspace-links.js";

/** Repository transports share the repository's admitted client. */
export class HttpRepositoryTransports {
  readonly searchSettings: HttpSearchSettings;
  readonly sync: HttpSyncSettings;
  readonly localWorkspaces: HttpWorkspaceLinks;
  constructor(private client: Client) {
    this.searchSettings = new HttpSearchSettings(client);
    this.sync = new HttpSyncSettings(client);
    this.localWorkspaces = new HttpWorkspaceLinks(client);
  }
  events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: Parameters<Client["events"]>[2],
  ) {
    return this.client.events(after, signal, deliver);
  }
}
