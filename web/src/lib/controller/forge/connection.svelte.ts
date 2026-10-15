import type { ForgeController } from "./controller.svelte.js";
import type { ForgeAction } from "./types.js";
/** Discovery has no publication authority; one repository owns its setup notice. */
export class ForgeConnectionLifecycle {
  draft = $state({
    account: "",
    remote: "",
    targetRemote: "",
    repository: "",
    target: "",
  });
  initialized = false;
  private requests: Promise<unknown> = Promise.resolve();
  constructor(readonly owner: ForgeController) {}
  call(action: ForgeAction) {
    const result = this.requests.then(async () => {
      if (this.owner.closed) throw new Error("Forge view closed");
      this.assertReady(action);
      const response = await this.owner.transport({
        version: 1,
        token: this.owner.token || null,
        ...(this.owner.accounts.managed &&
        this.owner.data?.detection?.account_id
          ? { account_id: this.owner.data.detection.account_id }
          : {}),
        action,
      });
      if (this.owner.closed) throw new Error("Forge view closed");
      return response;
    });
    this.requests = result.catch(() => undefined);
    return result;
  }
  get ready() {
    return this.owner.accounts.managed
      ? ["connected", "legacy"].includes(
          this.owner.data?.detection?.readiness ?? "",
        )
      : Boolean(this.owner.data?.connection);
  }
  assertReady(action: ForgeAction) {
    if (
      this.owner.accounts.managed &&
      !this.ready &&
      !["detect", "bind_account"].includes(action.kind)
    )
      throw new Error(
        "Configure forge access in Integrations before using pull requests.",
      );
  }
  initialize() {
    if (this.initialized || !this.owner.data?.detection) return;
    this.initialized = true;
    const connection = this.owner.data.connection;
    this.draft = {
      account: this.owner.data.detection.account_id ?? "",
      remote: connection?.remote ?? "",
      targetRemote: connection?.target_remote ?? "",
      repository: connection?.repository ?? "",
      target: connection?.target ?? "",
    };
  }
  async discover() {
    if (!this.owner.accounts.managed) return true;
    const result = await this.owner.call({
      kind: "detect",
      auto_connect: !this.owner.pending,
    });
    this.owner.receive(result);
    const detection = result.detection;
    const notifications = this.owner.notifications;
    const scope = `forge-setup:${this.owner.accounts.scope ?? "repository"}`;
    if (detection?.provider === "github" && !this.owner.ready) {
      notifications?.service.publish({
        scope,
        key: "detected",
        title: "GitHub detected",
        message:
          detection.readiness === "reconnect_required"
            ? "Reconnect your GitHub account to access pull requests."
            : "Configure access to use pull requests for this repository.",
        severity: "info",
        lifetime: "condition",
        action: this.owner.accounts.openSettings
          ? {
              id: "configure-forge",
              label: "Configure access",
              run: this.owner.accounts.openSettings,
            }
          : undefined,
      });
    } else notifications?.service.resolve(scope, "detected");
    return this.owner.ready;
  }
}
