<script lang="ts">
  import { onDestroy } from "svelte";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import type { StoreSettingsSection } from "./model.js";
  let {
    controller,
    section,
    disabled = false,
    placement = "header",
  }: {
    controller: StoreController;
    section: StoreSettingsSection;
    disabled?: boolean;
    placement?: "header" | "inline";
  } = $props();
  let switching = $state(false),
    selection = $state(""),
    error = $state("");
  let disposed = false;
  onDestroy(() => {
    disposed = true;
  });
  const view = $derived(controller.state);
  const settings = $derived(view.settings);
  const current = $derived(view.route.projectsHub ? "" : view.route.repository);
  const hasRepository = $derived(!!current && current === view.repository.id);
  const reason = $derived(
    disabled
      ? "Finish the current edit before changing repositories."
      : settingsNavigationReason(settings) ||
          (switching || view.status.connection === "connecting"
            ? "Opening repository…"
            : ""),
  );
  $effect(() => {
    selection = current;
  });
  async function select(repository: string) {
    if (reason || repository === current) {
      selection = current;
      return;
    }
    switching = true;
    error = "";
    try {
      await controller.actions.openRepository(repository);
      if (disposed) return;
      if (controller.state.status.connection === "failed")
        error = controller.state.status.error || "Could not open repository.";
    } catch (cause) {
      if (!disposed) error = storeErrorMessage(cause);
    } finally {
      if (!disposed) {
        selection = current;
        switching = false;
      }
    }
  }
</script>

{#if section !== "appearance" && section !== "service" && section !== "agents" && (placement === "inline" || section !== "repository" || hasRepository)}
  <div
    class="store-settings-repository"
    class:store-settings-repository--inline={placement === "inline"}
  >
    <Tooltip.Provider>
      <Tooltip.Root disabled={!reason}>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <span
              {...props}
              class="store-settings-repository__control"
              aria-label={reason || undefined}
              role="group"
            >
              <StoreRepositoryPicker
                label="Settings repository"
                variant={placement === "inline" ? "outline" : "ghost"}
                repositories={view.repositories}
                bind:value={selection}
                disabled={!!reason}
                onChange={(repository) => void select(repository)}
              />
            </span>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>{reason}</Tooltip.Content>
      </Tooltip.Root>
    </Tooltip.Provider>
    <StoreNotice
      noticeKey="settings-repository"
      title="Could not open repository"
      message={error}
      pending={switching}
    />
  </div>
{/if}

<style>
  .store-settings-repository {
    width: fit-content;
    max-width: min(11rem, 100%);
    min-width: 0;
    margin-inline: auto;
  }
  .store-settings-repository__control {
    display: block;
  }
  .store-settings-repository--inline {
    max-width: min(20rem, 100%);
  }
</style>
