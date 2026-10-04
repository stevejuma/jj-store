<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { onDestroy, untrack } from "svelte";
  import { ApiError } from "../../api/client.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import { resolveDiffLanguage } from "@lapismd/design-core/diff";
  import SaveIcon from "@lucide/svelte/icons/save";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import XIcon from "@lucide/svelte/icons/x";
  import type { StoreController } from "../../controller/contracts.js";
  import type { WorkingFile } from "../../controller/working-copy-types.js";
  import RepositoryFileCodeEditor from "../community-projects/RepositoryFileCodeEditor.svelte";
  import CommunityProjectsMarkdownPreview from "../community-projects/CommunityProjectsMarkdownPreview.svelte";
  import "../community-projects/store-file-editor.css";
  let {
    controller,
    file,
    mode,
    isMarkdown,
    onCancel,
  }: {
    controller: StoreController;
    file: WorkingFile;
    mode: "preview" | "source" | "edit";
    isMarkdown: boolean;
    onCancel(): void;
  } = $props();
  let selected = $state(untrack(() => file));
  let contents = $state(untrack(() => file.contents));
  let pending = $state(false);
  let unknown = $state(false);
  let message = $state("");
  let error = $state("");
  let active = true;
  onDestroy(() => {
    active = false;
  });
  const dirty = $derived(contents !== selected.contents);
  const language = $derived(
    resolveDiffLanguage(file.selection.path) ?? "plaintext",
  );
  async function save() {
    if (pending || unknown || !dirty) return;
    if (new TextEncoder().encode(contents).length > 256 * 1024) {
      error = "Edited text must be 256 KiB or smaller.";
      return;
    }
    pending = true;
    message = "";
    error = "";
    const draft = contents;
    try {
      const saved = await controller.actions.saveWorkingFile!(
        selected.selection,
        draft,
      );
      if (!active) return;
      selected = saved;
      message = "Saved to the working file. Snapshot is a separate action.";
    } catch (cause) {
      if (!active) return;
      if (
        cause instanceof ApiError &&
        [
          "stale_working_file",
          "invalid_request",
          "unsupported_working_file",
          "limit_exceeded",
        ].includes(cause.code)
      ) {
        error =
          cause.code === "stale_working_file"
            ? "The working file changed. Your draft is retained; check the current file before saving again."
            : "The file could not be saved. Your draft is retained.";
      } else {
        unknown = true;
        error =
          "Save outcome is unknown. Your draft is retained; check the current file.";
      }
    } finally {
      if (active) pending = false;
    }
  }
  async function check() {
    if (pending) return;
    pending = true;
    try {
      const current = await controller.actions.openWorkingFile!(
        selected.selection.path,
      );
      if (!active) return;
      if (current.contents === contents) {
        selected = current;
        unknown = false;
        error = "";
        message = "The working file matches your draft.";
      } else {
        error =
          "The working file differs from your draft. Cancel and reopen Working copy to load it; your draft is retained here.";
      }
    } catch {
      if (active)
        error = "Could not read the working file. Your draft is retained.";
    } finally {
      if (active) pending = false;
    }
  }
</script>

<section
  class="store-file-editor"
  aria-label={`Working copy ${file.selection.path}`}
>
  <p class="community-projects__file-panel-stats">
    Working copy · Physical file
  </p>
  {#if mode === "edit"}
    <RepositoryFileCodeEditor
      path={file.selection.path}
      bind:value={contents}
    />
  {:else if mode === "preview" && isMarkdown}
    <CommunityProjectsMarkdownPreview
      content={contents}
      label={`Working copy preview for ${file.selection.path}`}
    />
  {:else}
    <CodeBlock
      code={contents}
      {language}
      hasLineNumbers
      hasCopyButton={false}
      width="100%"
      container="section"
    />
  {/if}
  {#if error}<p class="store-file-editor__error" role="alert">{error}</p>{/if}
  <StoreNotice
    noticeKey={`working-file:${file.selection.path}`}
    title="Working file saved"
    {message}
    severity="info"
    lifetime="timed"
  />
  <footer class="store-file-editor__actions">
    <Button
      type="button"
      variant="outline"
      disabled={pending}
      onclick={onCancel}><XIcon aria-hidden="true" />Cancel</Button
    >
    {#if error || unknown}<Button
        type="button"
        variant="outline"
        disabled={pending}
        onclick={check}
        ><RefreshCwIcon aria-hidden="true" />Check working file</Button
      >{/if}
    <Button type="button" disabled={pending || unknown || !dirty} onclick={save}
      ><SaveIcon aria-hidden="true" />Save changes</Button
    >
  </footer>
</section>
