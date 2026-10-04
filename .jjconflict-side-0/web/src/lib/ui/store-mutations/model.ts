import type {
  StoreMutationCommand,
  StoreMutationKind,
} from "../../controller/store-mutation-types.js";

export type StoreMutationAction =
  | "new"
  | "describe"
  | "bookmark-create"
  | "bookmark-move"
  | "bookmark-delete"
  | "reconcile"
  | "abandon";

export interface StoreMutationFormValue {
  description: string;
  workspace: string;
  bookmark: string;
  target: string;
  base: string;
  expectedTargets: readonly string[];
  retainBookmarks: boolean;
  restoreDescendants: boolean;
}

export const mutationActionKind: Record<
  StoreMutationAction,
  StoreMutationKind
> = {
  new: "new",
  describe: "describe",
  "bookmark-create": "bookmark_create",
  "bookmark-move": "bookmark_move",
  "bookmark-delete": "bookmark_delete",
  reconcile: "reconcile_operations",
  abandon: "abandon",
};

export const mutationActionCopy: Record<
  StoreMutationAction,
  { title: string; description: string; preview: string; destructive: boolean }
> = {
  new: {
    title: "Create change",
    description: "Create a native change at the captured repository head.",
    preview: "Preview change",
    destructive: false,
  },
  describe: {
    title: "Edit description",
    description: "Rewrite the selected change with a new description.",
    preview: "Preview description",
    destructive: false,
  },
  "bookmark-create": {
    title: "Create bookmark",
    description: "Create a JJ bookmark at an explicit commit.",
    preview: "Preview bookmark",
    destructive: false,
  },
  "bookmark-move": {
    title: "Move bookmark",
    description: "Move the selected bookmark from its exact current targets.",
    preview: "Preview move",
    destructive: false,
  },
  "bookmark-delete": {
    title: "Delete bookmark",
    description: "Delete the selected bookmark from its exact current targets.",
    preview: "Preview deletion",
    destructive: true,
  },
  reconcile: {
    title: "Reconcile operations",
    description: "Merge the captured operation heads from an explicit base.",
    preview: "Preview reconciliation",
    destructive: false,
  },
  abandon: {
    title: "Abandon change",
    description: "Abandon the selected change and rewrite its descendants.",
    preview: "Preview abandon",
    destructive: true,
  },
};

export function buildMutationCommand(
  action: StoreMutationAction,
  form: StoreMutationFormValue,
): StoreMutationCommand {
  const target = form.target.trim();
  const name = form.bookmark.trim();
  if (
    ["describe", "bookmark-create", "bookmark-move", "abandon"].includes(action)
  )
    requireFullId(target, "commit");
  if (action === "reconcile") requireFullId(form.base.trim(), "operation");
  if (action.startsWith("bookmark-") && !name)
    throw new Error("Enter a bookmark name.");
  if (action === "new") {
    if (target) requireFullId(target, "parent commit");
    return {
      type: "new",
      parents: target ? [target] : [],
      description: form.description,
      workspace: form.workspace.trim() || null,
    };
  }
  if (action === "describe")
    return { type: "describe", commit: target, description: form.description };
  if (action === "bookmark-create")
    return { type: "bookmark_create", name, target };
  if (action === "bookmark-move")
    return {
      type: "bookmark_move",
      name,
      expected: [...form.expectedTargets],
      target,
    };
  if (action === "bookmark-delete")
    return {
      type: "bookmark_delete",
      name,
      expected: [...form.expectedTargets],
    };
  if (action === "reconcile")
    return { type: "reconcile_operations", base: form.base.trim() };
  return {
    type: "abandon",
    commits: [target],
    retain_bookmarks: form.retainBookmarks,
    restore_descendants: form.restoreDescendants,
  };
}

function requireFullId(value: string, label: string) {
  if (!/^[0-9a-f]{128}$/u.test(value))
    throw new Error(`Choose a full native ${label} ID.`);
}
