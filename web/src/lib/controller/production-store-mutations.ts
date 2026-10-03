import { untrack } from "svelte";
import { isLocal, type RepositoryHead } from "./repository-profile.js";
import type { Head } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import {
  mutationReceipt,
  mutationRequest,
  mutationResult,
  type StoreMutationCommand,
  type StoreMutationKind,
  type StoreMutationReceipt,
  type MutableStoreMutationState,
  type StoreMutationRequest,
} from "./store-mutation-types.js";

export function createProductionStoreMutations(options: {
  state: MutableStoreMutationState;
  source: () => StoreDataSource;
  head: () => RepositoryHead | null;
  current: (source: StoreDataSource) => boolean;
  committed: (
    receipt: StoreMutationReceipt,
    command: StoreMutationCommand,
  ) => Promise<void>;
}) {
  let request: StoreMutationRequest | null = null;
  const state = options.state;

  const unresolved = () =>
    state.status === "unknown" || state.status === "applying";
  function reset(force = false) {
    if (!force && untrack(unresolved)) return;
    request = null;
    Object.assign(state, {
      status: "idle",
      requestId: null,
      command: null,
      result: null,
      error: "",
    });
  }

  function configure(available: readonly string[]) {
    state.available = available.filter(isMutationKind);
  }

  async function captureReconciliation(): Promise<Head> {
    if (unresolved())
      throw new Error("A pending command must be recovered first.");
    if (!state.available.includes("reconcile_operations"))
      throw new Error("Reconciliation is unavailable.");
    const source = options.source();
    const head = await source.snapshot(true);
    if (!options.current(source))
      throw new Error("Repository connection changed.");
    if (isLocal(head) || head.heads.length < 2)
      throw new Error(
        "Reconciliation requires multiple current operation heads.",
      );
    return head;
  }

  async function preview(command: StoreMutationCommand, captured?: Head) {
    if (unresolved())
      throw new Error("A pending command must be recovered first.");
    const source = options.source();
    const head = captured ?? options.head();
    if (!head || isLocal(head))
      throw new Error("No captured repository head is available.");
    if (
      captured &&
      (command.type !== "reconcile_operations" ||
        !captured.heads.includes(command.base) ||
        captured.repo_id !== source.repository)
    )
      throw new Error("Choose one captured operation head as the base.");
    request = mutationRequest(head, command);
    Object.assign(state, {
      status: "previewing",
      requestId: request.request_id,
      command,
      result: null,
      error: "",
    });
    try {
      const value = await source.previewMutation(request);
      if (!options.current(source)) return;
      state.result = mutationResult(value);
      const receipt = mutationReceipt(value);
      state.status = receipt ? "committed" : "prepared";
      if (receipt) await options.committed(receipt, request.command);
    } catch (cause) {
      if (!options.current(source)) return;
      state.status = "failed";
      state.error = storeErrorMessage(cause);
    }
  }

  async function apply() {
    if (!request) return;
    const source = options.source();
    state.status = "applying";
    state.error = "";
    try {
      const value = await source.applyMutation(request);
      if (!options.current(source)) return;
      state.result = mutationResult(value) ?? state.result;
      const receipt = mutationReceipt(value);
      state.status = receipt ? "committed" : "unknown";
      if (receipt) await options.committed(receipt, request.command);
    } catch (cause) {
      if (!options.current(source)) return;
      state.status = "unknown";
      state.error = storeErrorMessage(cause);
    }
  }

  async function checkStatus() {
    if (!request) return;
    const source = options.source();
    state.status = "applying";
    state.error = "";
    try {
      const value = await source.mutationStatus(request.request_id);
      if (!options.current(source)) return;
      state.result = mutationResult(value) ?? state.result;
      const receipt = mutationReceipt(value);
      state.status = receipt ? "committed" : "unknown";
      if (receipt) await options.committed(receipt, request.command);
    } catch (cause) {
      if (!options.current(source)) return;
      state.status = "unknown";
      state.error = storeErrorMessage(cause);
    }
  }

  return {
    configure,
    reset: () => reset(true),
    captureReconciliation,
    preview,
    apply,
    checkStatus,
    actions: {
      captureReconciliation,
      previewMutation: preview,
      applyMutation: apply,
      checkMutationStatus: checkStatus,
      resetMutation: () => reset(),
    },
  };
}

const mutationKinds = new Set<StoreMutationKind>([
  "new",
  "describe",
  "bookmark_create",
  "bookmark_move",
  "bookmark_delete",
  "reconcile_operations",
  "abandon",
  "resolve_conflicts",
  "edit_file",
]);

function isMutationKind(value: string): value is StoreMutationKind {
  return mutationKinds.has(value as StoreMutationKind);
}
