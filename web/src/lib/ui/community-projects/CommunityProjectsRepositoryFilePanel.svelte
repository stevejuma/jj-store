<script lang="ts">
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

  let fileView = $state<"preview" | "source" | "edit">("preview");
  let displayedPath = $state("");
  let editSessionPath = $state("");
  let draftContents = $state("");
  const language = $derived(
    blob ? (resolveDiffLanguage(blob.path) ?? "plaintext") : "plaintext",
  );
  const fileStats = $derived(blob ? describeFileStats(blob) : null);
  const selectedBookmark = $derived(
    controller.state.bookmarks.find(
      (candidate) => candidate.name === controller.state.route.bookmark,
    ),
  );
  const editUnavailable = $derived.by(() => {
    const view = controller.state;
    if (!view.authenticated)
      return "Connect with repository write access to edit this file.";
    if (!view.mutation.available.includes("edit_file"))
      return "This repository server does not support file editing.";
    if (!view.route.bookmark || !selectedBookmark)
      return "Choose a local bookmark to edit this file.";
    if (
      selectedBookmark.targets.length !== 1 ||
      view.targets.length !== 1 ||
      selectedBookmark.targets[0] !== view.targets[0]
    )
      return "Choose the bookmark's single current target to edit this file.";
    if (!blob || blob.binary || blob.too_large || blob.contents === undefined)
      return "Only bounded UTF-8 text files can be edited.";
    return "";
  });

  $effect(() => {
    if (selectedPath !== displayedPath) {
      displayedPath = selectedPath;
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
    editing = false;
    fileView = isMarkdown ? "preview" : "source";
  }

  function describeFileStats(blob: ProjectForgeBlobResponse): string {
    if (blob.binary) {
      return "Binary file · Size unavailable";
    }
    const contents = blob.contents ?? "";
    const normalized = contents.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
    const contentWithoutTerminalNewline = normalized.endsWith("\n")
      ? normalized.slice(0, -1)
      : normalized;
    const lines = contentWithoutTerminalNewline
      ? contentWithoutTerminalNewline.split("\n")
      : [];
    const lineCount = lines.length;
    const locCount = lines.filter((line) => line.trim().length > 0).length;
    const byteCount = new TextEncoder().encode(contents).length;
    const lineLabel = lineCount === 1 ? "line" : "lines";
    const byteLabel = byteCount === 1 ? "Byte" : "Bytes";
    return `${lineCount} ${lineLabel} (${locCount} loc) · ${byteCount} ${byteLabel}`;
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
                  title={editUnavailable || "Edit file"}
                  disabled={Boolean(editUnavailable)}
                  onclick={() => {
                    if (editing) fileView = "edit";
                    else beginEditing();
                  }}
                >
                  <PencilIcon data-icon="inline-start" aria-hidden="true" />
                  Edit
                </Button>
              </ButtonGroup.Root>
            </header>
            {#if editing}
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
