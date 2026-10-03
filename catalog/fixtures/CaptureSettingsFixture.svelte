<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { ApiError } from "../../web/src/lib/api/api-error.js";
  import { createCaptureManagement } from "../../web/src/lib/controller/capture-management.svelte.js";
  import StoreCaptureSettings from "../../web/src/lib/ui/store-settings/StoreCaptureSettings.svelte";
  import type {
    CaptureImport,
    CaptureTransport,
  } from "../../web/src/lib/controller/capture-types.js";
  import {
    captureReceipt,
    captureSources,
    captureStatus,
  } from "./capture-management.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    supported = true,
    enabled = true,
    unknown = false,
    importUnknown = false,
    degraded = false,
  }: {
    supported?: boolean;
    enabled?: boolean;
    unknown?: boolean;
    importUnknown?: boolean;
    degraded?: boolean;
  } = $props();
  let value = captureStatus();
  let accepted: CaptureImport | null = null;
  let controlAttempts = 0,
    importAttempts = 0;
  const transport: CaptureTransport = {
    async status() {
      const result = structuredClone(value);
      if (!enabled)
        return { version: 1, enabled: false, policy: null, runtime: null };
      if (degraded) {
        result.runtime!.degraded_sources = 1;
        result.runtime!.discovery.unavailable_roots = 1;
        result.runtime!.ingress_required_capacity_bytes = 8192;
      }
      return result;
    },
    async sources() {
      return enabled
        ? captureSources()
        : {
            version: 1,
            enabled: false,
            runtime_generation: null,
            phase: null,
            total: 0,
            sources: [],
            next_cursor: null,
          };
    },
    async control(command) {
      controlAttempts++;
      if (unknown && controlAttempts === 1)
        throw new TypeError("lost response");
      if (unknown && controlAttempts === 2)
        throw new ApiError("capture_policy_changed");
      value = captureStatus(
        command.action === "pause",
        BigInt(command.expected_revision) + 1n,
      );
      return {
        version: 1,
        receipt: {
          generation: command.generation,
          request_id: command.request_id,
          expected_revision: command.expected_revision,
          revision: value.policy!.revision,
          paused: value.policy!.paused,
        },
        replayed: controlAttempts > 1,
        runtime_requested: controlAttempts === 1,
        status: structuredClone(value),
      };
    },
    async rescan() {
      value.runtime!.discovery.rescan_pending = true;
      return { version: 1, accepted: true, status: structuredClone(value) };
    },
    async import(command) {
      accepted = command;
      importAttempts++;
      if (importUnknown && importAttempts === 1)
        throw new TypeError("lost response");
      return captureReceipt(command);
    },
    async inspect(id) {
      const outcome = captureReceipt(accepted ?? undefined);
      return {
        version: 1,
        request_id: id,
        receipt: accepted?.request_id === id ? outcome.receipt : null,
        progress:
          accepted?.request_id === id
            ? outcome.progress
            : {
                ...outcome.progress,
                phase: "pending",
                boundary_available: false,
              },
      };
    },
    close() {},
  };
  const management = untrack(() =>
    createCaptureManagement({
      transport,
      available: () => supported,
      requestId: () => "cd".repeat(16),
    }),
  );
  onDestroy(() => management.destroy());
</script>

<StoreCaptureSettings
  {management}
  repositoryName={() => "Capture repository"}
/>
