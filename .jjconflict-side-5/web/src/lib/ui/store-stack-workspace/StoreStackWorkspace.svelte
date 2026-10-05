<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Folder from "@lucide/svelte/icons/folder";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import File from "@lucide/svelte/icons/file";
  import Pencil from "@lucide/svelte/icons/pencil";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreStackWorkspaceCanvas from "./StoreStackWorkspaceCanvas.svelte";
  import StoreStackWorkspaceForm from "./StoreStackWorkspaceForm.svelte";
  import StoreStackWorkspacePreview from "./StoreStackWorkspacePreview.svelte";
  import StoreStackWorkspaceConflict from "./StoreStackWorkspaceConflict.svelte";
  import { activeWorkspace } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  import "../store-work/community-work-layout.css";
  import "../store-application/store-application.css";
  import StoreStackWorkspacePicker from "./StoreStackWorkspacePicker.svelte";
  import StoreStackWorkspaceActions from "./StoreStackWorkspaceActions.svelte";
  import "./stack-workspace.css";
  import "./stack-workspace-layout.css";
  import "./stack-workspace-drag.css";
  let {
    controller,
    showStackActions = true,
    showPicker = true,
  }: {
    controller: StackWorkspaceController;
    showStackActions?: boolean;
    showPicker?: boolean;
  } = $props();
  let formOpen = $state(false),
    conflictOpen = $state(false),
    formCommand = $state<WorkspaceCommand | null>(null);
  const notifications = ownNotifications();
  const snapshot = $derived(controller.snapshot);
  const eligible = $derived(
    snapshot?.availability === "ready" && Boolean(snapshot.folder),
  );
  function command(value: WorkspaceCommand) {
    if (controller.locked) return;
    const eligibility = controller.eligibility(value.kind);
    if (!eligibility.allowed) {
      controller.validation = eligibility.reason;
      return;
    }
    controller.validation = "";
    if (value.kind === "move_commit" && value.branch) {
      void controller.prepare(value);
      return;
    }
    if (value.kind === "edit_commit" && controller.snapshot) {
      if (!controller.commitDrafts.edit(controller.snapshot, value.commit))
        controller.validation =
          "Finish the current draft before editing this commit.";
      return;
    }
    if (
      value.kind === "create_commit" &&
      controller.page === "workspace" &&
      controller.snapshot &&
      activeWorkspace(controller.snapshot).applied.includes(value.stack)
    ) {
      for (const selection of value.selections ?? [value.selection])
        controller.commitDrafts.open(
          controller.snapshot,
          value.stack,
          selection,
        );
      return;
    }
    if (
      ("name" in value &&
        value.kind !== "delete_branch" &&
        value.kind !== "squash_branch") ||
      "message" in value ||
      value.kind === "move_commit" ||
      value.kind === "transfer"
    ) {
      formCommand = value;
      formOpen = true;
    } else void controller.prepare(value);
  }
</script>

<section
  class="stack-workspace community-projects"
  data-page={controller.page}
  aria-label="Local stack workspace"
>
  {#if showPicker}<header class="stack-workspace__header">
      <StoreStackWorkspacePicker {controller} />
    </header>{/if}
  <StoreNotice
    noticeKey="stack-workspace-command"
    title="Workspace command failed"
    message={controller.error}
    lifetime="condition"
  />
  <StoreNotificationHost owner={notifications} />
  {#if !snapshot || (controller.loading && !snapshot)}<CommunityProjectsRepositorySkeleton
      layout="graph"
      label="Loading stack workspace"
      count={5}
    />
  {:else if !eligible}<StoreStackWorkspaceActions
      {controller}
      {command}
    /><CommunityProjectsColumnAlert
      title="Local checkout unavailable"
      description={snapshot.availability === "unlinked"
        ? "Link a local JJ checkout to use stack workspaces."
        : "Reconnect this local checkout to use stack workspaces."}
    />
  {:else}
    {#if snapshot.conflicts.length}<CommunityProjectsColumnAlert
        title="Workspace has unresolved conflicts"
        description={snapshot.conflicts.map((item) => item.path).join(", ")}
        ><Button
          size="sm"
          disabled={!controller.can("resolve_conflict")}
          onclick={() => (conflictOpen = true)}>Resolve conflicts</Button
        ></CommunityProjectsColumnAlert
      >{/if}
    {#if controller.validation && !controller.preview && !formOpen}<p
        role="alert"
        class="stack-workspace__empty"
      >
        {controller.validation}
      </p>{/if}
    <Tabs.Root
      value={controller.page}
      onValueChange={(value) => {
        if (!controller.locked)
          controller.page = value as "branches" | "workspace";
      }}
      class="stack-workspace__views"
    >
      <nav
        class="community-projects__tabs stack-workspace__nav"
        aria-label="Stack workspace navigation"
      >
        <Tabs.List
          class="community-projects__tab-list"
          aria-label="Local repository views"
          ><Tabs.Trigger
            class="community-projects__tab-trigger"
            value="branches"
            disabled={controller.locked}>Branches</Tabs.Trigger
          ><Tabs.Trigger
            class="community-projects__tab-trigger"
            value="workspace"
            disabled={controller.locked}>Workspace</Tabs.Trigger
          ></Tabs.List
        >
        <StoreStackWorkspaceActions {controller} {command} />
      </nav>
      <Tabs.Content value={controller.page} class="stack-workspace__content"
        ><StoreStackWorkspaceCanvas
          {controller}
          {showStackActions}
          onCommand={command}
        /></Tabs.Content
      >
    </Tabs.Root>
    <footer class="stack-workspace__footer">
      <div class="stack-workspace__footer-items">
        <CommunityChip
          variant="muted"
          shape="square"
          class="stack-workspace__folder"
        >
          {#snippet leading()}<Folder size={14} aria-hidden="true" />{/snippet}
          {snapshot.folder}
        </CommunityChip>
        <CommunityChip variant="muted" shape="square">
          {#snippet leading()}<FolderKanban
              size={14}
              aria-hidden="true"
            />{/snippet}
          {activeWorkspace(snapshot).name}
        </CommunityChip>
        <CommunityChip variant="muted" shape="square">
          {#snippet leading()}<GitBranch
              size={14}
              aria-hidden="true"
            />{/snippet}
          {snapshot.base}
        </CommunityChip>
        <CommunityChip variant="muted" shape="square">
          {#snippet leading()}<GitBranchPlus
              size={14}
              aria-hidden="true"
            />{/snippet}
          {activeWorkspace(snapshot).applied.length} applied stacks
        </CommunityChip>
        <CommunityChip variant="muted" shape="square">
          {#snippet leading()}<File size={14} aria-hidden="true" />{/snippet}
          {activeWorkspace(snapshot).unassigned.length} unstaged files
        </CommunityChip>
      </div>
      <span role="status">{controller.outcome}</span
      >{#if formCommand && !controller.locked}<CommunityChip
          variant="muted"
          shape="square"
          onclick={() => (formOpen = true)}
          >{#snippet leading()}<Pencil aria-hidden="true" />{/snippet}Edit
          retained draft</CommunityChip
        >{/if}
    </footer>
  {/if}
  <StoreStackWorkspaceForm
    {controller}
    command={formCommand}
    bind:open={formOpen}
  />
  <StoreStackWorkspacePreview {controller} />
  <StoreStackWorkspaceConflict {controller} bind:open={conflictOpen} />
</section>
