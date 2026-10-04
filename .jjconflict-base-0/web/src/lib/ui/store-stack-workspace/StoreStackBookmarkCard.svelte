<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraph from "../community-projects/CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsRepositoryLogFiles from "../community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import StoreStackBookmarkActions from "./StoreStackBookmarkActions.svelte";
  import StoreStackCommitActions from "./StoreStackCommitActions.svelte";
  import {
    workspaceBranchSource,
    workspaceSourceFiles,
    workspaceAggregateSource,
  } from "../../controller/stack-workspace/files.js";
  import { workspaceFileDrag } from "../../controller/stack-workspace/file-drag.js";
  import {
    buildRepositoryLogGraph,
    repositoryLogSvgWidth,
  } from "../../projects/repository-log-graph.js";
  import { stackRevisions } from "../../controller/stack-workspace/presentation.js";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import { commitDropPlacement } from "../../controller/stack-workspace/commit-placement.js";
  import { WORKSPACE_DRAG_TYPE } from "../../controller/stack-workspace/drag.js";
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
    commits,
    showActions = true,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    stack: WorkspaceStack;
    name: string;
    commits: StackCommit[];
    showActions?: boolean;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  const graphInset = $derived(
    repositoryLogSvgWidth(buildRepositoryLogGraph(stackRevisions(stack))),
  );
  const diff = $derived(
    controller.diffs.find((diff) => diff.origin === `stack-${stack.id}`),
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
  const branchSource = $derived(workspaceBranchSource(stack.id, name));
  const headerTarget = $derived({
    kind: "stack" as const,
    id: stack.id,
    branch: name,
  });
  const branchSelected = $derived(
    controller.stackSelections[stack.id] === branchSource,
  );
  const tip = $derived(commits[0]);
  const selected = $derived(
    commits.find(
      (commit) => commit.id === controller.selectedCommitFor(stack.id),
    ),
  );
  function eventCommit(event: DragEvent) {
    return event.target instanceof Element
      ? event.target.closest<HTMLElement>("[data-revision-id]")?.dataset
          .revisionId
      : undefined;
  }
  function rowTarget(event: DragEvent) {
    const row =
      event.target instanceof Element
        ? event.target.closest<HTMLElement>("[data-revision-id]")
        : null;
    if (!row?.dataset.revisionId) return null;
    return {
      kind: "commit" as const,
      id: row.dataset.revisionId,
      placement: commitDropPlacement(
        controller.drag.payload ??
          event.dataTransfer?.getData(WORKSPACE_DRAG_TYPE) ??
          null,
        row.getBoundingClientRect(),
        event.clientY,
      ),
    };
  }
</script>

<Card.Root
  class="stack-workspace__bookmark-card"
  style={`--stack-graph-inset: ${graphInset}px`}
>
  <Card.Header
    class="stack-workspace__bookmark-header stack-workspace__drop-target"
    data-selected={branchSelected || undefined}
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
      if (!controller.locked) controller.selectStackFiles(stack.id, name);
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
          aria-pressed={branchSelected}
          ><ColumnCanvas.Title>{name}</ColumnCanvas.Title></button
        ></Card.Title
      >
      {#if tip}<Card.Description
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
      </div>
    </div>
  </Card.Header>
  {#if branchSelected}
    <Card.Content class="stack-workspace__branch-files">
      <CommunityProjectsRepositoryLogFiles
        title="Changed files"
        ariaLabel={`Changed files in branch ${name}`}
        compact
        scrollStrategy="styled"
        files={workspaceSourceFiles(controller.snapshot!, branchSource).map(
          (file) => ({
            path: file.path,
            additions: file.hunks.length,
            deletions: file.hunks.length,
          }),
        )}
        selectedPath={diff?.source === branchSource ? diff.path : null}
        onSelectPath={(path) => controller.selectFile(branchSource, path)}
        fileDrag={workspaceFileDrag(controller, () => branchSource)}
      />
    </Card.Content>
  {/if}
  {#if showActions}<StoreStackBookmarkActions
      stack={stack.id}
      {name}
      canAddCommit={name === stack.heads[0]?.name}
      commit={stack.heads.find((head) => head.name === name)?.commit ??
        stack.base}
      empty={!commits.length}
      below={stack.heads[
        stack.heads.findIndex((head) => head.name === name) + 1
      ]?.commit ?? stack.base}
      canRewrite={!commits.some((commit) => commit.immutable)}
      canSquash={commits.length > 1 &&
        !commits.some((commit) => commit.immutable)}
      disabled={controller.locked}
      {onCommand}
    />{/if}
  <!-- Native drag events delegate to the owned graph's revision rows. -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    ondragstart={(event) => {
      const id = eventCommit(event);
      if (
        id &&
        !controller.locked &&
        !commits.find((commit) => commit.id === id)?.immutable
      )
        controller.drag.start(event, { kind: "commit", commit: id });
    }}
    ondragover={(event) => {
      const target = rowTarget(event);
      if (target)
        controller.drag.over(
          event,
          target,
          controller.snapshot,
          controller.locked,
        );
    }}
    ondragleave={(event) => {
      const id = eventCommit(event);
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
      const target = rowTarget(event);
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
      {graphInset}
      continuous
      rowDropState={(revision) =>
        controller.drag.state({ kind: "commit", id: revision.eventId })}
      rowDropPlacement={(revision) =>
        controller.drag.for({ kind: "commit", id: revision.eventId })
          ?.placement}
      endBorder={false}
      selectedId={controller.selectedCommitFor(stack.id)}
      graphLabel={`Commits in ${name}`}
      onSelect={(revision) => {
        if (!controller.locked) controller.selectCommit(revision.eventId);
      }}
      draggable={(revision) =>
        !controller.locked &&
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
          selection={diffSelection}
          {onCommand}
        />
      {/snippet}
      {#snippet selectedFiles()}
        {#if selected}<CommunityProjectsRepositoryLogFiles
            title="Changed files"
            ariaLabel={`Changed files in ${name}`}
            files={selected.files.map((file) => ({
              path: file.path,
              additions: file.hunks.length,
              deletions: file.hunks.length,
            }))}
            compact
            scrollStrategy="styled"
            selectedPath={diff?.source === selected.id ? diff.path : null}
            onSelectPath={(path) => controller.selectFile(selected.id, path)}
            fileDrag={workspaceFileDrag(controller, () => selected!.id)}
          />{/if}
      {/snippet}
    </CommunityProjectsRevisionGraph>
  </div>
</Card.Root>
