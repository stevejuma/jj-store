<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Table from "@lapismd/design-core/shadcn/table";

  import type {
    RepositoryBlob,
    RepositoryChoice,
  } from "../../controller/contracts.js";
  import type { RepositoryLogRevision as ProjectForgeCommit } from "../../projects/repository-log.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { RepositoryBrowserEntry } from "./repository-browser.js";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRepositoryBranchSelector from "./CommunityProjectsRepositoryBranchSelector.svelte";
  import CommunityProjectsRepositoryDirectoryRows from "./CommunityProjectsRepositoryDirectoryRows.svelte";
  import CommunityProjectsRepositoryDocuments from "./CommunityProjectsRepositoryDocuments.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import "./repository-directory.css";

  let {
    fileTreeOpen = $bindable(false),
    now,
    currentPath,
    entries,
    pathCommits,
    latestCommit,
    latestCommitAuthor,
    previewDocuments = [],
    branch,
    branches = [],
    canLoadPathCommit = false,
    onOpenDirectory,
    onOpenFile,
    onEditFile,
    editUnavailable = "",
    onSelectCommit,
    onSelectBranch,
    hasMore = false,
    loading = false,
    hasMoreReferences = false,
    loadingReferences = false,
    onLoadMore = () => {},
    onLoadMoreReferences,
  }: {
    fileTreeOpen?: boolean | undefined;
    now: number;
    currentPath: string;
    entries: readonly RepositoryBrowserEntry[];
    pathCommits: Readonly<Record<string, ProjectForgeCommit | undefined>>;
    latestCommit?: ProjectForgeCommit | undefined;
    latestCommitAuthor?: CommunityMemberModel | undefined;
    previewDocuments?: readonly RepositoryBlob[] | undefined;
    branch: string;
    branches?: readonly RepositoryChoice[] | undefined;
    canLoadPathCommit?: boolean | undefined;
    onOpenDirectory: (path: string) => void;
    onOpenFile: (path: string) => void;
    onEditFile?: ((path: string) => void) | undefined;
    editUnavailable?: string | undefined;
    onSelectCommit?: ((commit: ProjectForgeCommit) => void) | undefined;
    onSelectBranch: (value: string | undefined) => void;
    hasMore?: boolean;
    loading?: boolean;
    hasMoreReferences?: boolean;
    loadingReferences?: boolean;
    onLoadMore?: () => void | Promise<void>;
    onLoadMoreReferences?: () => void;
  } = $props();

  const latestCommitDate = $derived(latestCommit?.createdAt);
  const latestAuthorName = $derived(
    latestCommitAuthor?.name ?? latestCommit?.author ?? "",
  );
</script>

<section class="community-projects__file-browser">
  <div class="community-projects__file-browser-card">
    <div class="community-projects__file-browser-summary">
      <div class="community-projects__file-browser-summary-primary">
        <Button
          type="button"
          variant="ghost"
          size="icon-sm"
          class="community-projects__file-browser-tree-toggle"
          aria-label={fileTreeOpen
            ? "Hide repository file tree"
            : "Show repository file tree"}
          title={fileTreeOpen
            ? "Hide repository file tree"
            : "Show repository file tree"}
          onclick={() => (fileTreeOpen = !fileTreeOpen)}
        >
          <CommunitySidebarToggleIcon side="left" expanded={fileTreeOpen} />
        </Button>
        {#if latestCommit}
          <CommunityProjectsGitAuthorAvatar
            email={latestCommit.authorEmail}
            name={latestAuthorName}
            size="xsmall"
          />
          <span class="community-projects__file-browser-summary-message">
            <strong>{latestAuthorName}</strong> committed
            <button
              type="button"
              class="community-projects__file-browser-commit-link"
              title={`View commit ${latestCommit.message}`}
              aria-label={`View commit ${latestCommit.message}`}
              onclick={() => onSelectCommit?.(latestCommit)}
            >
              {latestCommit.message}
            </button>
          </span>
        {:else}
          <span class="community-projects__file-browser-summary-message">
            Select a repository snapshot
          </span>
        {/if}
      </div>
      <div class="community-projects__file-browser-summary-actions">
        {#if latestCommit}
          <CommunityProjectsRevisionGraphIds
            commitId={latestCommit.eventId}
            commitLabel="Git revision"
            placement="inline"
          />
          {#if latestCommitDate}
            <CommunityRelativeTime timestamp={latestCommitDate} {now} />
          {/if}
        {/if}
        <CommunityProjectsRepositoryBranchSelector
          {branch}
          {branches}
          hasMore={hasMoreReferences}
          loading={loadingReferences}
          {onSelectBranch}
          onLoadMore={onLoadMoreReferences}
        />
      </div>
    </div>

    <Table.Root
      class="community-projects__file-browser-table"
      aria-label={currentPath ? `Files in ${currentPath}` : "Repository files"}
    >
      <Table.Header>
        <Table.Row>
          <Table.Head>Name</Table.Head>
          <Table.Head>Description</Table.Head>
          <Table.Head>Updated</Table.Head>
        </Table.Row>
      </Table.Header>
      <CommunityProjectsRepositoryDirectoryRows
        {currentPath}
        {entries}
        {pathCommits}
        {canLoadPathCommit}
        {now}
        {hasMore}
        {loading}
        {onOpenDirectory}
        {onOpenFile}
        onLoadMore={() => void onLoadMore()}
      />
    </Table.Root>
  </div>

  <CommunityProjectsRepositoryDocuments
    {currentPath}
    {previewDocuments}
    {onEditFile}
    {editUnavailable}
  />
</section>
