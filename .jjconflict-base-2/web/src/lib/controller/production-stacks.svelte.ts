import { LocalClient } from "../api/local-client.js";
import { ApiError, type Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreRoute } from "./contracts.js";
import { isLocal, type LocalReadSnapshot } from "./repository-profile.js";
import { reportProductionStoreError } from "./production-store-lifecycle.svelte.js";
import { StackWorkspaceController } from "./stack-workspace/controller.svelte.js";
import { ProductionStackWorkspaceAdapter } from "./stack-workspace/production-adapter.js";
import { productionStackSnapshot } from "./stack-workspace/production-snapshot.js";
import { localStackReferences } from "./stack-workspace/local-references.js";

/** Owns only binding discovery and Stacks reads; the repository owns the shared transport. */
export function createProductionStacks(options: {
  data: ProductionStoreState;
  source(): StoreDataSource | null;
  route(): StoreRoute;
  fetcher?: typeof fetch;
}) {
  let controller = $state<StackWorkspaceController>();
  let adapter: ProductionStackWorkspaceAdapter | undefined;
  let owner: StoreDataSource | null = null;
  let pending: AbortController | null = null;
  let epoch = 0;
  let loaded = false;
  function clear() {
    epoch++;
    pending?.abort();
    pending = null;
    controller?.close();
    controller = undefined;
    adapter = undefined;
    owner = null;
    loaded = false;
  }
  async function restore(load = true) {
    const source = options.source(),
      route = options.route(),
      operation = route.snapshotOperationId;
    if (
      options.data.accessMode !== "local_trusted" ||
      source?.profile !== "git_sha1_jj045" ||
      !options.data.capabilities?.read_api ||
      !operation
    ) {
      clear();
      return;
    }
    const ticket = ++epoch,
      abort = new AbortController();
    pending?.abort();
    pending = abort;
    const current = () =>
      ticket === epoch &&
      !abort.signal.aborted &&
      source === options.source() &&
      route === options.route();
    const client = new LocalClient("", options.fetcher);
    try {
      const registry = await client.request<Model<"LocalBindings">>(
        "repositories",
        undefined,
        abort.signal,
      );
      const binding = registry.bindings.find(
        (binding) => binding.repository === source.repository,
      );
      if (!current()) return;
      if (
        registry.version !== 1 ||
        !binding ||
        binding.availability !== "ready" ||
        !binding.path
      ) {
        clear();
        return;
      }
      const captured = await source.snapshot();
      if (!current()) return;
      if (
        !isLocal(captured) ||
        captured.repository !== source.repository ||
        String(binding.generation) !== String(captured.generation)
      )
        throw new ApiError("snapshot_mismatch");
      if (
        owner !== source ||
        !adapter ||
        String(adapter.binding.generation) !== String(binding.generation) ||
        adapter.binding.path !== binding.path
      ) {
        controller?.close();
        adapter = new ProductionStackWorkspaceAdapter(
          source,
          binding,
          operation,
        );
        const pin: LocalReadSnapshot = {
          version: 1,
          profile: "git_sha1_jj045",
          repository: source.repository,
          generation: captured.generation,
          heads: [...captured.heads],
          operation,
        };
        controller = new StackWorkspaceController(adapter, {
          initial: productionStackSnapshot(
            localStackReferences([]),
            pin,
            binding.path,
            null,
            1,
          ),
          report: (cause) => {
            if (
              cause instanceof ApiError &&
              !["invalid_stack_page", "invalid_stack_continuation"].includes(
                cause.code,
              )
            ) {
              if (
                [
                  "local_folder_unavailable",
                  "local_identity_changed",
                  "snapshot_mismatch",
                ].includes(cause.code)
              )
                clear();
              reportProductionStoreError(options.data, cause);
              return true;
            }
            return false;
          },
        });
        owner = source;
        loaded = false;
      }
      // Revalidate ownership above, but do not replace a pending selection/read or command.
      if (controller!.locked) return;
      adapter.setOperation(operation);
      if (load) {
        await controller!.refresh();
        if (!current()) return;
        loaded = true;
        if (controller!.selectedStack)
          await controller!.histories.load(controller!.selectedStack);
        if (controller!.targetHistory.active)
          await controller!.targetHistory.select();
        options.data.status.navigation = options.data.status.error
          ? "failed"
          : "ready";
      }
    } catch (cause) {
      if (current()) {
        clear();
        reportProductionStoreError(options.data, cause);
      }
    } finally {
      client.close();
      if (pending === abort) pending = null;
    }
  }
  return {
    get controller() {
      return controller;
    },
    clear,
    restore,
    async ensure() {
      await restore(false);
      if (controller && !loaded) await restore();
    },
  };
}
