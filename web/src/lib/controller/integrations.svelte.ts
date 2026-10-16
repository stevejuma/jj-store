import { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
import type { Exact } from "../api/client.js";
import type { operations } from "../api/generated.js";
import type { NotificationContext } from "../notifications/notification-context.svelte.js";
export type IntegrationInventory = Model<"ForgeIntegrationInventory">;
export type IntegrationCommand = Exact<
  NonNullable<
    operations["local_integrations_command"]["requestBody"]
  >["content"]["application/json"]
>;
/** Installation owner; navigation never owns saved credentials or auth sessions. */
export class Integrations {
  data = $state<IntegrationInventory>();
  busy = $state(false);
  flow = $state<"pat" | "enterprise" | "oauth" | "">("");
  token = $state("");
  api = $state("");
  notifications?: NotificationContext;
  changed?: () => void;
  private timer?: ReturnType<typeof setTimeout>;
  private epoch = 0;
  constructor(
    readonly available: () => boolean,
    readonly client = new LocalClient(""),
  ) {}
  error(cause: unknown, episode = "account-command") {
    this.notifications?.service.publish({
      scope: "integrations",
      key: episode,
      title: "Integration action failed",
      message: cause instanceof Error ? cause.message : String(cause),
      severity: "error",
      lifetime: "dismiss",
    });
  }
  async refresh() {
    if (!this.available() || this.busy) return;
    const epoch = this.epoch;
    try {
      const data =
        await this.client.request<IntegrationInventory>("integrations");
      if (epoch === this.epoch)
        this.data = {
          ...data,
          authorization: this.data?.authorization ?? null,
        };
      this.notifications?.service.resolve("integrations", "account-refresh");
    } catch (cause) {
      this.error(cause, "account-refresh");
    }
  }
  async command(command: IntegrationCommand) {
    if (!this.available() || this.busy) return false;
    this.busy = true;
    const epoch = this.epoch;
    try {
      const data = await this.client.request<IntegrationInventory>(
        "integrations",
        command,
      );
      if (epoch !== this.epoch) return false;
      this.data = data;
      this.notifications?.service.resolve("integrations", "account-command");
      if (command.kind === "add_token") {
        this.token = "";
        this.api = "";
        this.flow = "";
      }
      if (
        ["add_token", "forget"].includes(command.kind) ||
        data.authorization?.state === "complete"
      ) {
        if (data.authorization?.state === "complete") this.flow = "";
        this.changed?.();
        this.notifications?.service.publish({
          scope: "integrations",
          key: "account-saved",
          title:
            command.kind === "forget"
              ? "Account forgotten"
              : "GitHub connected",
          message:
            command.kind === "forget"
              ? "Local credentials were removed."
              : "Credentials saved in your OS credential store.",
          severity: "info",
          lifetime: "timed",
        });
      }
      if (data.authorization?.state === "pending") this.schedule();
      else if (
        data.authorization &&
        ["denied", "expired", "failed"].includes(data.authorization.state)
      )
        this.error(
          new Error(
            `GitHub authorization ${data.authorization.state}. Start again to get a new code.`,
          ),
        );
      return true;
    } catch (cause) {
      if (epoch === this.epoch) this.error(cause);
      return false;
    } finally {
      this.busy = false;
    }
  }
  async begin() {
    this.flow = "oauth";
    await this.command({ kind: "begin_oauth" });
  }
  private schedule() {
    clearTimeout(this.timer);
    const auth = this.data?.authorization;
    if (!auth || auth.state !== "pending") return;
    this.timer = setTimeout(
      () => void this.command({ kind: "poll_oauth", session: auth.session }),
      Number(auth.interval) * 1000,
    );
  }
  async cancel() {
    clearTimeout(this.timer);
    // A new epoch discards an in-flight poll. Backend cancellation invalidates its lease.
    this.epoch++;
    const session = this.data?.authorization?.session;
    this.flow = "";
    this.token = "";
    this.api = "";
    if (this.data) this.data.authorization = null;
    if (session) {
      try {
        await this.client.request("integrations", {
          kind: "cancel_oauth",
          session,
        });
      } catch (cause) {
        this.error(cause);
      }
    }
  }
  destroy() {
    clearTimeout(this.timer);
    this.epoch++;
    this.token = "";
    this.client.close();
  }
}
