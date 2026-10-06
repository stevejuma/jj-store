<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraph from "../community-projects/CommunityProjectsRevisionGraph.svelte";
  import { toggleStackSource } from "../../controller/stack-workspace/selection-actions.js";
  import { stackCardSelection } from "../../controller/stack-workspace/card-selection.js";
  import StoreStackSourceFiles from "./StoreStackSourceFiles.svelte";
  import StoreStackBookmarkActions from "./StoreStackBookmarkActions.svelte";
  import StoreStackCommitActions from "./StoreStackCommitActions.svelte";
  import { workspaceAggregateSource } from "../../controller/stack-workspace/files.js";
  import {
    buildRepositoryLogGraph,
    repositoryLogSvgWidth,
  } from "../../projects/repository-log-graph.js";
  import {
    stackRevisions,
    isAppliedBranch,
    isWorkspaceBranch,
  } from "../../controller/stack-workspace/presentation.js";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import {
    commitEventRow,
    commitRowTarget,
  } from "../../controller/stack-workspace/commit-placement.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    StackCommit,
    WorkspaceCommand,
    WorkspaceStack,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    stack,
    name,
    aliases = [],
    shared = false,
    commits,
    showActions = true,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    stack: WorkspaceStack;
    name: string;
    aliases?: string[];
    shared?: boolean;
    commits: StackCommit[];
    showActions?: boolean;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  const graphWidth = $derived(
    repositoryLogSvgWidth(buildRepositoryLogGraph(stackRevisions(stack))),
  );
  const card = $derived(
    stackCardSelection(
      controller.snapshot!,
      controller.stackSelections,
      stack,
      name,
      aliases,
      commits,
    ),
  );
  const diff = $derived(
    controller.diffs.find((diff) => diff.origin === `stack-${card.owner}`),
  );
  const diffSelection = $derived(
    diff && !workspaceAggregateSource(diff.source)
      ? {
          source: diff.source,
          path: diff.path,
          ...(diff.hunks ? { hunks: diff.hunks } : {}),
        }
      : null,
  );
  const branchSource = $derived(card.branchSource);
  const headerTarget = $derived({
    kind: "stack" as const,
    id: card.owner,
    branch: card.branch,
  });
  const tip = $derived(commits[0]);
  const selected = $derived(card.commit);
</script>

<Card.Root
  class="stack-workspace__bookmark-card"
  style={`min-width: calc(${graphWidth}px + 18rem)`}
