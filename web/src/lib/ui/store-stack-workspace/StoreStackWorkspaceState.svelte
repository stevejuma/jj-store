<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    loading = false,
    onRetry,
  }: {
    controller?: StackWorkspaceController;
    loading?: boolean;
    onRetry?: () => void | Promise<void>;
  } = $props();
  const pending = $derived(
    loading ||
      Boolean(
        controller &&
        !controller.snapshotLoaded &&
        (controller.loading || !controller.error),
      ),
  );
  const unavailable = $derived(Boolean(controller?.snapshotLoaded));
  const retry = $derived(
    onRetry ?? (controller ? () => controller.refresh() : undefined),
  );
</script>

{#if pending}
  <CommunityProjectsRepositorySkeleton
    layout="graph"
    label="Loading stack workspace"
    count={5}
  />
{:else}
  <Empty.Root>
    <Empty.Header>
      <Empty.Media variant="icon"
        ><FolderKanban aria-hidden="true" /></Empty.Media
      >
      <Empty.Title
        >{unavailable
          ? "Local checkout unavailable"
          : controller
            ? "Workspace could not be loaded"
            : "Local stack workspace unavailable"}</Empty.Title
      >
      {#if unavailable || !controller}<Empty.Description
          >{controller?.snapshot?.availability === "unlinked"
            ? "Link a local JJ checkout to use stack workspaces."
            : "Stacks require an available linked local JJ checkout."}</Empty.Description
        >{/if}
    </Empty.Header>
    {#if retry}<Empty.Content
        ><Button variant="outline" size="sm" onclick={() => retry?.()}
          >Retry workspace</Button
        ></Empty.Content
      >{/if}
  </Empty.Root>
{/if}
