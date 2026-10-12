import { withAgentDefinition } from "./server-agent-administration-types.js";
import { ApiError, pretty } from "../api/client.js";
import {
  recoverAdministration,
  rejectedAdministration,
  administrationNames,
  administrationPrincipals,
} from "./server-agent-administration-recovery.js";
import { ServerAdministrationClient } from "../api/server-administration-client.js";
import type {
  AdministrationPending,
  IdentityBinding,
  OperatorAccess,
  ServerAgentConfiguration,
  ServerAgentDefinition,
  ServerAgentReport,
} from "./server-agent-administration-types.js";
export class ServerAgentAdministration {
  access = $state<OperatorAccess | null>(null);
  report = $state<ServerAgentReport | null>(null);
  draft = $state<ServerAgentConfiguration | null>(null);
  baseRevision = $state("");
  private original = $state("null");
  identities = $state<IdentityBinding[]>([]);
  profiles = $state<Record<string, string>>({});
  onboarding = $state({ principal: "", credential: "", repository: "" });
  open = $state(false);
  directoryOpen = false;
  busy = $state(false);
  error = $state("");
  notice = $state("");
  pending = $state<AdministrationPending | null>(null);
  private client: ServerAdministrationClient;
  private refreshing = false;
  private timer: ReturnType<typeof setInterval>;
  constructor(
    readonly available: () => boolean,
    token: () => string | null,
    fetcher: typeof fetch = fetch,
  ) {
    this.client = new ServerAdministrationClient(available, token, fetcher);
    this.timer = setInterval(() => {
      if (!this.available() || this.busy) return;
      if (this.open) void this.refresh();
      else if (this.directoryOpen) void this.discover();
    }, 5000);
  }
  get dirty() {
    return (
      !!this.report && pretty($state.snapshot(this.draft)) !== this.original
    );
  }
  get conflict() {
    return this.dirty && this.baseRevision !== this.report?.revision;
  }
  updateDefinition(definition: ServerAgentDefinition) {
    if (!this.draft || this.pending || this.busy) return;
    this.draft.agents = withAgentDefinition(
      $state.snapshot(this.draft.agents),
      $state.snapshot(definition),
    );
  }
  private request<T>(path: string, body?: unknown): Promise<T> {
    return this.client.request<T>(path, body);
  }
  async discover() {
    if (!this.available() || this.refreshing) return;
    this.refreshing = true;
    try {
      this.access = await this.request<OperatorAccess>("operator/access");
    } catch {
      /* Operator controls are opt-in; ordinary agent execution is independent. */
    } finally {
      this.refreshing = false;
    }
  }
  async refresh() {
    if (!this.available() || this.refreshing) return;
    this.refreshing = true;
    try {
      const access = await this.request<OperatorAccess>("operator/access");
      this.access = access;
      if (access.agents) {
        const report = await this.request<ServerAgentReport>("operator/agents");
        const dirty = this.dirty;
        this.report = report;
        if (!dirty && !this.pending) {
          this.draft = structuredClone(report.desired);
          this.baseRevision = report.revision;
          this.original = pretty(report.desired);
        }
        this.profiles = await administrationNames(
          this.request.bind(this),
          administrationPrincipals(report),
        );
      }
      if (access.identities) {
        this.identities = await this.request<IdentityBinding[]>(
          "directory/identities",
        );
        this.profiles = {
          ...this.profiles,
          ...(await administrationNames(this.request.bind(this), [
            ...new Set(this.identities.map((b) => b.principal)),
          ])),
        };
      }
    } catch (cause) {
      if (this.client.lifetime.signal.aborted) return;
      this.error =
        cause instanceof ApiError
          ? cause.code
          : "Unable to refresh server access.";
    } finally {
      this.refreshing = false;
    }
  }
  reset() {
    if (!this.pending && !this.busy) {
      this.draft = structuredClone(
        $state.snapshot(this.report?.desired ?? null),
      );
      this.baseRevision = this.report?.revision ?? "";
      this.original = pretty(this.report?.desired ?? null);
      this.error = "";
    }
  }
  useLatestRevision() {
    if (this.report && !this.pending && !this.busy) {
      this.baseRevision = this.report.revision;
      this.error = "";
    }
  }
  async save() {
    if (
      !this.report ||
      !this.access?.agents ||
      this.pending ||
      this.busy ||
      this.conflict
    )
      return;
    this.pending = {
      kind: "configuration",
      body: {
        expected_revision: this.baseRevision,
        configuration: structuredClone($state.snapshot(this.draft)),
      },
    };
    await this.submit();
  }
  async enroll(principal: string, credential: string) {
    if (!this.access?.identities || this.pending || this.busy) return;
    this.pending = {
      kind: "identity",
      path: "identities",
      body: {
        request_id: crypto.randomUUID().replaceAll("-", ""),
        principal,
        credential,
        expires_at: null,
      },
    };
    await this.submit();
  }
  async revoke(token: string) {
    if (!this.access?.identities || this.pending || this.busy) return;
    this.pending = {
      kind: "identity",
      path: "identity-revoke",
      body: { request_id: crypto.randomUUID().replaceAll("-", ""), token },
    };
    await this.submit();
  }
  async grant(repository: string, principal: string, credential: string) {
    if (this.pending || this.busy || !/^[0-9a-f]{32}$/.test(repository)) return;
    this.busy = true;
    this.error = "";
    try {
      const expected = await this.request(`repos/${repository}/agent-access`);
      this.pending = {
        kind: "grant",
        repository,
        body: {
          request_id: crypto.randomUUID().replaceAll("-", ""),
          expected,
          principal,
          credential,
        },
      };
    } catch (cause) {
      if (this.client.lifetime.signal.aborted) return;
      this.error =
        cause instanceof ApiError
          ? cause.code
          : "Unable to read repository permission.";
    } finally {
      this.busy = false;
    }
    if (this.pending) await this.submit();
  }
  private async submit() {
    if (!this.pending || this.busy) return;
    this.busy = true;
    this.error = "";
    this.notice = "";
    const intent = $state.snapshot(this.pending);
    try {
      if (intent.kind === "configuration") {
        this.report = await this.request<ServerAgentReport>(
          "operator/agents",
          intent.body,
        );
        this.draft = structuredClone($state.snapshot(this.report.desired));
        this.baseRevision = this.report.revision;
        this.original = pretty(this.report.desired);
        this.notice =
          "Saved. Use Reload saved settings to activate configuration and sharing changes.";
      } else {
        const result = await this.request<{
          request_id: string;
          published?: boolean;
        }>(
          intent.kind === "identity"
            ? `directory/${intent.path}`
            : `repos/${intent.repository}/agent-access`,
          intent.body,
        );
        if (
          result.request_id !== intent.body.request_id ||
          (intent.kind === "grant" && !result.published)
        )
          throw new ApiError("administration_outcome_unknown");
        this.notice =
          intent.kind === "grant"
            ? "Repository Write access granted. Agent sharing remains separate."
            : "Installation identity updated. Repository and agent permissions remain separate.";
      }
      this.pending = null;
      await this.refresh();
    } catch (cause) {
      if (this.client.lifetime.signal.aborted) return;
      this.error =
        cause instanceof ApiError
          ? cause.code
          : "Response lost. Check the original outcome before continuing.";
      if (cause instanceof ApiError && rejectedAdministration(cause.code))
        this.pending = null;
    } finally {
      this.busy = false;
    }
  }
  async checkOutcome() {
    if (!this.pending || this.busy) return;
    this.busy = true;
    const intent = $state.snapshot(this.pending);
    try {
      const recovered = await recoverAdministration(
        this.request.bind(this),
        intent,
      );
      if (recovered.report) this.report = recovered.report;
      if (recovered.error) {
        this.error = recovered.error;
        return;
      }
      if (recovered.report) {
        this.draft = structuredClone(recovered.report.desired);
        this.baseRevision = recovered.report.revision;
        this.original = pretty(recovered.report.desired);
      }
      this.pending = null;
      this.error = "";
      this.notice = "Original outcome confirmed.";
      await this.refresh();
    } catch (cause) {
      if (this.client.lifetime.signal.aborted) return;
      this.error =
        cause instanceof ApiError
          ? cause.code
          : "Unable to check the original outcome.";
    } finally {
      this.busy = false;
    }
  }
  destroy() {
    this.client.close();
    clearInterval(this.timer);
    this.access = null;
    this.report = null;
    this.draft = null;
    this.pending = null;
    this.identities = [];
    this.onboarding = { principal: "", credential: "", repository: "" };
    this.profiles = {};
    this.open = false;
    this.directoryOpen = false;
    this.error = "";
    this.notice = "";
  }
}
