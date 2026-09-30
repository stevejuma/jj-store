<script lang="ts">
  import {
    MergeEditor,
    type MergeResolvedChange,
  } from "@lapismd/design-core/diff";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import FileWarningIcon from "@lucide/svelte/icons/file-warning";
  import type { Model } from "../../api/client.js";
  import type { StoreConflictResolution } from "./model.js";
  import { repositoryEditorLanguage } from "../community-projects/repository-editor-language.js";

  let {
    conflict,
    resolution,
    actions,
    uploading = false,
    uploadError = "",
    onSelect,
    onStage,
  }: {
    conflict: Model<"ConflictDescriptor">;
    resolution?: StoreConflictResolution;
    actions: readonly string[];
    uploading?: boolean;
    uploadError?: string;
    onSelect: (resolution?: StoreConflictResolution) => void;
    onStage: (contents: string) => void | Promise<void>;
  } = $props();

  let merged = $state("");
  let initializedPath = "";
  $effect(() => {
    if (initializedPath === conflict.path || conflict.editor.status !== "text")
      return;
    initializedPath = conflict.path;
    merged = conflict.editor.contents[1];
  });
  function resolved(value: MergeResolvedChange) {
    merged = value.content;
  }
  function changed(contents: string) {
    if (conflict.editor.status !== "text") return;
    const side = [0, 2].find(
      (index) =>
        conflict.editor.status === "text" &&
        conflict.editor.contents[index] === contents,
    );
    const term = side === undefined ? undefined : conflict.terms[side];
    onSelect(
      term && actions.includes("select_term")
        ? { type: "select_term", term: term.id }
        : undefined,
    );
  }
  const language = $derived(repositoryEditorLanguage(conflict.path).id);
  const unsupported = $derived(
    conflict.editor.status === "unsupported"
      ? conflict.editor.reason.replaceAll("_", " ")
      : "",
  );
</script>

<section class="store-conflict-path" aria-label={`Conflict ${conflict.path}`}>
  {#if conflict.editor.status === "text"}
    <div class="store-conflict-path__editor">
      <MergeEditor
        path={conflict.path}
        {language}
        left={conflict.editor.contents[0]}
        base={conflict.editor.contents[1]}
        right={conflict.editor.contents[2]}
        baseLabel="Resolved"
        editable={actions.includes("uploaded_file")}
        readOnly={uploading ||
          (!actions.includes("uploaded_file") &&
            !actions.includes("select_term"))}
        editableSides={["base"]}
        onResolvedChange={resolved}
        onBaseChange={changed}
      />
    </div>
  {:else if conflict.editor.status === "unsupported"}
    <div class="store-conflict-path__unsupported">
      <Alert.Root>
        <FileWarningIcon aria-hidden="true" />
        <Alert.Title>Merge editor unavailable</Alert.Title>
        <Alert.Description>
          The native descriptor reports {unsupported}.
        </Alert.Description>
      </Alert.Root>
      {#if actions.includes("select_term")}
        <div
          class="store-conflict-path__terms"
          aria-label="Native conflict terms"
        >
          {#each conflict.terms as term (term.id)}
            <Button
              type="button"
              size="sm"
              variant={resolution?.type === "select_term" &&
              resolution.term === term.id
                ? "secondary"
                : "outline"}
              aria-pressed={resolution?.type === "select_term" &&
                resolution.term === term.id}
              onclick={() => onSelect({ type: "select_term", term: term.id })}
              >Use {term.label ?? `term ${term.id}`}</Button
            >
          {/each}
        </div>
      {/if}
    </div>
  {/if}

  {#if uploadError}<p class="store-conflict-path__error" role="alert">
      {uploadError}
    </p>{/if}
  <div class="store-conflict-path__controls">
    {#if actions.includes("delete")}
      <Button
        type="button"
        size="sm"
        disabled={uploading}
        variant={resolution?.type === "delete" ? "destructive" : "outline"}
        aria-pressed={resolution?.type === "delete"}
        onclick={() => onSelect({ type: "delete" })}>Delete path</Button
      >
    {/if}
    {#if resolution?.type === "uploaded_file"}<span role="status"
        >Merged file staged</span
      >{/if}
    {#if conflict.editor.status === "text" && actions.includes("uploaded_file")}
      <Button
        type="button"
        size="sm"
        class="store-conflict-path__stage"
        disabled={uploading}
        onclick={() => onStage(merged)}
        >{#if uploading}<Spinner data-icon="inline-start" />{/if}Stage merged
        file</Button
      >
    {/if}
  </div>
</section>
