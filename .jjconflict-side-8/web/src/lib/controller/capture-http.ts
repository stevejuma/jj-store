import type { CaptureSaveSettings } from "./capture-configuration.js";
import type { Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import { captureReadBusy } from "./capture-errors.js";
import type {
  CaptureControl,
  CaptureImport,
  CaptureRescan,
  CaptureTransport,
} from "./capture-types.js";
/** Installation management deliberately has no repository or bearer token. */
export class HttpCaptureManagement implements CaptureTransport {
  constructor(private client = new LocalClient("")) {}
  private async observe<T>(path: string, signal?: AbortSignal): Promise<T> {
    const lifetime = AbortSignal.any([
      this.client.closed.signal,
      signal ?? new AbortController().signal,
    ]);
    for (let attempt = 0; ; attempt++) {
      lifetime.throwIfAborted();
      try {
        return await this.client.request<T>(path, undefined, lifetime);
      } catch (cause) {
        if (!captureReadBusy(cause) || attempt === 4) throw cause;
        await new Promise<void>((resolve, reject) => {
          const cancel = () => {
            clearTimeout(timer);
            reject(lifetime.reason);
          };
          const timer = setTimeout(
            () => {
              lifetime.removeEventListener("abort", cancel);
              resolve();
            },
            (attempt + 1) * 150,
          );
          lifetime.addEventListener("abort", cancel, { once: true });
          if (lifetime.aborted) cancel();
        });
      }
    }
  }
  close() {
    this.client.close();
  }
  status(signal?: AbortSignal) {
    return this.observe<Model<"CaptureStatus">>("capture", signal);
  }
  settings(signal?: AbortSignal) {
    return this.observe<Model<"LocalCaptureSettings">>(
      "capture/settings",
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
    return this.observe<Model<"CaptureSourcePage">>(
      `capture/sources?limit=100${cursor ? `&cursor=${encodeURIComponent(cursor)}` : ""}`,
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
    return this.observe<Model<"CaptureImportStatus">>(
      `capture/imports?request_id=${encodeURIComponent(id)}`,
      signal,
    );
  }
}
