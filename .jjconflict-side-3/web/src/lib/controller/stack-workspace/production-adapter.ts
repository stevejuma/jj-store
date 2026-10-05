import { readIssuedStackHistory } from "./issued-history.js";
import { stackReadContext, unavailableStackCommand } from "./read-context.js";
import { stackSourceRange } from "./source-range.js";
import { ApiError, fullId, type Model } from "../../api/client.js";
import type { StoreDataSource } from "../native-data-source.js";
import {
  sameCodeSnapshot,
  type LocalReadSnapshot,
} from "../repository-profile.js";
import { LoadedStackRanges } from "./loaded-ranges.js";
import { stackReferencePage, stackReferenceWindow } from "./reference-pages.js";
import { StackWorkspaceReads } from "./reads.js";
import {
  productionStackSnapshot,
  verifiedLocalStackPin,
} from "./production-snapshot.js";
import { localStackReferences } from "./local-references.js";
import {
  cloneWorkspace,
  WorkspaceCommandError,
  type StackWorkspaceAdapter,
  type WorkspaceSnapshot,
  type WorkspaceSourceRequest,
  type WorkspaceHistoryRequest,
  type WorkspaceHistoryPage,
  type WorkspaceRequest,
  type WorkspaceStackHistoryRequest,
} from "./types.js";

