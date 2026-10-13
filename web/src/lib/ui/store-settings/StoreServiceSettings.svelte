<script lang="ts">
  import StoreWorkflowHostSettings from "./StoreWorkflowHostSettings.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/server";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreConfigurationEditor from "./StoreConfigurationEditor.svelte";
  import StoreDeploymentConfiguration from "./StoreDeploymentConfiguration.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    active = $bindable("overview"),
  }: { controller: StoreController; active?: string } = $props();
  $effect(() => {
    if (
      (active === "protection" ||
        active === "storage" ||
        active === "workflows") &&
      !controller.configuration?.available() &&
      controller.state.accessMode !== "bootstrapping"
    ) {
      controller.actions.selectGlobalSettings("service", "overview", true);
    }
  });
  const view = $derived(controller.state);
</script>

<StoreSettingsPage
  section="service"
  bind:value={active}
  tabs={[
    { value: "overview", label: "Overview" },
    { value: "limits", label: "Limits" },
    { value: "launch", label: "Launch" },
    ...(controller.configuration?.available()
      ? [
          { value: "workflows", label: "Workflows" },
          { value: "protection", label: "Protection" },
          { value: "storage", label: "Storage" },
        ]
      : []),
  ]}
  onRefresh={controller.configuration?.observe}
>
  <Tabs.Content value="overview">
    <StoreSettingsCard
      id="service-status"
      title="Service status"
      Icon={SectionIcon}
    >
      <dl class="store-settings__rows">
        <StoreReadOnlyRow
          label="Access mode"
          value={view.accessMode === "local_trusted"
            ? "Local trusted"
            : "Repository token"}
        />
        <StoreReadOnlyRow
          label="Authoritative storage"
          tone={view.storageReady ? "ready" : "error"}
          value={view.storageReady ? "Ready" : "Unavailable"}
        />
        <StoreReadOnlyRow
          label="HTTP connection"
          value={view.status.connection}
        />
        <StoreReadOnlyRow label="Live events" value={view.status.live} />
        <StoreReadOnlyRow
          label="Live updates"
          value={view.status.refreshAvailable ? "Updating…" : "Current"}
        />
      </dl>
    </StoreSettingsCard>

    {#if view.accessMode === "local_trusted"}<StoreLocalFolders
        {controller}
      />{/if}
  </Tabs.Content><Tabs.Content value="limits"
    ><StoreConfigurationEditor
      management={controller.configuration}
      section="service"
    /></Tabs.Content
  >
  <Tabs.Content value="workflows"
    ><StoreWorkflowHostSettings
      {controller}
      management={controller.configuration}
      reload={controller.reload}
    /></Tabs.Content
  >
  <Tabs.Content value="launch"
    ><StoreDeploymentConfiguration
      management={controller.configuration}
      section="service"
    /></Tabs.Content
  >
  <Tabs.Content value="protection"
    ><StoreConfigurationEditor
      management={controller.configuration}
      section="protection"
    /></Tabs.Content
  >
  <Tabs.Content value="storage"
    ><StoreDeploymentConfiguration
      management={controller.configuration}
      section="storage"
    /></Tabs.Content
  >
</StoreSettingsPage>
