<script lang="ts">
  import { connectForgeView } from "../../controller/forge/view-lifecycle.js";
  import { untrack } from "svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import type { StoreRoute } from "../../controller/contracts.js";
  import type { Snippet } from "svelte";
  import type { StoreStacksView } from "../../controller/store-stacks-navigation.js";
  import StoreRepositorySettingsButton from "../store-settings/StoreRepositorySettingsButton.svelte";
  import StoreStacksNavigation from "./StoreStacksNavigation.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackFolderCopy from "./StoreStackFolderCopy.svelte";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import File from "@lucide/svelte/icons/file";
  import Pencil from "@lucide/svelte/icons/pencil";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreStackWorkspaceState from "./StoreStackWorkspaceState.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreStackWorkspaceSetup from "./StoreStackWorkspaceSetup.svelte";
  import StoreStackWorkspaceCanvas from "./StoreStackWorkspaceCanvas.svelte";
  import StoreStackWorkspaceForm from "./StoreStackWorkspaceForm.svelte";
  import StoreStackWorkspacePreview from "./StoreStackWorkspacePreview.svelte";
  import {
    selectedWorkspace,
    isWorkspaceBranch,
  } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  import { applyWorkspaceMembership } from "../../controller/stack-workspace/command-request.js";
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
    onRepositorySettings,
    focusedView,
    route,
    layoutStorage,
    onSelectView,
    historyMode,
    onHistoryMode,
    onBack,
    history,
  }: {
    controller: StackWorkspaceController;
    showStackActions?: boolean;
    showPicker?: boolean;
    onSettings?: () => void;
    onRepositorySettings?: () => void;
    focusedView?: StoreStacksView;
    route?: StoreRoute;
    layoutStorage?: Storage;
    onSelectView?: (view: StoreStacksView) => void;
    historyMode?: "live" | "pinned";
    onHistoryMode?: (mode: "live" | "pinned") => void;
    onBack?: () => void;
    history?: Snippet;
  } = $props();
  let formOpen = $state(false),
    formCommand = $state<WorkspaceCommand | null>(null);
  const notifications = ownNotifications();
  $effect(() => connectForgeView(controller.forge, notifications.context));
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
    if (value.kind === "apply_stack" || value.kind === "unapply_stack")
      return void applyWorkspaceMembership(controller, value);
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
      isWorkspaceBranch(
        controller.snapshot,
        controller.snapshot.stacks.find((stack) => stack.id === value.stack)
          ?.name ?? "",
      )
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
    action={snapshot?.conflicts.length &&
    controller.eligibility("resolve_conflict").allowed
      ? {
          id: "resolve-workspace-conflicts",
          label: "Resolve conflicts",
          variant: "secondary",
          run: () => controller.openConflicts(),
        }
      : undefined}
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
        {historyMode}
        {onHistoryMode}
      />{#if onRepositorySettings}<StoreRepositorySettingsButton
          disabled={controller.locked}
          onOpen={onRepositorySettings}
        />{/if}{/snippet}
  </StoreStacksNavigation>
  {#if !snapshot || !controller.snapshotLoaded || !eligible}
    <StoreStackWorkspaceState
      {controller}
      view={focusedView ?? controller.page}
      {route}
      {layoutStorage}
    />
  {:else}
    {#if (focusedView ?? controller.page) === "workspace" && snapshot.conflicts.length}<CommunityProjectsColumnAlert
        inline
        description="Workspace has unresolved conflicts"
      >
        {#snippet details()}<CommunityChip
            size="indicator"
            variant="muted"
            class="stack-workspace__conflict-count"
            ariaLabel={`${snapshot.conflicts.length} ${snapshot.conflicts.length === 1 ? "conflict" : "conflicts"}`}
            >{snapshot.conflicts.length}</CommunityChip
          >{/snippet}
        <Button
          size="sm"
          disabled={!controller.can("resolve_conflict")}
          onclick={() => controller.openConflicts()}>Resolve conflicts</Button
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
            {#if snapshot.folder}<StoreStackFolderCopy
                path={snapshot.folder}
              />{/if}
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
</section>
