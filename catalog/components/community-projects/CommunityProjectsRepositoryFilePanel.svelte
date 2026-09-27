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

  import type { RepositoryBlob as ProjectForgeBlobResponse } from "../../controller/contracts.js";
  import type { RepositoryLogRevision as ProjectForgeCommit } from "../../projects/repository-log.js";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import { tokenizeRepositorySource } from "./repository-source-tokenizer.js";

  let {
    loading = false,
    selectedPath = "",
    blob,
    latestCommit,
    now,
    isMarkdown,
  }: {
    loading?: boolean | undefined;
    selectedPath?: string | undefined;
    blob?: ProjectForgeBlobResponse | undefined;
    latestCommit?: ProjectForgeCommit | undefined;
    now?: number | undefined;
    isMarkdown: boolean;
  } = $props();

  let markdownView = $state<"preview" | "source">("preview");
  let markdownPath = $state("");
  const language = $derived(
    blob ? (resolveDiffLanguage(blob.path) ?? "plaintext") : "plaintext",
  );
  const fileStats = $derived(blob ? describeFileStats(blob) : null);

  $effect(() => {
    const nextPath = isMarkdown ? selectedPath : "";
    if (nextPath !== markdownPath) {
      markdownPath = nextPath;
      markdownView = "preview";
    }
  });

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
        {#if latestCommit}
          <section
            class="community-projects__file-panel-info"
            aria-label="Selected file metadata"
          >
            <CommunityProjectsGitAuthorAvatar
              email={latestCommit.authorEmail}
              name={latestCommit.author}
              size="small"
            />
            <span class="community-projects__file-panel-info-message">
              <strong>{latestCommit.author}</strong> committed
              <span>{latestCommit.message}</span>
            </span>
            <CommunityProjectsRevisionGraphIds
              commitId={latestCommit.eventId}
              changeId={latestCommit.changeId}
              placement="inline"
            />
            {#if latestCommit.createdAt}
              <CommunityRelativeTime timestamp={latestCommit.createdAt} {now} />
            {/if}
          </section>
        {/if}
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
          class={isMarkdown
            ? "community-projects__file-markdown-card"
            : "community-projects__file-code-card"}
          aria-label={`Source preview for ${blob.path}`}
        >
          {#if blob.binary}
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
          {:else if blob.too_large}
            <p class="community-projects__status">
              {blob.path} is too large to preview.
            </p>
          {:else if isMarkdown}
            <header class="community-projects__file-markdown-header">
              <span>{blob.path}</span>
              <ButtonGroup.Root
                class="community-projects__file-view-toggle"
                aria-label="Markdown display"
              >
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  data-selected={markdownView === "preview"}
                  aria-pressed={markdownView === "preview"}
                  onclick={() => (markdownView = "preview")}
                >
                  <EyeIcon data-icon="inline-start" aria-hidden="true" />
                  Preview
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  data-selected={markdownView === "source"}
                  aria-pressed={markdownView === "source"}
                  onclick={() => (markdownView = "source")}
                >
                  <CodeXmlIcon data-icon="inline-start" aria-hidden="true" />
                  Source
                </Button>
              </ButtonGroup.Root>
            </header>
            {#if markdownView === "preview"}
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
                tokenizer={tokenizeRepositorySource}
                highlightMode={language === "rust" ? "spans" : "auto"}
              />
            {/if}
          {:else}
            <CodeBlock
              code={blob.contents ?? ""}
              {language}
              title={blob.path}
              hasLanguageLabel
              hasCopyButton={false}
              hasLineNumbers
              width="100%"
              container="card"
              tokenizer={tokenizeRepositorySource}
              highlightMode={language === "rust" ? "spans" : "auto"}
            />
          {/if}
        </section>
      {:else}
        <p class="community-projects__status">Select a file to preview it.</p>
      {/if}
    </div>
  </ScrollArea>
</section>
