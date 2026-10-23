<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { untrack } from "svelte";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreWorkspaceSettings from "../../web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte";
  import { emptyWorkspaceLinks } from "../../web/src/lib/controller/workspace-link-types.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    supported = true,
    embedded = false,
    loading = false,
    pickerSupported = true,
    selection = "selected",
  }: {
    supported?: boolean;
    embedded?: boolean;
    loading?: boolean;
    pickerSupported?: boolean;
    selection?: "selected" | "cancelled" | "failed";
  } = $props();
  const settings = $state(emptyWorkspaceLinks());
  settings.workspaceLinks = {
    version: 1,
    revision: 1,
    links: [
      {
        id: "01",
        path: "/workspaces/moved-project",
        profile: "native",
        availability: "unavailable",
      },
    ],
  };
  settings.workspaceLinksState = "ready";
  const loaded = untrack(() =>
    structuredClone($state.snapshot(settings.workspaceLinks)),
  );
  if (untrack(() => loading)) {
    settings.workspaceLinks = null;
    settings.workspaceLinksState = "loading";
  }
  async function link() {
    settings.workspaceLinking = true;
    settings.workspaceLinksError = "";
    settings.workspaceValidation = "";
    await Promise.resolve();
    if (settings.workspacePath === "/wrong") {
      settings.workspaceValidation =
        "Invalid workspace binding. Check the folder path.";
    } else {
      settings.workspaceLinks!.links.push({
        id: settings.workspacePath,
        path: settings.workspacePath,
        profile: "native",
        availability: "ready",
      });
      settings.workspacePath = "";
    }
    settings.workspaceLinking = false;
  }
</script>

<main>
  {#if loading}
    <FixtureControls floating={false}
      ><button
        onclick={() => {
          settings.workspaceLinks = loaded;
          settings.workspaceLinksState = "ready";
        }}>Finish loading</button
      ></FixtureControls
    >
    <FixtureControls floating={false}
      ><button onclick={() => (settings.workspaceLinksState = "refreshing")}
        >Start background refresh</button
      ></FixtureControls
    >
  {/if}
  <StoreSettingsPage
    {embedded}
    section="repository"
    refreshLabel="Refresh folders"
    onRefresh={supported ? async () => {} : undefined}
  >
    <StoreWorkspaceSettings
      {settings}
      {supported}
      {pickerSupported}
      onBrowse={async () => {
        settings.workspacePicking = true;
        await Promise.resolve();
        if (selection === "selected")
          settings.workspacePath = "/workspaces/picked folder";
        if (selection === "failed")
          settings.workspacePickerError =
            "Folder picker unavailable. Enter the folder path manually.";
        settings.workspacePicking = false;
      }}
      onPath={(path) => {
        settings.workspacePath = path;
        settings.workspaceValidation = "";
        if (!path) settings.workspaceLinksError = "";
      }}
      onLink={link}
    />
  </StoreSettingsPage>
</main>
