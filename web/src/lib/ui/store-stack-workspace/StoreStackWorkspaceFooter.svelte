<script lang="ts">
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { selectedWorkspace } from "../../controller/stack-workspace/presentation.js";
  import StoreStackFolderCopy from "./StoreStackFolderCopy.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import File from "@lucide/svelte/icons/file";
  import Pencil from "@lucide/svelte/icons/pencil";
  let {
    controller,
    retainedDraft,
    onEdit,
  }: {
    controller: StackWorkspaceController;
    retainedDraft: boolean;
    onEdit(): void;
  } = $props();
  const snapshot = $derived(controller.snapshot);
  const workspace = $derived(
    snapshot ? selectedWorkspace(snapshot) : undefined,
  );
</script>

{#if snapshot}
  <footer class="stack-workspace__footer">
    <div class="stack-workspace__footer-items">
      {#if snapshot.folder}<StoreStackFolderCopy path={snapshot.folder} />{/if}
      <CommunityChip variant="muted" shape="square">
        {#snippet leading()}<FolderKanban
            size={14}
            aria-hidden="true"
          />{/snippet}
        {workspace?.name ?? "No stack workspace"}
      </CommunityChip>
      <CommunityChip variant="muted" shape="square">
        {#snippet leading()}<GitBranch size={14} aria-hidden="true" />{/snippet}
        {snapshot.base ?? "Choose target branch"}
      </CommunityChip>
      <CommunityChip variant="muted" shape="square">
        {#snippet leading()}<GitBranchPlus
            size={14}
            aria-hidden="true"
          />{/snippet}
        {workspace?.applied.length ?? 0} applied stacks
      </CommunityChip>
      <CommunityChip variant="muted" shape="square">
        {#snippet leading()}<File size={14} aria-hidden="true" />{/snippet}
        {controller.filesFor("unassigned").length}{controller.details.fileState(
          "unassigned",
        ).nextCursor
          ? "+"
          : ""} unstaged files
      </CommunityChip>
    </div>
    <span role="status">{controller.outcome}</span
    >{#if retainedDraft && !controller.locked}<CommunityChip
        variant="muted"
        shape="square"
        onclick={onEdit}
        >{#snippet leading()}<Pencil aria-hidden="true" />{/snippet}Edit
        retained draft</CommunityChip
      >{/if}
  </footer>
{/if}
