<script lang="ts">
  import StoreRepositoryIdentity from "./StoreRepositoryIdentity.svelte";
  import StoreSettingsRepository from "./StoreSettingsRepository.svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreRepositoryConversion from "./StoreRepositoryConversion.svelte";
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { emptyRoute } from "../../controller/global-search-presentation.js";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreConfigurationEditor from "./StoreConfigurationEditor.svelte";
  import StoreWorkspaceSettings from "./StoreWorkspaceSettings.svelte";
  import StoreGraphIndexSettings from "./StoreGraphIndexSettings.svelte";
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose?: () => void } = $props();
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
  let active = $state("general");
  const view = $derived(controller.state);
  const settings = $derived(view.settings);
  const capabilities = $derived(settings.capabilities);
  const hasRepository = $derived(
    !view.route.projectsHub &&
      !!view.route.repository &&
      view.route.repository === view.repository.id,
  );
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
  section="repository"
  bind:value={active}
  onRefresh={!hasRepository
    ? undefined
    : active === "protection"
      ? controller.configuration?.observe
      : active === "folders" &&
          view.repository.profile !== "git_sha1_jj045" &&
          capabilities?.local_workspace_links
        ? controller.actions.refreshWorkspaceLinks
        : undefined}
  refreshLabel="Refresh folders"
  refreshDisabled={active === "protection"
    ? (controller.configuration?.state.saving ?? false)
    : settings.workspaceLinking ||
      settings.workspacePicking ||
      ["loading", "refreshing"].includes(settings.workspaceLinksState)}
  tabs={hasRepository
    ? [
        { value: "general", label: "General" },
        { value: "folders", label: "Local folders" },
        { value: "advanced", label: "Advanced" },
        { value: "protection", label: "Protection" },
      ]
    : []}
>
  {#if !hasRepository}
    <Empty.Root>
      <Empty.Header>
        <Empty.Media variant="icon"
          ><FolderGitIcon aria-hidden="true" /></Empty.Media
        >
        <Empty.Title>Choose a repository</Empty.Title>
        <Empty.Description
          >Select a repository to view its identity and settings.</Empty.Description
        >
      </Empty.Header>
      <Empty.Content
        ><StoreSettingsRepository
          {controller}
          section="repository"
          placement="inline"
        /></Empty.Content
      >
    </Empty.Root>
  {:else}
    {#if navigationError}<Alert.Root variant="destructive"
        ><Alert.Description>{navigationError}</Alert.Description></Alert.Root
      >{/if}
    <Tabs.Content value="general">
      <StoreRepositoryIdentity
        {controller}
        onOpen={open}
        navigationBlocked={!!navigationBlocked}
      />
      <StoreRepositoryConversion {controller} {onClose} />
    </Tabs.Content>
    <Tabs.Content value="folders">
      {#if view.repository.profile === "git_sha1_jj045"}<StoreLocalFolders
          {controller}
          repository={view.repository.id}
        />{:else}<StoreWorkspaceSettings
          {settings}
          supported={Boolean(settings.capabilities?.local_workspace_links)}
          onPath={controller.actions.setWorkspacePath}
          onLink={controller.actions.linkWorkspace}
          pickerSupported={Boolean(settings.capabilities?.local_folder_picker)}
          onBrowse={controller.actions.pickWorkspaceFolder}
        />{/if}
    </Tabs.Content>
    <Tabs.Content value="advanced">
      <StoreGraphIndexSettings {controller} />

      {#if capabilities}
        <section
          class="store-settings__section"
          aria-labelledby="repository-capabilities"
        >
          <header><h2 id="repository-capabilities">Capabilities</h2></header>
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
        </section>
        {#if "limits" in capabilities}<section
            class="store-settings__section"
            aria-labelledby="repository-limits"
          >
            <header>
              <h2 id="repository-limits">Read and event limits</h2>
            </header>
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
          </section>{/if}
      {/if}
    </Tabs.Content>
    <Tabs.Content value="protection"
      ><StoreConfigurationEditor
        management={controller.configuration}
        section="protection"
      /></Tabs.Content
    >
  {/if}
</StoreSettingsPage>
