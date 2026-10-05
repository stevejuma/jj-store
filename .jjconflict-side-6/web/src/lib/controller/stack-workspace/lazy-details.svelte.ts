import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type WorkspaceSnapshot,
  type WorkspaceFile,
  type WorkspaceSourceRequest,
} from "./types.js";
export type WorkspaceReadState = {
  loading: boolean;
  loaded: boolean;
  error: string;
  nextCursor?: string;
};
const idle = (): WorkspaceReadState => ({
  loading: false,
  loaded: false,
  error: "",
});
export const workspacePatchKey = (source: string, path: string) =>
  JSON.stringify([source, path]);
/** File summaries and selected patches have independent read ownership. */
export class WorkspaceLazyDetails {
  files = $state<Record<string, WorkspaceFile[]>>({});
  #fileStates = $state<Record<string, WorkspaceReadState>>({});
  #patchStates = $state<Record<string, WorkspaceReadState>>({});
  #scope = "";
  #context: Pick<
    WorkspaceSourceRequest,
    "repository" | "revision" | "readKey"
  > | null = null;
  #closed = false;
  #requests = new Map<string, AbortController>();
  #cursors = new Map<string, Set<string>>();
  constructor(
    readonly adapter: StackWorkspaceAdapter,
    readonly onFiles?: (
      source: string,
      files: WorkspaceFile[],
      complete: boolean,
    ) => void,
  ) {}
  get lazy() {
    return Boolean(this.adapter.readSourceFiles);
  }
  fileState(source: string) {
    return this.#fileStates[source] ?? idle();
  }
  patchState(source: string, path: string) {
    return this.#patchStates[workspacePatchKey(source, path)] ?? idle();
  }
  sync(snapshot: WorkspaceSnapshot) {
    const context = {
      repository: snapshot.repository,
      revision: snapshot.revision,
      ...(snapshot.readKey ? { readKey: snapshot.readKey } : {}),
    };
    const scope = JSON.stringify([context, snapshot.active]);
    if (scope === this.#scope) return;
    for (const request of this.#requests.values()) request.abort();
    this.#requests.clear();
    this.#cursors.clear();
    this.#scope = scope;
    this.#context = context;
    this.files = {};
    this.#fileStates = {};
    this.#patchStates = {};
  }
  async loadFiles(source: string, more = false) {
    const state = this.fileState(source);
    if (
      this.#closed ||
      !this.#context ||
      !this.adapter.readSourceFiles ||
      state.loading ||
      (state.loaded && !more && !state.error) ||
      (more && !state.nextCursor)
    )
      return;
    const key = `files:${source}`,
      owner = new AbortController(),
      scope = this.#scope;
    const cursor = state.nextCursor;
    this.#requests.set(key, owner);
    this.#fileStates = {
      ...this.#fileStates,
      [source]: { ...state, loading: true, error: "" },
    };
    try {
      const page = await this.adapter.readSourceFiles({
        ...this.#context,
        source,
        ...(cursor ? { cursor } : {}),
        signal: owner.signal,
      });
      if (!this.#current(scope, key, owner)) return;
      const previous = this.files[source] ?? [],
        known = new Set(previous.map((file) => file.path));
      const cursors = this.#cursors.get(source) ?? new Set<string>();
      if (
        page.files.length > 50 ||
        page.files.some((file) => {
          if (known.has(file.path)) return true;
          known.add(file.path);
          return false;
        }) ||
        (page.nextCursor &&
          (!page.files.length ||
            page.nextCursor === cursor ||
            cursors.has(page.nextCursor)))
      )
        throw new Error(
          "The changed-file page did not advance within its bound.",
        );
      if (cursor) cursors.add(cursor);
      this.#cursors.set(source, cursors);
      this.files = {
        ...this.files,
        [source]: [...previous, ...cloneWorkspace(page.files)],
      };
      this.#fileStates = {
        ...this.#fileStates,
        [source]: {
          loading: false,
          loaded: true,
          error: "",
          ...(page.nextCursor ? { nextCursor: page.nextCursor } : {}),
        },
      };
      this.onFiles?.(source, this.files[source]!, !page.nextCursor);
    } catch (error) {
      if (this.#current(scope, key, owner))
        this.#fileStates = {
          ...this.#fileStates,
          [source]: {
            ...state,
            loading: false,
            error: error instanceof Error ? error.message : String(error),
          },
        };
    } finally {
      if (this.#requests.get(key) === owner) this.#requests.delete(key);
    }
  }
  async loadDiff(source: string, path: string) {
    const id = workspacePatchKey(source, path),
      state = this.patchState(source, path);
    if (
      this.#closed ||
      !this.#context ||
      !this.adapter.readFileDiff ||
      state.loading ||
      state.loaded ||
      !this.files[source]?.some((file) => file.path === path)
    )
      return;
    const key = `patch:${id}`,
      owner = new AbortController(),
      scope = this.#scope;
    this.#requests.set(key, owner);
    this.#patchStates = {
      ...this.#patchStates,
      [id]: { ...state, loading: true, error: "" },
    };
    try {
      const file = await this.adapter.readFileDiff({
        ...this.#context,
        source,
        path,
        signal: owner.signal,
      });
      if (!this.#current(scope, key, owner)) return;
      if (file.path !== path)
        throw new Error("The patch does not match the selected file.");
      this.files = {
        ...this.files,
        [source]: this.files[source]!.map((item) =>
          item.path === path ? cloneWorkspace(file) : item,
        ),
      };
      this.#patchStates = {
        ...this.#patchStates,
        [id]: { loading: false, loaded: true, error: "" },
      };
      this.onFiles?.(source, this.files[source]!, false);
    } catch (error) {
      if (this.#current(scope, key, owner))
        this.#patchStates = {
          ...this.#patchStates,
          [id]: {
            ...state,
            loading: false,
            error: error instanceof Error ? error.message : String(error),
          },
        };
    } finally {
      if (this.#requests.get(key) === owner) this.#requests.delete(key);
    }
  }
  cancelDiff(source: string, path: string) {
    const id = workspacePatchKey(source, path),
      key = `patch:${id}`;
    this.#requests.get(key)?.abort();
    this.#requests.delete(key);
    const state = this.patchState(source, path);
    this.#patchStates = {
      ...this.#patchStates,
      [id]: { ...state, loading: false },
    };
  }
  cancelFiles(source: string) {
    const key = `files:${source}`;
    this.#requests.get(key)?.abort();
    this.#requests.delete(key);
    const state = this.fileState(source);
    if (state.loading)
      this.#fileStates = {
        ...this.#fileStates,
        [source]: { ...state, loading: false },
      };
  }
  #current(scope: string, key: string, owner: AbortController) {
    return (
      !this.#closed &&
      !owner.signal.aborted &&
      scope === this.#scope &&
      this.#requests.get(key) === owner
    );
  }
  close() {
    this.#closed = true;
    for (const owner of this.#requests.values()) owner.abort();
    this.#requests.clear();
  }
}
