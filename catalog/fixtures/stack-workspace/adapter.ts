import { commandTitle } from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import {
  cloneWorkspace,
  WorkspaceCommandError,
  type StackWorkspaceAdapter,
  type WorkspaceOutcome,
  type WorkspaceRequest,
  type ConfiguredWorkspaceSnapshot as WorkspaceSnapshot,
  type WorkspaceHistoryRequest,
  type StackCommit,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { workspaceFixture } from "./data.js";
import { fixtureTargetHistory } from "./target-history.js";
import { reduceFixture } from "./reducer.js";
import { fixtureSummary } from "./preview.js";
import { validateFixtureCommand } from "./validation.js";

export type FixtureFault = "none" | "unknown" | "failed" | "stale";
export class MockStackWorkspaceAdapter implements StackWorkspaceAdapter {
  snapshot: WorkspaceSnapshot;
  fault: FixtureFault;
  readonly calls: WorkspaceRequest[] = [];
  readonly historyCalls: WorkspaceHistoryRequest[] = [];
  targetCommits: StackCommit[];
  #receipts = new Map<
    string,
    { request: WorkspaceRequest; snapshot: WorkspaceSnapshot }
  >();
  #history: WorkspaceSnapshot[] = [];
  #future: WorkspaceSnapshot[] = [];
  #sequence = 0;
  constructor(snapshot = workspaceFixture(), fault: FixtureFault = "none") {
    this.snapshot = cloneWorkspace(snapshot);
    this.targetCommits = fixtureTargetHistory(snapshot.target.name);
    this.fault = fault;
  }
  async read() {
    return cloneWorkspace(this.snapshot);
  }
  async readTargetHistory(request: WorkspaceHistoryRequest) {
    this.historyCalls.push(cloneWorkspace(request));
    if (
      request.repository !== this.snapshot.repository ||
      request.revision !== this.snapshot.revision ||
      request.branch !== this.snapshot.target.name ||
      request.head !== this.snapshot.target.commit
    )
      throw new WorkspaceCommandError(
        "stale",
        "Refresh the workspace before loading this branch history.",
      );
    const offset = request.cursor ? Number(request.cursor) : 0;
    if (
      !Number.isSafeInteger(offset) ||
      offset < 0 ||
      offset > this.targetCommits.length ||
      !Number.isInteger(request.limit) ||
      request.limit < 1 ||
      request.limit > 40
    )
      throw new WorkspaceCommandError(
        "invalid",
        "Invalid branch history page.",
      );
    const end = Math.min(offset + request.limit, this.targetCommits.length);
    return {
      commits: cloneWorkspace(this.targetCommits.slice(offset, end)),
      ...(end < this.targetCommits.length ? { nextCursor: String(end) } : {}),
      total: this.targetCommits.length,
    };
  }
  async preview(request: WorkspaceRequest) {
    this.validate(request);
    const command = request.command;
    const stack =
      "stack" in command
        ? this.snapshot.stacks.find((item) => item.id === command.stack)
        : undefined;
    return {
      request: cloneWorkspace(request),
      title: commandTitle(command),
      summary: fixtureSummary(this.snapshot, command),
      conflicts:
        command.kind === "apply_stack" && stack?.conflicts
          ? ["src/settings.ts"]
          : [],
    };
  }
  async apply(request: WorkspaceRequest): Promise<WorkspaceOutcome> {
    this.calls.push(cloneWorkspace(request));
    const receipt = this.#receipts.get(request.id);
    if (receipt) {
      if (JSON.stringify(receipt.request) !== JSON.stringify(request))
        throw new WorkspaceCommandError(
          "invalid",
          "Request identity was reused with different intent.",
        );
      return {
        status: "committed",
        snapshot: cloneWorkspace(receipt.snapshot),
      };
    }
    if (this.fault === "stale") {
      this.snapshot.revision++;
      this.fault = "none";
    }
    this.validate(request);
    if (this.fault === "failed") {
      this.fault = "none";
      throw new WorkspaceCommandError(
        "failed",
        "The workspace command failed. Your changes are retained.",
      );
    }
    if (request.command.kind === "undo") {
      this.#future.push(cloneWorkspace(this.snapshot));
      const previous = this.#history.pop()!;
      previous.revision = this.snapshot.revision + 1;
      this.snapshot = previous;
    } else if (request.command.kind === "redo") {
      this.#history.push(cloneWorkspace(this.snapshot));
      const next = this.#future.pop()!;
      next.revision = this.snapshot.revision + 1;
      this.snapshot = next;
    } else {
      this.#future = [];
      this.#history.push(cloneWorkspace(this.snapshot));
      reduceFixture(
        this.snapshot,
        request.command,
        () => `fixture-${++this.#sequence}`,
      );
      this.snapshot.revision++;
    }
    this.snapshot.canUndo = this.#history.length > 0;
    this.snapshot.canRedo = this.#future.length > 0;
    this.#receipts.set(request.id, {
      request: cloneWorkspace(request),
      snapshot: cloneWorkspace(this.snapshot),
    });
    if (this.fault === "unknown") {
      this.fault = "none";
      return { status: "unknown" };
    }
    return { status: "committed", snapshot: cloneWorkspace(this.snapshot) };
  }
  async status(id: string): Promise<WorkspaceOutcome> {
    const receipt = this.#receipts.get(id);
    return receipt
      ? { status: "committed", snapshot: cloneWorkspace(receipt.snapshot) }
      : { status: "unknown" };
  }
  private validate(request: WorkspaceRequest) {
    if (
      request.repository !== this.snapshot.repository ||
      request.revision !== this.snapshot.revision
    )
      throw new WorkspaceCommandError(
        "stale",
        "The workspace changed. Cancel this preview, refresh, and preview again; your draft is retained.",
      );
    validateFixtureCommand(this.snapshot, request.command);
  }
}
