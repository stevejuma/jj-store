import {
  validateReloadStatus,
  validateReloadOutcome,
} from "./runtime-reload-validation.js";
import { ServerAdministrationClient } from "../api/server-administration-client.js";
import { ApiError } from "../api/client.js";
import type { Model } from "../api/client.js";
export type ReloadStatus = Model<"RuntimeReloadStatus">;
export type ReloadCommand = Model<"RuntimeReloadCommand">;
export type ReloadOutcome = Model<"RuntimeReloadOutcome">;
export class RuntimeReload {
  value = $state<ReloadStatus | null>(null);
  outcome = $state<ReloadOutcome | null>(null);
  pending = $state<ReloadCommand | null>(null);
  busy = $state(false);
  error = $state("");
  open = $state(false);
  returnFocus?: () => void;
  private epoch = 0;
  private disposed = false;
  private views = 0;
  private timer: ReturnType<typeof setInterval>;
  private client: ServerAdministrationClient;
  constructor(
    readonly available: () => boolean,
    token: () => string | null,
    fetcher: typeof fetch = fetch,
    private readonly onApplied: () => Promise<void> = async () => {},
  ) {
    this.client = new ServerAdministrationClient(
      available,
      token,
      fetcher,
      true,
    );
    this.timer = setInterval(() => {
      if (this.views && this.value && this.available() && !this.busy)
        void this.observe();
    }, 5000);
  }
  watch() {
    this.views++;
    void this.observe();
    return () => {
      this.views--;
    };
  }
  async observe() {
    if (this.disposed || !this.available() || this.busy) return;
    const epoch = ++this.epoch;
    try {
      const value = await this.client.request<ReloadStatus>(
        "configuration/reload",
      );
      validateReloadStatus(value);
      if (this.available() && epoch === this.epoch) {
        this.value = value;
        if (value.original) {
          this.pending = value.original;
          this.outcome = value.outcome ?? null;
        }
      }
    } catch (cause) {
      if (
        !this.disposed &&
        this.available() &&
        epoch === this.epoch &&
        cause instanceof ApiError &&
        cause.code === "configuration_reload_denied"
      )
        this.value = null;
    }
  }
  async run() {
    if (
      this.disposed ||
      !this.available() ||
      !this.value ||
      !this.value.supported.length ||
      this.busy ||
      this.pending
    )
      return;
    this.pending = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      expected_revision: this.value.saved_revision,
      domains: [...this.value.supported],
    };
    await this.request(false);
  }
  async check() {
    if (this.pending && !this.busy) await this.request(true);
  }
  private async request(observe: boolean) {
    const command = this.pending;
    if (this.disposed || !command || !this.available() || this.busy) return;
    this.busy = true;
    this.error = "";
    this.epoch++;
    let accepted = false;
    try {
      let outcome = await this.client.request<ReloadOutcome>(
        observe ? "configuration/reload/outcome" : "configuration/reload",
        command,
      );
      validateReloadOutcome(outcome, command);
      accepted = true;
      while (this.available() && outcome.state === "running") {
        this.outcome = outcome;
        await new Promise<void>((resolve) => setTimeout(resolve, 700));
        this.client.lifetime.signal.throwIfAborted();
        outcome = await this.client.request<ReloadOutcome>(
          "configuration/reload/outcome",
          command,
        );
        validateReloadOutcome(outcome, command);
      }
      if (this.disposed || !this.available()) return;
      this.outcome = outcome;
      if (outcome.state === "unknown") {
        this.error =
          "Reload outcome unknown. Check the original outcome before sending another reload.";
      } else {
        this.pending = null;
        try {
          await this.onApplied();
        } catch {
          this.error =
            "Reload completed. Refresh to read the latest effective settings.";
        }
      }
    } catch (cause) {
      if (this.disposed || !this.available()) return;
      if (
        !observe &&
        !accepted &&
        cause instanceof ApiError &&
        [
          "configuration_reload_changed",
          "invalid_configuration_reload",
          "configuration_reload_denied",
          "configuration_reload_busy",
          "configuration_reload_intent_conflict",
        ].includes(cause.code)
      ) {
        this.pending = null;
        this.error =
          "Reload was not accepted. Refresh and review the saved configuration.";
      } else {
        this.error =
          "Reload outcome unknown. The original request is retained; check its outcome.";
      }
    } finally {
      this.busy = false;
    }
    await this.observe();
  }
  destroy() {
    this.disposed = true;
    clearInterval(this.timer);
    this.epoch++;
    this.client.close();
  }
}
