<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import XIcon from "@lucide/svelte/icons/x";
  import { MediaQuery } from "svelte/reactivity";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import StoreConflictBody from "./StoreConflictBody.svelte";
  import StoreConflictHeaderActions from "./StoreConflictHeaderActions.svelte";
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
  let drafts = $state<Record<string, string>>({});
  let browserOpen = $state(true);
  const compact = new MediaQuery("(max-width: 700px)");
  $effect(() => {
    if (compact.current) browserOpen = false;
  });
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
      drafts = {};
      browserOpen = !compact.current;
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
    conflict: import("../../controller/repository-read-types.js").CodeConflict,
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
  <AppShell.SurfaceLayer.Header class="store-conflict-editor__header">
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      aria-label={`${browserOpen ? "Hide" : "Show"} conflict file browser`}
      title={`${browserOpen ? "Hide" : "Show"} conflict file browser`}
      aria-expanded={browserOpen}
      aria-controls="conflict-file-browser"
      onclick={() => (browserOpen = !browserOpen)}
    >
      <CommunitySidebarToggleIcon expanded={browserOpen} />
    </Button>
    <h1 class="store-conflict-editor__title">Resolve conflicts</h1>
    <StoreConflictHeaderActions
      {controller}
      {active}
      resolution={active ? selections.get(active.path) : undefined}
      uploading={Boolean(uploadingPath)}
      selectedCount={selections.size}
      pathCount={view.conflicts.length}
      onDelete={() => {
        if (active) choose(active.path, { type: "delete" });
      }}
      onStage={() => {
        if (active?.editor.status === "text")
          return stage(
            active,
            drafts[active.path] ?? active.editor.contents[1],
          );
      }}
      onPreview={preview}
    />
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
    <StoreConflictBody
      files={view.conflicts}
      activePath={active?.path ?? ""}
      {browserOpen}
      compact={compact.current}
      locked={pending || Boolean(mutation.result) || Boolean(uploadingPath)}
      onSelect={(path) => {
        activePath = path;
        if (compact.current) browserOpen = false;
      }}
    >
      {#if mutation.result}
        <StoreMutationPreview result={mutation.result} />
        {#if mutation.status === "unknown"}<Alert.Root>
            <Alert.Title>Publication outcome unknown</Alert.Title>
            <Alert.Description>
              Keep this request identity and check its durable status. The
              staged file handles and conflict fingerprints remain unchanged.
            </Alert.Description>
          </Alert.Root>
        {:else if mutation.status === "committed"}<StoreNotice
            noticeKey={`mutation-result:${mutation.requestId ?? "new"}`}
            title="Conflicts resolved"
            message="The atomic resolution receipt was committed and the snapshot was refreshed."
            severity="info"
            lifetime="timed"
          />
        {:else if mutation.status === "failed"}<StoreNotice
            noticeKey={`mutation-failure:${mutation.requestId ?? "new"}`}
            title="Resolution rejected"
            message={mutation.error}
            severity="error"
            lifetime="dismiss"
          />{/if}
      {/if}
      {#each view.conflicts as conflict (conflict.path)}
        <div
          class="store-conflict-editor__path"
          hidden={Boolean(mutation.result) || active?.path !== conflict.path}
        >
          <StoreConflictPath
            {conflict}
            resolution={selections.get(conflict.path)}
            actions={view.conflictResolutionActions}
            uploading={uploadingPath === conflict.path}
            uploadError={active?.path === conflict.path ? uploadError : ""}
            onSelect={(resolution) => choose(conflict.path, resolution)}
            bind:merged={
              () =>
                drafts[conflict.path] ??
                (conflict.editor.status === "text"
                  ? conflict.editor.contents[1]
                  : ""),
              (contents) => (drafts[conflict.path] = contents)
            }
          />
        </div>
      {/each}
      {#if validation || ((!mutation.result || mutation.status === "unknown") && mutation.error)}<p
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
    </StoreConflictBody>
  </AppShell.SurfaceLayer.Body>
</AppShell.SurfaceLayer>
