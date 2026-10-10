<script lang="ts">
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  let {
    controller,
    stackWorkspace,
    current,
    settingsOpen,
    conflictOpen = $bindable(false),
    searchOpen = $bindable(false),
    conflictReturnFocus,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    current: StoreRoute;
    settingsOpen: boolean;
    conflictOpen?: boolean;
    searchOpen?: boolean;
    conflictReturnFocus: HTMLElement | null;
  } = $props();
</script>

<StoreLazyView
  load={views.conflicts}
  label="Conflict editor"
  active={conflictOpen}
  >{#snippet content(View)}<View
      {controller}
      open={conflictOpen}
      returnFocus={conflictReturnFocus}
      onClose={() => (conflictOpen = false)}
    />{/snippet}</StoreLazyView
>
{#if stackWorkspace && isStacksRoute(current)}
  <StoreLazyView
    load={views.stackConflicts}
    label="Workspace conflicts"
    active={Boolean(stackWorkspace.conflictOpen)}
    >{#snippet content(View)}<View
        controller={stackWorkspace}
        bind:open={stackWorkspace.conflictOpen}
      />{/snippet}</StoreLazyView
  >
{/if}
{#if !settingsOpen}<StoreLazyView
    load={views.search}
    label="Search"
    active={searchOpen}
    >{#snippet content(View)}<View
        {controller}
        bind:open={searchOpen}
      />{/snippet}</StoreLazyView
  >{/if}
