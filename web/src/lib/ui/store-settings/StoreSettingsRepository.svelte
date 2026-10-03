<script lang="ts">
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
  }: {
    controller: StoreController;
    section: StoreSettingsSection;
    disabled?: boolean;
  } = $props();
  let switching = $state(false),
    selection = $state(""),
    error = $state("");
  const view = $derived(controller.state);
  const settings = $derived(view.settings);
  const current = $derived(view.route.projectsHub ? "" : view.route.repository);
  const reason = $derived(
    disabled
      ? "Finish the current edit before changing repositories."
      : settings.searchSettings.dirty
        ? "Save or cancel your Search changes before changing repositories."
        : settings.workspacePath
          ? "Link or cancel your folder draft before changing repositories."
          : settings.storageSettings.pending
            ? "Resolve the pending Storage request before changing repositories."
            : settings.workspaceLinking ||
                settings.workspacePicking ||
                settings.searchSettings.changing ||
                settings.storageSettings.changing ||
                settings.syncSettings.changing ||
                settings.indexPreparation === "loading" ||
                ["capturing", "waiting"].includes(
                  settings.syncSettings.wait.state,
                )
              ? "Wait for the current Settings action to finish before changing repositories."
              : switching || view.status.connection === "connecting"
                ? "Opening repository…"
                : "",
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
      await controller.actions.refreshSettings();
      if (controller.state.status.connection === "failed")
        error = controller.state.status.error || "Could not open repository.";
    } catch (cause) {
      error = storeErrorMessage(cause);
    } finally {
      selection = current;
      switching = false;
    }
  }
</script>

{#if section !== "appearance" && section !== "service"}
  <div class="store-settings-repository">
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
                variant="ghost"
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
</style>
