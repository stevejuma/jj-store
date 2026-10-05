import { ApiError, fullId } from "../../api/client.js";
import type {
  StoreDataSource,
  StackHistoryQuery,
} from "../native-data-source.js";
import type { CodeEnvelope } from "../repository-read-types.js";
import {
  codeId,
  isLocal,
  sameCodeSnapshot,
  snapshotOperation,
  snapshotRepository,
  type CodeSnapshot,
} from "../repository-profile.js";
import { presentStackCommit, presentStackFile } from "./read-presentation.js";
import type { WorkspaceFile, WorkspaceHistoryPage } from "./types.js";

export type StackTreeRange = { from: string; to: string };
export type StackFilePage = { files: WorkspaceFile[]; nextCursor?: string };

/** One explicit operation/binding pin; no combination setup or mutation. */
export class StackWorkspaceReads {
  readonly #pin: CodeSnapshot;
  readonly #closed = new AbortController();
  readonly operation: string;
  constructor(
    readonly source: StoreDataSource,
    snapshot: CodeSnapshot,
  ) {
    if (
      snapshotRepository(snapshot) !== source.repository ||
      isLocal(snapshot) !== (source.profile === "git_sha1_jj045")
    )
      throw new ApiError("snapshot_mismatch");
    this.operation = fullId(snapshotOperation(snapshot));
    this.#pin = structuredClone(snapshot);
  }
  #signal(input?: AbortSignal) {
    const signal = input
      ? AbortSignal.any([input, this.#closed.signal])
      : this.#closed.signal;
    signal.throwIfAborted();
    return signal;
  }
  #accept<T>(result: CodeEnvelope<T>, signal: AbortSignal) {
    signal.throwIfAborted();
    if (!sameCodeSnapshot(this.#pin, result.snapshot))
      throw new ApiError("snapshot_mismatch");
    return result.data;
  }
  #range(range: StackTreeRange) {
    return {
      from: codeId(range.from, this.source.profile),
      to: codeId(range.to, this.source.profile),
    };
  }
  #page(
    cursor: string | undefined,
    next: string | null,
    count: number,
    limit: number,
  ) {
    if (count > limit || (next && (next === cursor || !count)))
      throw new ApiError("invalid_stack_page");
  }
  async history(
    input: Omit<StackHistoryQuery, "operation">,
  ): Promise<WorkspaceHistoryPage> {
    const signal = this.#signal(input.signal),
      limit = input.limit ?? 40;
    if (!Number.isInteger(limit) || limit < 1 || limit > 500)
      throw new ApiError("invalid_stack_page");
    if (!this.source.stackHistory)
      throw new ApiError("stack_history_unavailable");
    const head = codeId(input.head, this.source.profile);
    const base =
      input.base === undefined ? null : codeId(input.base, this.source.profile);
    const result = await this.source.stackHistory({
      ...input,
      operation: this.operation,
      head,
      ...(base ? { base } : {}),
      limit,
      signal,
    });
    const page = this.#accept(result, signal);
    if (
      page.version !== 1 ||
      page.operation !== this.operation ||
      page.head !== head ||
      page.base !== base
    )
      throw new ApiError("snapshot_mismatch");
    this.#page(input.cursor, result.next_cursor, page.commits.length, limit);
    if (
      Boolean(page.next) !== Boolean(result.next_cursor) ||
      new Set(page.commits.map((node) => node.commit.id)).size !==
        page.commits.length
    )
      throw new ApiError("invalid_stack_page");
    return {
      commits: page.commits.map(presentStackCommit),
      ...(result.next_cursor ? { nextCursor: result.next_cursor } : {}),
    };
  }
  async files(
    input: StackTreeRange,
    cursor?: string,
    caller?: AbortSignal,
  ): Promise<StackFilePage> {
    const signal = this.#signal(caller),
      range = this.#range(input);
    const result = await this.source.diffPaths(
      this.operation,
      range.from,
      range.to,
      cursor,
      signal,
    );
    const page = this.#accept(result, signal);
    if (page.version !== 1 || page.from !== range.from || page.to !== range.to)
      throw new ApiError("snapshot_mismatch");
    this.#page(cursor, result.next_cursor, page.changes.length, 50);
    if (
      new Set(page.changes.map((change) => change.path)).size !==
      page.changes.length
    )
      throw new ApiError("invalid_stack_page");
    return {
      files: page.changes.map((change) =>
        presentStackFile(change, [this.operation, range.from, range.to]),
      ),
      ...(result.next_cursor ? { nextCursor: result.next_cursor } : {}),
    };
  }
  async diff(
    input: StackTreeRange,
    path: string,
    caller?: AbortSignal,
  ): Promise<WorkspaceFile> {
    const signal = this.#signal(caller),
      range = this.#range(input);
    const result = await this.source.diff(
      this.operation,
      range.from,
      range.to,
      path,
      signal,
    );
    const page = this.#accept(result, signal);
    if (
      page.version !== 1 ||
      page.from !== range.from ||
      page.to !== range.to ||
      page.path !== path
    )
      throw new ApiError("snapshot_mismatch");
    if (page.changes.length !== 1 || page.changes[0]!.path !== path)
      throw new ApiError("stack_diff_unavailable");
    return presentStackFile(page.changes[0]!, [
      this.operation,
      range.from,
      range.to,
    ]);
  }
  close() {
    this.#closed.abort();
  }
}
