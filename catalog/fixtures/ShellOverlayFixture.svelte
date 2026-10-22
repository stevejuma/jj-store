<script lang="ts">
  import { onDestroy } from "svelte";
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import StoreDiscussionSheet from "../../web/src/lib/ui/store-discussion/StoreDiscussionSheet.svelte";
  let { mobile = false }: { mobile?: boolean } = $props();
  const shell = new AppShellController();
  onDestroy(() => void shell.dispose());
  let mainElement = $state<HTMLElement | null>(null);
  let mobileTarget = $state<HTMLElement | null>(null);
  let open = $state(false);
  let draft = $state("");
</script>

<CommunityAppShell
  controller={shell}
  displayMode={mobile ? "mobile" : "desktop"}
  {mainElement}
  {mobileTarget}
>
  <AppShell.Main bind:ref={mainElement}>
    <AppShell.Topbar.Target bind:ref={mobileTarget} />
    <AppShell.Body>
      <Button onclick={() => (open = true)}>Open overlay page</Button>
    </AppShell.Body>
  </AppShell.Main>
  <StoreDiscussionSheet
    {open}
    title="Overlay page"
    scrollable={false}
    onClose={() => (open = false)}
  >
    <div class="overlay-content" role="region" aria-label="Page content">
      <Input aria-label="Overlay draft" bind:value={draft} />
      {#each Array.from({ length: 60 }, (_, i) => i) as row}
        <p>Page row {row + 1}</p>
      {/each}
    </div>
    {#snippet footer()}
      <Button onclick={() => (open = false)}>Keep draft and close</Button>
    {/snippet}
  </StoreDiscussionSheet>
</CommunityAppShell>

<style>
  .overlay-content {
    flex: 1;
    min-height: 0;
    overflow: auto;
    padding: 1rem;
  }
</style>
