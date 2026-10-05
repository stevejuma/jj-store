import { ApiError, type Model } from "../../api/client.js";
import type { Bookmark } from "../../projects/presentation.js";
import type {
  StackCommit,
  WorkspaceStack,
  WorkspaceTargetBranch,
} from "./types.js";

/** Reference metadata is already bounded by the provider; it carries no loaded files. */
export function localStackReferences(
  references: Model<"LocalReferencesReference">[],
) {
  const names = new Set<string>();
  const unavailable: Record<string, string> = {};
  const bookmarks: Bookmark[] = [],
    targets: WorkspaceTargetBranch[] = [],
    stacks: WorkspaceStack[] = [];
  for (const reference of references) {
    if (reference.kind !== "bookmark")
      throw new ApiError("invalid_stack_references");
    if (reference.name.startsWith("jj-store/workspaces/")) continue;
    if (names.has(reference.name))
      throw new ApiError("invalid_stack_references");
    names.add(reference.name);
    const target =
      reference.targets.length === 1 ? reference.targets[0] : undefined;
    const metadata = target
      ? {
          message: target.description || "No description",
          author: target.author.name || target.author.email,
          email: target.author.email,
          createdAt: Number(BigInt(target.committer.timestamp.millis) / 1000n),
        }
      : undefined;
    bookmarks.push({
      name: reference.name,
      targets: reference.targets.map((target) => target.commit),
      ...(metadata
        ? {
            description: metadata.message,
            author: metadata.author,
            authorEmail: metadata.email,
            updatedAt: metadata.createdAt,
          }
        : {}),
    });
    if (
      reference.conflicted ||
      !target ||
      reference.terms.length !== 1 ||
      reference.terms[0] !== target.commit
    ) {
      unavailable[reference.name] = reference.conflicted
        ? "This bookmark has conflicted targets. Resolve it before selecting a stack."
        : "This bookmark has no available target at this operation.";
      continue;
    }
    const head = { name: reference.name, commit: target.commit };
    targets.push({ ...head, ...metadata! });
    const commit: StackCommit = {
      id: target.commit,
      changeId: target.change,
      ...metadata!,
      immutable: true,
      files: [],
      graph: {
        parents: [...target.parents],
        edges: target.parents.map((parent) => ({
          target: parent,
          kind: "direct",
        })),
        bookmarks: [
          {
            name: reference.name,
            added: true,
            removed: false,
            conflicted: reference.conflicted,
          },
        ],
        workspaces: [...target.workspaces],
        conflicted: target.conflicted,
        divergent: false,
      },
    };
    stacks.push({
      id: `branch:${JSON.stringify([reference.name])}`,
      name: reference.name,
      base: "",
      heads: [head],
      commits: [commit],
      conflicts: target.conflicted,
    });
  }
  return { bookmarks, targets, stacks, unavailable };
}
