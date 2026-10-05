<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import type { Snippet } from "svelte";
  import type { StoreStacksView } from "../../controller/store-stacks-navigation.js";
  import StoreStacksNavigation from "./StoreStacksNavigation.svelte";
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
  import StoreStackWorkspaceSetup from "./StoreStackWorkspaceSetup.svelte";
  import StoreStackWorkspaceCanvas from "./StoreStackWorkspaceCanvas.svelte";
  import StoreStackWorkspaceForm from "./StoreStackWorkspaceForm.svelte";
  import StoreStackWorkspacePreview from "./StoreStackWorkspacePreview.svelte";
  import StoreStackWorkspaceConflict from "./StoreStackWorkspaceConflict.svelte";
  import { selectedWorkspace } from "../../controller/stack-workspace/presentation.js";
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
    onSettings,
    focusedView,
    onSelectView,
    onBack,
    history,
  }: {
    controller: StackWorkspaceController;
    showStackActions?: boolean;
    showPicker?: boolean;
    onSettings?: () => void;
    focusedView?: StoreStacksView;
    onSelectView?: (view: StoreStacksView) => void;
    onBack?: () => void;
    history?: Snippet;
  } = $props();
  let formOpen = $state(false),
    conflictOpen = $state(false),
    formCommand = $state<WorkspaceCommand | null>(null);
  const notifications = ownNotifications();
  const showCanvas = $derived(
    !focusedView || focusedView === "branches" || focusedView === "workspace",
  );
  const snapshot = $derived(controller.snapshot);
  const workspace = $derived(
    snapshot ? selectedWorkspace(snapshot) : undefined,
  );
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
      selectedWorkspace(controller.snapshot)?.applied.includes(value.stack)
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
  <StoreNotice
    noticeKey="local-stack-settings"
    title="Local workspace needs attention"
    message={snapshot?.workspaceWarning ?? ""}
    lifetime="condition"
  />
  <StoreStacksNavigation
    {controller}
    view={focusedView}
    {onSelectView}
    {onBack}
  >
    {#snippet actions()}<StoreStackWorkspaceActions
        {controller}
        {command}
        {onSettings}
      />{/snippet}
  </StoreStacksNavigation>
  {#if !snapshot || (controller.loading && !snapshot)}<CommunityProjectsRepositorySkeleton
      layout="graph"
      label="Loading stack workspace"
      count={5}
    />
  {:else if !eligible}<CommunityProjectsColumnAlert
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
    <div class="stack-workspace__views">
      <div
        class="stack-workspace__canvas-view"
        hidden={!showCanvas}
        inert={!showCanvas}
      >
        <div
          class="stack-workspace__content"
          role="tabpanel"
          aria-label={controller.page === "workspace"
            ? "Workspace"
            : "Branches"}
        >
          {#if !snapshot.target}<StoreStackWorkspaceSetup {controller} />{/if}
          {#if controller.page === "workspace" && !workspace}
            <Empty.Root class="flex-1">
              <Empty.Header>
                <Empty.Media variant="icon"
                  ><FolderKanban aria-hidden="true" /></Empty.Media
                >
                <Empty.Title>No stack workspace</Empty.Title>
                <Empty.Description
                  >Choose a target branch, then set up a named workspace to
                  combine stacks and retain unfinished work.</Empty.Description
                >
              </Empty.Header>
              <Empty.Content
                ><Button
                  disabled={!controller.can("create_workspace")}
                  onclick={() =>
                    command({ kind: "create_workspace", name: "" })}
                  >Create workspace</Button
                ></Empty.Content
              >
            </Empty.Root>
          {:else if snapshot.target}<StoreStackWorkspaceCanvas
              {controller}
              {showStackActions}
              onCommand={command}
            />{/if}
        </div>
        <footer class="stack-workspace__footer">
          <div class="stack-workspace__footer-items">
            <CommunityChip
              variant="muted"
              shape="square"
              class="stack-workspace__folder"
            >
              {#snippet leading()}<Folder
                  size={14}
                  aria-hidden="true"
                />{/snippet}
              {snapshot.folder}
            </CommunityChip>
            <CommunityChip variant="muted" shape="square">
              {#snippet leading()}<FolderKanban
                  size={14}
                  aria-hidden="true"
                />{/snippet}
              {workspace?.name ?? "No stack workspace"}
            </CommunityChip>
            <CommunityChip variant="muted" shape="square">
              {#snippet leading()}<GitBranch
                  size={14}
                  aria-hidden="true"
                />{/snippet}
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
              {#snippet leading()}<File
                  size={14}
                  aria-hidden="true"
                />{/snippet}
              {controller.filesFor("unassigned")
                .length}{controller.details.fileState("unassigned").nextCursor
                ? "+"
                : ""} unstaged files
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
      </div>
      {#if !showCanvas}{@render history?.()}{/if}
    </div>
  {/if}
  <StoreStackWorkspaceForm
    {controller}
    command={formCommand}
    bind:open={formOpen}
  />
  <StoreStackWorkspacePreview {controller} />
  <StoreStackWorkspaceConflict {controller} bind:open={conflictOpen} />
</section>
