<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { ApiError } from "../../web/src/lib/api/api-error.js";
  import { createCaptureManagement } from "../../web/src/lib/controller/capture-management.svelte.js";
  import StoreAgentSettings from "../../web/src/lib/ui/store-settings/StoreAgentSettings.svelte";
  import { ownNotifications } from "../../web/src/lib/notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../../web/src/lib/ui/store-notifications/StoreNotificationHost.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type {
    CaptureImport,
    CaptureTransport,
  } from "../../web/src/lib/controller/capture-types.js";
  import {
    captureReceipt,
    captureSources,
    captureStatus,
    captureSettings,
    mixedCaptureSources,
  } from "./capture-management.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    supported = true,
    enabled = true,
    unknown = false,
    importUnknown = false,
    degraded = false,
    feedbackControls = false,
    configurationFailure = "",
    delayedObservations = false,
    busyObservations = false,
    multipleHarnesses = false,
  }: {
    supported?: boolean;
    enabled?: boolean;
    unknown?: boolean;
    importUnknown?: boolean;
    degraded?: boolean;
    feedbackControls?: boolean;
    configurationFailure?: "" | "stale" | "unknown";
    delayedObservations?: boolean;
    busyObservations?: boolean;
    multipleHarnesses?: boolean;
  } = $props();
  const notifications = ownNotifications();
  let failedObservations = false;
  let busyReads = untrack(() => busyObservations);
  let releaseReads!: () => void;
  let readGate = untrack(() =>
    delayedObservations
      ? new Promise<void>((resolve) => {
          releaseReads = resolve;
        })
      : Promise.resolve(),
  );
  let visible = $state(true);
  let value = captureStatus();
  let settings = untrack(() => captureSettings(enabled));
  let openedRepository = $state("");
  let accepted = $state<CaptureImport | null>(null);
  let controlAttempts = 0,
    importAttempts = 0;
  const transport: CaptureTransport = {
    async settings() {
      await readGate;
      return structuredClone(settings);
    },
    async saveSettings(command) {
      if (configurationFailure === "stale")
        throw new ApiError("capture_settings_changed");
      settings = {
        ...settings,
        revision: "cd".repeat(32),
        desired: structuredClone(command.configuration),
        restart_required: true,
      };
      if (configurationFailure === "unknown")
        throw new TypeError("lost response");
      return structuredClone(settings);
    },
    async status() {
      if (failedObservations)
        throw new ApiError("capture_management_unavailable");
      if (busyReads) throw new ApiError("capture_control_busy");
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
        ? multipleHarnesses
          ? mixedCaptureSources()
          : captureSources()
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
      const outcome = captureReceipt(command);
      if (multipleHarnesses) {
        const source = mixedCaptureSources().sources.find(
          (s) => s.selector === command.selector,
        )!;
        outcome.receipt.profile = source.profile;
        outcome.receipt.product_version = source.product_version;
      }
      return outcome;
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

{#if delayedObservations}<Button onclick={() => releaseReads()}
    >Finish capture load</Button
  >
  <Button
    onclick={() => {
      readGate = new Promise<void>((resolve) => {
        releaseReads = resolve;
      });
      void management.refresh();
    }}>Delay capture refresh</Button
  >{/if}
{#if busyObservations}<Button
    onclick={() => {
      busyReads = false;
      void management.refresh();
    }}>Allow capture observations</Button
  >{/if}
{#if feedbackControls}
  <Button
    onclick={() => {
      failedObservations = true;
      void management.refresh();
    }}>Fail capture observations</Button
  >
  <Button
    onclick={() => {
      failedObservations = false;
      void management.refresh();
    }}>Recover capture observations</Button
  >
  <Button onclick={() => (visible = !visible)}>Toggle capture page</Button>
{/if}
{#if visible}<StoreAgentSettings
    {management}
    repositoryName={() => "Capture repository"}
    onRepository={(id) => (openedRepository = id)}
  />{/if}
<StoreNotificationHost owner={notifications} />

{#if multipleHarnesses && accepted}<output aria-label="Accepted source selector"
    >{accepted.selector}</output
  >{/if}

{#if openedRepository}<output aria-label="Opened capture repository"
    >{openedRepository}</output
  >{/if}
