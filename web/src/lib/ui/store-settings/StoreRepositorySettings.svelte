<script lang="ts">
  import BoxIcon from "@lucide/svelte/icons/box";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreRepositoryIdentity from "./StoreRepositoryIdentity.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreRepositoryConversion from "./StoreRepositoryConversion.svelte";
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { emptyRoute } from "../../controller/global-search-presentation.js";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreWorkspaceSettings from "./StoreWorkspaceSettings.svelte";
  import StoreGraphIndexSettings from "./StoreGraphIndexSettings.svelte";
  let {
    controller,
    onClose,
    section = "general",
    visible = true,
  }: {
    controller: StoreController;
    onClose?: () => void;
    section?: "general" | "folders" | "advanced";
    visible?: boolean;
  } = $props();
  let navigationError = $state("");
  const navigationBlocked = $derived(
    settingsNavigationReason(controller.state.settings),
  );
  async function open(operation?: string) {
    if (navigationBlocked || !controller.actions.openRoute) return;
    navigationError = "";
    try {
      await controller.actions.openRoute({
        ...emptyRoute(view.repository.id),
        page: operation ? "oplog" : "files",
        snapshotOperationId: operation ?? view.route.snapshotOperationId,
        inspectedOperationId: operation ?? null,
      });
      onClose?.();
    } catch (error) {
      navigationError = error instanceof Error ? error.message : String(error);
    }
  }

  const view = $derived(controller.state);
  const settings = $derived(view.settings);
  const capabilities = $derived(settings.capabilities);
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
  const words = (values: readonly string[]) =>
    values.length
      ? values.map((value) => value.replaceAll("_", " ")).join(", ")
      : "None";
  const featureList = (
    value: import("../../controller/repository-profile.js").RepositoryCapabilities,
  ) =>
    [
      [value.read_api, "Read API"],
      [value.browsing_windows, "Windowed browsing"],
      [value.events, "Live events"],
      [value.search, "Search"],
      [value.index_tasks, "Index tasks"],
    ]
      .filter(([enabled]) => enabled)
      .map(([, label]) => label)
      .join(", ");
</script>

<StoreSettingsPage
  embedded
  section={section === "general" ? "repository" : section}
  onRefresh={section === "folders" &&
  view.repository.profile !== "git_sha1_jj045" &&
  capabilities?.local_workspace_links
    ? controller.actions.refreshWorkspaceLinks
    : undefined}
  refreshLabel="Refresh folders"
  refreshDisabled={settings.workspaceLinking ||
    settings.workspacePicking ||
    ["loading", "refreshing"].includes(settings.workspaceLinksState)}
>
  {#if navigationError}<Alert.Root variant="destructive"
      ><Alert.Description>{navigationError}</Alert.Description></Alert.Root
    >{/if}
  {#if section === "general"}
    <StoreRepositoryIdentity
      {controller}
      onOpen={open}
      navigationBlocked={!!navigationBlocked}
    />
    <StoreRepositoryConversion {controller} {onClose} />
  {:else if section === "folders"}
    {#if view.repository.profile === "git_sha1_jj045"}<StoreLocalFolders
        {controller}
        repository={view.repository.id}
        {visible}
      />{:else}<StoreWorkspaceSettings
        {settings}
        supported={Boolean(settings.capabilities?.local_workspace_links)}
        onPath={controller.actions.setWorkspacePath}
        onLink={controller.actions.linkWorkspace}
        pickerSupported={Boolean(settings.capabilities?.local_folder_picker)}
        onBrowse={controller.actions.pickWorkspaceFolder}
      />{/if}
  {:else if section === "advanced"}
    {#if capabilities?.index_tasks && view.repository.permission === "admin"}<StoreGraphIndexSettings
        {controller}
      />{/if}

    {#if capabilities}
      <StoreSettingsCard
        id="repository-capabilities"
        title="Capabilities"
        description="Supported features and operational characteristics."
        Icon={BoxIcon}
      >
        <dl class="store-settings__rows">
          <StoreReadOnlyRow
            label="Native profile"
            value={"native_profile" in capabilities
              ? capabilities.native_profile
              : capabilities.profile}
          />
          <StoreReadOnlyRow
            label="Authentication"
            value={"authentication" in capabilities
              ? capabilities.authentication.replaceAll("_", " ")
              : "Local trusted"}
          />
          <StoreReadOnlyRow
            label="Supported APIs"
            value={featureList(capabilities)}
          />
          <StoreReadOnlyRow
            label="Typed mutations"
            value={words(capabilities.typed_mutations)}
          />
          <StoreReadOnlyRow
            label="Conflict actions"
            value={words(capabilities.conflict_resolution_actions)}
          />
          {#if "mutation_request_bytes" in capabilities}<StoreReadOnlyRow
              label="Mutation request limit"
              value={`${number(capabilities.mutation_request_bytes)} bytes`}
            />{/if}
          <StoreReadOnlyRow
            label="Conflict upload limit"
            value={`${number(capabilities.conflict_upload_bytes)} bytes`}
          />
        </dl>
      </StoreSettingsCard>
      {#if "limits" in capabilities}<StoreSettingsCard
          id="repository-limits"
          title="Read and event limits"
          description="Limits for read operations, event streams and related resources."
          Icon={BoxIcon}
        >
          <dl class="store-settings__rows">
            <StoreReadOnlyRow
              label="Requests / workers"
              value={`${number(capabilities.limits.requests)} / ${number(capabilities.limits.workers)}`}
            />
            <StoreReadOnlyRow
              label="Deadline"
              value={`${number(capabilities.limits.deadline_millis)} ms`}
            />
            <StoreReadOnlyRow
              label="Page / index commits"
              value={`${number(capabilities.limits.page)} / ${number(capabilities.limits.index_commits)}`}
            />
            <StoreReadOnlyRow
              label="JSON / file limits"
              value={`${number(capabilities.limits.json_bytes)} / ${number(capabilities.limits.file_bytes)} bytes`}
            />
            <StoreReadOnlyRow
              label="Event streams / replay"
              value={`${number(capabilities.event_limits.streams)} / ${number(capabilities.event_limits.replay_window)}`}
            />
            <StoreReadOnlyRow
              label="Event page / polling"
              value={`${number(capabilities.event_limits.page)} / ${number(capabilities.event_limits.poll_millis)} ms`}
            />
          </dl>
        </StoreSettingsCard>{/if}
    {/if}
  {/if}
</StoreSettingsPage>
