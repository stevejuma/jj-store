<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraph from "../community-projects/CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsRepositoryLogFiles from "../community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { stackRevisions } from "../../controller/stack-workspace/presentation.js";
  import {
    buildRepositoryLogGraph,
    repositoryLogSvgWidth,
  } from "../../projects/repository-log-graph.js";
  import { TARGET_HISTORY_ORIGIN } from "../../controller/stack-workspace/target-history.svelte.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let { controller }: { controller: StackWorkspaceController } = $props();
  const history = $derived(controller.targetHistory);
  const branch = $derived(controller.snapshot!.target);
  const stack = $derived({
    id: TARGET_HISTORY_ORIGIN,
    name: branch.name,
    base: branch.name,
    heads: [{ name: branch.name, commit: branch.commit }],
    commits: [...history.commits].reverse(),
    conflicts: false,
  });
  const selected = $derived(
    history.commits.find((commit) => commit.id === history.selectedId),
  );
  const revisions = $derived(stackRevisions(stack, history.commits));
  const graphInset = $derived(
    repositoryLogSvgWidth(buildRepositoryLogGraph(revisions)),
  );
</script>

<ColumnCanvas.Column
  resizable
  id={TARGET_HISTORY_ORIGIN}
  columnTitle={branch.name}
  aria-label={`Stack ${branch.name}`}
>
  <ColumnCanvas.Header>
    <GitBranch size={16} aria-hidden="true" />
    <ColumnCanvas.Title>{branch.name}</ColumnCanvas.Title>
    <ColumnCanvas.Count
      >{history.total ?? history.commits.length}</ColumnCanvas.Count
    >
    <ColumnCanvas.HeaderActions>
      <ColumnCanvas.Toggle aria-label={`Collapse ${branch.name}`} />
    </ColumnCanvas.HeaderActions>
  </ColumnCanvas.Header>
  <ColumnCanvas.Body scrollStrategy="styled">
    <StoreNotice
      noticeKey="stack-workspace-target-history"
      title="Branch history unavailable"
      message={history.error}
      lifetime="condition"
    />
    {#if history.error}
      <Button
        size="sm"
        variant="outline"
        disabled={history.loading}
        onclick={() => void history.loadMore()}>Retry branch history</Button
      >
    {/if}
    {#if history.loading && !history.commits.length}
      <CommunityProjectsRepositorySkeleton
        layout="graph"
        label="Loading branch history"
        count={5}
      />
    {:else if !history.commits.length && !history.error}
      <CommunityProjectsColumnAlert
        title="No commits"
        description="The target branch has no history."
      />
    {:else}
      <Card.Root
        class="stack-workspace__bookmark-card"
        style={`--stack-graph-inset: ${graphInset}px`}
        data-loaded-commits={history.commits.length}
      >
        <Card.Header class="stack-workspace__bookmark-header">
          <GitBranch aria-hidden="true" />
          <div>
            <Card.Title
              ><ColumnCanvas.Title>{branch.name}</ColumnCanvas.Title
              ></Card.Title
            >
            <Card.Description>
              <CommunityProjectsGitAuthorAvatar
                name={branch.author}
                email={branch.email}
                size="xsmall"
              />
              {branch.author} · <CommunityRelativeTime
                timestamp={branch.createdAt}
              />
            </Card.Description>
            <div class="stack-workspace__branch-chips">
              <CommunityChip variant="muted">Workspace target</CommunityChip>
            </div>
          </div>
        </Card.Header>
        <CommunityProjectsRevisionGraph
          {revisions}
          avatarSize="xsmall"
          {graphInset}
          continuous
          endBorder={false}
          selectedId={history.selectedId}
          graphLabel={`Commits in ${branch.name}`}
          onSelect={(revision) => history.selectCommit(revision.eventId)}
          hasMore={history.hasMore && !history.error}
          loading={history.loading}
          onLoadMore={() => void history.loadMore()}
        >
          {#snippet rowActions()}<CommunityChip variant="muted"
              >Immutable</CommunityChip
            >{/snippet}
          {#snippet selectedFiles()}
            {#if selected}
              <CommunityProjectsRepositoryLogFiles
                title="Changed files"
                ariaLabel={`Changed files in ${branch.name}`}
                compact
                scrollStrategy="styled"
                files={selected.files.map((file) => ({
                  path: file.path,
                  additions: file.hunks.length,
                  deletions: file.hunks.length,
                }))}
                selectedPath={history.diff?.source === selected.id
                  ? history.diff.path
                  : null}
                onSelectPath={(path) => history.selectFile(selected.id, path)}
              />
            {/if}
          {/snippet}
        </CommunityProjectsRevisionGraph>
      </Card.Root>
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
