import type { WorkspaceFileSource } from "./workspace-types.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";
import type { ChatBodyBlock } from "../ui/store-agent-chat-body/model.js";
import type { Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
import type { AgentSession } from "../controller/agent-execution-types.js";
import type { ConversationSession } from "./session.svelte.js";
import type { operations } from "../api/generated.js";
type Request = NonNullable<
  operations["local_agents_workspace_inspect"]["requestBody"]
>["content"]["application/json"];
export type WorkspaceBinding = {
  session: string;
  repository: string;
  repositoryName: string;
  workspace: string;
  agent: string;
};
export type WorkspaceRead = Model<"AgentWorkspaceInspection">;
type Tab = "files" | "changes";
class Listing {
  value = $state<WorkspaceRead>();
  loading = $state(false);
  error = $state("");
  selected = $state("");
  collapsed = $state<ReadonlySet<string>>(new Set());
  scroll = 0;
  generation = 0;
  abort?: AbortController;
}
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
  #fileGeneration = 0;
  #fileAbort?: AbortController;
  constructor(
    readonly client: LocalClient,
    readonly conversation: ConversationSession,
    readonly inspect: () => void,
  ) {}
  get binding() {
    return (
      this.bindings.find((b) => this.key(b) === this.bindingKey) ??
      this.bindings[0]
    );
  }
  key(b: WorkspaceBinding) {
    return `${b.session}:${b.repository}`;
  }
  get cacheKey() {
    return `${this.binding ? this.key(this.binding) : ""}:${this.tab}:${this.tab === "changes" ? `${this.range}:${this.base ?? ""}` : ""}`;
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
  ) {
    this.bindings = sessions.flatMap((s) =>
      s.repository_bindings.length
        ? s.repository_bindings.map((b) => ({
            session: s.id,
            repository: b.repository,
            repositoryName: names.get(b.repository) ?? b.repository.slice(0, 8),
            workspace: b.workspace.split("/").at(-1) ?? b.workspace,
            agent: s.title,
          }))
        : s.workspace_name
          ? [
              {
                session: s.id,
                repository: s.repository,
                repositoryName:
                  names.get(s.repository) ?? s.repository.slice(0, 8),
                workspace: s.workspace_name,
                agent: s.title,
              },
            ]
          : [],
    );
    if (!this.bindings.some((b) => this.key(b) === this.bindingKey))
      this.bindingKey = this.bindings[0] ? this.key(this.bindings[0]) : "";
  }
  query(
    path: string | null = null,
    cursor: string | null = null,
    files = false,
  ): Request {
    const b = this.binding;
    if (!b) throw new Error("This agent has no available code workspace.");
    return {
      version: 1,
      session: b.session,
      repository: b.repository,
      path,
      cursor,
      query:
        files || this.tab === "files"
          ? { view: "files" }
          : { view: "changes", range: this.range, base: this.base },
    };
  }
  async read(request: Request, signal?: AbortSignal) {
    const value = await this.client.request<WorkspaceRead>(
      "agents/workspace/inspect",
      request,
      signal,
    );
    if (
      value.session !== request.session ||
      value.repository !== request.repository ||
      (request.path && value.file?.path !== request.path)
    )
      throw new Error(
        "The workspace selection changed. Please select it again.",
      );
    return value;
  }
  async load(more = false) {
    const state = this.listing,
      key = this.cacheKey;
    if (state.loading) return;
    const cursor = more ? state.value?.next : null;
    if (more && !cursor) return;
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
      if (state.generation !== generation) return;
      if (more && value.selection !== state.value?.selection)
        throw new Error("The file listing changed. Refresh it to continue.");
      state.value = more
        ? { ...value, files: [...(state.value?.files ?? []), ...value.files] }
        : value;
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
    this.bindingKey = key;
    this.base = null;
    this.fileOpen = false;
    this.file = undefined;
    this.#fileGeneration++;
    this.#fileAbort?.abort();
    await this.load();
  }
  async selectTab(tab: Tab) {
    this.tab = tab;
    await this.load();
  }
  async compare(
    range: Model<"ComparisonRange">,
    base: string | null = this.base,
  ) {
    this.range = range;
    this.base = base;
    await this.load();
  }
  async select(path: string) {
    if (this.file?.file)
      this.#presentations.set(
        `${this.file.selection}:${this.file.file.path}`,
        this.presentation,
      );
    this.listing.selected = path;
    this.fileOpen = true;
    this.inspect();
    const generation = ++this.#fileGeneration;
    this.#fileAbort?.abort();
    this.#fileAbort = new AbortController();
    this.fileLoading = true;
    this.fileError = "";
    try {
      const value = await this.read(this.query(path), this.#fileAbort.signal);
      if (generation === this.#fileGeneration) {
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
  register(value: WorkspaceRead) {
    const file = value.file;
    if (!file || file.unavailable) {
      this.blocks = {};
      return;
    }
    const blocks: { before?: ChatBodyBlock; after?: ChatBodyBlock } = {};
    for (const side of ["before", "after"] as const) {
      const contents = side === "before" ? file.before : file.contents;
      if (contents === null) continue;
      const comparison = value.comparison,
        before = side === "before";
      const operation = before
        ? comparison?.before_operation
        : (comparison?.after_operation ?? value.operation);
      const commit = before
        ? comparison?.before_commit
        : (comparison?.after_commit ?? value.commit);
      if (!operation || !commit) continue;
      const current = !before && (!comparison || comparison.current_files);
      const hash = bytesToHex(sha256(new TextEncoder().encode(contents)));
      const revision = `${operation}:${commit}:${current ? file.fingerprint : hash}`;
      const block: ChatBodyBlock = {
        id: `workspace:${value.session}:${value.repository}:${file.path}:${side}:${revision}`,
        revision,
        kind: "code",
        label: file.path,
        path: file.path,
        text: contents,
      };
      blocks[side] = block;
      const source: WorkspaceFileSource = {
        session: value.session,
        repository: value.repository,
        workspace: value.workspace,
        path: file.path,
        operation,
        commit,
        content_sha256: hash,
        current_fingerprint: current ? file.fingerprint : null,
        selection: null,
        diff:
          comparison?.before_operation && comparison.before_commit
            ? {
                before_operation: comparison.before_operation,
                before_commit: comparison.before_commit,
                after_operation: comparison.after_operation,
                after_commit: comparison.after_commit,
                side,
              }
            : null,
      };
      this.conversation.workspaceSources.set(block.id, {
        block,
        source,
        reveal: () => {
          this.file = value;
          this.blocks = blocks;
          this.presentation = side;
          this.fileOpen = true;
          this.inspect();
        },
      });
    }
    this.blocks = blocks;
  }
  toggleDirectory(path: string) {
    const next = new Set(this.listing.collapsed);
    next.has(path) ? next.delete(path) : next.add(path);
    this.listing.collapsed = next;
  }
}
