import { projectStackRange, shareStackSegments } from "./stack-segments.js";
import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type WorkspaceSnapshot,
  type StackCommit,
} from "./types.js";
type PageState = {
  loading: boolean;
  loaded: boolean;
  error: string;
  cursor?: string;
  hasMore: boolean;
  commits: StackCommit[];
};
const empty = (): PageState => ({
  loading: false,
  loaded: false,
  error: "",
  hasMore: false,
  commits: [],
});

/** Bounded real branch ranges; graph parents and segment boundaries stay explicit. */
export class WorkspaceStackHistory {
  #states = $state<Record<string, PageState>>({});
  #requests = new Map<string, AbortController>();
  #cursors = new Map<string, Set<string>>();
  #context: WorkspaceSnapshot | null = null;
  #scope = "";
  #closed = false;
  constructor(
    readonly adapter: StackWorkspaceAdapter,
    readonly accept: (snapshot: WorkspaceSnapshot) => void,
    readonly report?: (cause: unknown) => boolean,
  ) {}
  state(stack: string) {
    return this.#states[stack] ?? empty();
  }
  sync(snapshot: WorkspaceSnapshot) {
    const scope = JSON.stringify([
      snapshot.repository,
      snapshot.revision,
      snapshot.readKey,
      snapshot.target,
    ]);
    this.#context = snapshot;
    if (scope === this.#scope) return;
    this.#scope = scope;
    for (const request of this.#requests.values()) request.abort();
    this.#requests.clear();
    this.#cursors.clear();
    this.#states = Object.fromEntries(
      snapshot.stacks
        .filter((stack) => stack.segments)
        .map((stack) => {
          const segment = stack.segments!.at(-1)!;
          return [
            stack.id,
            {
              loading: false,
              loaded: true,
              error: "",
              commits: [...stack.commits].reverse(),
              cursor: segment.nextCursor,
              hasMore: Boolean(segment.nextCursor),
            },
          ];
        }),
    );
  }
  async load(id: string, more = false) {
    const snapshot = this.#context,
      state = this.state(id);
    const stack = snapshot?.stacks.find((stack) => stack.id === id),
      head = stack?.heads.find((head) => head.name === stack.name);
    if (
      this.#closed ||
      !snapshot?.target ||
      !stack ||
      !head ||
      !this.adapter.readStackHistory ||
      state.loading ||
      (state.loaded && !more && !state.error) ||
      (more && !state.cursor)
    )
      return;
    const scope = this.#scope,
      owner = new AbortController();
    this.#requests.set(id, owner);
    this.#states = {
      ...this.#states,
      [id]: { ...state, loading: true, error: "" },
    };
    try {
      const page = await this.adapter.readStackHistory(
        {
          repository: snapshot.repository,
          revision: snapshot.revision,
          readKey: snapshot.readKey,
          stack: id,
          branch: head.name,
          head: head.commit,
          base: snapshot.target.commit,
          limit: 40,
          cursor: state.cursor,
        },
        owner.signal,
      );
      if (this.#closed || owner.signal.aborted || scope !== this.#scope) return;
      const seen = this.#cursors.get(id) ?? new Set<string>();
      if (
        page.commits.length > 40 ||
        (page.nextCursor &&
          (page.nextCursor === state.cursor || seen.has(page.nextCursor)))
      )
        throw new Error(
          "The stack history page did not advance within its bound.",
        );
      const prior = more ? state.commits : [],
        ids = new Set(prior.map((commit) => commit.id));
      const added = page.commits.filter((commit) => {
        if (ids.has(commit.id)) return false;
        ids.add(commit.id);
        return true;
      });
      if (!added.length && page.nextCursor)
        throw new Error("The stack history page did not advance.");
      const commits = [...prior, ...cloneWorkspace(added)];
      if (state.cursor) seen.add(state.cursor);
      this.#cursors.set(id, seen);
      this.#states = {
        ...this.#states,
        [id]: {
          loading: false,
          loaded: true,
          error: "",
          commits,
          cursor: page.nextCursor,
          hasMore: Boolean(page.nextCursor),
        },
      };
      const current = this.#context!;
      const updated = {
        ...current,
        stacks: current.stacks.map((item) =>
          item.id === id
            ? projectStackRange(
                item,
                snapshot.target!.commit,
                commits,
                page.nextCursor,
              )
            : item,
        ),
      };
      shareStackSegments(
        updated,
        updated.stacks.find((stack) => stack.id === id)!,
      );
      this.accept(updated);
    } catch (error) {
      if (!this.#closed && !owner.signal.aborted && scope === this.#scope)
        this.#states = {
          ...this.#states,
          [id]: {
            ...state,
            loading: false,
            error: this.report?.(error)
              ? ""
              : error instanceof Error
                ? error.message
                : String(error),
          },
        };
    } finally {
      if (this.#requests.get(id) === owner) this.#requests.delete(id);
    }
  }
  close() {
    this.#closed = true;
    for (const request of this.#requests.values()) request.abort();
    this.#requests.clear();
  }
}
