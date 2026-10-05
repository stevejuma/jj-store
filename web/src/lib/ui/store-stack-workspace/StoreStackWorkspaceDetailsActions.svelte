<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FileWarning from "@lucide/svelte/icons/file-warning";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import Settings from "@lucide/svelte/icons/settings";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    workspace,
  }: { controller: StoreController; workspace: StackWorkspaceController } =
    $props();
  const snapshot = $derived(workspace.snapshot);
  const available = $derived(
    snapshot?.availability === "ready" && Boolean(snapshot.folder),
  );
</script>

<section class="stack-workspace-details-actions" aria-label="Workspace actions">
  <p>Actions</p>
  <div role="group" aria-label="Available workspace actions">
    <Button
      variant="ghost"
      class="lc-community-navigation__item"
      disabled={!snapshot?.conflicts.length ||
        !workspace.can("resolve_conflict")}
      title={workspace.eligibility("resolve_conflict").reason || undefined}
      onclick={() => workspace.openConflicts()}
    >
      <FileWarning data-icon="inline-start" aria-hidden="true" /><span
        >Resolve conflicts</span
      >
    </Button>
    <Button
      variant="ghost"
      class="lc-community-navigation__item"
      disabled={!available || !snapshot?.active || workspace.locked}
      onclick={() => (workspace.settingsOpen = true)}
    >
      <Settings data-icon="inline-start" aria-hidden="true" /><span
        >Workspace settings</span
      >
    </Button>
    <Button
      variant="ghost"
      class="lc-community-navigation__item"
      disabled={!available || workspace.locked}
      onclick={() => void controller.actions.selectStacksView("branches")}
    >
      <GitBranch data-icon="inline-start" aria-hidden="true" /><span
        >Browse branches</span
      >
    </Button>
    {#if snapshot?.membershipChanged}
      <Button
        variant="ghost"
        class="lc-community-navigation__item"
        disabled={!workspace.can("reconcile_workspace")}
        title={workspace.eligibility("reconcile_workspace").reason || undefined}
        onclick={() => void workspace.prepare({ kind: "reconcile_workspace" })}
      >
        <Refresh data-icon="inline-start" aria-hidden="true" /><span
          >Reapply saved branches</span
        >
      </Button>
    {/if}
    <Button
      variant="ghost"
      class="lc-community-navigation__item"
      disabled={workspace.locked}
      onclick={() => void workspace.refresh()}
    >
      <Refresh data-icon="inline-start" aria-hidden="true" /><span
        >Refresh workspace</span
      >
    </Button>
  </div>
</section>

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
</style>
