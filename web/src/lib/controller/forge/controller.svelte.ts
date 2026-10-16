import { ForgeInline } from "./inline.svelte.js";
import { ForgeConnectionLifecycle } from "./connection.svelte.js";
import {
  ChecksPresentation,
  summarizeChecks,
} from "./checks-presentation.svelte.js";
import { ForgePolling } from "./polling.svelte.js";
import { ForgeEffects } from "./effects.svelte.js";
import type {
  ForgeTransport,
  ForgeAction,
  ForgeConnection,
  ForgeResponse,
  ForgePr,
  ForgePrAction,
  ForgePlan,
} from "./types.js";
import type { NotificationContext } from "../../notifications/notification-context.svelte.js";

/** One repository owner. Explicit tokens and authored edits are memory-only. */
export class ForgeController {
  data = $state<ForgeResponse>();
  selected = $state<ForgePr>();
  plan = $state<ForgePlan>();
  open = $state(false);
  surface = $state<"column" | "inline">("column");
  detailsExpanded = $state(true);
  mergeMethod = $state("squash");
  readonly inline = new ForgeInline(this);
  readonly checksView = new ChecksPresentation();
  busy = $state(false);
  stale = $state(false);
  token = $state("");
  title = $state("");
  body = $state("");
  draftDirty = $state(false);
  logs = $state("");
  restack = $state<ForgeResponse["restack"]>();
  checks = $state<ForgeResponse["checks"]>();
  ci = $state<Record<string, NonNullable<ForgeResponse["checks"]>>>({});
  activeRun = $state("");
  merges = $state<Record<string, NonNullable<ForgeResponse["merge"]>>>({});
  get merge() {
    return this.selected
      ? this.merges[String(this.selected.number)]
      : undefined;
  }
  receive(response: ForgeResponse, replaceInventory = false) {
    const inventory = new Map(
      (replaceInventory ? [] : (this.data?.pull_requests ?? [])).map((pr) => [
        pr.number,
        pr,
      ]),
    );
    for (const pr of response.pull_requests) inventory.set(pr.number, pr);
    this.merges = { ...this.merges, ...response.merges };
    this.data = {
      ...this.data,
      ...response,
      connection: response.connection ?? this.data?.connection ?? null,
      pull_requests: [...inventory.values()],
      receipt: response.receipt ?? this.data?.receipt ?? null,
      restack: response.restack ?? this.data?.restack ?? null,
      operation: response.operation ?? this.data?.operation ?? null,
    };
  }
  #watched = new Map<string, number>();
  #selectionEpoch = 0;
  watch(branch: string) {
    this.#watched.set(branch, (this.#watched.get(branch) ?? 0) + 1);
    return () => {
      const count = (this.#watched.get(branch) ?? 1) - 1;
      if (count) this.#watched.set(branch, count);
      else this.#watched.delete(branch);
    };
  }
  checkSummary(branch: string) {
    return summarizeChecks(this, branch);
  }
  pending = $state<ForgeAction>();
  notifications?: NotificationContext;
  #drafts = new Map<
    ForgePr["number"],
    { title: string; body: string; original: string }
  >();
  #original = "";
  #closed = false;
  get closed() {
    return this.#closed;
  }
  watchedBranches() {
    return [...this.#watched.keys()];
  }
  readonly #polling = new ForgePolling(this);
  readonly #effects = new ForgeEffects(this);
  readonly connection = new ForgeConnectionLifecycle(this);
  get ready() {
    return this.connection.ready;
  }
  constructor(
    readonly transport: ForgeTransport,
    readonly dispose?: () => void,
    readonly restacked?: (operation: string) => void,
    readonly accounts: {
      managed?: boolean;
      scope?: string;
      openSettings?: () => void;
    } = {},
  ) {}
  pr(branch: string) {
    if (this.accounts.managed && !this.ready) return undefined;
    return (
      this.data?.pull_requests.find(
        (p) => p.head === branch && p.state === "open",
      ) ?? this.data?.pull_requests.find((p) => p.head === branch)
    );
  }
  error(error: unknown, episode: string) {
    this.stale = true;
    this.notifications?.service.publish({
      scope: this.notifications.scope,
      key: `forge:${episode}`,
      title: "Forge action failed",
      message: error instanceof Error ? error.message : String(error),
      severity: "error",
      lifetime: "dismiss",
    });
  }
  call(action: ForgeAction) {
    return this.connection.call(action);
  }
  refresh() {
    return this.#polling.refresh();
  }
  ensure() {
    return this.#polling.refresh(false);
  }
  refreshChecks() {
    return this.connection.refreshChecks();
  }
  accountsChanged() {
    return this.connection.accountsChanged();
  }
  async configure(connection: ForgeConnection) {
    await this.run({ kind: "configure", connection });
    await this.refresh();
  }
  select(branch: string, surface: "column" | "inline" = "column") {
    this.inline.leave();
    this.retainDraft();
    this.surface = surface;
    this.detailsExpanded = true;
    this.open = surface === "column";
    this.plan = undefined;
    this.checks = undefined;
    this.logs = "";
    const pr = this.pr(branch);
    this.selected = pr;
    this.activeRun = "";
    this.checksView.expanded = false;
    const epoch = ++this.#selectionEpoch;
    if (pr) {
      this.accept(pr);
      void this.detail(pr, epoch);
    }
  }
  private async detail(pr: ForgePr, epoch: number) {
    try {
      const response = await this.call({
        kind: "pull_request",
        number: pr.number,
      });
      if (epoch !== this.#selectionEpoch || this.selected?.number !== pr.number)
        return;
      const latest = response.pull_requests[0];
      if (latest) {
        this.selected = latest;
        if (!this.draftDirty) this.accept(latest);
      }
      if (this.data?.capabilities.checks)
        await this.loadSelectedChecks(latest ?? pr, epoch);
    } catch (error) {
      if (!this.#closed && epoch === this.#selectionEpoch)
        this.error(error, `detail:${pr.number}`);
    }
  }
  private async loadSelectedChecks(pr: ForgePr, epoch: number) {
    try {
      const response = await this.call({
        kind: "pr",
        request_id: crypto.randomUUID(),
        number: pr.number,
        action: { kind: "checks" },
      });
      if (epoch !== this.#selectionEpoch || this.selected?.number !== pr.number)
        return;
      if (response.checks) {
        if (!this.checks && !this.activeRun) this.checks = response.checks;
        this.ci = { ...this.ci, [pr.head]: response.checks };
      }
    } catch (error) {
      if (!this.#closed && epoch === this.#selectionEpoch)
        this.error(error, `checks:${pr.number}`);
    }
  }
  accept(pr: ForgePr) {
    this.selected = pr;
    const draft = this.#drafts.get(pr.number);
    this.title = draft?.title ?? pr.title;
    this.body = draft?.body ?? pr.body;
    this.#original = draft?.original ?? pr.body;
    this.draftDirty = Boolean(draft);
  }
  retainDraft() {
    if (this.selected && this.draftDirty)
      this.#drafts.set(this.selected.number, {
        title: this.title,
        body: this.body,
        original: this.#original,
      });
  }
  async preview(
    branch: string,
    ready = false,
    surface: "column" | "inline" = "column",
  ) {
    if (surface === "column") this.inline.leave();
    this.surface = surface;
    this.open = surface === "column";
    this.retainDraft();
    this.selected = undefined;
    this.restack = undefined;
    this.checks = undefined;
    this.logs = "";
    return this.run({ kind: "preview", bookmark: branch, draft: !ready });
  }
  async previewRestack(branch: string) {
    this.open = true;
    this.plan = undefined;
    await this.run({ kind: "restack_preview", bookmark: branch });
  }
  async applyRestack() {
    if (this.restack)
      await this.run({
        kind: "restack",
        plan: $state.snapshot(this.restack.plan),
      });
  }
  async edit() {
    if (!this.selected) return;
    const number = this.selected.number;
    const success = await this.act({
      kind: "edit",
      title: this.title,
      body: this.body,
      expected_body: this.#original,
    });
    if (success) {
      this.#drafts.delete(number);
      this.draftDirty = false;
      const latest = this.data?.pull_requests.find(
        (pr) => pr.number === number,
      );
      if (latest) this.accept(latest);
    } else this.retainDraft();
  }
  async act(action: ForgePrAction) {
    if (!this.selected) return false;
    if (action.kind === "jobs") this.activeRun = action.run;
    if (action.kind === "checks") this.activeRun = "";
    return this.run({
      kind: "pr",
      request_id: crypto.randomUUID(),
      number: this.selected.number,
      action,
    });
  }
  run(action: ForgeAction) {
    return this.#effects.run(action);
  }
  checkPending() {
    return this.#effects.checkPending();
  }
  submit() {
    return this.#effects.submit();
  }
  recover() {
    return this.#effects.recover();
  }
  closePanel() {
    this.retainDraft();
    this.open = false;
  }
  close() {
    this.#closed = true;
    this.connection.close();
    this.token = "";
    this.inline.clear();
    this.#drafts.clear();
    this.dispose?.();
  }
}
