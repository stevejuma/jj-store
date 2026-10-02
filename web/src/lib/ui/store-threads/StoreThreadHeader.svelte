<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Hash from "@lucide/svelte/icons/hash";
  import Users from "@lucide/svelte/icons/users";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import StoreToolbarSearch from "../StoreToolbarSearch.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import { threadMembers } from "./thread-view.js";
  let {
    controller,
    query = $bindable(""),
  }: { controller: StoreController; query?: string } = $props();
  let searchOpen = $state(false);
  const threads = $derived(controller.threads);
  const panel = useThreadPanel();
  const members = $derived(threadMembers(controller));
</script>

<AppShell.Toolbar
  label="Thread toolbar"
  class="community-application__room-toolbar"
>
  <div class="community-application__toolbar-start">
    {#if threads?.detail}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="All threads"
        onclick={() => {
          panel.close();
          void threads?.open(null);
        }}><ArrowLeft aria-hidden="true" /></Button
      >{/if}
    {#if !threads?.detail}<StoreToolbarSearch
        label="Search threads"
        scope={controller.state.route.repository}
        {query}
        bind:open={searchOpen}
        onQuery={(value) => {
          query = value;
        }}
      />{/if}
    {#if threads?.detail || !searchOpen}<div
        class="community-application__room-title"
      >
        <h1 data-ui-part="room-title">
          <Hash aria-hidden="true" />{threads?.detail?.thread.value.title ??
            "Threads"}
        </h1>
      </div>{/if}
  </div>
  <div class="community-application__toolbar-actions">
    {#if threads?.detail}<Button
        variant="outline"
        size="sm"
        aria-label={`View thread contributors (${members.length})`}
        onclick={() => panel.open("members")}
        ><Users
          data-icon="inline-start"
          aria-hidden="true"
        />{members.length}</Button
      >
    {/if}
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Refresh threads"
      disabled={threads?.busy}
      onclick={() => threads?.refresh()}><Refresh aria-hidden="true" /></Button
    >
    {#if threads?.detail}<Button
        variant="ghost"
        size="icon-sm"
        aria-label="Thread settings"
        onclick={() => panel.open("settings")}
        ><Ellipsis aria-hidden="true" /></Button
      >{/if}
  </div>
</AppShell.Toolbar>

<style>
  :global(
    .community-app-shell__surface
      [data-ui-part="toolbar"].community-application__room-toolbar
  ) {
    border-block-end: 0;
    flex: 0 0 auto;
    justify-content: space-between;
  }
  .community-application__toolbar-start,
  .community-application__toolbar-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }
  .community-application__room-title {
    min-width: 0;
  }
  .community-application__toolbar-start {
    flex: 1;
  }
  [data-ui-part="room-title"] {
    margin: 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: var(--community-text-body);
    font-weight: 700;
  }
  [data-ui-part="room-title"] :global(svg) {
    width: 1rem;
    height: 1rem;
  }
</style>
