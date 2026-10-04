<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import XIcon from "@lucide/svelte/icons/x";
  import type { Model } from "../../api/client.js";
  let {
    open = $bindable(false),
    path = $bindable(""),
    name = $bindable(""),
    reconnect,
    busy,
    pending,
    inspected,
    message,
    onPathChange,
    onBrowse,
    onInspect,
    onSave,
  }: {
    open?: boolean;
    path?: string;
    name?: string;
    reconnect: boolean;
    busy: boolean;
    pending: boolean;
    inspected: Model<"LocalFolder"> | null;
    message: string;
    onPathChange: () => void;
    onBrowse: () => void;
    onInspect: () => void;
    onSave: () => void;
  } = $props();
  const formId = $props.id();
</script>

<Dialog.Root bind:open>
  <DialogFrame
    size="wide"
    preventScroll={false}
    {busy}
    title={reconnect ? "Reconnect local folder" : "Add local JJ folder"}
    description={reconnect
      ? "Select the original repository at its new location."
      : "Inspect an existing folder, then explicitly add it to this installation."}
  >
    <div class="store-work__form-sections">
      <Field.Group class="store-work__fields community-settings__option-group">
        <StoreWorkField
          label="Folder"
          bordered
          id={`${formId}-path`}
          description="Enter an absolute folder path or browse for an existing Git-backed JJ repository."
          disabled={busy || pending}
        >
          <ButtonGroup.Root
            class="store-settings__folder-picker"
            style="width: 100%"
          >
            <Input
              id={`${formId}-path`}
              value={path}
              placeholder="/path/to/repository"
              disabled={busy || pending}
              oninput={(event) => {
                path = event.currentTarget.value;
                onPathChange();
              }}
            />
            <Button
              type="button"
              variant="outline"
              disabled={busy || pending}
              onclick={onBrowse}
            >
              <GitBranchIcon
                data-icon="inline-start"
                aria-hidden="true"
              />Browse
            </Button>
          </ButtonGroup.Root>
        </StoreWorkField>
        {#if !reconnect}<StoreWorkField
            label="Name"
            bordered
            id={`${formId}-name`}
            description="Choose the name shown in this installation."
            disabled={busy || pending}
          >
            <Input
              id={`${formId}-name`}
              bind:value={name}
              placeholder="Repository name"
              disabled={busy || pending}
            />
          </StoreWorkField>{/if}
        <StoreWorkField
          label="Inspection"
          description="Check the folder before adding it. Companion metadata is initialized only when you add the folder."
          disabled={busy || pending}
        >
          <Button
            type="button"
            variant="outline"
            disabled={busy || pending || !path}
            onclick={onInspect}
          >
            <FolderGitIcon data-icon="inline-start" aria-hidden="true" />Inspect
            folder
          </Button>
        </StoreWorkField>
      </Field.Group>
      {#if inspected}<StoreSettingsNotice>
          Supported Git-backed JJ folder. {inspected.repository
            ? `Repository ${inspected.repository}`
            : "Companion metadata will be initialized on Add."}
        </StoreSettingsNotice>{/if}
      {#if message}<StoreSettingsNotice>{message}</StoreSettingsNotice>{/if}
    </div>
    {#snippet footer()}
      <Button
        type="button"
        variant="outline"
        disabled={busy}
        onclick={() => (open = false)}
      >
        <XIcon data-icon="inline-start" aria-hidden="true" />Cancel
      </Button>
      <Button
        type="button"
        disabled={busy ||
          (!pending && (!inspected || (!reconnect && !name.trim())))}
        onclick={onSave}
      >
        <PlusIcon data-icon="inline-start" aria-hidden="true" />{pending
          ? "Retry Add"
          : reconnect
            ? "Reconnect"
            : "Add folder"}
      </Button>
    {/snippet}
  </DialogFrame>
</Dialog.Root>

<style>
  :global(.store-settings__folder-picker) {
    min-width: 0;
  }
  :global(.store-settings__folder-picker input) {
    flex: 1;
    min-width: 0;
  }
  :global(.store-settings__folder-picker button) {
    flex-shrink: 0;
  }
</style>
