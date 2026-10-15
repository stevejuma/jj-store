import type { ForgeController } from "./controller.svelte.js";
import type { ForgePlan } from "./types.js";
/** Session-only creation drafts; floating windows never own publication intent. */
export class ForgeInline {
  creating = $state("");
  #plans = new Map<string, ForgePlan>();
  constructor(readonly owner: ForgeController) {}
  retain() {
    const key = this.creating || this.owner.plan?.items.at(-1)?.segment.branch;
    if (key && this.owner.plan) this.#plans.set(key, this.owner.plan);
  }
  private resetPlan() {
    this.owner.plan = undefined;
  }
  leave() {
    this.retain();
    this.creating = "";
  }
  async start(branch: string) {
    if (!this.owner.data?.connection || this.owner.busy || this.owner.pending)
      return;
    this.retain();
    this.creating = branch;
    const cached = this.#plans.get(branch);
    if (cached) {
      this.owner.retainDraft();
      this.owner.selected = undefined;
      this.owner.surface = "inline";
      this.owner.open = false;
      this.owner.plan = cached;
      return;
    }
    this.resetPlan();
    const success = await this.owner.preview(branch, false, "inline");
    if (!success || this.creating !== branch || !this.owner.plan) return;
    for (const item of this.owner.plan.items)
      if (!item.pr) item.segment.title = item.segment.branch;
    this.retain();
  }
  cancel() {
    if (this.owner.busy || this.owner.pending) return;
    this.leave();
    this.owner.plan = undefined;
    this.owner.open = false;
  }
  complete(branch: string) {
    this.#plans.delete(branch);
    this.creating = "";
  }
  clear() {
    this.#plans.clear();
    this.creating = "";
  }
}
