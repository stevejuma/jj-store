<script lang="ts">
  import StoreWorkingFileEditor from "../store-working-copy/StoreWorkingFileEditor.svelte";
  import { createWorkingFileEdit } from "../store-working-copy/working-copy-editor.svelte.js";
  import {
    describeFileStats,
    fileEditUnavailable,
  } from "../../projects/repository-file-stats.js";
  import { resolveDiffLanguage } from "@lapismd/design-core/diff";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import CodeXmlIcon from "@lucide/svelte/icons/code-xml";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import FileIcon from "@lucide/svelte/icons/file";
  import PencilIcon from "@lucide/svelte/icons/pencil";

  import type {
    RepositoryBlob as ProjectForgeBlobResponse,
    StoreController,
  } from "../../controller/contracts.js";
  import type { RepositoryLogRevision as ProjectForgeCommit } from "../../projects/repository-log.js";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import CommunityProjectsRepositoryFilePanelHeader from "./CommunityProjectsRepositoryFilePanelHeader.svelte";
  import { tokenizeRepositorySource } from "./repository-source-tokenizer.js";
  import StoreFileEditor from "./StoreFileEditor.svelte";

  let {
    loading = false,
    selectedPath = "",
    editing = $bindable(false),
    filename = $bindable(""),
    blob,
    latestCommit,
    now,
    isMarkdown,
    controller,
    fileTreeOpen,
    onToggleFileTree,
  }: {
    loading?: boolean | undefined;
    selectedPath?: string | undefined;
    editing?: boolean;
    filename?: string;
    blob?: ProjectForgeBlobResponse | undefined;
    latestCommit?: ProjectForgeCommit | undefined;
    now?: number | undefined;
    isMarkdown: boolean;
    controller: StoreController;
    fileTreeOpen: boolean;
    onToggleFileTree: () => void;
  } = $props();

  const working = createWorkingFileEdit(
    () => controller,
    () => blob?.path,
    () => {
      editing = true;
      fileView = "edit";
    },
  );
  const localEditing = $derived(
    controller.state.repository.profile === "git_sha1_jj045",
  );
  let fileView = $state<"preview" | "source" | "edit">("preview");
  let displayedPath = $state("");
  let editSessionPath = $state("");
  let draftContents = $state("");
  const language = $derived(
    blob ? (resolveDiffLanguage(blob.path) ?? "plaintext") : "plaintext",
  );
  const fileStats = $derived(blob ? describeFileStats(blob) : null);
  const editUnavailable = $derived(fileEditUnavailable(controller.state, blob));

  $effect(() => {
    if (selectedPath !== displayedPath) {
      displayedPath = selectedPath;
      working.clear();
      if (!editing) fileView = isMarkdown ? "preview" : "source";
    }
  });

  $effect(() => {
    if (!editing) {
      if (editSessionPath) {
        editSessionPath = "";
        fileView = isMarkdown ? "preview" : "source";
      }
      return;
    }
    if (blob && editSessionPath !== blob.path) {
      editSessionPath = blob.path;
      filename = blob.path;
      draftContents = blob.contents ?? "";
      fileView = "edit";
    }
  });

  function beginEditing() {
    if (!blob || editUnavailable) return;
    filename = blob.path;
    draftContents = blob.contents ?? "";
    editSessionPath = blob.path;
    editing = true;
    fileView = "edit";
  }

  function cancelEditing() {
    filename = blob?.path ?? selectedPath;
    draftContents = blob?.contents ?? "";
    editSessionPath = "";
    working.clear();
    editing = false;
    fileView = isMarkdown ? "preview" : "source";
  }
</script>

