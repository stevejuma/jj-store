<script lang="ts">
  import StoreStackLaneMenu from "./StoreStackLaneMenu.svelte";
  import { toggleStackSource } from "../../controller/stack-workspace/selection-actions.js";
  import { untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import StoreStackCommitComposer from "./StoreStackCommitComposer.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreStackHistorySkeleton from "./StoreStackHistorySkeleton.svelte";
  import Trash from "@lucide/svelte/icons/trash-2";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import LogIn from "@lucide/svelte/icons/log-in";
  import Eject from "@lucide/svelte/icons/eject";
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
  const history = $derived(controller.histories.state(stack.id));
  const preparing = $derived(
    controller.adapter.readStackHistory !== undefined &&
      !history.loaded &&
      stack.heads.length > 0,
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
      if (!(
        event.target instanceof Element &&
        event.target.closest('[data-ui-part="column-header-actions"]')
      ))
        toggleStackSource(controller, stack.id, { branch: stack.name });
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
      <StoreStackLaneMenu {controller} {stack} {applied} {onCommand} />
      <ColumnCanvas.Toggle aria-label={`Collapse ${stack.name}`} />
    </ColumnCanvas.HeaderActions>
  </ColumnCanvas.Header>
  <ColumnCanvas.Body
    scrollStrategy="styled"
    orientation="both"
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
      message={history.error}
      lifetime="condition"
    />
    {#if history.error}<Button
        variant="outline"
        size="sm"
        onclick={() =>
          void controller.histories.load(stack.id, Boolean(history.cursor))}
        >Retry stack history</Button
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
          context={stack.name}
          onOtherDrop={(event) => onDrop(event, "stack", member.id)}
        />
      {/each}
    {/if}
    {#if preparing}
      <div class="stack-workspace__bookmarks">
        <StoreStackHistorySkeleton
          label={`Loading history for ${stack.name}`}
          count={5}
        />
      </div>
      {#if !history.loading && !history.error}
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
          {#if group.unloaded}
            <StoreStackHistorySkeleton
              label={`Loading history for ${group.head.name}`}
              count={3}
            />
            <Button
              variant="outline"
              size="sm"
              disabled={history.loading}
              onclick={() => void controller.histories.load(stack.id, true)}
              >Load history for {group.head.name}</Button
            >
          {:else}
            <StoreStackBookmarkCard
              {controller}
              {stack}
              showActions={showStackActions}
              submitStack={index === 0}
              name={group.head.name}
              aliases={group.head.aliases ?? []}
              shared={group.head.shared ?? false}
              commits={group.commits}
              {onCommand}
            />
          {/if}
        {/each}
      </div>
      {#if !stack.commits.length}<p class="stack-workspace__empty">
          No commits outside the workspace target in this branch.
        </p>{/if}
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
