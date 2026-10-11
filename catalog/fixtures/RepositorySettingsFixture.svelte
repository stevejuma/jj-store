<script lang="ts">
  import { untrack, onDestroy } from "svelte";
  import type {
    StoreController,
    StoreRoute,
  } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositorySettingsView from "../../web/src/lib/ui/store-settings/StoreRepositorySettingsView.svelte";
  import { createRepositorySettingsNavigation } from "../../web/src/lib/controller/repository-settings-navigation.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createServerSettings } from "../../web/src/lib/controller/server-settings.svelte.js";
  import { serverSettingsValue } from "./server-settings.js";
  let {
    local = false,
    reader = false,
    section = "general",
    tab,
    narrow = false,
    loading = false,
    healthLoading = false,
  }: {
    narrow?: boolean;
    loading?: boolean;
    healthLoading?: boolean;
    local?: boolean;
    reader?: boolean;
    section?:
      "general" | "folders" | "search" | "storage" | "sync" | "advanced";
    tab?: "health" | "scopes";
  } = $props();
  const base = untrack(() => createInMemoryStoreController("files"));
  const capabilities = untrack(() => base.state.settings.capabilities);
  if (!capabilities || !("native_profile" in capabilities))
    throw new Error("Native Settings fixture required.");
  const repository = "ab".repeat(16);
  let route = $state<StoreRoute>(
    untrack(() => ({
      ...base.state.route,
      repository,
      page: "settings",
      bookmark: "",
      commitId: null,
      repositorySettingsSection: section,
      repositorySettingsTab: tab,
    })),
  );
  let discovering = $state(untrack(() => loading));
  let folderPath = $state("");
  const navigation = createRepositorySettingsNavigation({
    route: () => route,
    push: (next) => (route = next),
    replace: (next) => (route = next),
  });
  const configuration = createServerSettings({
    available: () => !reader,
    client: { close() {}, request: async () => serverSettingsValue() } as never,
  });
  onDestroy(() => configuration.destroy());
  const controller: StoreController = {
    configuration,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        accessMode: reader ? "authenticated" : "local_trusted",
        repository: {
          id: repository,
          name: local ? "Folder repository" : "Managed repository",
          profile: local ? "git_sha1_jj045" : "native",
          permission: reader ? "read" : "admin",
        },
        route,
        settings: {
          ...base.state.settings,
          workspacePath: folderPath,
          ...(healthLoading
            ? { searchStatus: null, searchStatusState: "loading" as const }
            : {}),
          capabilities: discovering
            ? null
            : {
                ...capabilities,
                search: !local,
                index_tasks: !local,
                local_workspace_links: !local && !reader,
                local_storage_management: !local && !reader,
                local_sync_management: !local && !reader,
              },
        },
      };
    },
    actions: {
      ...base.actions,
      openRepositorySettings: navigation.open,
      selectRepositorySettings: navigation.select,
      leaveRepositorySettings: navigation.leave,
      setWorkspacePath(path) {
        folderPath = path;
      },
    },
  };
</script>

<main style:width={narrow ? "390px" : "100%"} style:max-width="100%">
  {#if loading}<button onclick={() => (discovering = false)}
      >Finish discovery</button
    >{/if}
  <StoreRepositorySettingsView {controller} /><output
    aria-label="Settings route"
    >{route.page}:{route.repositorySettingsSection}:{route.repositorySettingsTab ??
      ""}</output
  >
</main>