/** Read projection for one verified device binding and an explicitly selected JJ operation. */
export class ProductionStackWorkspaceAdapter implements StackWorkspaceAdapter {
  readonly #closed = new AbortController();
  #epoch = 0;
  #snapshot: WorkspaceSnapshot | null = null;
  #pin: LocalReadSnapshot | null = null;
  #reads: StackWorkspaceReads | null = null;
  #projection: ReturnType<typeof localStackReferences> | null = null;
  #references: Model<"LocalReferencesReference">[] = [];
  #pages = 1;
  #seen = new Set<string>();
  readonly ranges = new LoadedStackRanges();
  #commits = new Map<string, WorkspaceHistoryPage["commits"][number]>();
  constructor(
    readonly source: StoreDataSource,
    readonly binding: Model<"LocalBinding">,
    public operation: string,
  ) {
    fullId(operation);
  }
  setOperation(operation: string) {
    fullId(operation);
    if (operation !== this.operation) {
      this.operation = operation;
      this.#epoch++;
    }
  }
  async read() {
    this.#closed.signal.throwIfAborted();
    const epoch = ++this.#epoch;
    const captured = await this.source.snapshot();
    this.#closed.signal.throwIfAborted();
    const pin = verifiedLocalStackPin(
      this.source,
      this.binding,
      captured,
      this.operation,
    );
    const result = await stackReferenceWindow(
      this.source,
      pin,
      this.#closed.signal,
      this.#pages,
    );
    if (epoch !== this.#epoch) throw new ApiError("snapshot_mismatch");
    const projection = localStackReferences(result.references);
    const previous = this.#snapshot;
    const target =
      projection.targets.find(
        (target) =>
          target.name === previous?.target?.name &&
          target.commit === previous.target.commit,
      ) ?? null;
    if (!this.#pin || !sameCodeSnapshot(this.#pin, pin)) {
      const reads = new StackWorkspaceReads(this.source, pin);
      try {
        await this.ranges.refresh(
          reads,
          projection.stacks,
          target?.commit,
          () => {
            this.#closed.signal.throwIfAborted();
            if (epoch !== this.#epoch) throw new ApiError("snapshot_mismatch");
          },
          target ?? undefined,
        );
      } catch (error) {
        reads.close();
        throw error;
      }
      this.#reads?.close();
      this.#reads = reads;
      this.#commits.clear();
    }
    this.#pin = pin;
    for (const commit of this.ranges.commits())
      this.#commits.set(commit.id, commit);
    for (const stack of projection.stacks)
      for (const commit of stack.commits) this.#commits.set(commit.id, commit);
    this.#references = result.references;
    this.#pages = result.count;
    this.#seen = result.seen;
    this.#projection = projection;
    this.#snapshot = productionStackSnapshot(
      projection,
      pin,
      this.binding.path,
      target,
      previous?.revision ?? 1,
      result.cursor,
    );
    this.#snapshot.stacks = this.#snapshot.stacks.map((stack) =>
      this.ranges.restore(stack, target?.commit),
    );
    this.#snapshot.loadedTargetHistory = this.ranges.target(target?.commit);
    return cloneWorkspace(this.#snapshot);
  }
  #context(request: {
    repository: string;
    revision: number;
    readKey?: string;
  }) {
    return stackReadContext(this.#snapshot, request, this.#closed.signal);
  }
  async chooseTarget(
    request: Parameters<NonNullable<StackWorkspaceAdapter["chooseTarget"]>>[0],
  ) {
    const snapshot = this.#context(request);
    const target = snapshot.targetOptions?.find(
      (target) =>
        target.name === request.target.name &&
        target.commit === request.target.commit,
    );
    if (!target)
      throw new WorkspaceCommandError(
        "invalid",
        "Choose an exact available target branch.",
      );
    this.ranges.clear();
    this.#commits.clear();
    for (const stack of this.#projection!.stacks)
      for (const commit of stack.commits) this.#commits.set(commit.id, commit);
    this.#snapshot = productionStackSnapshot(
      this.#projection!,
      this.#pin!,
      this.binding.path,
      target,
      snapshot.revision + 1,
      snapshot.branchesNextCursor,
    );
    return cloneWorkspace(this.#snapshot);
  }
  async readBranches(request: {
    repository: string;
    revision: number;
    readKey?: string;
    cursor: string;
    signal?: AbortSignal;
  }) {
    const snapshot = this.#context(request);
    if (
      !snapshot.branchesNextCursor ||
      request.cursor !== snapshot.branchesNextCursor ||
      this.#seen.has(request.cursor)
    )
      throw new WorkspaceCommandError(
        "stale",
        "The branch continuation changed.",
      );
    const epoch = this.#epoch;
    const page = await stackReferencePage(
      this.source,
      this.#pin!,
      AbortSignal.any([
        this.#closed.signal,
        request.signal ?? this.#closed.signal,
      ]),
      request.cursor,
    );
    this.#context(request);
    if (epoch !== this.#epoch || (page.cursor && this.#seen.has(page.cursor)))
      throw new ApiError("invalid_stack_continuation");
    const references = [...this.#references, ...page.references];
    const projection = localStackReferences(references);
    this.#seen.add(request.cursor);
    this.#pages++;
    this.#references = references;
    this.#projection = projection;
    for (const stack of projection.stacks)
      for (const commit of stack.commits) this.#commits.set(commit.id, commit);
    this.#snapshot = productionStackSnapshot(
      projection,
      this.#pin!,
      this.binding.path,
      snapshot.target,
      snapshot.revision,
      page.cursor,
    );
    this.#snapshot.stacks = this.#snapshot.stacks.map((stack) =>
      this.ranges.restore(stack, snapshot.target?.commit),
    );
    this.#snapshot.loadedTargetHistory = this.ranges.target(
      snapshot.target?.commit,
    );
    return cloneWorkspace(this.#snapshot);
  }
  readTargetHistory(
    request: WorkspaceHistoryRequest | WorkspaceStackHistoryRequest,
    signal?: AbortSignal,
  ) {
    return readIssuedStackHistory(
      {
        reads: this.#reads!,
        current: () => this.#context(request),
        commits: this.#commits,
        ranges: this.ranges,
      },
      request,
      signal,
    );
  }
  readStackHistory(
    request: WorkspaceStackHistoryRequest,
    signal?: AbortSignal,
  ) {
    return this.readTargetHistory(request, signal);
  }
  async readSourceFiles(request: WorkspaceSourceRequest) {
    const range = stackSourceRange(
      this.#context(request),
      this.#commits,
      request.source,
    );
    const page = await this.#reads!.files(
      range,
      request.cursor,
      request.signal,
    );
    this.#context(request);
    return page;
  }
  async readFileDiff(request: WorkspaceSourceRequest & { path: string }) {
    const file = await this.#reads!.diff(
      stackSourceRange(this.#context(request), this.#commits, request.source),
      request.path,
      request.signal,
    );
    this.#context(request);
    return file;
  }
  preview(_request: WorkspaceRequest) {
    return unavailableStackCommand();
  }
  apply(_request: WorkspaceRequest) {
    return unavailableStackCommand();
  }
  status(_requestId: string) {
    return unavailableStackCommand();
  }
  close() {
    this.#closed.abort();
    this.#reads?.close();
    this.#commits.clear();
    this.ranges.clear();
  }
}
