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
    onSelect: (resolution: StoreConflictResolution) => void;
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
  function termDescription(term: Model<"ConflictTerm">) {
    if (!term.value) return "absent";
    if (term.value.type === "file")
      return `file ${term.value.id.slice(0, 12)}${term.value.executable ? " · executable" : ""}`;
    if (term.value.type === "submodule") return "submodule";
    return `${term.value.type} ${term.value.id.slice(0, 12)}`;
  }
  const unsupported = $derived(
    conflict.editor.status === "unsupported"
      ? conflict.editor.reason.replaceAll("_", " ")
      : "",
  );
</script>

<section class="store-conflict-path" aria-label={`Conflict ${conflict.path}`}>
  <header class="store-conflict-path__header">
    <div>
      <h3>{conflict.path}</h3>
      <p>{conflict.terms.length} native terms</p>
    </div>
    <code title={conflict.fingerprint}>{conflict.fingerprint}</code>
  </header>

  <div class="store-conflict-path__terms" aria-label="Native conflict terms">
    {#each conflict.terms as term (term.id)}
      <article class="store-conflict-path__term">
        <div>
          <strong>{term.label ?? `${term.role} term ${term.id}`}</strong>
          <span>Term {term.id} · {term.role} · {termDescription(term)}</span>
        </div>
        {#if actions.includes("select_term")}
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
        {/if}
      </article>
    {/each}
  </div>

  {#if conflict.editor.status === "text" && actions.includes("uploaded_file")}
    <div class="store-conflict-path__editor">
      <MergeEditor
        path={conflict.path}
        left={conflict.editor.contents[0]}
        base={conflict.editor.contents[1]}
        right={conflict.editor.contents[2]}
        leftLabel={conflict.terms[0]?.label ?? "Left"}
        baseLabel={conflict.terms[1]?.label ?? "Resolved"}
        rightLabel={conflict.terms[2]?.label ?? "Right"}
        editable
        editableSides={["base"]}
        onResolvedChange={resolved}
      />
      <div class="store-conflict-path__stage">
        <span>Stage the resolved center as a snapshot-bound upload.</span>
        <Button
          type="button"
          size="sm"
          disabled={uploading}
          onclick={() => onStage(merged)}
          >{#if uploading}<Spinner data-icon="inline-start" />{/if}Stage merged
          file</Button
        >
      </div>
    </div>
  {:else if conflict.editor.status === "unsupported"}
    <Alert.Root>
      <FileWarningIcon aria-hidden="true" />
      <Alert.Title>Merge editor unavailable</Alert.Title>
      <Alert.Description>
        The native descriptor reports {unsupported}. Select an exact term or
        delete the path if the server advertises those actions.
      </Alert.Description>
    </Alert.Root>
  {/if}

  {#if uploadError}<p class="store-conflict-path__error" role="alert">
      {uploadError}
    </p>{/if}
  <div class="store-conflict-path__delete">
    {#if actions.includes("delete")}
      <Button
        type="button"
        variant={resolution?.type === "delete" ? "destructive" : "outline"}
        aria-pressed={resolution?.type === "delete"}
        onclick={() => onSelect({ type: "delete" })}>Delete path</Button
      >
    {/if}
    {#if resolution?.type === "uploaded_file"}<span role="status"
        >Merged file staged · {resolution.upload.slice(0, 12)}</span
      >{/if}
  </div>
</section>
