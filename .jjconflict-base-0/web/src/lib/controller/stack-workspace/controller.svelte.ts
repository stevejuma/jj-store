import { WorkspaceDragSession } from "./drag-session.svelte.js";
import { WorkspaceCommitDrafts } from "./commit-drafts.svelte.js";
import {
  workspaceSourceFiles,
  workspaceSourceStack,
  workspaceBranchSource,
  workspaceAggregateSource,
} from "./files.js";
import { activeWorkspace, retainedWorkspaceSelection } from "./presentation.js";
import {
  cloneWorkspace,
  WorkspaceCommandError,
  type StackWorkspaceAdapter,
  type WorkspaceCommand,
  type WorkspacePreview,
  type WorkspaceRequest,
  type WorkspaceSelection,
  type WorkspaceOpenDiff,
  type WorkspaceSnapshot,
} from "./types.js";
/** Transport-independent state owner. Catalog supplies the mock adapter. */
export class StackWorkspaceController {
  readonly drag = new WorkspaceDragSession();
  readonly commitDrafts = new WorkspaceCommitDrafts();
  snapshot = $state<WorkspaceSnapshot | null>(null);
  loading = $state(false);
  page = $state<"branches" | "workspace">("workspace");
  selectedStack = $state<string | null>(null);
  selectedBranch = $state<string | null>(null);
  stackSelections = $state<Record<string, string>>({});
  selectedCommit = $state<string | null>(null);
  selectedPath = $state<string | null>(null);
  source = $state("unassigned");
  selection = $state<WorkspaceSelection | null>(null);
  diffs = $state<WorkspaceOpenDiff[]>([]);
  preview = $state<WorkspacePreview | null>(null);
  pending = $state<WorkspaceRequest | null>(null);
  busy = $state(false);
  error = $state("");
  validation = $state("");
  outcome = $state("");
  readonly locked = $derived(
    this.busy || this.loading || this.pending !== null || this.preview !== null,
  );
  #read = 0;
  #closed = false;
  #command = 0;
  constructor(readonly adapter: StackWorkspaceAdapter) {}
  async refresh() {
    if (this.#closed || this.pending || this.preview || this.busy) return;
    const ticket = ++this.#read;
    this.loading = true;
    this.error = "";
    try {
      const snapshot = await this.adapter.read();
      if (!this.#closed && ticket === this.#read) this.updateSnapshot(snapshot);
    } catch (error) {
      if (!this.#closed && ticket === this.#read) this.error = String(error);
    } finally {
      if (!this.#closed && ticket === this.#read) this.loading = false;
    }
  }
  selectStack(id: string, branch?: string) {
    if (this.locked) return;
    this.selectedStack = id;
    this.selectedBranch =
      branch ??
      this.snapshot?.stacks.find((stack) => stack.id === id)?.name ??
      null;
    this.selectedCommit = null;
    this.selectedPath = null;
    this.selection = null;
  }
  selectedCommitFor(stack: string) {
    const selection = this.stackSelections[stack];
    return selection && !workspaceAggregateSource(selection) ? selection : null;
  }
  selectStackFiles(id: string, branch?: string) {
    const stack = this.snapshot?.stacks.find((stack) => stack.id === id);
    const name = branch ?? stack?.name;
    if (this.locked || !stack?.heads.some((head) => head.name === name)) return;
    this.selectedStack = id;
    this.stackSelections = {
      ...this.stackSelections,
      [id]: workspaceBranchSource(id, name!),
    };
    this.selectedCommit = this.selectedPath = this.selection = null;
  }
  selectCommit(id: string) {
    if (this.locked) return;
    this.selectedCommit = id;
    const stack = this.snapshot?.stacks.find((stack) =>
      stack.commits.some((commit) => commit.id === id),
    );
    if (stack)
      this.stackSelections = { ...this.stackSelections, [stack.id]: id };
    this.selectedPath = null;
    this.selection = null;
  }
  selectFile(source: string, path: string, hunks?: string[]) {
    if (this.locked) return;
    const snapshot = this.snapshot;
    if (
      !snapshot ||
      !workspaceSourceFiles(snapshot, source).some((file) => file.path === path)
    )
      return;
    const origin =
      source === "unassigned" || source === "combined"
        ? "working"
        : `stack-${workspaceSourceStack(snapshot, source)!.id}`;
    const diff: WorkspaceOpenDiff = {
      origin,
      source,
      path,
      ...(hunks ? { hunks: [...hunks] } : {}),
    };
    const index = this.diffs.findIndex((item) => item.origin === origin);
    if (index < 0) this.diffs = [...this.diffs, diff];
    else this.diffs = this.diffs.map((item, i) => (i === index ? diff : item));
    this.activateDiff(diff);
  }
  closeDiff(origin: string) {
    this.diffs = this.diffs.filter((diff) => diff.origin !== origin);
    if (
      !this.diffs.some(
        (diff) =>
          diff.source === this.source && diff.path === this.selectedPath,
      )
    ) {
      this.selectedPath = null;
      this.selection = null;
    }
  }
  private activateDiff(diff: WorkspaceOpenDiff) {
    this.selectedPath = diff.path;
    this.source = diff.source;
    this.selectedCommit = diff.origin === "working" ? null : diff.source;
    this.selection = workspaceAggregateSource(diff.source)
      ? null
      : {
          source: diff.source,
          path: diff.path,
          ...(diff.hunks ? { hunks: [...diff.hunks] } : {}),
        };
  }
  private updateSnapshot(snapshot: WorkspaceSnapshot) {
    const switched = this.snapshot?.active !== snapshot.active;
    const diffs = new Map<string, WorkspaceOpenDiff>();
    for (const diff of this.diffs) {
      if (switched && diff.origin === "working") continue;
      const file = workspaceSourceFiles(snapshot, diff.source).find(
        (file) => file.path === diff.path,
      );
      if (!file) continue;
      const stack = workspaceSourceStack(snapshot, diff.source);
      const origin =
        diff.origin === "working"
          ? "working"
          : stack
            ? `stack-${stack.id}`
            : null;
      if (!origin) continue;
      diffs.set(origin, {
        ...diff,
        origin,
        ...(diff.hunks
          ? {
              hunks: diff.hunks.filter((id) =>
                file.hunks.some((hunk) => hunk.id === id),
              ),
            }
          : {}),
      });
    }
    this.stackSelections = Object.fromEntries(
      Object.entries(this.stackSelections).filter(([id, selection]) =>
        snapshot.stacks.some(
          (stack) =>
            stack.id === id &&
            (workspaceSourceStack(snapshot, selection)?.id === id ||
              stack.commits.some((commit) => commit.id === selection)),
        ),
      ),
    );
    this.snapshot = snapshot;
    this.diffs = [...diffs.values()];
    const active = this.diffs.find(
      (diff) => diff.source === this.source && diff.path === this.selectedPath,
    );
    if (active) this.activateDiff(active);
    else {
      this.selectedPath = null;
      this.selection = null;
    }
  }
  async prepare(command: WorkspaceCommand) {
    const snapshot = this.snapshot;
    if (!snapshot || this.locked || this.#closed) return;
    this.error = this.validation = this.outcome = "";
    if (
      snapshot.availability !== "ready" ||
      !snapshot.folder ||
      !snapshot.capabilities.includes(command.kind)
    ) {
      this.validation = "This command requires an available local checkout.";
      return;
    }
    const request: WorkspaceRequest = {
      id: globalThis.crypto.randomUUID(),
      repository: snapshot.repository,
      revision: snapshot.revision,
      command: cloneWorkspace(command),
    };
    const ticket = ++this.#command;
    this.busy = true;
    try {
      const preview = await this.adapter.preview(cloneWorkspace(request));
      if (!this.#closed && ticket === this.#command)
        this.preview = { ...preview, request };
    } catch (error) {
      if (!this.#closed) this.failed(error);
    } finally {
      if (!this.#closed) this.busy = false;
    }
  }
  cancelPreview() {
    if (this.busy || this.pending) return;
    this.preview = null;
  }
  async apply() {
    const request = this.pending ?? this.preview?.request;
    if (!request || this.busy || this.#closed) return;
    this.busy = true;
    this.error = this.validation = "";
    try {
      this.accept(await this.adapter.apply(cloneWorkspace(request)), request);
    } catch (error) {
      if (!this.#closed) {
        if (!(error instanceof WorkspaceCommandError)) this.pending = request;
        this.failed(error);
      }
    } finally {
      if (!this.#closed) this.busy = false;
    }
  }
  async checkStatus() {
    const request = this.pending;
    if (!request || this.busy || this.#closed) return;
    this.busy = true;
    try {
      this.accept(await this.adapter.status(request.id), request);
    } catch (error) {
      if (!this.#closed) this.failed(error);
    } finally {
      if (!this.#closed) this.busy = false;
    }
  }
  private accept(
    result: Awaited<ReturnType<StackWorkspaceAdapter["apply"]>>,
    request: WorkspaceRequest,
  ) {
    if (this.#closed) return;
    if (result.status === "unknown") {
      this.pending = request;
      return;
    }
    if (this.snapshot)
      this.commitDrafts.committed(this.snapshot, request.command);
    this.updateSnapshot(result.snapshot);
    const created = this.commitDrafts.created(result.snapshot, request.command);
    if (created) {
      this.selectedStack = created.id;
      this.selectedBranch = created.name;
      this.page = "workspace";
    }
    this.pending = null;
    this.preview = null;
    this.outcome = "Command completed";
    Object.assign(this, retainedWorkspaceSelection(result.snapshot, this));
  }
  private failed(error: unknown) {
    if (error instanceof WorkspaceCommandError && error.code !== "failed")
      this.validation = error.message;
    else this.error = error instanceof Error ? error.message : String(error);
  }
  close() {
    this.drag.clear();
    this.#closed = true;
    this.#read++;
    this.#command++;
  }
}
