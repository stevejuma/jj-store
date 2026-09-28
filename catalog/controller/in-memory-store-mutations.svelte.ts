import {
  initialStoreMutationState,
  type StoreMutationCommand,
  type StoreMutationPrepared,
} from "../../web/src/lib/controller/store-mutation-types.js";

const available = [
  "new",
  "describe",
  "bookmark_create",
  "bookmark_move",
  "bookmark_delete",
  "reconcile_operations",
  "abandon",
  "resolve_conflicts",
] as const;

export function createInMemoryStoreMutations(options: {
  onCommitted: (
    command: StoreMutationCommand,
    result: StoreMutationPrepared,
  ) => void;
  firstApply?: "committed" | "unknown";
  previewFailure?: string;
}) {
  const state = $state({
    ...initialStoreMutationState(),
    available: [...available],
  });
  let command: StoreMutationCommand | null = null;
  let unknown = false;

  async function preview(next: StoreMutationCommand) {
    command = structuredClone(next);
    Object.assign(state, {
      status: "previewing",
      requestId: "ab".repeat(16),
      command,
      result: null,
      error: "",
    });
    await Promise.resolve();
    if (options.previewFailure) {
      state.status = "failed";
      state.error = options.previewFailure;
      return;
    }
    state.result = prepared(command);
    state.status = "prepared";
  }

  async function apply() {
    if (!command) return;
    state.status = "applying";
    await Promise.resolve();
    if (options.firstApply === "unknown" && !unknown) {
      unknown = true;
      state.status = "unknown";
      return;
    }
    options.onCommitted(command, state.result!);
    state.status = "committed";
  }

  async function checkStatus() {
    if (!command) return;
    state.status = "applying";
    await Promise.resolve();
    options.onCommitted(command, state.result!);
    state.status = "committed";
  }

  function reset() {
    command = null;
    Object.assign(state, {
      status: "idle",
      requestId: null,
      command: null,
      result: null,
      error: "",
    });
  }

  return {
    state,
    actions: {
      previewMutation: preview,
      applyMutation: apply,
      checkMutationStatus: checkStatus,
      resetMutation: reset,
    },
  };
}

function prepared(command: StoreMutationCommand): StoreMutationPrepared {
  const target = "cd".repeat(64);
  const result: StoreMutationPrepared = {
    version: 1,
    operation: "ef".repeat(64),
    created: command.type === "new" ? [target] : [],
    rewrites:
      command.type === "describe" ||
      command.type === "abandon" ||
      command.type === "resolve_conflicts"
        ? [
            {
              commit: target,
              change: "12".repeat(16),
              predecessors:
                command.type === "describe" ||
                command.type === "resolve_conflicts"
                  ? [command.commit]
                  : [...command.commits],
            },
          ]
        : [],
    bookmarks: bookmarkChanges(command),
    workspaces: {},
    stats: {
      metadata_nodes: 4,
      predicate_file_bytes: 0,
      predicate_lines: 0,
      object_reads: 6,
      object_bytes: 8192,
      work: 10,
    },
    staged_objects: 6,
    staged_bytes: 8192,
    ...(command.type === "abandon" ? { abandoned: [...command.commits] } : {}),
    ...(command.type === "resolve_conflicts"
      ? {
          conflict_resolution: {
            target: { before: command.commit, after: target },
            selected_paths: command.paths.map((path) => path.path),
            remaining: {},
          },
        }
      : {}),
  };
  return result;
}

function bookmarkChanges(command: StoreMutationCommand) {
  if (command.type === "bookmark_create")
    return { [command.name]: { before: [], after: [command.target] } };
  if (command.type === "bookmark_move")
    return {
      [command.name]: {
        before: [...command.expected],
        after: [command.target],
      },
    };
  if (command.type === "bookmark_delete")
    return { [command.name]: { before: [...command.expected], after: [] } };
  return {};
}
