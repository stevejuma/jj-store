import { inspectLocalCommits } from "./local-inspection.js";
import { localMutationKinds } from "./local-mutations.js";
import type { StackCommit } from "./types.js";
import { ApiError, fullId, type Model } from "../../api/client.js";
import type { StoreDataSource } from "../native-data-source.js";
import { isLocal, type LocalReadSnapshot } from "../repository-profile.js";
import { stackReadContext, unavailableStackCommand } from "./read-context.js";
import { projectLocalStacks, localStacksCommand } from "./local-state.js";
import {
  WorkspaceCommandError,
  type WorkspaceRequest,
  type WorkspaceSnapshot,
  type WorkspaceTargetBranch,
} from "./types.js";
/** Local command state stays separate from operation-pinned browsing and its continuations. */
export class LocalStacksSession {
  state: Model<"LocalStacksState"> | null = null;
  #confirmed: Model<"LocalStacksState"> | null = null;
  constructor(
    readonly source: StoreDataSource,
    readonly binding: Model<"LocalBinding">,
    readonly signal: AbortSignal,
    readonly current: () => WorkspaceSnapshot | null,
    readonly pin: () => LocalReadSnapshot,
    readonly read: (
      state?: Model<"LocalStacksState">,
    ) => Promise<WorkspaceSnapshot>,
  ) {}
  get retryPending() {
    return !this.source.stacks;
  }
  async refresh(known?: Model<"LocalStacksState">) {
    if (!this.source.stacks) return null;
    const state =
      known ??
      (await this.source.stacks.execute({
        version: 1,
        binding_generation: this.binding.generation,
        operation: null,
        revision: null,
        command: { kind: "refresh" },
      }));
    if (state.version !== 1) throw new ApiError("invalid_local_stacks");
    fullId(state.operation);
    this.state = state;
    return state;
  }
  active() {
    return this.state?.workspaces.find(
      (workspace) => workspace.id === this.state?.active,
    );
  }
  targetFor(
    targets: WorkspaceTargetBranch[],
    previous?: WorkspaceTargetBranch | null,
  ) {
    const active = this.active();
    const selected = active
      ? { name: active.target, commit: active.target_commit }
      : previous;
    return (
      targets.find(
        (target) =>
          target.name === selected?.name && target.commit === selected.commit,
      ) ?? null
    );
  }
  async inspect(commits: StackCommit[]) {
    await inspectLocalCommits(
      this.source.stacks,
      this.state,
      this.binding.generation,
      commits,
      this.signal,
    );
  }
  async project(snapshot: WorkspaceSnapshot) {
    const result = this.state
      ? projectLocalStacks(snapshot, this.state)
      : snapshot;
    if (!this.source.stacks?.preview)
      result.capabilities = result.capabilities.filter(
        (kind) => !localMutationKinds.some((mutation) => mutation === kind),
      );
    if (!this.source.stacks?.inspect)
      result.capabilities = result.capabilities.filter(
        (kind) =>
          !localMutationKinds.some(
            (mutation) => mutation !== "create_commit" && mutation === kind,
          ),
      );
    await this.inspect(result.stacks.flatMap((stack) => stack.commits));
    return result;
  }
  async references(
    references: Model<"LocalReferencesReference">[],
    pin: LocalReadSnapshot,
  ) {
    this.signal.throwIfAborted();
    const active = this.active();
    if (!active) return;
    for (const name of new Set([active.target, ...active.applied])) {
      if (
        references.some(
          (reference) =>
            reference.kind === "bookmark" && reference.name === name,
        )
      )
        continue;
      const exact = await this.source.references(pin.operation, undefined, {
        kind: "bookmarks",
        exact: name,
        signal: this.signal,
      });
      if (
        !isLocal(exact.snapshot) ||
        exact.snapshot.operation !== pin.operation
      )
        throw new ApiError("snapshot_mismatch");
      references.push(
        ...exact.data.references.filter(
          (reference) => reference.kind === "bookmark",
        ),
      );
    }
  }
  async target(target: WorkspaceTargetBranch) {
    const state = await this.source.stacks!.execute({
      version: 1,
      binding_generation: this.binding.generation,
      operation: this.pin().operation,
      revision: this.state!.revision,
      command: { kind: "set_target", branch: target.name },
    });
    return this.read(state);
  }
  async preview(request: WorkspaceRequest) {
    if (!this.source.stacks) return unavailableStackCommand();
    const snapshot = stackReadContext(this.current(), request, this.signal);
    const command = request.command;
    const local = localStacksCommand(snapshot, command);
    if (localMutationKinds.some((kind) => kind === command.kind)) {
      if (!this.source.stacks.preview) return unavailableStackCommand();
      const preview = await this.source.stacks.preview({
        version: 1,
        binding_generation: this.binding.generation,
        operation: this.pin().operation,
        revision: this.state!.revision,
        command: local,
      });
      return {
        request,
        title: preview.title,
        summary: preview.summary,
        conflicts: [],
      };
    }
    return {
      request,
      title:
        command.kind === "delete_workspace"
          ? "Delete workspace?"
          : "Review workspace change",
      summary:
        command.kind === "delete_workspace"
          ? `Remove the saved configuration for “${snapshot.workspaces.find((w) => w.id === command.workspace)?.name}”. Branches, commits, the current checkout and unfinished JJ work are retained.`
          : command.kind === "update_workspace"
            ? `Update “${snapshot.workspaces.find((w) => w.id === command.workspace)?.name}” to name “${command.name.trim()}” and target “${command.targetBranch.name}”. Applied branches and unfinished JJ work are retained.`
            : "Apply this change using the current local JJ workspace. Unfinished changes are retained; JJ will represent any conflicts.",
      conflicts: [],
    };
  }
  async apply(request: WorkspaceRequest) {
    if (!this.source.stacks) return unavailableStackCommand();
    const snapshot = stackReadContext(this.current(), request, this.signal);
    try {
      const state = await this.source.stacks.execute({
        version: 1,
        binding_generation: this.binding.generation,
        operation: this.pin().operation,
        revision: this.state!.revision,
        command: localStacksCommand(snapshot, request.command),
      });
      this.#confirmed = state;
      const result = await this.read(state);
      this.#confirmed = null;
      return { status: "committed" as const, snapshot: result };
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        [
          "stale_local_stacks",
          "invalid_local_stacks",
          "local_stacks_failed",
        ].includes(cause.code)
      )
        throw new WorkspaceCommandError(
          cause.code === "stale_local_stacks" ? "stale" : "invalid",
          cause.code === "stale_local_stacks"
            ? "The local workspace changed. Cancel the preview, refresh and review again."
            : "JJ could not apply this workspace change. Refresh and inspect its current state.",
        );
      throw cause;
    }
  }
  async status() {
    if (!this.source.stacks) return unavailableStackCommand();
    const confirmed = this.#confirmed;
    const snapshot = await this.read(confirmed ?? undefined);
    this.#confirmed = null;
    return {
      status: confirmed ? ("committed" as const) : ("refreshed" as const),
      snapshot,
    };
  }
}
