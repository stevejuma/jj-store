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

  function reset() {
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

  async function preview(command: StoreMutationCommand) {
    const source = options.source();
    const head = options.head();
    if (!head || isLocal(head))
      throw new Error("No captured repository head is available.");
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
      state.status = "failed";
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
      state.status = "failed";
      state.error = storeErrorMessage(cause);
    }
  }

  return {
    configure,
    reset,
    preview,
    apply,
    checkStatus,
    actions: {
      previewMutation: preview,
      applyMutation: apply,
      checkMutationStatus: checkStatus,
      resetMutation: reset,
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
