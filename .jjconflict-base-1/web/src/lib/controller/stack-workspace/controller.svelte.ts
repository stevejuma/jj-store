import {
  selectWorkspaceFile,
  closeWorkspaceDiff,
  activateWorkspaceDiff,
} from "./diff-selection.js";
import { WorkspaceDragSession } from "./drag-session.svelte.js";
import { WorkspaceReadSession } from "./read-session.js";
import { WorkspaceTargetHistory } from "./target-history.svelte.js";
import { WorkspaceCommitDrafts } from "./commit-drafts.svelte.js";
import { WorkspaceLazyDetails } from "./lazy-details.svelte.js";
import { workspaceCommandEligibility } from "./eligibility.js";
import {
  retainedWorkspaceDiffs,
  reconcileSourceDiffs,
} from "./retained-diffs.js";
import {
  workspaceSourceFiles,
  workspaceBranchSource,
  workspaceAggregateSource,
  retainedStackSelections,
} from "./files.js";
import { retainedWorkspaceSelection } from "./presentation.js";
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
  type WorkspaceTargetBranch,
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
  readonly reads: WorkspaceReadSession;
  #closed = false;
  #command = 0;
  readonly targetHistory: WorkspaceTargetHistory;
  readonly details: WorkspaceLazyDetails;
  constructor(readonly adapter: StackWorkspaceAdapter) {
    this.reads = new WorkspaceReadSession(this, (snapshot) =>
      this.updateSnapshot(snapshot),
    );
    this.details = new WorkspaceLazyDetails(
      adapter,
      (source, files, complete) => {
        this.diffs = reconcileSourceDiffs(this.diffs, source, files, complete);
        if (this.targetHistory.diff)
          this.targetHistory.diff =
            reconcileSourceDiffs(
              [this.targetHistory.diff],
              source,
              files,
              complete,
            )[0] ?? null;
        const active = this.diffs.find(
          (diff) =>
            diff.source === this.source && diff.path === this.selectedPath,
        );
        if (active) activateWorkspaceDiff(this, active);
      },
    );
    this.targetHistory = new WorkspaceTargetHistory(adapter, {
      files: (source) => this.filesFor(source),
      selectCommit: (source) => {
        void this.details.loadFiles(source);
      },
      selectFile: (source, path) => {
        void this.details.loadDiff(source, path);
      },
    });
  }
  eligibility(kind: WorkspaceCommand["kind"]) {
    return workspaceCommandEligibility(this.snapshot, kind);
  }
  can(kind: WorkspaceCommand["kind"]) {
    return !this.locked && this.eligibility(kind).allowed;
  }
  filesFor(source: string) {
    if (this.details.lazy) return this.details.files[source] ?? [];
    return (
      this.targetHistory.commits.find((commit) => commit.id === source)
        ?.files ??
      (this.snapshot ? workspaceSourceFiles(this.snapshot, source) : [])
    );
  }
  async selectTargetBranch() {
    if (this.locked || !this.snapshot?.target || this.#closed) return;
    this.selectedStack = this.selectedBranch = null;
    this.selectedCommit = this.selectedPath = this.selection = null;
    await this.targetHistory.select();
  }
  async chooseTarget(target: WorkspaceTargetBranch) {
    const snapshot = this.snapshot;
    if (!snapshot || this.locked || !this.adapter.chooseTarget || this.#closed)
      return;
    await this.reads.run(() =>
      this.adapter.chooseTarget!({
        repository: snapshot.repository,
        revision: snapshot.revision,
        ...(snapshot.readKey ? { readKey: snapshot.readKey } : {}),
        target: { name: target.name, commit: target.commit },
      }),
    );
  }
  async refresh() {
    if (this.#closed || this.pending || this.preview || this.busy) return;
    await this.reads.run(() => this.adapter.read());
  }

  selectStack(id: string, branch?: string) {
    if (this.locked) return;
    this.targetHistory.active = false;
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
    void this.details.loadFiles(workspaceBranchSource(id, name!));
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
    void this.details.loadFiles(id);
  }
  selectFile(source: string, path: string, hunks?: string[]) {
    selectWorkspaceFile(this, source, path, hunks);
  }
  closeDiff(origin: string) {
    closeWorkspaceDiff(this, origin);
  }
  private updateSnapshot(snapshot: WorkspaceSnapshot) {
    this.details.sync(snapshot);
    const diffs = retainedWorkspaceDiffs(
      snapshot,
      this.snapshot?.active,
      this.diffs,
      this.details.lazy
        ? (source) => this.details.files[source] ?? []
        : undefined,
    );
    this.stackSelections = retainedStackSelections(
      snapshot,
      this.stackSelections,
    );
    this.snapshot = snapshot;
    this.targetHistory.sync(snapshot);
    this.diffs = diffs;
    const active = this.diffs.find(
      (diff) => diff.source === this.source && diff.path === this.selectedPath,
    );
    if (active) activateWorkspaceDiff(this, active);
    else {
      this.selectedPath = null;
      this.selection = null;
    }
  }
  async prepare(command: WorkspaceCommand) {
    const snapshot = this.snapshot;
    if (!snapshot || this.locked || this.#closed) return;
    this.error = this.validation = this.outcome = "";
    const eligibility = this.eligibility(command.kind);
    if (!eligibility.allowed) {
      this.validation = eligibility.reason;
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
    this.targetHistory.close();
    this.details.close();
    this.#closed = true;
    this.reads.close();
    this.#command++;
  }
}
