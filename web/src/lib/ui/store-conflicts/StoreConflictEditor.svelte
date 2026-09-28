<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import {
    ConfirmButton,
    DialogFrame,
  } from "@lapismd/design-core/forms/dialog";
  import CheckIcon from "@lucide/svelte/icons/check";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMutationPreview from "../store-mutations/StoreMutationPreview.svelte";
  import StoreConflictPath from "./StoreConflictPath.svelte";
  import {
    buildConflictCommand,
    type StoreConflictResolution,
  } from "./model.js";
  import "./store-conflicts.css";

  let { controller, open }: { controller: StoreController; open: boolean } =
    $props();
  const view = $derived(controller.state);
  const mutation = $derived(view.mutation);
  const pending = $derived(
    mutation.status === "previewing" || mutation.status === "applying",
  );
  const commit = $derived(view.route.commitId ?? "");
  let selections = $state(new Map<string, StoreConflictResolution>());
  let activePath = $state("");
  let uploadingPath = $state("");
  let uploadError = $state("");
  let validation = $state("");
  let initialized = false;
  const active = $derived(
    view.conflicts.find((conflict) => conflict.path === activePath) ??
      view.conflicts[0],
  );

  $effect(() => {
    if (open && !initialized) {
      selections = new Map();
      activePath = view.conflicts[0]?.path ?? "";
      uploadError = "";
      validation = "";
      controller.actions.resetMutation();
      initialized = true;
    } else if (!open && initialized) {
      controller.actions.resetMutation();
      initialized = false;
    }
  });

  function choose(path: string, resolution: StoreConflictResolution) {
    selections = new Map(selections).set(path, resolution);
    validation = "";
    if (mutation.status !== "idle") controller.actions.resetMutation();
  }
  async function stage(contents: string) {
    if (!active || !commit) return;
    uploadingPath = active.path;
    uploadError = "";
    try {
      const upload = await controller.actions.uploadConflict(
        commit,
        active.path,
        active.fingerprint,
        new TextEncoder().encode(contents),
      );
      choose(active.path, {
        type: "uploaded_file",
        upload: upload.id,
        executable: active.editor.status === "text" && active.editor.executable,
        copy_from_term: active.terms[0]?.id ?? null,
      });
    } catch (cause) {
      uploadError =
        cause instanceof Error ? cause.message : "Conflict upload failed.";
    } finally {
      uploadingPath = "";
    }
  }
  async function preview() {
    validation = "";
    try {
      await controller.actions.previewMutation(
        buildConflictCommand(commit, view.conflicts, selections),
      );
    } catch (cause) {
      validation =
        cause instanceof Error ? cause.message : "Invalid conflict selection.";
    }
  }
</script>

<DialogFrame
  title="Resolve native conflicts"
  description="Choose one explicit resolution for every native path. The complete selection is previewed and published atomically."
  size="wide"
  surface="inset"
  busy={pending}
  preventScroll={false}
>
  {#snippet sidebar()}
    <nav class="store-conflict-editor__paths" aria-label="Conflict paths">
      {#each view.conflicts as conflict (conflict.path)}
        <Button
          type="button"
          variant={active?.path === conflict.path ? "secondary" : "ghost"}
          onclick={() => (activePath = conflict.path)}
        >
          <span>{conflict.path}</span>
          {#if selections.has(conflict.path)}<CheckIcon
              aria-hidden="true"
            />{/if}
        </Button>
      {/each}
    </nav>
  {/snippet}

  {#if mutation.result}
    <StoreMutationPreview result={mutation.result} />
    {#if mutation.status === "unknown"}<Alert.Root>
        <Alert.Title>Publication outcome unknown</Alert.Title>
        <Alert.Description>
          Keep this request identity and check its durable status. The staged
          file handles and conflict fingerprints remain unchanged.
        </Alert.Description>
      </Alert.Root>
    {:else if mutation.status === "committed"}<Alert.Root>
        <Alert.Title>Conflicts resolved</Alert.Title>
        <Alert.Description>
          The atomic resolution receipt was committed and the snapshot was
          refreshed.
        </Alert.Description>
      </Alert.Root>
    {:else if mutation.status === "failed"}<Alert.Root variant="destructive">
        <Alert.Title>Resolution rejected</Alert.Title>
        <Alert.Description>{mutation.error}</Alert.Description>
      </Alert.Root>{/if}
  {:else if active}
    <StoreConflictPath
      conflict={active}
      resolution={selections.get(active.path)}
      actions={view.conflictResolutionActions}
      uploading={uploadingPath === active.path}
      {uploadError}
      onSelect={(resolution) => choose(active.path, resolution)}
      onStage={stage}
    />
    {#if validation || mutation.error}<p
        class="store-conflict-editor__error"
        role="alert"
      >
        {validation || mutation.error}
      </p>{/if}
  {:else}<Alert.Root>
      <Alert.Title>No conflict descriptors</Alert.Title>
      <Alert.Description>
        Select a conflicted commit before opening this editor.
      </Alert.Description>
    </Alert.Root>{/if}

  {#snippet footer()}
    {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
      <Dialog.Close>
        {#snippet child({ props })}<Button
            {...props}
            type="button"
            variant="outline">Cancel</Button
          >{/snippet}
      </Dialog.Close>
      <Button type="button" disabled={pending} onclick={preview}
        >Preview {selections.size}/{view.conflicts.length}</Button
      >
    {:else if mutation.status === "prepared"}
      <Button
        type="button"
        variant="outline"
        onclick={controller.actions.resetMutation}>Back</Button
      >
      <ConfirmButton
        title="Apply atomic conflict resolution?"
        description="Publish the exact prepared native command for every selected path."
        confirmLabel="Apply resolution"
        onConfirm={controller.actions.applyMutation}
        >Apply resolution</ConfirmButton
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
    {:else}<Button type="button" disabled
        ><Spinner data-icon="inline-start" />Working</Button
      >{/if}
  {/snippet}
</DialogFrame>