>
  <Card.Header
    class="stack-workspace__bookmark-header stack-workspace__drop-target"
    data-selected={card.branchSelected || undefined}
    data-drop-target-state={controller.drag.state(headerTarget)}
    ondragover={(event) =>
      controller.drag.over(
        event,
        headerTarget,
        controller.snapshot,
        controller.locked,
      )}
    ondragleave={(event) => controller.drag.leave(event, headerTarget)}
    ondrop={(event) => {
      const command = controller.drag.drop(
        event,
        headerTarget,
        controller.snapshot,
        controller.locked,
      );
      if (command) onCommand(command);
    }}
    onclick={() => {
      if (!controller.locked)
        toggleStackSource(controller, card.owner, { branch: card.branch });
    }}
  >
    <StoreStackDropGuide feedback={controller.drag.for(headerTarget)} />
    <GitBranch aria-hidden="true" />
    <div>
      <Card.Title
        ><button
          type="button"
          class="stack-workspace__branch-title"
          disabled={controller.locked}
          aria-label={`Show all changes in ${name}`}
          aria-pressed={card.branchSelected}
          ><ColumnCanvas.Title>{name}</ColumnCanvas.Title></button
        ></Card.Title
      >
      {#if tip && controller.page === "workspace"}<Card.Description
          ><CommunityProjectsGitAuthorAvatar
            name={tip.author}
            email={tip.email}
            size="xsmall"
          />{tip.author} · <CommunityRelativeTime
            timestamp={tip.createdAt}
          /></Card.Description
        >{/if}
      <div class="stack-workspace__branch-chips">
        <CommunityChip variant="muted">Local bookmark</CommunityChip>
        {#if shared}<CommunityChip variant="muted">Shared</CommunityChip>{/if}
        {#each aliases as alias}<CommunityChip variant="muted"
            >{alias}</CommunityChip
          >{/each}
      </div>
    </div>
  </Card.Header>
  {#if card.branchSelected}
    <Card.Content class="stack-workspace__branch-files">
      <StoreStackSourceFiles
        {controller}
        source={branchSource}
        ariaLabel={`Changed files in branch ${name}${shared ? ` in stack ${stack.name}` : ""}`}
        selectedPath={diff?.source === branchSource ? diff.path : null}
        onSelectPath={(path) => controller.selectFile(branchSource, path)}
      />
    </Card.Content>
  {/if}
  {#if showActions}<StoreStackBookmarkActions
      stack={stack.id}
      commitStack={controller.snapshot?.stacks.find(
        (item) => item.name === name,
      )?.id ?? stack.id}
      {name}
      applied={isAppliedBranch(controller.snapshot!, name)}
      canAddCommit={isWorkspaceBranch(controller.snapshot!, name) &&
        stack.heads.find((head) => head.name === name)?.baseCommit !== null}
      commit={stack.heads.find((head) => head.name === name)?.commit ??
        stack.base}
      empty={!commits.length}
      below={stack.heads.find((head) => head.name === name)?.baseCommit ??
        stack.heads[stack.heads.findIndex((head) => head.name === name) + 1]
          ?.commit ??
        stack.base}
      canRewrite={!commits.some((commit) => commit.immutable)}
      canSquash={commits.length > 1 &&
        !commits.some((commit) => commit.immutable)}
      disabled={controller.locked}
      enabled={(kind) => controller.can(kind)}
      reason={(kind) => controller.eligibility(kind).reason}
      {onCommand}
    />{/if}
  <!-- Native drag events delegate to the owned graph's revision rows. -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    ondragstart={(event) => {
      const id = commitEventRow(event)?.dataset.revisionId;
      if (
        id &&
        (controller.can("move_commit") || controller.can("reorder_commit")) &&
        !commits.find((commit) => commit.id === id)?.immutable
      )
        controller.drag.start(event, { kind: "commit", commit: id });
    }}
    ondragover={(event) => {
      const target = commitRowTarget(event, controller.drag.payload);
      if (target)
        controller.drag.over(
          event,
          target,
          controller.snapshot,
          controller.locked,
        );
    }}
    ondragleave={(event) => {
      const id = commitEventRow(event)?.dataset.revisionId;
      if (id)
        controller.drag.leave(
          event,
          { kind: "commit", id },
          event.target instanceof Element
            ? event.target.closest("[data-revision-id]")
            : undefined,
        );
    }}
    ondrop={(event) => {
      const target = commitRowTarget(event, controller.drag.payload);
      if (target) {
        const command = controller.drag.drop(
          event,
          target,
          controller.snapshot,
          controller.locked,
        );
        if (command) onCommand(command);
      }
    }}
  >
    <CommunityProjectsRevisionGraph
      revisions={stackRevisions(stack, commits)}
      avatarSize="xsmall"
      continuous
      hasMore={name === stack.heads.at(-1)?.name &&
        controller.histories.state(stack.id).hasMore}
      loading={controller.histories.state(stack.id).loading}
      onLoadMore={() => void controller.histories.load(stack.id, true)}
      rowDropState={(revision) =>
        controller.drag.state({ kind: "commit", id: revision.eventId })}
      rowDropPlacement={(revision) =>
        controller.drag.for({ kind: "commit", id: revision.eventId })
          ?.placement}
      endBorder={false}
      selectedId={selected?.id ?? null}
      graphLabel={`Commits in ${name}`}
      onSelect={(revision) => {
        if (!controller.locked)
          toggleStackSource(controller, card.owner, {
            commit: revision.eventId,
          });
      }}
      draggable={(revision) =>
        (controller.can("move_commit") || controller.can("reorder_commit")) &&
        !commits.find((commit) => commit.id === revision.eventId)?.immutable}
    >
      {#snippet rowActions(revision)}
        <StoreStackDropGuide
          feedback={controller.drag.for({
            kind: "commit",
            id: revision.eventId,
          })}
        />
        {@const commit = commits.find((item) => item.id === revision.eventId)!}
        {#if commit.immutable}<CommunityChip variant="muted"
            >Immutable</CommunityChip
          >{/if}
        <StoreStackCommitActions
          {commit}
          {stack}
          disabled={controller.locked}
          enabled={(kind) => controller.can(kind)}
          reason={(kind) => controller.eligibility(kind).reason}
          selection={diffSelection}
          {onCommand}
        />
      {/snippet}
      {#snippet selectedFiles()}
        {#if selected}<StoreStackSourceFiles
            {controller}
            source={selected.id}
            ariaLabel={`Changed files in ${name}${shared ? ` in stack ${stack.name}` : ""}`}
            selectedPath={diff?.source === selected.id ? diff.path : null}
            onSelectPath={(path) =>
              controller.selectFile(selected.id, path, undefined, card.owner)}
          />{/if}
      {/snippet}
    </CommunityProjectsRevisionGraph>
  </div>
</Card.Root>
