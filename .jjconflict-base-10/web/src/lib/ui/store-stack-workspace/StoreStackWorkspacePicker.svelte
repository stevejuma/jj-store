<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let { controller }: { controller: StackWorkspaceController } = $props();
  const snapshot = $derived(controller.snapshot);
</script>

{#if snapshot}
  <div
    class="stack-workspace-picker"
    role="group"
    aria-label="Stack workspace selection"
  >
    <StoreRepositoryPicker
      label="Active stack workspace"
      emptyLabel="No stack workspace"
      variant="ghost"
      value={snapshot.active ?? ""}
      repositories={snapshot.workspaces.map((item) => ({
        id: item.id,
        name: item.name,
        description:
          `${item.applied.length} applied stacks` +
          (item.id === snapshot.active
            ? ` · ${controller.filesFor("unassigned").length}${controller.details.fileState("unassigned").nextCursor ? "+" : ""} unstaged files`
            : item.wipCommit
              ? ""
              : ` · ${item.unassigned.length} unstaged files`),
      }))}
      searchLabel="Search workspaces"
      groupLabel="Workspaces"
      emptyResult="No matching workspaces."
      controlled
      scrollStrategy="styled"
      disabled={!controller.can("switch_workspace")}
      onChange={(workspace) => {
        if (controller.can("switch_workspace"))
          void controller.prepare({ kind: "switch_workspace", workspace });
      }}
    />
  </div>
{/if}

<style>
  .stack-workspace-picker {
    width: fit-content;
    max-width: min(11rem, 100%);
    min-width: 0;
    margin-inline: auto;
  }
</style>
