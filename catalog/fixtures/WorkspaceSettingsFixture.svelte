<script lang="ts">
  import StoreWorkspaceSettings from "../../web/src/lib/ui/store-settings/StoreWorkspaceSettings.svelte";
  import { emptyWorkspaceLinks } from "../../web/src/lib/controller/workspace-link-types.js";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    supported = true,
    pickerSupported = true,
    selection = "selected",
  }: {
    supported?: boolean;
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
  async function link() {
    settings.workspaceLinking = true;
    settings.workspaceLinksError = "";
    await Promise.resolve();
    if (settings.workspacePath === "/wrong") {
      settings.workspaceLinksError =
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

<main class="store-settings">
  <h1>Repository settings</h1>
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
    onRefresh={async () => {}}
    onPath={(path) => {
      settings.workspacePath = path;
      if (!path) settings.workspaceLinksError = "";
    }}
    onLink={link}
  />
</main>
