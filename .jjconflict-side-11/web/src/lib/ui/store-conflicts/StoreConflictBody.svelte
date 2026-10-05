<script lang="ts">
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import type { FileListingViewMode } from "@lapismd/design-core/diff";
  import type { Snippet } from "svelte";
  import CommunityProjectsRepositoryFileTree from "../community-projects/CommunityProjectsRepositoryFileTree.svelte";
  import "../community-projects/repository-files.css";

  let {
    files,
    activePath,
    browserOpen,
    compact,
    locked,
    onSelect,
    children,
  }: {
    files: readonly { path: string }[];
    activePath: string;
    browserOpen: boolean;
    compact: boolean;
    locked: boolean;
    onSelect: (path: string) => void;
    children: Snippet;
  } = $props();
  const shell = new AppShellController();
  let mode = $state<FileListingViewMode>("tree");
  let collapsed = $state(new Set<string>());
  function toggleDirectory(path: string) {
    const next = new Set(collapsed);
    if (!next.delete(path)) next.add(path);
    collapsed = next;
  }
</script>

<AppShell.Root
  controller={shell}
  displayMode="desktop"
  desktopMinMainWidth={0}
  class="store-conflict-editor__body-shell"
>
  <AppShell.Main>
    <AppShell.Body
      layout="regions"
      element="section"
      label="Conflict editor body"
    >
      <AppShell.Body.Sidebar
        id="conflict-file-browser"
        panelId="conflict-file-browser"
        side="left"
        label="Conflict file browser"
        open={browserOpen}
        scrollbarVisibility="hover"
        class="community-projects__file-tree-region"
        style={`--ui-shell-body-sidebar-width: ${compact ? "100%" : "15rem"}`}
      >
        <div
          class="store-conflict-editor__file-browser"
          inert={locked}
          aria-disabled={locked}
        >
          <CommunityProjectsRepositoryFileTree
            {files}
            {mode}
            collapsedDirectoryPaths={collapsed}
            selectedPath={activePath}
            onSelectPath={(path) => {
              if (!locked) onSelect(path);
            }}
            onModeChange={(next) => (mode = next)}
            onDirectoryToggle={toggleDirectory}
          />
        </div>
      </AppShell.Body.Sidebar>
      <AppShell.Body.Content
        label="Conflict merge content"
        scrollbarVisibility="hover"
        class="store-conflict-editor__content"
        hidden={compact && browserOpen}
      >
        <div class="store-conflict-editor__content-body">
          {@render children()}
        </div>
      </AppShell.Body.Content>
    </AppShell.Body>
  </AppShell.Main>
</AppShell.Root>
