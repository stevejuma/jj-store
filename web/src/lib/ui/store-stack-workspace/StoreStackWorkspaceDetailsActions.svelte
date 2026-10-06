<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FileWarning from "@lucide/svelte/icons/file-warning";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import Settings from "@lucide/svelte/icons/settings";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { selectedWorkspace } from "../../controller/stack-workspace/presentation.js";
  let { workspace }: { workspace: StackWorkspaceController } = $props();
  const snapshot = $derived(workspace.snapshot);
  const available = $derived(
    snapshot?.availability === "ready" && Boolean(snapshot.folder),
  );
  const resolve = $derived(
    Boolean(snapshot?.conflicts.length) && workspace.can("resolve_conflict"),
  );
  const settings = $derived(
    available &&
      Boolean(snapshot && selectedWorkspace(snapshot)) &&
      !workspace.locked,
  );
  const reconcile = $derived(
    Boolean(snapshot?.membershipChanged) &&
      workspace.can("reconcile_workspace"),
  );
  const refresh = $derived(Boolean(snapshot?.folder) && !workspace.locked);
</script>

{#if resolve || settings || reconcile || refresh}
  <section
    class="stack-workspace-details-actions"
    aria-label="Workspace actions"
  >
    <p>Actions</p>
    <div role="group" aria-label="Available workspace actions">
      {#if resolve}<Button
          variant="ghost"
          class="stack-workspace-details-actions__item"
          onclick={() => workspace.openConflicts()}
        >
          <FileWarning data-icon="inline-start" aria-hidden="true" /><span
            >Resolve conflicts</span
          >
        </Button>{/if}
      {#if settings}<Button
          variant="ghost"
          class="stack-workspace-details-actions__item"
          onclick={() => (workspace.settingsOpen = true)}
        >
          <Settings data-icon="inline-start" aria-hidden="true" /><span
            >Workspace settings</span
          >
        </Button>{/if}
      {#if reconcile}
        <Button
          variant="ghost"
          class="stack-workspace-details-actions__item"
          onclick={() =>
            void workspace.prepare({ kind: "reconcile_workspace" })}
        >
          <Refresh data-icon="inline-start" aria-hidden="true" /><span
            >Reapply saved branches</span
          >
        </Button>
      {/if}
      {#if refresh}<Button
          variant="ghost"
          class="stack-workspace-details-actions__item"
          onclick={() => void workspace.refresh()}
        >
          <Refresh data-icon="inline-start" aria-hidden="true" /><span
            >Refresh workspace</span
          >
        </Button>{/if}
    </div>
  </section>
{/if}

<style>
  .stack-workspace-details-actions {
    padding: 0.85rem 0.75rem 0.35rem;
  }
  p {
    margin: 0.05rem 0.4rem 0.3rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  [role="group"] {
    display: grid;
    gap: 0.15rem;
  }
  :global(.stack-workspace-details-actions__item[data-ui-component="button"]) {
    display: grid;
    width: 100%;
    height: 1.75rem;
    grid-template-columns: 0.875rem minmax(0, 1fr);
    justify-content: stretch;
    gap: 0.75rem;
    padding-inline: 0.5rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
    font-weight: 400;
    line-height: 1.25rem;
    text-align: start;
  }
  :global(.stack-workspace-details-actions__item > svg) {
    width: 0.875rem;
    height: 0.875rem;
  }
</style>
