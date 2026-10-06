import { workspaceBranchSource } from "./files.js";
import type {
  StackCommit,
  WorkspaceSnapshot,
  WorkspaceStack,
} from "./types.js";

/** Render retained raw branch selections inside a regrouped card without rekeying them. */
export function stackCardSelection(
  snapshot: WorkspaceSnapshot,
  selections: Record<string, string>,
  stack: WorkspaceStack,
  name: string,
  aliases: string[],
  commits: StackCommit[],
) {
  const branches = snapshot.stacks.filter((item) =>
    [name, ...aliases].includes(item.name),
  );
  const candidates = [...branches, stack];
  const selected = candidates.find((item) => {
    const value = selections[item.id];
    return (
      value ===
        workspaceBranchSource(
          item.id,
          branches.includes(item) ? item.name : name,
        ) || commits.some((commit) => commit.id === value)
    );
  });
  const owner = selected ?? branches[0] ?? stack;
  const branchSource = workspaceBranchSource(
    owner.id,
    branches.includes(owner) ? owner.name : name,
  );
  const selection = selections[owner.id];
  return {
    owner: owner.id,
    branch: branches.includes(owner) ? owner.name : name,
    branchSource,
    branchSelected: selection === branchSource,
    commit: commits.find((commit) => commit.id === selection),
  };
}
