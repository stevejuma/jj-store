import type { ForgeController } from "./controller.svelte.js";
/** Bounded repository observation; teardown stops every scheduled refresh. */
export class ForgePolling {
  #timer?: ReturnType<typeof setTimeout>;
  #refreshing = false;
  constructor(readonly owner: ForgeController) {}
  async refresh() {
    if (this.owner.closed || this.#refreshing) return;
    if (this.owner.busy) {
      clearTimeout(this.#timer);
      this.#timer = setTimeout(() => void this.owner.refresh(), 15000);
      return;
    }
    this.#refreshing = true;
    try {
      const response = await this.owner.call({ kind: "status" });
      this.owner.receive(response, true);
      this.owner.stale = false;
      if (response.notice) {
        this.owner.error(new Error(response.notice), "maintenance");
      }
      if (
        response.operation &&
        ["running", "unknown"].includes(response.operation.outcome) &&
        !this.owner.pending
      ) {
        this.owner.pending = {
          kind: "pr",
          request_id: response.operation.id,
          number: response.operation.number,
          action: response.operation.action,
        };
      }
      for (const [number, merge] of Object.entries(this.owner.merges).slice(
        -16,
      )) {
        const pr = response.pull_requests.find(
          (pr) => String(pr.number) === number,
        );
        if (pr?.state === "merged") {
          this.owner.merges = {
            ...this.owner.merges,
            [number]: { ...merge, state: "merged" },
          };
        } else if (
          merge.receipt &&
          ["pending", "accepted", "enqueued", "queued", "in_progress"].includes(
            merge.state,
          )
        ) {
          const result = await this.owner.call({
            kind: "pr",
            request_id: crypto.randomUUID(),
            number: pr?.number ?? BigInt(number),
            action: { kind: "merge_status", receipt: merge.receipt },
          });
          if (result.merge)
            this.owner.merges = {
              ...this.owner.merges,
              [number]: result.merge,
            };
        }
      }
      if (response.connection && response.capabilities.checks) {
        const branches = [
          ...new Set([
            ...this.owner.watchedBranches(),
            ...(this.owner.selected ? [this.owner.selected.head] : []),
          ]),
        ].slice(0, 16);
        for (let offset = 0; offset < branches.length; offset += 4) {
          await Promise.all(
            branches.slice(offset, offset + 4).map(async (branch) => {
              const pr = response.pull_requests.find(
                (p) => p.head === branch && p.state === "open",
              );
              if (!pr) return;
              const detail = await this.owner.call({
                kind: "pr",
                request_id: crypto.randomUUID(),
                number: pr.number,
                action: { kind: "checks" },
              });
              if (detail.checks)
                this.owner.ci = { ...this.owner.ci, [branch]: detail.checks };
              const selected = this.owner.selected;
              if (selected?.number === pr.number) {
                const latest = detail.pull_requests.find(
                  (p) => p.number === pr.number,
                );
                if (latest) {
                  this.owner.selected = latest;
                  if (!this.owner.draftDirty) this.owner.accept(latest);
                }
                if (this.owner.checks && this.owner.checks.sha === pr.sha) {
                  this.owner.checks = this.owner.activeRun
                    ? (
                        await this.owner.call({
                          kind: "pr",
                          request_id: crypto.randomUUID(),
                          number: pr.number,
                          action: { kind: "jobs", run: this.owner.activeRun },
                        })
                      ).checks
                    : detail.checks;
                }
              }
            }),
          );
        }
      }
      if (this.owner.selected && !this.owner.draftDirty) {
        const pr = response.pull_requests.find(
          (p) => p.number === this.owner.selected?.number,
        );
        if (pr)
          this.owner.accept({
            ...pr,
            mergeable: pr.mergeable ?? this.owner.selected.mergeable,
          });
      }
      this.owner.notifications?.service.resolve(
        this.owner.notifications.scope,
        "forge:refresh",
      );
    } catch (error) {
      if (!this.owner.closed) this.owner.error(error, "refresh");
    } finally {
      this.#refreshing = false;
      clearTimeout(this.#timer);
      if (!this.owner.closed)
        this.#timer = setTimeout(
          () => void this.owner.refresh(),
          this.owner.stale ? 60000 : 15000,
        );
    }
  }
  close() {
    clearTimeout(this.#timer);
  }
}
