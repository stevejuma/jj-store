<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import CodeXmlIcon from "@lucide/svelte/icons/code-xml";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import { untrack } from "svelte";
  import type {
    RepositoryBlob,
    StoreController,
  } from "../../controller/contracts.js";
  import StoreMutationPreview from "../store-mutations/StoreMutationPreview.svelte";
  import CommunityProjectsMarkdownPreview from "./CommunityProjectsMarkdownPreview.svelte";
  import "./store-file-editor.css";

  let {
    controller,
    blob,
    filename = $bindable(""),
    isMarkdown,
    onCancel,
  }: {
    controller: StoreController;
    blob: RepositoryBlob;
    filename?: string;
    isMarkdown: boolean;
    onCancel: () => void;
  } = $props();

  const view = $derived(controller.state);
  const mutation = $derived(view.mutation);
  const bookmark = $derived(
    view.bookmarks.find((candidate) => candidate.name === view.route.bookmark),
  );
  let contents = $state(untrack(() => blob.contents ?? ""));
  let description = $state(untrack(() => `Edit ${blob.path}`));
  let previousFilename = $state(untrack(() => filename));
  let previousDescription = $state(untrack(() => description));
  let mode = $state<"edit" | "preview">("edit");
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

  function changed() {
    validation = "";
    if (mutation.status !== "idle") controller.actions.resetMutation();
  }

  async function preview() {
    validation = "";
    const destination = filename.trim();
    const target = bookmark?.targets[0];
    if (!destination) {
      validation = "Enter a repository file name.";
      return;
    }
    if (!bookmark || bookmark.targets.length !== 1 || !target) {
      validation = "Choose a single-target local bookmark before editing.";
      return;
    }
    if (new TextEncoder().encode(contents).length > 256 * 1024) {
      validation = "Edited text must be 256 KiB or smaller.";
      return;
    }
    await controller.actions.previewMutation({
      type: "edit_file",
      bookmark: bookmark.name,
      expected: [target],
      source: blob.path,
      destination,
      contents,
      description,
    });
  }

  function cancel() {
    controller.actions.resetMutation();
    onCancel();
  }
</script>

<section class="store-file-editor" aria-label={`Edit ${blob.path}`}>
  <header class="store-file-editor__header">
    <strong>Edit file</strong>
    {#if isMarkdown}
      <ButtonGroup.Root aria-label="Editor display">
        <Button
          type="button"
          variant="outline"
          size="sm"
          data-selected={mode === "edit"}
          aria-pressed={mode === "edit"}
          onclick={() => (mode = "edit")}
        >
          <CodeXmlIcon data-icon="inline-start" aria-hidden="true" />Edit
        </Button>
        <Button
          type="button"
          variant="outline"
          size="sm"
          data-selected={mode === "preview"}
          aria-pressed={mode === "preview"}
          onclick={() => (mode = "preview")}
        >
          <EyeIcon data-icon="inline-start" aria-hidden="true" />Preview
        </Button>
      </ButtonGroup.Root>
    {/if}
  </header>

  <Field.Group>
    <Field.Field>
      <Field.Label for="store-file-editor-contents">Contents</Field.Label>
      {#if isMarkdown && mode === "preview"}
        <div class="store-file-editor__preview">
          <CommunityProjectsMarkdownPreview
            content={contents}
            label={`Edited Markdown preview for ${filename || blob.path}`}
          />
        </div>
      {:else}
        <Textarea
          id="store-file-editor-contents"
          class="store-file-editor__contents"
          bind:value={contents}
          oninput={changed}
          spellcheck="false"
        />
      {/if}
    </Field.Field>
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

  {#if mutation.result}
    <StoreMutationPreview result={mutation.result} />
  {/if}
  {#if mutation.status === "unknown"}
    <Alert.Root>
      <Alert.Title>Publication outcome unknown</Alert.Title>
      <Alert.Description>
        Keep this editor open and check the durable request status. Retrying
        uses the same request ID.
      </Alert.Description>
    </Alert.Root>
  {:else if mutation.status === "failed"}
    <Alert.Root variant="destructive">
      <Alert.Title>File update failed</Alert.Title>
      <Alert.Description>{mutation.error}</Alert.Description>
    </Alert.Root>
  {/if}
  {#if validation}<p class="store-file-editor__error" role="alert">
      {validation}
    </p>{/if}

  <footer class="store-file-editor__actions">
    <Button type="button" variant="outline" disabled={pending} onclick={cancel}
      >Cancel</Button
    >
    {#if mutation.status === "prepared"}
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
    {:else if mutation.status !== "committed"}
      <Button type="button" disabled={pending} onclick={() => void preview()}>
        {#if pending}<Spinner data-icon="inline-start" />{/if}Preview changes
      </Button>
    {/if}
  </footer>
</section>
