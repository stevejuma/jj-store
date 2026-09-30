<script lang="ts">
  import { resolveDiffLanguage } from "@lapismd/design-core/diff";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CodeBlock } from "@lapismd/design-core/shadcn/code-block";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { untrack } from "svelte";
  import type {
    RepositoryBlob,
    StoreController,
  } from "../../controller/contracts.js";
  import StoreMutationPreview from "../store-mutations/StoreMutationPreview.svelte";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import RepositoryFileCodeEditor from "./RepositoryFileCodeEditor.svelte";
  import { tokenizeRepositorySource } from "./repository-source-tokenizer.js";
  import "./store-file-editor.css";
  import "../editor-focus.css";

  let {
    controller,
    blob,
    filename = $bindable(""),
    contents = $bindable(""),
    isMarkdown,
    mode,
    onCancel,
  }: {
    controller: StoreController;
    blob: RepositoryBlob;
    filename?: string;
    contents?: string;
    isMarkdown: boolean;
    mode: "preview" | "source" | "edit";
    onCancel: () => void;
  } = $props();

  const view = $derived(controller.state);
  const mutation = $derived(view.mutation);
  const bookmark = $derived(
    view.bookmarks.find((candidate) => candidate.name === view.route.bookmark),
  );
  const language = $derived(
    resolveDiffLanguage(filename || blob.path) ?? "plaintext",
  );
  const dirty = $derived(
    filename !== blob.path || contents !== (blob.contents ?? ""),
  );
  let description = $state(untrack(() => `Edit ${blob.path}`));
  let previousFilename = $state(untrack(() => filename));
  let previousDescription = $state(untrack(() => description));
  let saveDialogOpen = $state(false);
  let dialogWasOpen = false;
  let validation = $state("");
  const pending = $derived(
    mutation.status === "previewing" || mutation.status === "applying",
  );

  $effect(() => {
    controller.actions.resetMutation();
  });

  $effect(() => {
    const current = filename;
    if (current === previousFilename) return;
    previousFilename = current;
    untrack(changed);
  });

  $effect(() => {
    const current = description;
    if (current === previousDescription) return;
    previousDescription = current;
    untrack(changed);
  });

  $effect(() => {
    const open = saveDialogOpen;
    if (open && !dialogWasOpen) {
      validation = "";
      controller.actions.resetMutation();
    } else if (!open && dialogWasOpen) {
      controller.actions.resetMutation();
    }
    dialogWasOpen = open;
  });

  function changed() {
    validation = "";
    if (mutation.status !== "idle") controller.actions.resetMutation();
  }

  function validateDraft(): boolean {
    validation = "";
    if (!filename.trim()) {
      validation = "Enter a repository file name.";
      return false;
    }
    if (new TextEncoder().encode(contents).length > 256 * 1024) {
      validation = "Edited text must be 256 KiB or smaller.";
      return false;
    }
    return true;
  }

  function requestSave() {
    if (!validateDraft()) return;
    saveDialogOpen = true;
  }

  async function preview() {
    if (!validateDraft()) return;
    const target = bookmark?.targets[0];
    if (!bookmark || bookmark.targets.length !== 1 || !target) {
      validation = "Choose a single-target local bookmark before editing.";
      return;
    }
    if (!description.trim()) {
      validation = "Enter a change description.";
      return;
    }
    await controller.actions.previewMutation({
      type: "edit_file",
      bookmark: bookmark.name,
      expected: [target],
      source: blob.path,
      destination: filename.trim(),
      contents,
      description: description.trim(),
    });
  }

  function cancel() {
    saveDialogOpen = false;
    controller.actions.resetMutation();
    onCancel();
  }
</script>

<section
  class="store-file-editor"
  data-mode={mode}
  aria-label={`Edit ${blob.path}`}
