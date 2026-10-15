import { ForgeReadCache } from "./read-cache.js";
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
  readonly cache = new ForgeReadCache();
  private identity = "";
  private credential = "";
  private generation = 0;
  private scope() {
    const c = this.owner.data?.connection;
    return JSON.stringify([
      this.owner.accounts.scope,
      this.owner.data?.detection?.account_id,
      this.owner.data?.detection?.readiness,
      c?.provider,
      c?.api_url,
      c?.repository,
      c?.source_repository,
    ]);
  }
  invalidate() {
    this.cache.clear();
    this.generation++;
  }
  close() {
    this.invalidate();
    this.credential = "";
    this.identity = "";
  }
  refreshChecks() {
    this.invalidate();
    return this.owner.act(
      this.owner.activeRun
        ? { kind: "jobs", run: this.owner.activeRun }
        : { kind: "checks" },
    );
  }
  accountsChanged() {
    this.invalidate();
    // Hide actions immediately; preserve authored drafts and exact pending effects.
    if (this.owner.data?.detection?.provider === "github") {
      this.owner.retainDraft();
      this.owner.data.detection.readiness = "unconfigured";
      this.owner.plan = undefined;
    }
    return this.owner.refresh();
  }
  constructor(readonly owner: ForgeController) {}
  call(action: ForgeAction) {
    const result = this.requests.then(async () => {
      if (this.owner.closed) throw new Error("Forge view closed");
      this.assertReady(action);
      const identity = this.scope();
      if (identity !== this.identity || this.credential !== this.owner.token) {
        this.invalidate();
        this.identity = identity;
        this.credential = this.owner.token;
      }
      const revision =
        "number" in action
          ? (this.owner.data?.pull_requests.find(
              (p) => p.number === action.number,
            )?.sha ??
            this.owner.selected?.sha ??
            "")
          : "";
      const key = this.cache.key(action, revision);
      const cached = this.cache.get(key);
      if (cached) return cached;
      const generation = this.generation;
      const credential = this.owner.token;
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
      if (identity !== this.scope() || credential !== this.owner.token)
        throw new Error(
          ["submit", "recover", "restack"].includes(action.kind) ||
            (action.kind === "pr" &&
              !["checks", "jobs", "logs", "merge_status"].includes(
                action.action.kind,
              ))
            ? "Forge access changed; original action outcome unknown. Check the original action before continuing."
            : "Forge access changed; refresh to load the selected account.",
        );
      if (generation === this.generation)
        this.cache.put(key, $state.snapshot(response));
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
