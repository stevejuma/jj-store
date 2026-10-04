import type { CaptureSaveSettings } from "./capture-configuration.js";
import type { Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import type {
  CaptureControl,
  CaptureImport,
  CaptureRescan,
  CaptureTransport,
} from "./capture-types.js";
/** Installation management deliberately has no repository or bearer token. */
export class HttpCaptureManagement implements CaptureTransport {
  constructor(private client = new LocalClient("")) {}
  close() {
    this.client.close();
  }
  status(signal?: AbortSignal) {
    return this.client.request<Model<"CaptureStatus">>(
      "capture",
      undefined,
      signal,
    );
  }
  settings(signal?: AbortSignal) {
    return this.client.request<Model<"LocalCaptureSettings">>(
      "capture/settings",
      undefined,
      signal,
    );
  }
  saveSettings(command: CaptureSaveSettings) {
    return this.client.request<Model<"LocalCaptureSettings">>(
      "capture/settings",
      command,
    );
  }
  sources(cursor?: string, signal?: AbortSignal) {
    return this.client.request<Model<"CaptureSourcePage">>(
      `capture/sources?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`,
      undefined,
      signal,
    );
  }
  control(command: CaptureControl) {
    return this.client.request<Model<"CaptureControlOutcome">>(
      "capture/control",
      command,
    );
  }
  rescan(command: CaptureRescan) {
    return this.client.request<Model<"CaptureRescanOutcome">>(
      "capture/rescan",
      command,
    );
  }
  import(command: CaptureImport) {
    return this.client.request<Model<"CaptureImportOutcome">>(
      "capture/import",
      command,
    );
  }
  inspect(id: string, signal?: AbortSignal) {
    return this.client.request<Model<"CaptureImportStatus">>(
      `capture/imports?request_id=${encodeURIComponent(id)}`,
      undefined,
      signal,
    );
  }
}
