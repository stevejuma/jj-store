<script lang="ts">
  import { provideSettingsPresentation } from "./settings-presentation.js";
  provideSettingsPresentation({ value: "global" });
  import { untrack } from "svelte";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import EllipsisVerticalIcon from "@lucide/svelte/icons/ellipsis-vertical";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { RuntimeReload } from "../../controller/runtime-reload.svelte.js";
  let {
    manager,
    menu = false,
    disabled = false,
    triggerless = false,
  }: {
    manager?: RuntimeReload;
    menu?: boolean;
    disabled?: boolean;
    triggerless?: boolean;
  } = $props();
  let trigger = $state<HTMLButtonElement | null>(null);
  function open() {
    manager!.returnFocus = () => trigger?.focus();
    manager!.open = true;
    void manager!.observe();
  }
  const names: Record<string, string> = {
    workflows: "Workflow hosts",
    service: "Service limits",
    protection: "Protection rules",
    sync: "Sync configuration",
    capture: "Transcript capture",
    remote_agents: "Server agents and sharing",
    search: "Search configuration",
    agents: "Agent launch settings",
    compute: "Compute",
  };
  const states: Record<string, string> = {
    running: "Reloading…",
    applied: "Applied",
    current: "Current",
    deferred: "Deferred",
    failed: "Failed",
    unknown: "Outcome unknown",
  };
  $effect(() => {
    const owner = manager;
    if (owner?.available()) return untrack(() => owner.watch());
  });
</script>

{#if manager?.value}
  {#if !triggerless}
    {#if menu}
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              bind:ref={trigger}
              variant="ghost"
              size="icon-sm"
              aria-label="Application menu"
              title="Application menu"
              {disabled}
            >
              <EllipsisVerticalIcon aria-hidden="true" />
            </Button>
          {/snippet}
        </DropdownMenu.Trigger>
        <DropdownMenu.Content
          align="end"
          style="min-width: 14rem; max-width: calc(100vw - 2rem)"
          aria-label="Application actions"
          onCloseAutoFocus={(event) => {
            if (manager!.open) event.preventDefault();
          }}
        >
          <DropdownMenu.Group>
            <DropdownMenu.Item onSelect={open}>
              <RotateCwIcon aria-hidden="true" />Reload saved settings
            </DropdownMenu.Item>
          </DropdownMenu.Group>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    {:else}
      <Button
        bind:ref={trigger}
        variant="outline"
        size="sm"
        {disabled}
        onclick={open}
      >
        <RotateCwIcon data-icon="inline-start" aria-hidden="true" />Reload saved
        settings
      </Button>
    {/if}
  {/if}
  <Dialog.Root
    open={manager.open}
    onOpenChange={(open) => {
      if (!manager!.busy) manager!.open = open;
    }}
  >
    <DialogFrame
      title="Reload saved settings"
      closeLabel="Close configuration reload"
      busy={manager.busy}
      aria-describedby={undefined}
      onCloseAutoFocus={(event) => {
        event.preventDefault();
        (manager.returnFocus ?? (() => trigger?.focus()))();
        manager.returnFocus = undefined;
      }}
    >
      <div class="store-settings--global">
        <StoreNotice
          noticeKey="configuration-reload"
          title="Configuration reload"
          message={manager.error}
        />
        <StoreSettingsCard
          id="runtime-reload"
          title="Saved application configuration"
          Icon={RotateCwIcon}
          description="Apply saved settings to this server. Unsaved drafts and active sessions are retained. Individual workers may drain or wait for their next start."
        >
          <dl class="store-settings__rows">
            {#each manager.value.supported as domain}
              {@const result = manager.outcome?.results.find(
                (result) => result.domain === domain,
              )}
              <StoreReadOnlyRow
                label={names[domain] ?? domain}
                value={result
                  ? `${states[result.state]}${result.reason ? ` · ${result.reason}` : ""}`
                  : manager.value.pending.includes(domain)
                    ? "Saved changes awaiting reload"
                    : "Current"}
              />
            {/each}
          </dl>
          <StoreSettingsFooter>
            {#snippet description()}Reload keeps the server online. Activation
              never installs runtimes or replays prompts.{/snippet}
            <Button
              variant="outline"
              disabled={manager.busy}
              onclick={() => (manager!.open = false)}>Close</Button
            >
            {#if manager.pending}<Button
                disabled={manager.busy}
                onclick={() => void manager!.check()}
                >{manager.busy
                  ? "Reloading…"
                  : "Check original outcome"}</Button
              >
            {:else}<Button
                disabled={manager.busy}
                onclick={() => void manager!.run()}
                >{manager.busy ? "Reloading…" : "Reload saved settings"}</Button
              >{/if}
          </StoreSettingsFooter>
        </StoreSettingsCard>
      </div>
    </DialogFrame>
  </Dialog.Root>
{/if}
