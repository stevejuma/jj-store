import type { Model } from "../../api/client.js";
import type { WorkspaceSnapshot, StackHead } from "./types.js";

/** Keep raw bookmark identities as state owners; lanes are only a presentation. */
export function projectStackTopology(
  snapshot: WorkspaceSnapshot,
  topology: Model<"LocalStacksTopology">,
) {
  snapshot.topology = topology;
  for (const lane of [...(topology.inventory ?? []), ...topology.lanes]) {
    const heads: StackHead[] = lane.heads
      .filter((head) => head.commit)
      .map((head) => ({
        name: head.names[0]!,
        commit: head.commit,
        aliases: head.names.slice(1),
        ...(head.base_commit !== undefined
          ? { baseCommit: head.base_commit }
          : {}),
        ...(head.shared !== undefined ? { shared: head.shared } : {}),
      }));
    for (const [index, head] of heads.entries()) {
      for (const name of [head.name, ...(head.aliases ?? [])]) {
        let stack = snapshot.stacks.find((stack) => stack.name === name);
        if (!stack) {
          stack = {
            id: `branch:${JSON.stringify([name])}`,
            name,
            base: snapshot.base ?? "",
            heads: [],
            commits: [],
            conflicts: false,
          };
          snapshot.stacks.push(stack);
        }
        const own = {
          ...head,
          name,
          aliases: [head.name, ...(head.aliases ?? [])].filter(
            (alias) => alias !== name,
          ),
        };
        stack.topologyHeads = [own, ...heads.slice(index + 1)];
        stack.heads = stack.topologyHeads;
        if (index === 0) {
          stack.appliedBranches = lane.applied;
          stack.unapplyBranches = lane.unapply;
        }
        if (!snapshot.bookmarks?.some((bookmark) => bookmark.name === name)) {
          (snapshot.bookmarks ??= []).push({ name, targets: [head.commit] });
        }
      }
    }
  }
  return snapshot;
}
