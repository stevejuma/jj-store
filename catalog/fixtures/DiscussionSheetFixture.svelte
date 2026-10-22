<script lang="ts">
  import { onDestroy } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import StoreDiscussionWorkspace from "../../web/src/lib/ui/store-discussion/StoreDiscussionWorkspace.svelte";
  import StoreFocusedDiscussion from "../../web/src/lib/ui/store-discussion/StoreFocusedDiscussion.svelte";
  import { provideDiscussionFocus } from "../../web/src/lib/ui/store-discussion/discussion-focus.svelte.js";
  import { repositoryDetailsPanelId } from "../../web/src/lib/ui/layout-storage.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let { mobile = false }: { mobile?: boolean } = $props();
  const appearance = ownAppearance(
    createAppearanceStore(createMemoryStoryStorage()),
  );
  const shell = createStoreShell(undefined, true);
  shell.right.open();
  const rail = shell.getPanel(repositoryDetailsPanelId)!;
  rail.open();
  onDestroy(() => void shell.dispose());
  provideDiscussionFocus();
  let mainElement = $state<HTMLElement | null>(null);
  let mobileTarget = $state<HTMLElement | null>(null);
  const first = $state({ open: false, value: "" });
  const second = $state({ open: false, value: "" });
</script>

<CommunityAppShell
  controller={shell}
  {mainElement}
  {mobileTarget}
  displayMode={mobile ? "mobile" : "desktop"}
>
  <AppShell.Main bind:ref={mainElement}>
    <AppShell.Topbar.Target bind:ref={mobileTarget} />
    <AppShell.Toolbar>
      <AppShell.Sidebar.Toggle side="right" sidebarName="standard pane" />
      <Button onclick={() => appearance.update({ threads: "focus" })}
        >Focus layout</Button
      >
      <Button onclick={() => appearance.update({ threads: "split" })}
        >Split layout</Button
      >
    </AppShell.Toolbar>
    <AppShell.Body>
      {#each [first, second] as draft, index (draft)}
        <StoreDiscussionWorkspace
          open={draft.open}
          focusKey={draft}
          title={`Discussion ${index + 1}`}
          onClose={() => (draft.open = false)}
        >
          {#snippet context()}
            <Button onclick={() => (draft.open = true)}
              >Open discussion {index + 1}</Button
            >
          {/snippet}
          {#snippet discussion()}
            <Input aria-label={`Draft ${index + 1}`} bind:value={draft.value} />
          {/snippet}
        </StoreDiscussionWorkspace>
      {/each}
    </AppShell.Body>
  </AppShell.Main>
  <AppShell.Sidebar
    side="right"
    variant="attached"
    label="Standard right pane"
    resizeLabel="Resize standard pane"
    closeable
  >
    <AppShell.Sidebar.Header>
      <strong>Discussion</strong>
      <AppShell.Sidebar.Close label="Close standard pane" />
    </AppShell.Sidebar.Header>
    <AppShell.Sidebar.Body>Standard right pane content</AppShell.Sidebar.Body>
  </AppShell.Sidebar>
  <AppShell.Sidebar
    side="right"
    sidebarController={rail}
    variant="rail"
    label="Outer details rail"
    resizeLabel="Resize outer details rail"
  >
    <AppShell.Sidebar.Body>Outer rail content</AppShell.Sidebar.Body>
  </AppShell.Sidebar>
  <StoreFocusedDiscussion />
</CommunityAppShell>