<section class="community-projects__file-panel">
  <ScrollArea
    type="hover"
    orientation="both"
    class="community-projects__file-body"
  >
    <div class="community-projects__file-panel-stack">
      {#if loading && !blob}
        <div
          class="community-projects__blob-skeleton"
          role="status"
          aria-label={`Loading ${selectedPath}`}
        >
          {#if isMarkdown}
            <Skeleton class="community-projects__markdown-skeleton-title" />
            <Skeleton class="community-projects__markdown-skeleton-copy" />
            <Skeleton class="community-projects__markdown-skeleton-copy" />
            <Skeleton class="community-projects__markdown-skeleton-heading" />
            <Skeleton class="community-projects__markdown-skeleton-copy" />
            <Skeleton class="community-projects__markdown-skeleton-code" />
          {:else}
            <div class="community-projects__files-loading-code">
              {#each [0, 1, 2, 3, 4, 5, 6] as row (row)}
                <div>
                  <Skeleton />
                  <Skeleton />
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {:else if blob}
        <CommunityProjectsRepositoryFilePanelHeader
          {blob}
          {latestCommit}
          {now}
          {fileTreeOpen}
          {onToggleFileTree}
        />
        {#if fileStats}
          <p
            class="community-projects__file-panel-stats"
            role="status"
            aria-label="File statistics"
          >
            {fileStats}
          </p>
        {/if}
        <section
          class="community-projects__file-content-card"
          aria-label={`Source preview for ${blob.path}`}
        >
          {#if blob.binary}
            <div class="community-projects__file-content-unavailable">
              <Empty.Root>
                <Empty.Header>
                  <Empty.Media variant="icon"
                    ><FileIcon aria-hidden="true" /></Empty.Media
                  >
                  <Empty.Title>Binary file</Empty.Title>
                  <Empty.Description
                    >{blob.path} cannot be previewed as text.</Empty.Description
                  >
                </Empty.Header>
              </Empty.Root>
            </div>
          {:else if blob.too_large}
            <div class="community-projects__file-content-unavailable">
              <p class="community-projects__status">
                {blob.path} is too large to preview.
              </p>
            </div>
          {:else if blob.contents !== undefined}
            <header class="community-projects__file-content-header">
              <span>{blob.path}</span>
              <ButtonGroup.Root
                class="community-projects__file-view-toggle"
                aria-label="File display"
              >
                {#if isMarkdown}<Button
                    type="button"
                    variant="outline"
                    size="sm"
                    data-selected={fileView === "preview"}
                    aria-pressed={fileView === "preview"}
                    onclick={() => (fileView = "preview")}
                  >
                    <EyeIcon data-icon="inline-start" aria-hidden="true" />
                    Preview
                  </Button>{/if}
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  data-selected={fileView === "source"}
                  aria-pressed={fileView === "source"}
                  onclick={() => (fileView = "source")}
                >
                  <CodeXmlIcon data-icon="inline-start" aria-hidden="true" />
                  Source
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  data-selected={fileView === "edit"}
                  aria-pressed={fileView === "edit"}
                  title={localEditing
                    ? "Edit the physical working file"
                    : editUnavailable || "Edit file"}
                  disabled={localEditing
                    ? !working.available() || working.pending
                    : Boolean(editUnavailable)}
                  onclick={() => {
                    if (editing) fileView = "edit";
                    else if (localEditing) void working.open();
                    else beginEditing();
                  }}
                >
                  <PencilIcon data-icon="inline-start" aria-hidden="true" />
                  {localEditing ? "Working copy" : "Edit"}
                </Button>
              </ButtonGroup.Root>
            </header>
            {#if working.error}<p role="alert">{working.error}</p>{/if}
            {#if editing && working.file}
              {#key working.file}
                <StoreWorkingFileEditor
                  {controller}
                  file={working.file}
                  mode={fileView}
                  {isMarkdown}
                  onCancel={cancelEditing}
                />
              {/key}
            {:else if editing}
              <StoreFileEditor
                {controller}
                {blob}
                {isMarkdown}
                mode={fileView}
                bind:filename
                bind:contents={draftContents}
                onCancel={cancelEditing}
              />
            {:else if fileView === "preview" && isMarkdown}
              <CommunityProjectsMarkdownPreview
                content={blob.contents ?? ""}
                label={`Markdown preview for ${blob.path}`}
              />
            {:else}
              <CodeBlock
                code={blob.contents ?? ""}
                {language}
                hasLanguageLabel
                hasCopyButton={false}
                hasLineNumbers
                width="100%"
                container="section"
                tokenizer={language === "rust"
                  ? tokenizeRepositorySource
                  : undefined}
                highlightMode="spans"
              />
            {/if}
          {/if}
        </section>
      {:else}
        <p class="community-projects__status">Select a file to preview it.</p>
      {/if}
    </div>
  </ScrollArea>
</section>
