import type { StackFilePage, StackTreeRange } from "./reads.js";
import { workspaceSourceStack } from "./files.js";
import type { StackCommit, WorkspaceFile, WorkspaceSnapshot } from "./types.js";

/** Net patches are never authority. Resolve only an exact, unique native delta. */
export class BranchFileOwners {
  #scope = "";
  #files = new Map<string, { files: WorkspaceFile[]; pages: number }>();
  async resolve(
    snapshot: WorkspaceSnapshot,
    source: string,
    range: StackTreeRange,
    files: WorkspaceFile[],
    read: (range: StackTreeRange, cursor?: string) => Promise<StackFilePage>,
    current: () => void,
  ): Promise<WorkspaceFile[]> {
    const scope = snapshot.readKey ?? "";
    if (scope !== this.#scope) {
      this.#scope = scope;
      this.#files.clear();
    }
    const stack = workspaceSourceStack(snapshot, source);
    if (!stack || !snapshot.capabilities.includes("transfer")) return files;
    const commits: StackCommit[] = [];
    const seen = new Set<string>();
    let id = range.to;
    while (id !== range.from) {
      const commit = stack.commits.find((item) => item.id === id);
      if (
        !commit?.graph ||
        commit.graph.parents.length !== 1 ||
        commit.graph.conflicted ||
        commit.graph.divergent ||
        seen.has(id) ||
        commits.length === 40
      )
        return files;
      seen.add(id);
      commits.push(commit);
      id = commit.graph.parents[0]!;
    }
    const changes = new Map<string, { owner: string; file: WorkspaceFile }[]>();
    let pages = 0;
    for (const commit of commits) {
      const cached = this.#files.get(commit.id);
      let summary = cached?.files;
      if (cached) {
        pages += cached.pages;
        if (pages > 50) return files;
      } else {
        summary = [];
        let commitPages = 0;
        let cursor: string | undefined;
        const cursors = new Set<string>();
        do {
          if (++pages > 50) return files;
          commitPages++;
          const page = await read(
            { from: commit.graph!.parents[0]!, to: commit.id },
            cursor,
          );
          current();
          if (page.nextCursor && cursors.has(page.nextCursor)) return files;
          if (page.nextCursor) cursors.add(page.nextCursor);
          summary.push(...page.files);
          cursor = page.nextCursor;
        } while (cursor);
        if (scope === this.#scope)
          this.#files.set(commit.id, { files: summary, pages: commitPages });
      }
      for (const file of summary!) {
        const owners = changes.get(file.path) ?? [];
        owners.push({ owner: commit.id, file });
        changes.set(file.path, owners);
      }
    }
    return files.map((file) => {
      const owners = changes.get(file.path);
      const owned = owners?.length === 1 ? owners[0] : undefined;
      const commit = commits.find((item) => item.id === owned?.owner);
      return owned &&
        !commit?.immutable &&
        file.change &&
        owned.file.change &&
        JSON.stringify(file.change) === JSON.stringify(owned.file.change)
        ? { ...file, selectionSource: owned.owner }
        : file;
    });
  }
}
