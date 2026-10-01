<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    type FileDiffComposerFileContext,
    type FileDiffLineContext,
    type FileDiffViewMode,
  } from "@lapismd/design-core/diff";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageSquarePlusIcon from "@lucide/svelte/icons/message-square-plus";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import "../editor-focus.css";
  import { reviewPatchMatchesLine } from "../../collaboration/presentation.js";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityCommitDiff from "../community-commit-diff/CommunityCommitDiff.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";

  let {
    controller,
    path,
    patch,
    viewMode,
    wrap,
    single = false,
    fileHeaderTrailing,
  }: {
    controller?: StoreController;
    path: string;
    patch: string;
    viewMode: FileDiffViewMode;
    wrap: boolean;
    single?: boolean;
    fileHeaderTrailing?: Snippet<[FileDiffComposerFileContext]>;
  } = $props();

  let draftLine = $state<FileDiffLineContext>();
  let draft = $state("");

  function sameLine(line: FileDiffLineContext) {
    return (
      draftLine?.path === line.path &&
      draftLine.lineNumber === line.lineNumber &&
      draftLine.variant === line.variant
    );
  }

  async function submit(line: FileDiffLineContext, content: string) {
    if (!controller) return;
    await controller.actions.commentOnDiffLine(line, content);
    if (controller.state.status.reviewAction === "ready") {
      draft = "";
      draftLine = undefined;
    }
  }
</script>

{#snippet lineAccessory(line: FileDiffLineContext)}
  {#if controller && controller.state.repository.profile !== "git_sha1_jj045" && line.lineNumber !== null}
    <Button
      type="button"
      variant="secondary"
      size="icon-xs"
      aria-label={`Comment on ${line.path} line ${line.lineNumber}`}
      title="Add comment"
      onclick={() => {
        draftLine = line;
        draft = "";
      }}
    >
      <MessageSquarePlusIcon aria-hidden="true" />
    </Button>
  {/if}
{/snippet}

{#snippet lineAnnotation(line: FileDiffLineContext)}
  {#if controller && reviewPatchMatchesLine(controller.state.reviewPatch, line)}
    <div class="store-inline-diff-review" data-ui-part="diff-line-review">
      <StoreReviewConversation
        {controller}
        inline
        showComposer={false}
        showDiff={false}
      />
    </div>
  {/if}
  {#if controller && sameLine(line)}
    <div class="store-inline-diff-composer" data-ui-part="diff-line-composer">
      <CommunityChatComposer
        mode="form"
        showSubmitButton
        bind:value={draft}
        disabled={controller.state.status.reviewAction === "refreshing"}
        busy={controller.state.status.reviewAction === "refreshing"}
        ariaLabel={`Add a comment to ${line.path} line ${line.lineNumber}`}
        submitLabel="Comment"
        placeholder="Add a review comment"
        onCancel={() => {
          draft = "";
          draftLine = undefined;
        }}
        onSubmit={(submission) => submit(line, submission.content)}
      />
      {#if controller.state.status.reviewAction === "failed" || controller.state.status.reviewAction === "unavailable"}
        <p role="alert">{controller.state.reviewActionMessage}</p>
      {/if}
    </div>
  {/if}
{/snippet}

<CommunityCommitDiff
  {single}
  files={[{ path, patch }]}
  {viewMode}
  {wrap}
  {fileHeaderTrailing}
  {lineAccessory}
  {lineAnnotation}
/>

<style>
  .store-inline-diff-review,
  .store-inline-diff-composer {
    min-width: 0;
    padding: 0.5rem 0.75rem 0.6rem 2.6rem;
    background: var(--background);
  }

  .store-inline-diff-composer > p {
    margin: 0.4rem 0 0;
    color: var(--destructive);
    font-size: var(--community-text-secondary);
  }
</style>
