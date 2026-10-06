import { projectStackRange } from "./stack-segments.js";
import type { StackWorkspaceReads } from "./reads.js";
import {
  cloneWorkspace,
  type StackCommit,
  type WorkspaceStack,
  type WorkspaceSnapshot,
} from "./types.js";

export const TARGET_RANGE = "__target_history__";

/** Exact immutable range identities, never inferred from presentation order. */
export class LoadedStackRanges {
  #ranges = new Map<
    string,
    {
      head: string;
      base: string | undefined;
      commits: StackCommit[];
      cursor?: string;
    }
  >();
  record(
    stack: string,
    head: string,
    base: string | undefined,
    commits: StackCommit[],
    more: boolean,
    cursor?: string,
  ) {
    const old = this.#ranges.get(stack);
    const prior =
      more && old?.head === head && old.base === base ? old.commits : [];
    const unique = new Map(
      [...prior, ...cloneWorkspace(commits)].map((commit) => [
        commit.id,
        commit,
      ]),
    );
    this.#ranges.set(stack, {
      head,
      base,
      commits: [...unique.values()],
      cursor,
    });
  }
  restore(stack: WorkspaceStack, base?: string) {
    const range = this.#ranges.get(stack.id),
      head = stack.heads.find((head) => head.name === stack.name);
    if (!range || !head || range.head !== head.commit || range.base !== base)
      return stack;
    // Refresh the tip annotation from bookmark metadata without discarding loaded ancestry.
    const commits = range.commits.map((commit) =>
      commit.id === head.commit
        ? {
            ...commit,
            graph: {
              ...commit.graph!,
              bookmarks: [
                ...new Map(
                  [
                    ...commit.graph!.bookmarks,
                    ...(stack.commits.find((item) => item.id === head.commit)
                      ?.graph?.bookmarks ?? []),
                  ].map((bookmark) => [
                    JSON.stringify([bookmark.name, bookmark.remote]),
                    bookmark,
                  ]),
                ).values(),
              ],
            },
          }
        : commit,
    );
    return projectStackRange(stack, base!, commits, range.cursor);
  }
  project(snapshot: WorkspaceSnapshot) {
    snapshot.stacks = snapshot.stacks.map((stack) =>
      this.restore(stack, snapshot.target?.commit),
    );
    snapshot.loadedTargetHistory = this.target(snapshot.target?.commit);
  }
  async refresh(
    reads: StackWorkspaceReads,
    stacks: WorkspaceStack[],
    base: string | undefined,
    current: () => void,
    target?: { commit: string },
  ) {
    for (const [id, range] of this.#ranges) {
      const stack = stacks.find((stack) => stack.id === id),
        head =
          id === TARGET_RANGE
            ? target && { commit: target.commit }
            : stack?.heads.find((head) => head.name === stack.name);
      if (
        !head ||
        range.head !== head.commit ||
        range.base !== (id === TARGET_RANGE ? undefined : base)
      ) {
        this.#ranges.delete(id);
        continue;
      }
      let cursor: string | undefined;
      const commits: StackCommit[] = [],
        seen = new Set<string>();
      do {
        const page = await reads.history({
          head: range.head,
          base: range.base,
          limit: 40,
          cursor,
        });
        current();
        commits.push(...page.commits);
        if (cursor) seen.add(cursor);
        cursor = page.nextCursor;
        if (cursor && seen.has(cursor))
          throw new Error("The stack history continuation did not advance.");
      } while (cursor && commits.length < range.commits.length);
      this.#ranges.set(id, { ...range, commits, cursor });
    }
  }
  target(head?: string) {
    const range = this.#ranges.get(TARGET_RANGE);
    return range && range.head === head
      ? { commits: cloneWorkspace(range.commits), nextCursor: range.cursor }
      : undefined;
  }
  commits() {
    return [...this.#ranges.values()].flatMap((range) => range.commits);
  }
  clear() {
    this.#ranges.clear();
  }
}
