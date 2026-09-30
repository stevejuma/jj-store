<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { ConfirmButton } from "@lapismd/design-core/forms/dialog";
  import Trash2Icon from "@lucide/svelte/icons/trash-2";
  import FileCheck2Icon from "@lucide/svelte/icons/file-check-2";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import CheckIcon from "@lucide/svelte/icons/check";
  import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreConflictResolution } from "./model.js";

  let {
    controller,
    active,
    resolution,
    uploading,
    selectedCount,
    pathCount,
    onDelete,
    onStage,
    onPreview,
  }: {
    controller: StoreController;
    active?: Model<"ConflictDescriptor">;
    resolution?: StoreConflictResolution;
    uploading: boolean;
    selectedCount: number;
    pathCount: number;
    onDelete: () => void;
    onStage: () => void | Promise<void>;
    onPreview: () => void | Promise<void>;
  } = $props();
  const mutation = $derived(controller.state.mutation);
  const actions = $derived(controller.state.conflictResolutionActions);
</script>

<div
  class="store-conflict-editor__actions"
  role="group"
  aria-label="Conflict actions"
>
  {#if mutation.status === "idle" || (mutation.status === "failed" && !mutation.result)}
    {#if active && actions.includes("delete")}
      <Button
        type="button"
        size="sm"
        disabled={uploading}
        variant={resolution?.type === "delete" ? "destructive" : "outline"}
        aria-label="Delete path"
        title="Delete path"
        aria-pressed={resolution?.type === "delete"}
        onclick={onDelete}
      >
        <Trash2Icon data-icon="inline-start" aria-hidden="true" />
        <span class="store-conflict-editor__action-label">Delete path</span>
      </Button>
    {/if}
    {#if resolution?.type === "uploaded_file"}
      <span class="store-conflict-editor__staged" role="status"
        >Merged file staged</span
      >
    {/if}
    {#if active?.editor.status === "text" && actions.includes("uploaded_file")}
      <Button
        type="button"
        size="sm"
        variant="outline"
        disabled={uploading}
        aria-label="Stage merged file"
        title="Stage merged file"
        onclick={onStage}
      >
        {#if uploading}<Spinner
            data-icon="inline-start"
          />{:else}<FileCheck2Icon
            data-icon="inline-start"
            aria-hidden="true"
          />{/if}
        <span class="store-conflict-editor__action-label"
          >Stage merged file</span
        >
      </Button>
    {/if}
    <Button type="button" size="sm" disabled={uploading} onclick={onPreview}>
      <EyeIcon data-icon="inline-start" aria-hidden="true" />Preview {selectedCount}/{pathCount}
    </Button>
  {:else if mutation.status === "prepared"}
    <Button
      type="button"
      size="sm"
      variant="outline"
      onclick={controller.actions.resetMutation}
    >
      <ArrowLeftIcon data-icon="inline-start" aria-hidden="true" />Back
    </Button>
    <ConfirmButton
      size="sm"
      title="Apply atomic conflict resolution?"
      description="Publish the exact prepared native command for every selected path."
      confirmLabel="Apply resolution"
      onConfirm={controller.actions.applyMutation}
    >
      <CheckIcon data-icon="inline-start" aria-hidden="true" />Apply resolution
    </ConfirmButton>
  {:else if mutation.status === "unknown"}
    <Button
      type="button"
      size="sm"
      variant="outline"
      onclick={controller.actions.applyMutation}
    >
      <RotateCcwIcon data-icon="inline-start" aria-hidden="true" />Retry request
    </Button>
    <Button
      type="button"
      size="sm"
      onclick={controller.actions.checkMutationStatus}
    >
      <RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Check status
    </Button>
  {:else if mutation.status === "failed" && mutation.result}
    <Button
      type="button"
      size="sm"
      variant="outline"
      onclick={controller.actions.resetMutation}
    >
      <ArrowLeftIcon data-icon="inline-start" aria-hidden="true" />Back
    </Button>
    <Button type="button" size="sm" onclick={controller.actions.applyMutation}>
      <RotateCcwIcon data-icon="inline-start" aria-hidden="true" />Retry request
    </Button>
  {:else if mutation.status !== "committed"}
    <Button type="button" size="sm" disabled
      ><Spinner data-icon="inline-start" />Working</Button
    >
  {/if}
</div>
