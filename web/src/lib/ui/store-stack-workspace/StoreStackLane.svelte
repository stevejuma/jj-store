<script lang="ts">
  import { untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import StoreStackCommitComposer from "./StoreStackCommitComposer.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Pencil from "@lucide/svelte/icons/pencil";
  import Trash from "@lucide/svelte/icons/trash-2";
  import LogIn from "@lucide/svelte/icons/log-in";
  import Eject from "@lucide/svelte/icons/eject";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import {
    repositoryLogLaneX,
    REPOSITORY_LOG_STROKE_WIDTH,
  } from "../../projects/repository-log-graph.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreStackBookmarkCard from "./StoreStackBookmarkCard.svelte";
  import {
    stackGroups,
    stackLaneMembers,
  } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceCommand,
    WorkspaceStack,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    showStackActions = true,
    stack,
    applied,
    onCommand,
    onDrop,
  }: {
    controller: StackWorkspaceController;
    showStackActions?: boolean;
    stack: WorkspaceStack;
    applied: boolean;
    onCommand: (command: WorkspaceCommand) => void;
    onDrop: (event: DragEvent, target: "stack" | "commit", id: string) => void;
  } = $props();
  const preparing = $derived(
    Boolean(
      controller.adapter.readStackHistory &&
      !controller.histories.state(stack.id).loaded &&
      stack.heads.length,
    ),
  );
  $effect(() => {
    const id = stack.id;
    controller.snapshot?.readKey;
    untrack(() => {
      void controller.histories.load(id);
    });
  });
</script>

<ColumnCanvas.Column
  resizable
  id={`stack-${stack.id}`}
  aria-label={`Stack ${stack.name}`}