>
  {#if mode === "edit"}
    <RepositoryFileCodeEditor
      path={filename || blob.path}
      bind:value={contents}
      onChange={changed}
    />
  {:else if mode === "preview" && isMarkdown}
    <div class="store-file-editor__preview">
      <CommunityProjectsMarkdownPreview
        content={contents}
        label={`Edited Markdown preview for ${filename || blob.path}`}
      />
    </div>
  {:else}
    <CodeBlock
      code={contents}
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

  {#if validation && !saveDialogOpen}<p
      class="store-file-editor__error"
      role="alert"
    >
      {validation}
    </p>{/if}

  <footer class="store-file-editor__actions">
    <Button type="button" variant="outline" disabled={pending} onclick={cancel}
      >Cancel</Button
    >
    <Button type="button" disabled={!dirty || pending} onclick={requestSave}
      >Save changes</Button
    >
  </footer>
</section>

<Dialog.Root bind:open={saveDialogOpen}>
  <DialogFrame
    title="Save changes"
    description="Describe this file change before previewing and applying it."
    size="form"
    busy={pending}
    preventScroll={false}
  >
    {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
      <form
        id="store-file-save-form"
        onsubmit={(event) => {
          event.preventDefault();
          void preview();
        }}
      >
        <Field.Group>
          <Field.Field>
            <Field.Label>Change description</Field.Label>
            <CommunityChatComposer
              mode="form"
              showSubmitButton={false}
              bind:value={description}
              ariaLabel="Change description"
              placeholder="Describe this file change"
            />
          </Field.Field>
        </Field.Group>
      </form>
    {:else if mutation.result}
      <StoreMutationPreview result={mutation.result} />
      {#if mutation.status === "unknown"}
        <Alert.Root>
          <Alert.Title>Publication outcome unknown</Alert.Title>
          <Alert.Description>
            Keep this dialog open and check the durable request status. Retrying
            uses the same request ID.
          </Alert.Description>
        </Alert.Root>
      {:else if mutation.status === "committed"}
        <Alert.Root>
          <Alert.Title>Repository updated</Alert.Title>
          <Alert.Description>
            The committed receipt was selected and the displayed snapshot was
            refreshed.
          </Alert.Description>
        </Alert.Root>
      {:else if mutation.status === "failed"}
        <Alert.Root variant="destructive">
          <Alert.Title>File update failed</Alert.Title>
          <Alert.Description>{mutation.error}</Alert.Description>
        </Alert.Root>
      {/if}
    {/if}

    {#if validation || (mutation.status === "failed" && !mutation.result)}<p
        class="store-file-editor__error"
        role="alert"
      >
        {validation || mutation.error}
      </p>{/if}

    {#snippet footer()}
      {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
        <Dialog.Close>
          {#snippet child({ props })}<Button
              {...props}
              type="button"
              variant="outline">Cancel</Button
            >{/snippet}
        </Dialog.Close>
        <Button type="submit" form="store-file-save-form" disabled={pending}>
          {#if pending}<Spinner data-icon="inline-start" />{/if}Preview changes
        </Button>
      {:else if mutation.status === "prepared"}
        <Button
          type="button"
          variant="outline"
          onclick={controller.actions.resetMutation}>Back</Button
        >
        <Button type="button" onclick={controller.actions.applyMutation}
          >Apply changes</Button
        >
      {:else if mutation.status === "unknown"}
        <Button
          type="button"
          variant="outline"
          onclick={controller.actions.applyMutation}>Retry request</Button
        >
        <Button type="button" onclick={controller.actions.checkMutationStatus}
          >Check status</Button
        >
      {:else if mutation.status === "failed" && mutation.result}
        <Button
          type="button"
          variant="outline"
          onclick={controller.actions.resetMutation}>Back</Button
        >
        <Button type="button" onclick={controller.actions.applyMutation}
          >Retry request</Button
        >
      {:else if mutation.status === "committed"}
        <Dialog.Close>
          {#snippet child({ props })}<Button {...props} type="button"
              >Close</Button
            >{/snippet}
        </Dialog.Close>
      {:else}
        <Button type="button" disabled
          ><Spinner data-icon="inline-start" />Working</Button
        >
      {/if}
    {/snippet}
  </DialogFrame>
</Dialog.Root>
