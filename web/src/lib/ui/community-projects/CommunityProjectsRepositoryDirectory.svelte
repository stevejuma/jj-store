<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Table from "@lapismd/design-core/shadcn/table";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import FileIcon from "@lucide/svelte/icons/file";
  import FolderIcon from "@lucide/svelte/icons/folder";

  import type {
    RepositoryBlob,
    RepositoryChoice,
  } from "../../controller/contracts.js";
  import type { RepositoryLogRevision as ProjectForgeCommit } from "../../projects/repository-log.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { RepositoryBrowserEntry } from "./repository-browser.js";
  import {
    repositoryLandingDocumentLabel,
    repositoryParentPath,
  } from "./repository-browser.js";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import CommunityProjectsRepositoryBranchSelector from "./CommunityProjectsRepositoryBranchSelector.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";

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
    onSelectCommit,
    onSelectBranch,
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
    onSelectCommit?: ((commit: ProjectForgeCommit) => void) | undefined;
    onSelectBranch: (value: string | undefined) => void;
  } = $props();

  const latestCommitDate = $derived(latestCommit?.createdAt);
  const latestAuthorName = $derived(
    latestCommitAuthor?.name ?? latestCommit?.author ?? "",
  );
  const landingDocuments = $derived(
    currentPath
      ? []
      : previewDocuments
          .map((document) => ({
            ...document,
            label: repositoryLandingDocumentLabel(document.path),
          }))
          .filter(
            (
              document,
            ): document is RepositoryBlob & {
              label: string;
              contents: string;
            } =>
              Boolean(document.label) &&
              !document.binary &&
              !document.too_large &&
              document.contents !== undefined,
          ),
  );
  let selectedLandingDocument = $state("");
  $effect(() => {
    const first = landingDocuments[0]?.path ?? "";
    if (
      first &&
      !landingDocuments.some(
        (document) => document.path === selectedLandingDocument,
      )
    ) {
      selectedLandingDocument = first;
    } else if (!first) {
      selectedLandingDocument = "";
    }
  });
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
            placement="inline"
          />
          {#if latestCommitDate}
            <CommunityRelativeTime timestamp={latestCommitDate} {now} />
          {/if}
        {/if}
        <CommunityProjectsRepositoryBranchSelector
          {branch}
          {branches}
          {onSelectBranch}
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
      <Table.Body>
        {#if currentPath}
          <Table.Row>
            <Table.Cell>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                class="community-projects__file-entry"
                aria-label="Go to parent directory"
                onclick={() =>
                  onOpenDirectory(repositoryParentPath(currentPath))}
              >
                <FolderIcon data-icon="inline-start" aria-hidden="true" />
                <strong>..</strong>
              </Button>
            </Table.Cell>
            <Table.Cell><span aria-hidden="true">—</span></Table.Cell>
            <Table.Cell><span aria-hidden="true">—</span></Table.Cell>
          </Table.Row>
        {/if}
        {#each entries as entry (entry.path)}
          {@const entryCommit = pathCommits[entry.path]}
          {@const entryCommitDate = entryCommit?.createdAt}
          {@const commitLoaded = Object.prototype.hasOwnProperty.call(
            pathCommits,
            entry.path,
          )}
          <Table.Row>
            <Table.Cell>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                class="community-projects__file-entry"
                aria-label={`${entry.type === "tree" ? "Open directory" : "Open file"} ${entry.name}`}
                onclick={() =>
                  entry.type === "tree"
                    ? onOpenDirectory(entry.path)
                    : onOpenFile(entry.path)}
              >
                {#if entry.type === "tree"}
                  <FolderIcon data-icon="inline-start" aria-hidden="true" />
                {:else}
                  <FileIcon data-icon="inline-start" aria-hidden="true" />
                {/if}
                <strong>{entry.name}</strong>
              </Button>
            </Table.Cell>
            <Table.Cell>
              <span
                class="community-projects__file-commit-message"
                title={entryCommit?.message}
              >
                {entryCommit?.message ??
                  (canLoadPathCommit && !commitLoaded ? "Loading…" : "—")}
              </span>
            </Table.Cell>
            <Table.Cell>
              {#if entryCommitDate}
                <CommunityRelativeTime timestamp={entryCommitDate} {now} />
              {:else}
                <span aria-hidden="true">—</span>
              {/if}
            </Table.Cell>
          </Table.Row>
        {:else}
          <Table.Row>
            <Table.Cell colspan={3}>
              <p class="community-projects__status">
                This directory does not contain any files.
              </p>
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>

  {#if landingDocuments.length > 0}
    <section
      class="community-projects__repository-document-card"
      aria-label="Repository document preview"
    >
      <Tabs.Root
        bind:value={selectedLandingDocument}
        class="community-projects__repository-document-tabs"
      >
        <Tabs.List variant="line" aria-label="Repository documents">
          {#each landingDocuments as document (document.path)}
            <Tabs.Trigger value={document.path}>
              {#if document.label === "README"}
                <BookOpenIcon data-icon="inline-start" aria-hidden="true" />
              {:else}
                <FileIcon data-icon="inline-start" aria-hidden="true" />
              {/if}
              {document.label}
            </Tabs.Trigger>
          {/each}
        </Tabs.List>
        {#each landingDocuments as document (document.path)}
          <Tabs.Content value={document.path}>
            <CommunityProjectsMarkdownPreview
              content={document.contents}
              label={`${document.label} preview`}
            />
          </Tabs.Content>
        {/each}
      </Tabs.Root>
    </section>
  {/if}
</section>
