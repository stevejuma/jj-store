<script lang="ts">
  import "../../web/src/lib/ui/store-application/store-application.css";
  import { untrack } from "svelte";
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import StorePageSkeleton from "../../web/src/lib/ui/store-application/StorePageSkeleton.svelte";
  import RepositorySkeleton from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import RepositoryFileLoading from "../../web/src/lib/ui/community-projects/RepositoryFileLoading.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    path = "README.md",
    mode = "detail",
    width,
  }: {
    path?: string;
    mode?: "detail" | "initial" | "module";
    width?: number;
  } = $props();
  const shell = new AppShellController();
  const controller = untrack(() => createInMemoryStoreController("files"));
  const storage = createMemoryStoryStorage();
</script>

<div style:width={width ? `${width}px` : "100%"} style:max-width="100%">
  <AppShell.Root
    controller={shell}
    displayMode="desktop"
    desktopMinMainWidth={0}
    style="--ui-shell-height: 100vh; --ui-shell-main-block-inset: 0px"
  >
    <AppShell.Main
      ><AppShell.Body layout="regions" label="Repository files workspace">
        {#if mode === "module"}<StorePageSkeleton
            route={{ ...controller.state.route, path }}
            label="repository files"
            layoutStorage={storage}
          />
        {:else if mode === "initial"}<RepositorySkeleton
            layout="source"
            {path}
            label="Loading repository files"
          />
        {:else}<div class="community-projects__file-panel-stack">
            <RepositoryFileLoading {path} isMarkdown={/\.md$/i.test(path)} />
          </div>{/if}
      </AppShell.Body></AppShell.Main
    >
  </AppShell.Root>
</div>
