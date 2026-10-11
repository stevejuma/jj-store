<script lang="ts">
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
  import StoreStackWorkspacePicker from "../store-stack-workspace/StoreStackWorkspacePicker.svelte";
  import StoreProjectsBreadcrumb from "../store-work/StoreProjectsBreadcrumb.svelte";
  import StoreThreadsBreadcrumb from "../store-threads/StoreThreadsBreadcrumb.svelte";
  let {
    controller,
    stackWorkspace,
    current,
    editorOpen,
    hidden,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    current: StoreRoute;
    editorOpen: boolean;
    hidden: boolean;
  } = $props();
</script>

{#if !hidden && isStacksRoute(current) && stackWorkspace?.snapshot?.repository === current.repository}<StoreStackWorkspacePicker
    controller={stackWorkspace}
    disabled={editorOpen}
  />{:else if !hidden && current.page === "projects" && controller.projects && !isRepositoryProjects(current)}<StoreProjectsBreadcrumb
    {controller}
  />{:else if !hidden && current.page === "threads"}<StoreThreadsBreadcrumb
    {controller}
  />{/if}