>
  <ColumnCanvas.Header
    class="stack-workspace__stack-header"
    onclick={(event) => {
      if (
        !controller.locked &&
        !(
          event.target instanceof Element &&
          event.target.closest('[data-ui-part="column-header-actions"]')
        )
      )
        controller.selectStackFiles(stack.id);
    }}
  >
    <GitBranch size={16} aria-hidden="true" /><ColumnCanvas.Title
      ><button
        type="button"
        class="stack-workspace__branch-title"
        disabled={controller.locked}
        aria-label={`Show all changes in stack ${stack.name}`}
        >{stack.name}</button
      ></ColumnCanvas.Title
    >
    <ColumnCanvas.Count>{stack.commits.length}</ColumnCanvas.Count>
    {#if stack.conflicts}<CommunityChip tone="warning">Conflicts</CommunityChip
      >{/if}
    <ColumnCanvas.HeaderActions>
      <Menu.Root
        ><Menu.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              class="stack-workspace__menu-trigger"
              aria-label={`Actions for ${stack.name}`}
              disabled={controller.locked}
              ><Ellipsis aria-hidden="true" /></Button
            >{/snippet}</Menu.Trigger
        >
        <Menu.Content
          align="end"
          preventScroll={false}
          class="stack-workspace__action-menu"
          ><Menu.Group>
            <Menu.Item
              disabled={!controller.can(
                applied ? "unapply_stack" : "apply_stack",
              )}
              onSelect={() =>
                onCommand({
                  kind: applied ? "unapply_stack" : "apply_stack",
                  stack: stack.id,
                  ...(controller.page === "branches"
                    ? { branch: stack.name }
                    : {}),
                })}
              >{#if applied}<Eject aria-hidden="true" />{:else}<LogIn
                  aria-hidden="true"
                />{/if}{applied ? "Unapply stack" : "Apply stack"}</Menu.Item
            >
            <Menu.Item
              disabled={!controller.can("rename_stack")}
              title={controller.eligibility("rename_stack").reason || undefined}
              onSelect={() =>
                onCommand({
                  kind: "rename_stack",
                  stack: stack.id,
                  name: stack.name,
                })}><Pencil aria-hidden="true" />Rename stack</Menu.Item
            >
            <Menu.Item
              disabled={!controller.can("rebase")}
              title={controller.eligibility("rebase").reason || undefined}
              onSelect={() => onCommand({ kind: "rebase", stack: stack.id })}
              ><GitBranch aria-hidden="true" />Rebase onto base</Menu.Item
            >
            <Menu.Item
              disabled={!controller.can("delete_stack")}
              title={controller.eligibility("delete_stack").reason || undefined}
              onSelect={() =>
                onCommand({ kind: "delete_stack", stack: stack.id })}
              ><Trash aria-hidden="true" />Delete stack</Menu.Item
            >
          </Menu.Group></Menu.Content
        >
      </Menu.Root>
      <ColumnCanvas.Toggle aria-label={`Collapse ${stack.name}`} />
    </ColumnCanvas.HeaderActions>
  </ColumnCanvas.Header>
  <ColumnCanvas.Body
    scrollStrategy="styled"
    class="stack-workspace__drop-target"
    data-drop-target-state={controller.drag.state({
      kind: "stack",
      id: stack.id,
    })}
    ondragover={(event) =>
      controller.drag.over(
        event,
        { kind: "stack", id: stack.id },
        controller.snapshot,
        controller.locked,
      )}
    ondragleave={(event) =>
      controller.drag.leave(event, { kind: "stack", id: stack.id })}
    ondrop={(event) => onDrop(event, "stack", stack.id)}
  >
    <StoreStackDropGuide
      feedback={controller.drag.for({ kind: "stack", id: stack.id })}
    />
    <StoreNotice
      noticeKey={`stack-history-${stack.id}`}
      title="Stack history unavailable"
      message={controller.histories.state(stack.id).error}
      lifetime="condition"
    />
    {#if controller.histories.state(stack.id).error}<Button
        variant="outline"
        size="sm"
        onclick={() =>
          void controller.histories.load(
            stack.id,
            Boolean(controller.histories.state(stack.id).cursor),
          )}>Retry stack history</Button
      >{/if}
    {#if controller.page === "branches"}
      <Card.Root class="stack-workspace__stage-card">
        <Card.Content class="stack-workspace__branch-actions">
          <Button
            size="sm"
            disabled={!controller.can(
              applied ? "unapply_stack" : "apply_stack",
            )}
            aria-label={`${applied ? "Unapply" : "Apply"} ${stack.name}`}
            onclick={() =>
              onCommand({
                kind: applied ? "unapply_stack" : "apply_stack",
                stack: stack.id,
                branch: stack.name,
              })}
          >
            {applied ? "Unapply from workspace" : "Apply to workspace"}
            {#if applied}<Eject
                data-icon="inline-end"
                aria-hidden="true"
              />{:else}<LogIn data-icon="inline-end" aria-hidden="true" />{/if}
          </Button>
          <Button
            variant="outline"
            size="sm"
            disabled={!controller.can("delete_stack")}
            title={controller.eligibility("delete_stack").reason || undefined}
            onclick={() => onCommand({ kind: "delete_stack", stack: stack.id })}
          >
            Delete local<Trash data-icon="inline-end" aria-hidden="true" />
          </Button>
        </Card.Content>
      </Card.Root>
    {/if}
    {#if controller.page === "workspace"}<StoreStackCommitComposer
        {controller}
        {stack}
        onOtherDrop={(event) => onDrop(event, "stack", stack.id)}
      />
      {#each stackLaneMembers(controller.snapshot!, stack).filter((member) => member.id !== stack.id && controller.commitDrafts.get(controller.snapshot!.active!, member.id)) as member (member.id)}
        <StoreStackCommitComposer
          {controller}
          stack={member}
          onOtherDrop={(event) => onDrop(event, "stack", member.id)}
        />
      {/each}
    {/if}
    {#if preparing}
      <CommunityProjectsRepositorySkeleton
        layout="graph"
        label={`Loading history for ${stack.name}`}
        count={5}
      />
      {#if !controller.histories.state(stack.id).loading && !controller.histories.state(stack.id).error}
        <Button
          variant="outline"
          size="sm"
          onclick={() => void controller.histories.load(stack.id)}
          >Retry stack history</Button
        >
      {/if}
    {:else}
      <div class="stack-workspace__bookmarks">
        {#each stackGroups(stack) as group, index (group.head.name)}
          {#if index > 0}<svg
              class="stack-workspace__bookmark-link"
              data-ui-part="stack-link"
              aria-hidden="true"
              ><line
                x1={repositoryLogLaneX(0)}
                x2={repositoryLogLaneX(0)}
                y1="0"
                y2="100%"
                stroke="var(--ui-log-lane-1, #2563eb)"
                stroke-width={REPOSITORY_LOG_STROKE_WIDTH}
              /></svg
            >{/if}
          <StoreStackBookmarkCard
            {controller}
            {stack}
            showActions={showStackActions}
            name={group.head.name}
            aliases={group.head.aliases ?? []}
            commits={group.commits}
            {onCommand}
          />
        {/each}
      </div>
      {#if !stack.commits.length}<p class="stack-workspace__empty">
          No commits yet. Create a commit or assign selected changes.
        </p>{/if}
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
