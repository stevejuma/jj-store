<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { ConfirmButton } from "@lapismd/design-core/forms/dialog";
  import CheckIcon from "@lucide/svelte/icons/check";
  import XIcon from "@lucide/svelte/icons/x";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreMutationPreview from "../store-mutations/StoreMutationPreview.svelte";
  import StoreConflictPath from "./StoreConflictPath.svelte";
  import {
    buildConflictCommand,
    type StoreConflictResolution,
  } from "./model.js";
  import "./store-conflicts.css";

  let {
    controller,
    open,
    onClose,
    returnFocus,
  }: {
    controller: StoreController;
    open: boolean;
    onClose: () => void;
    returnFocus?: HTMLElement | null;
  } = $props();
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

  function choose(path: string, resolution?: StoreConflictResolution) {
    const next = new Map(selections);
    if (resolution) next.set(path, resolution);
    else next.delete(path);
    selections = next;
    validation = "";
    if (mutation.status !== "idle") controller.actions.resetMutation();
  }
  async function stage(
    conflict: Model<"ConflictDescriptor">,
    contents: string,
  ) {
    if (!commit) return;
    uploadingPath = conflict.path;
    uploadError = "";
    try {
      const upload = await controller.actions.uploadConflict(
        commit,
        conflict.path,
        conflict.fingerprint,
        new TextEncoder().encode(contents),
      );
      choose(conflict.path, {
        type: "uploaded_file",
        upload: upload.id,
        executable:
          conflict.editor.status === "text" && conflict.editor.executable,
        copy_from_term: conflict.terms[0]?.id ?? null,
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
  function close() {
    if (!pending && !uploadingPath) onClose();
  }
</script>

<AppShell.SurfaceLayer
  {open}
  {returnFocus}
  label="Resolve native conflicts"
  coverPanelIds={["right"]}
  mobilePlacement="root"
  dismissOnEscape={!pending && !uploadingPath}
  onDismiss={close}
  class="store-conflict-editor"
>
  <AppShell.SurfaceLayer.Header>
    <h1 class="store-conflict-editor__title">Resolve conflicts</h1>
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      aria-label="Close conflict editor"
      disabled={pending || Boolean(uploadingPath)}
      onclick={close}><XIcon aria-hidden="true" /></Button
    >
  </AppShell.SurfaceLayer.Header>
  <AppShell.SurfaceLayer.Body class="store-conflict-editor__body">
    <div class="store-conflict-editor__layout">
      <div class="store-conflict-editor__main">
        <nav class="store-conflict-editor__paths" aria-label="Conflict paths">
          {#each view.conflicts as conflict (conflict.path)}
            <Button
              type="button"
              variant={active?.path === conflict.path ? "secondary" : "ghost"}
              disabled={pending ||
                Boolean(mutation.result) ||
                Boolean(uploadingPath)}
              onclick={() => (activePath = conflict.path)}
            >
              <span>{conflict.path}</span>
              {#if selections.has(conflict.path)}<CheckIcon
                  aria-hidden="true"
                />{/if}
            </Button>
          {/each}
        </nav>
        <div class="store-conflict-editor__content">
          {#if mutation.result}
            <StoreMutationPreview result={mutation.result} />
            {#if mutation.status === "unknown"}<Alert.Root>
                <Alert.Title>Publication outcome unknown</Alert.Title>
                <Alert.Description>
                  Keep this request identity and check its durable status. The
                  staged file handles and conflict fingerprints remain
                  unchanged.
                </Alert.Description>
              </Alert.Root>
            {:else if mutation.status === "committed"}<Alert.Root>
                <Alert.Title>Conflicts resolved</Alert.Title>
                <Alert.Description>
                  The atomic resolution receipt was committed and the snapshot
                  was refreshed.
                </Alert.Description>
              </Alert.Root>
            {:else if mutation.status === "failed"}<Alert.Root
                variant="destructive"
              >
                <Alert.Title>Resolution rejected</Alert.Title>
                <Alert.Description>{mutation.error}</Alert.Description>
              </Alert.Root>{/if}
          {/if}
          {#each view.conflicts as conflict (conflict.path)}
            <div
              class="store-conflict-editor__path"
              hidden={Boolean(mutation.result) ||
                active?.path !== conflict.path}
            >
              <StoreConflictPath
                {conflict}
                resolution={selections.get(conflict.path)}
                actions={view.conflictResolutionActions}
                uploading={uploadingPath === conflict.path}
                uploadError={active?.path === conflict.path ? uploadError : ""}
                onSelect={(resolution) => choose(conflict.path, resolution)}
                onStage={(contents) => stage(conflict, contents)}
              />
            </div>
          {/each}
          {#if validation || mutation.error}<p
              class="store-conflict-editor__error"
              role="alert"
            >
              {validation || mutation.error}
            </p>{/if}
          {#if !active && !mutation.result}<Alert.Root>
              <Alert.Title>No conflict descriptors</Alert.Title>
              <Alert.Description>
                Select a conflicted commit before opening this editor.
              </Alert.Description>
            </Alert.Root>{/if}
        </div>
      </div>

      <footer class="store-conflict-editor__footer">
        {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
          <Button
            type="button"
            variant="outline"
            disabled={Boolean(uploadingPath)}
            onclick={close}>Cancel</Button
          >
          <Button
            type="button"
            disabled={pending || Boolean(uploadingPath)}
            onclick={preview}
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
          <Button type="button" onclick={close}>Close</Button>
        {:else}<Button type="button" disabled
            ><Spinner data-icon="inline-start" />Working</Button
          >{/if}
      </footer>
    </div>
  </AppShell.SurfaceLayer.Body>
</AppShell.SurfaceLayer>
