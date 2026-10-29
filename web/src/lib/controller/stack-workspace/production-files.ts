import { BranchFileOwners } from "./branch-file-owners.js";
import { stackSourceRange } from "./source-range.js";
import type { LocalStacksSession } from "./local-session.js";
import type { StackWorkspaceReads, StackTreeRange } from "./reads.js";
import type {
  StackCommit,
  WorkspaceFile,
  WorkspaceSnapshot,
  WorkspaceSourceRequest,
} from "./types.js";

/** Owns lazy file reads and concrete native selection resolution at one scope. */
export class ProductionStackFileReads {
  readonly #owners = new BranchFileOwners();
  constructor(
    readonly reads: () => StackWorkspaceReads,
    readonly context: (request: WorkspaceSourceRequest) => WorkspaceSnapshot,
    readonly commits: Map<string, StackCommit>,
    readonly local: LocalStacksSession,
  ) {}
  #current(request: WorkspaceSourceRequest) {
    request.signal?.throwIfAborted();
    return this.context(request);
  }
  #resolve(
    request: WorkspaceSourceRequest,
    range: StackTreeRange,
    files: WorkspaceFile[],
  ) {
    return this.#owners.resolve(
      this.#current(request),
      request.source,
      range,
      files,
      (range, cursor) => this.reads().files(range, cursor, request.signal),
      () => {
        this.#current(request);
      },
    );
  }
  async files(request: WorkspaceSourceRequest) {
    const range = stackSourceRange(
      this.#current(request),
      this.commits,
      request.source,
    );
    const page = await this.reads().files(
      range,
      request.cursor,
      request.signal,
    );
    this.#current(request);
    if (request.source.startsWith("branch:"))
      page.files = await this.#resolve(request, range, page.files);
    return page;
  }
  async diff(request: WorkspaceSourceRequest & { path: string }) {
    const range = stackSourceRange(
      this.#current(request),
      this.commits,
      request.source,
    );
    let file = await this.reads().diff(range, request.path, request.signal);
    this.#current(request);
    if (request.source.startsWith("branch:")) {
      [file] = (await this.#resolve(request, range, [file])) as [WorkspaceFile];
      if (!file.selectionSource) return file;
    }
    const result = await this.local.hunks(
      file.selectionSource ?? request.source,
      file,
    );
    this.#current(request);
    return result;
  }
}
