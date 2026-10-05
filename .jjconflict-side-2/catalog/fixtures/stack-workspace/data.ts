import type {
  StackCommit,
  WorkspaceFile,
  ConfiguredWorkspaceSnapshot as WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";

export function fixtureFile(path: string, labels: string[]): WorkspaceFile {
  return {
    path,
    hunks: labels.map((label, index) => ({
      id: `${path}:${label}`,
      label,
      patch: `@@ -${index * 10 + 1},1 +${index * 10 + 1},1 @@\n-old_${label.replaceAll(" ", "_")}\n+new_${label.replaceAll(" ", "_")}\n`,
    })),
  };
}
export function fixtureCommit(
  id: string,
  message: string,
  files: WorkspaceFile[],
): StackCommit {
  return {
    id,
    changeId: Array.from(id, (char) => char.charCodeAt(0).toString(16))
      .join("")
      .padEnd(32, "0")
      .slice(0, 32),
    message,
    author: "Alex Morgan",
    createdAt: 1791057600,
    email: "alex@example.test",
    immutable: false,
    files,
  };
}
export function workspaceFixture(
  profile: WorkspaceSnapshot["profile"] = "git_sha1_jj045",
): WorkspaceSnapshot {
  return {
    repository: "local-demo",
    folder: "/home/example/code/local-demo",
    profile,
    availability: "ready",
    revision: 1,
    base: "main",
    target: {
      name: "main",
      commit: "main-history-240",
      message: "Main history 240",
      author: "Alex Morgan",
      email: "alex@example.test",
      createdAt: 1791057600,
    },
    active: "daily",
    workspaces: [
      {
        id: "daily",
        name: "Daily work",
        applied: ["api", "ui"],
        unassigned: [
          fixtureFile("src/settings.ts", [
            "add keyboard shortcut",
            "remember preference",
          ]),
          fixtureFile("README.md", ["document shortcut"]),
        ],
      },
      {
        id: "experiment",
        name: "Experiment",
        applied: ["ui"],
        unassigned: [
          fixtureFile("src/experiment.ts", ["try alternate layout"]),
        ],
      },
    ],
    stacks: [
      {
        id: "api",
        name: "feature/api",
        base: "main",
        conflicts: false,
        heads: [
          { name: "feature/api", commit: "api2" },
          { name: "feature/model", commit: "api1" },
        ],
        commits: [
          fixtureCommit("api1", "Add repository model", [
            fixtureFile("src/model.ts", ["repository model"]),
          ]),
          fixtureCommit("api2", "Expose repository API", [
            fixtureFile("src/api.ts", ["query endpoint", "mutation endpoint"]),
          ]),
        ],
      },
      {
        id: "ui",
        name: "feature/workspace",
        base: "main",
        conflicts: false,
        heads: [{ name: "feature/workspace", commit: "ui1" }],
        commits: [
          fixtureCommit("ui1", "Add workspace toolbar", [
            fixtureFile("src/toolbar.ts", ["workspace picker"]),
          ]),
        ],
      },
      {
        id: "search",
        name: "feature/search",
        base: "main",
        conflicts: true,
        heads: [{ name: "feature/search", commit: "search1" }],
        commits: [
          fixtureCommit("search1", "Add search preference", [
            fixtureFile("src/settings.ts", ["search preference"]),
          ]),
        ],
      },
    ],
    conflicts: [],
    canUndo: false,
    capabilities: [
      "apply_stack",
      "unapply_stack",
      "create_stack",
      "create_branch",
      "rename_branch",
      "delete_branch",
      "squash_branch",
      "add_empty_commit",
      "delete_commit",
      "edit_commit",
      "rename_stack",
      "delete_stack",
      "create_workspace",
      "switch_workspace",
      "delete_workspace",
      "update_workspace",
      "create_commit",
      "reword",
      "move_commit",
      "reorder_commit",
      "squash",
      "split",
      "transfer",
      "uncommit",
      "rebase",
      "resolve_conflict",
      "undo",
    ],
  };
}

/** Rich catalog examples; unit fixtures keep their small, explicit command topology. */
export function workspaceStoryFixture(profile?: WorkspaceSnapshot["profile"]) {
  const snapshot = workspaceFixture(profile);
  for (const stack of snapshot.stacks) {
    const commits: StackCommit[] = [];
    for (const commit of stack.commits) {
      commits.push(
        fixtureCommit(
          `${commit.id}-setup`,
          `Prepare ${commit.message.toLowerCase()}`,
          [fixtureFile(`tests/${commit.id}.test.ts`, ["cover stack behavior"])],
        ),
        commit,
      );
    }
    stack.commits = commits;
  }
  return snapshot;
}
