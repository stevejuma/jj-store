import type { ForgeController } from "./controller.svelte.js";
import type { ForgeAction } from "./types.js";
/** Exact pending intent and external-effect receipts remain with the owning view. */
export class ForgeEffects {
  constructor(readonly owner: ForgeController) {}
  async checkPending() {
    const pending = this.owner.pending;
    if (!pending || this.owner.busy) return;
    this.owner.busy = true;
    try {
      const id =
        pending.kind === "pr" || pending.kind === "recover"
          ? pending.request_id
          : pending.kind === "submit" || pending.kind === "restack"
            ? pending.plan.request_id
            : undefined;
      if (!id) return;
      const response = await this.owner.call(
        pending.kind === "pr"
          ? { kind: "operation_status", request_id: id }
          : { kind: "recover", request_id: id },
      );
      const outcome =
        response.operation?.outcome ??
        response.receipt?.outcome ??
        response.restack?.outcome;
      if (outcome === "complete" || outcome === "rejected")
        this.owner.pending = undefined;
      this.owner.receive(response);
      if (response.restack?.operation)
        this.owner.restacked?.(response.restack.operation);
      if (response.operation?.merge)
        this.owner.merges = {
          ...this.owner.merges,
          [String(response.operation.number)]: response.operation.merge,
        };
    } catch (error) {
      if (!this.owner.closed) this.owner.error(error, "recovery");
    } finally {
      this.owner.busy = false;
    }
  }
  async submit() {
    if (this.owner.plan) {
      const branch = this.owner.plan.items.at(-1)!.segment.branch;
      if (
        await this.owner.run({
          kind: "submit",
          plan: $state.snapshot(this.owner.plan),
        })
      ) {
        this.owner.plan = undefined;
        await this.owner.refresh();
        this.owner.select(branch);
      }
    }
  }
  async recover() {
    const id = this.owner.data?.receipt?.plan.request_id;
    if (id) {
      this.owner.pending = undefined;
      await this.owner.run({ kind: "recover", request_id: id });
    }
  }
  async run(action: ForgeAction) {
    if (this.owner.busy || this.owner.pending) return false;
    this.owner.busy = true;
    const selection = this.owner.selected?.number;
    const mutating =
      ["submit", "recover", "restack"].includes(action.kind) ||
      (action.kind === "pr" &&
        !["checks", "jobs", "logs", "merge_status"].includes(
          action.action.kind,
        ));
    if (mutating) this.owner.pending = $state.snapshot(action);
    try {
      const response = await this.owner.call(action);
      this.owner.plan = response.plan ?? this.owner.plan;
      this.owner.restack = response.restack ?? this.owner.restack;
      if (response.restack?.operation && action.kind !== "restack_preview")
        this.owner.restacked?.(response.restack.operation);
      if (response.checks && selection === this.owner.selected?.number) {
        this.owner.checks = response.checks;
        if (
          action.kind === "pr" &&
          action.action.kind === "checks" &&
          this.owner.selected
        )
          this.owner.ci = {
            ...this.owner.ci,
            [this.owner.selected.head]: response.checks,
          };
      }
      if (response.log != null && selection === this.owner.selected?.number)
        this.owner.logs = response.log;
      this.owner.receive(response, action.kind === "status");
      if (response.merge && action.kind === "pr")
        this.owner.merges = {
          ...this.owner.merges,
          [String(action.number)]: response.merge,
        };
      if (response.restack?.outcome === "rejected") {
        this.owner.error(
          new Error(
            `Restack applied; inspect native operation ${response.restack.operation ?? "unknown"} before continuing.`,
          ),
          response.restack.plan.request_id,
        );
        this.owner.pending = undefined;
        return false;
      }
      if (response.receipt && response.receipt.outcome !== "complete") {
        this.owner.error(
          new Error(response.receipt.message ?? "Submission needs recovery"),
          response.receipt.plan.request_id,
        );
        this.owner.pending = undefined;
        return false;
      }
      this.owner.pending = undefined;
      if (this.owner.selected && !this.owner.draftDirty) {
        const pr = response.pull_requests.find(
          (p) => p.number === this.owner.selected!.number,
        );
        if (pr) this.owner.accept(pr);
      }
      this.owner.stale = false;
      if (mutating) queueMicrotask(() => void this.owner.refresh());
      return true;
    } catch (error) {
      if (!this.owner.closed)
        this.owner.error(
          error,
          action.kind === "pr" ? action.request_id : action.kind,
        );
      // Preserve an exact pending effect when transport cannot establish the outcome.
      if (
        !mutating ||
        (error instanceof Error &&
          !(error instanceof SyntaxError) &&
          !(error instanceof TypeError) &&
          !/unknown|timeout|timed out|deadline|fetch|network|abort|unexpected.response|response.limit|local.request.failed/i.test(
            error.message,
          ))
      )
        this.owner.pending = undefined;
      return false;
    } finally {
      this.owner.busy = false;
    }
  }
}
