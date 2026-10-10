import { RepositoryInspection } from "./repository-inspection.js";
import {
  workspaceQuery,
  readWorkspace,
  showSavedTurn,
  registerWorkspaceRead,
} from "./workspace-observation.js";
import type {
  AgentTurnSelection,
  AgentChangeSummary,
} from "../controller/agent-workspace-changes.js";
import { Listing } from "./workspace-listing.svelte.js";
import { WorkspaceReadCache } from "./workspace-read-cache.js";
import {
  inspectionBindings,
  activeWorkspaceBinding,
  workspaceBindingKey,
  type WorkspaceBinding,
} from "./workspace-bindings.js";
import type { ApplicationFreshness } from "../controller/application-freshness.js";
import { watchWorkspace } from "./workspace-freshness.js";
import type { ChatBodyBlock } from "../ui/store-agent-chat-body/model.js";
import type { Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
import type { AgentSession } from "../controller/agent-execution-types.js";
import type { ConversationSession } from "./session.svelte.js";
import type { operations } from "../api/generated.js";
type Request = NonNullable<
  operations["local_agents_workspace_inspect"]["requestBody"]
>["content"]["application/json"];
export type WorkspaceRead = Model<"AgentWorkspaceInspection">;
type Tab = "files" | "changes";
/** Session-owned observation state. No browser persistence and no checkout mutation. */
export class WorkspaceBrowser {
  bindings = $state<WorkspaceBinding[]>([]);
  bindingKey = $state("");
  tab = $state<Tab>("files");
  range = $state<Model<"ComparisonRange">>("branch");
  base = $state<string | null>(null);
  details = $state<WorkspaceRead>();
  file = $state<WorkspaceRead>();
  fileOpen = $state(false);
  fileLoading = $state(false);
  fileError = $state("");
  railOpen = $state(false);
  mode =
    $state<import("@lapismd/design-core/diff").FileListingViewMode>("tree");
  presentation = $state<"diff" | "before" | "after">("diff");
  #presentations = new Map<string, "diff" | "before" | "after">();
  diffMode = $state<"unified" | "split">("unified");
  #listings = new Map<string, Listing>();
  #files = new WorkspaceReadCache();
  #epoch = 0;
  #requestRefresh?: () => void;
  #historical = false;
  savedTurn = $state<{
    selection: AgentTurnSelection;
    summary: AgentChangeSummary;
  }>();
  #fileKey = "";
  #fileRequest?: Request;
  #fileGeneration = 0;
  #fileAbort?: AbortController;
  repositoryReads?: RepositoryInspection;
  constructor(
    readonly client: LocalClient,
    readonly conversation: ConversationSession,
    readonly inspect: () => void,
  ) {}
  /** One application-feed reader, owned by the shell rather than either panel. */
  watch(freshness: ApplicationFreshness | undefined, active: () => boolean) {
    const watch = watchWorkspace(this, freshness, active);
    this.#requestRefresh = watch.request;
    return () => {
      this.#requestRefresh = undefined;
      watch.stop();
      this.dispose();
    };
  }
  invalidate(repository?: string) {
    this.#epoch++;
    for (const [key, state] of this.#listings)
      if (!repository || key.split(":")[1] === repository) state.dirty = true;
    for (const [key, value] of this.#files)
      if (!repository || value.repository === repository)
        this.#files.delete(key);
  }
  async refreshVisible() {
    if (this.railOpen) await this.load();
    if (
      this.fileOpen &&
      !this.#historical &&
      this.#fileKey &&
      !this.#files.has(this.#fileKey)
    )
      await this.select(
        this.#fileRequest?.path ?? this.listing.selected,
        this.#fileRequest,
      );
    return (!this.railOpen || !this.listing.dirty) && !this.fileError;
  }
  async refresh() {
    this.invalidate(this.binding?.repository);
    return this.refreshVisible();
  }
  dispose() {
    this.#epoch++;
    this.#fileGeneration++;
    this.#fileAbort?.abort();
    for (const state of this.#listings.values()) {
      state.generation++;
      state.abort?.abort();
    }
    this.#files.clear();
  }
  get binding() {
    return activeWorkspaceBinding(this.bindings, this.bindingKey);
  }
  key = workspaceBindingKey;
  get cacheKey() {
    return `${this.binding ? this.key(this.binding) : ""}:${this.tab}:${this.tab === "changes" ? `${this.range}:${this.base ?? ""}:${this.savedTurn?.selection.requestId ?? ""}` : ""}`;
  }
  get listing() {
    const key = this.cacheKey;
    let state = this.#listings.get(key);
    if (!state) {
      state = new Listing();
      this.#listings.set(key, state);
    }
    return state;
  }
  setBindings(
    sessions: readonly AgentSession[],
    names: ReadonlyMap<string, string>,
    repositories: readonly string[] = [],
  ) {
    const previous = this.bindings;
    this.bindings = inspectionBindings(sessions, names, repositories);
    for (const old of previous) {
      const next = this.bindings.find((b) => this.key(b) === this.key(old));
      if (
        !next ||
        next.location !== old.location ||
        next.revision !== old.revision
      ) {
        this.invalidate(old.repository);
        this.#requestRefresh?.();
        if (!next && this.file?.repository === old.repository) {
          this.file = undefined;
          this.blocks = {};
          this.fileOpen = false;
          this.#fileGeneration++;
          this.#fileAbort?.abort();
        }
      }
    }
    if (!this.bindings.some((b) => this.key(b) === this.bindingKey))
      this.bindingKey = this.bindings[0] ? this.key(this.bindings[0]) : "";
  }
  query(
    path: string | null = null,
    cursor: string | null = null,
    files = false,
  ): Request {
    return workspaceQuery(this, path, cursor, files);
  }
  read(request: Request, signal?: AbortSignal) {
    return readWorkspace(this, request, signal);
  }
  async load(more = false) {
    const state = this.listing,
      key = this.cacheKey;
    if (state.loading || (!more && state.value && !state.dirty)) return;
    const cursor = more ? state.value?.next : null;
    if (more && !cursor) return;
    const epoch = this.#epoch;
    const generation = ++state.generation;
    state.abort?.abort();
    state.abort = new AbortController();
    state.loading = true;
    state.error = "";
    try {
      const value = await this.read(
        this.query(null, cursor),
        state.abort.signal,
      );
      if (state.generation !== generation || epoch !== this.#epoch) return;
      if (more && value.selection !== state.value?.selection)
        throw new Error("The file listing changed. Refresh it to continue.");
      state.value = more
        ? { ...value, files: [...(state.value?.files ?? []), ...value.files] }
        : value;
      state.dirty = false;
      if (this.cacheKey === key) this.details = value;
    } catch (error) {
      if (!state.abort.signal.aborted && state.generation === generation)
        state.error =
          error instanceof Error
            ? error.message
            : "Workspace files are unavailable.";
    } finally {
      if (state.generation === generation) state.loading = false;
    }
  }
  async selectBinding(key: string) {
    this.savedTurn = undefined;
    this.bindingKey = key;
    this.base = null;
    this.fileOpen = false;
    this.file = undefined;
    this.#fileGeneration++;
    this.#fileAbort?.abort();
    if (this.railOpen) await this.load();
  }
  async selectTab(tab: Tab) {
    if (
      tab === "changes" &&
      this.binding?.source === "repository" &&
      this.range === "branch" &&
      !this.base
    )
      this.range = "since_last_commit";
    this.tab = tab;
    await this.load();
  }
  async compare(
    range: Model<"ComparisonRange">,
    base: string | null = this.base,
  ) {
    this.savedTurn = undefined;
    this.range = range;
    this.base = base;
    await this.load();
  }
  async select(path: string, retainedRequest?: Request) {
    if (!path) return;
    this.#historical = false;
    if (this.file?.file)
      this.#presentations.set(
        `${this.file.selection}:${this.file.file.path}`,
        this.presentation,
      );
    if (!retainedRequest) this.listing.selected = path;
    this.fileOpen = true;
    if (!retainedRequest) this.inspect();
    const request = retainedRequest ?? this.query(path);
    this.#fileRequest = request;
    const generation = ++this.#fileGeneration;
    this.#fileAbort?.abort();
    this.#fileAbort = new AbortController();
    const epoch = this.#epoch;
    const key = JSON.stringify([
      request,
      this.savedTurn?.selection.requestId ?? "",
    ]);
    this.#fileKey = key;
    const cached = this.#files.get(key);
    this.fileLoading = !cached;
    this.fileError = "";
    try {
      const value =
        cached ?? (await this.read(request, this.#fileAbort.signal));
      if (generation === this.#fileGeneration && epoch === this.#epoch) {
        this.#files.remember(key, value);
        this.file = value;
        this.presentation =
          this.#presentations.get(`${value.selection}:${value.file?.path}`) ??
          (value.comparison ? "diff" : "after");
        this.register(value);
      }
    } catch (error) {
      if (
        generation === this.#fileGeneration &&
        !this.#fileAbort.signal.aborted
      )
        this.fileError =
          error instanceof Error
            ? error.message
            : "File content is unavailable.";
    } finally {
      if (generation === this.#fileGeneration) this.fileLoading = false;
    }
  }
  blocks = $state<{ before?: ChatBodyBlock; after?: ChatBodyBlock }>({});
  showSavedTurn(selection: AgentTurnSelection, summary: AgentChangeSummary) {
    showSavedTurn(this, selection, summary);
  }
  register(value: WorkspaceRead) {
    registerWorkspaceRead(this, value, () => {
      this.#historical = true;
      this.#fileGeneration++;
      this.#fileAbort?.abort();
      this.fileLoading = false;
      this.fileError = "";
    });
  }
  toggleDirectory(path: string) {
    const next = new Set(this.listing.collapsed);
    next.has(path) ? next.delete(path) : next.add(path);
    this.listing.collapsed = next;
  }
}
