import { ApiError, fullId, type Model } from "../api/client.js";
import { LocalEventParser } from "../api/local-events.js";
import {
  readLocalReviewActivity,
  readLocalReviewPage,
} from "./http-local-review-reader.js";
import { assertLocalReviewSnapshot } from "./local-review-snapshot.js";
import { HttpLocalIndexDataSource } from "./http-local-index-data-source.js";
import { HttpLocalWorkDataSource } from "./http-local-work-data-source.js";
import { HttpWorkingTree } from "./http-working-tree.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  unavailableNativeManagement,
  type LocalCapabilities,
} from "./repository-profile.js";
import type { AgentDetail } from "./repository-read-types.js";
export class HttpLocalStoreDataSource
  extends HttpLocalIndexDataSource
  implements StoreDataSource
{
  readonly workingTree = new HttpWorkingTree(this.client);
  readonly workingCopy: import("./working-copy-types.js").WorkingCopyTransport =
    {
      read: async (path) =>
        this.validateWorkingFile(
          await this.read<Model<"WorkingFile">>("working-copy/read", { path }),
          path,
        ),
      save: async (selection, contents) =>
        this.validateWorkingFile(
          await this.client.request<Model<"WorkingFile">>("working-copy/save", {
            selection,
            contents,
          }),
          selection.path,
        ),
    };
  private validateWorkingFile(value: Model<"WorkingFile">, path: string) {
    const selection = value.selection;
    if (
      selection.version !== 1 ||
      selection.repository !== this.repository ||
      selection.path !== path ||
      BigInt(selection.generation) <= 0n ||
      !/^[0-9a-f]{64}$/.test(selection.workspace) ||
      !/^[0-9a-f]{64}$/.test(selection.fingerprint) ||
      new TextEncoder().encode(value.contents).length > 256 * 1024
    )
      throw new ApiError("snapshot_mismatch");
    return value;
  }
  readonly work: HttpLocalWorkDataSource = new HttpLocalWorkDataSource(
    this.client,
    () => this.metadata(),
    this,
    this.read.bind(this),
  );
  async capabilities(): Promise<LocalCapabilities> {
    const value =
      await this.read<Model<"LocalRepositoryCapabilities">>("capabilities");
    if (value.profile !== this.profile)
      throw new ApiError("unsupported_local_profile");
    return {
      ...value,
      read_api: value.code_browse,
      browsing_windows: value.code_browse,
      events: true,
      index_tasks: value.index_tasks,
      typed_mutations: [],
      conflict_resolution_actions: [],
      conflict_upload_bytes: 0,
      ...unavailableNativeManagement,
    };
  }
  private async agent<T extends { snapshot: Model<"LocalAgentSnapshot"> }>(
    path: string,
    query: unknown,
    cursor?: string,
  ) {
    const pin = await this.metadata();
    const response = await this.read<{
      data: T;
      next_cursor: string | null;
    }>(`agents/${path}`, {
      metadata_commit: pin,
      query,
      cursor: cursor ?? null,
    });
    if (
      response.data.snapshot.repo_id !== this.repository ||
      response.data.snapshot.metadata_commit !== pin ||
      response.data.snapshot.profile !== "local_jj_git"
    )
      throw new ApiError("snapshot_mismatch");
    return { ...response.data, next_cursor: response.next_cursor };
  }
  agentSessions(cursor?: string) {
    return this.agent<Model<"LocalBrowserAgentPage">>(
      "sessions",
      { limit: 50 },
      cursor,
    );
  }
  async agentSession(id: string): Promise<AgentDetail> {
    const pin = await this.metadata();
    const value = await this.read<Model<"LocalBrowserAgentDetail">>(
      "agents/session",
      {
        metadata_commit: pin,
        query: { session_id: fullId(id, 32) },
        cursor: null,
      },
    );
    if (
      value.snapshot.repo_id !== this.repository ||
      value.snapshot.metadata_commit !== pin ||
      value.session.session_id !== id
    )
      throw new ApiError("snapshot_mismatch");
    return {
      ...value,
      activity: value.activity.map((entry) =>
        entry.type === "tool"
          ? {
              ...entry,
              tool: {
                ...entry.tool,
                code: value.code.filter((a) =>
                  a.tool_call_ids.includes(entry.tool.tool_call_id),
                ),
              },
            }
          : entry,
      ),
    };
  }
  async agentRelations(
    subject: { commit: string } | { operation: string },
    cursor?: string,
  ) {
    const query =
      "commit" in subject
        ? {
            subject: "revision" as const,
            repository: this.repository,
            commit_id: fullId(subject.commit, 40),
          }
        : {
            subject: "operation" as const,
            repository: this.repository,
            operation_id: fullId(subject.operation),
          };
    const page = await this.agent<Model<"LocalBrowserAgentRelations">>(
      "relations",
      { subject: query, limit: 50 },
      cursor,
    );
    return {
      ...page,
      relations: page.associations.map((association) => ({
        kind: "association" as const,
        object_id: association.object_id,
        association,
      })),
    };
  }
  agentTranscript(
    session: string,
    transcript: Model<"AgentTranscriptDescriptor">,
    cursor?: string,
  ) {
    return this.agent<Model<"LocalBrowserTranscriptPage">>(
      "transcript",
      {
        session_id: fullId(session, 32),
        transcript_id: fullId(transcript.transcript_id, 64),
        source_instance_id: transcript.source_instance_id,
        source_generation: transcript.source_generation,
        limit: 64,
        max_bytes: 256 * 1024,
      },
      cursor,
    );
  }
  async events(
    after: string | undefined,
    signal: AbortSignal,
    deliver: Parameters<StoreDataSource["events"]>[2],
  ) {
    if (after) throw new ApiError("invalid_local_event_cursor");
    const response = await this.client.fetch("events", { signal });
    if (
      !response.ok ||
      !response.headers.get("content-type")?.startsWith("text/event-stream")
    ) {
      await response.body?.cancel();
      throw new ApiError("events_unavailable");
    }
    const reader = response.body?.getReader();
    if (!reader) throw new ApiError("events_unavailable");
    const parser = new LocalEventParser(this.repository, (value) =>
      deliver(value),
    );
    const decoder = new TextDecoder("utf-8", { fatal: true });
    try {
      while (!signal.aborted) {
        const next = await reader.read();
        if (next.done) break;
        parser.push(decoder.decode(next.value, { stream: true }));
      }
      parser.push(decoder.decode());
    } finally {
      await reader.cancel().catch(() => {});
      reader.releaseLock();
    }
  }
  private unsupported(): Promise<never> {
    return Promise.reject(new ApiError("unsupported_local_capability"));
  }
  uploadConflict = () => this.unsupported();
  search = () => this.unsupported();
  searchStatus = () => this.unsupported();
  reviewActivity(signal?: AbortSignal) {
    return readLocalReviewActivity(
      this.repository,
      () => this.metadata(),
      this.read.bind(this),
      signal,
    );
  }
  reviewThreads(cursor?: string, signal?: AbortSignal) {
    return readLocalReviewPage(
      this.repository,
      () => this.metadata(),
      this.read.bind(this),
      cursor,
      signal,
    );
  }
  async reviewThread(id: string) {
    const pin = await this.metadata();
    const detail = await this.read<Model<"LocalReviewThreadDetail">>(
      "reviews/detail",
      {
        metadata_commit: pin,
        thread_id: fullId(id, 32),
      },
    );
    assertLocalReviewSnapshot(detail.snapshot, this.repository, pin);
    if (detail.thread.thread_id !== id) throw new ApiError("snapshot_mismatch");
    return detail;
  }
  private async confirmedReview(outcome: Model<"LocalReviewOutcome">) {
    if (outcome.status === "committed") {
      if (
        outcome.receipt.repository !== this.repository ||
        !outcome.receipt.command
      )
        throw new ApiError("snapshot_mismatch");
      const current = await this.snapshot();
      const fresh =
        await this.read<Model<"LocalRepositorySnapshot">>("snapshot");
      if (
        fresh.repository !== this.repository ||
        String(fresh.generation) !== String(current.generation)
      )
        throw new ApiError("snapshot_mismatch");
      // Confirmed user actions advance companion state; external code remains pinned.
      this.head = {
        ...current,
        metadata_commit: fresh.metadata_commit,
        actor: fresh.actor,
      };
    }
    return outcome;
  }
  async applyReviewCommand(
    request: Parameters<StoreDataSource["applyReviewCommand"]>[0],
  ) {
    const outcome = await this.client.request<Model<"LocalReviewOutcome">>(
      "reviews/commands",
      request,
    );
    if (
      outcome.status === "committed" &&
      outcome.receipt.request_id !== request.request_id
    )
      throw new ApiError("snapshot_mismatch");
    return this.confirmedReview(outcome);
  }
  async reviewCommandStatus(id: string) {
    const outcome = await this.read<Model<"LocalReviewOutcome">>(
      `reviews/commands/${fullId(id, 32)}`,
    );
    if (outcome.status === "committed" && outcome.receipt.request_id !== id)
      throw new ApiError("snapshot_mismatch");
    return this.confirmedReview(outcome);
  }
  previewMutation = () => this.unsupported();
  applyMutation = () => this.unsupported();
  mutationStatus = () => this.unsupported();
}
