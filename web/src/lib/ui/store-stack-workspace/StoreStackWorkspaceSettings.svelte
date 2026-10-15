<script lang="ts">
  import { untrack } from "svelte";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Separator } from "@lapismd/design-core/shadcn/separator";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import Save from "@lucide/svelte/icons/save";
  import X from "@lucide/svelte/icons/x";
  import Trash from "@lucide/svelte/icons/trash-2";
  import CommunityChannelSettings from "../community-channel-settings/CommunityChannelSettings.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import { selectedWorkspace } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import "../store-settings/StoreSettings.css";
  let {
    controller,
    onClose,
  }: { controller: StackWorkspaceController; onClose: () => void } = $props();
  const workspace = $derived(selectedWorkspace(controller.snapshot!));
  const targets = $derived(
    controller.snapshot?.targetOptions ??
      (controller.snapshot?.target ? [controller.snapshot.target] : []),
  );
  let name = $state(untrack(() => workspace!.name));
  let target = $state(
    untrack(() =>
      JSON.stringify([
        controller.snapshot!.target?.name,
        controller.snapshot!.target?.commit,
      ]),
    ),
  );
  const selected = $derived(
    targets.find((item) => JSON.stringify([item.name, item.commit]) === target),
  );
  const disabled = $derived(!controller.can("update_workspace"));
  const dirty = $derived(
    name.trim() !== workspace?.name ||
      selected?.name !== controller.snapshot?.target?.name,
  );
  const problem = $derived(
    !name.trim()
      ? "Enter a workspace name."
      : new TextEncoder().encode(name).length > 128 ||
          /[\u0000-\u001f\u007f]/u.test(name)
        ? "Use a name without control characters, up to 128 bytes."
        : controller.snapshot?.workspaces.some(
              (w) => w.id !== workspace?.id && w.name === name.trim(),
            )
          ? "This workspace name is already in use."
          : !selected
            ? "Choose an available target branch."
            : "",
  );
  function reset() {
    name = workspace!.name;
    target = JSON.stringify([
      controller.snapshot!.target?.name,
      controller.snapshot!.target?.commit,
    ]);
  }
</script>

{#if workspace}
  <CommunityChannelSettings
    title={workspace.name}
    id={workspace.id}
    heading="Workspace settings"
    closeLabel="Close workspace settings"
    {onClose}
  >
    {#snippet identity()}<FolderKanban data-workspace-icon />{/snippet}
    {#snippet details()}
      <form
        class="store-work__form-sections"
        aria-label="Workspace settings"
        onsubmit={(event) => {
          event.preventDefault();
          if (!disabled && !problem && selected)
            void controller.prepare({
              kind: "update_workspace",
              workspace: workspace!.id,
              name: name.trim(),
              targetBranch: { name: selected.name, commit: selected.commit },
            });
        }}
      >
        <Field.Group
          class="stack-workspace-settings__details community-settings__option-group lc-channel-settings__details-card"
        >
          <StoreReadOnlyRow
            label="Workspace ID"
            value={workspace.id}
            standalone
            stacked
            copyable
          />
          <StoreWorkField
            label="Workspace name"
            id="stack-workspace-name"
            {disabled}
            compact
          >
            <Input
              id="stack-workspace-name"
              bind:value={name}
              {disabled}
              required
              maxlength={128}
            />
          </StoreWorkField>
          <StoreWorkField
            label="Target branch"
            id="stack-workspace-target"
            {disabled}
            compact
          >
            <StoreRepositoryPicker
              id="stack-workspace-target"
              label="Workspace target branch"
              repositories={targets.map((item) => ({
                id: JSON.stringify([item.name, item.commit]),
                name: item.name,
                description: item.message.split("\n")[0],
              }))}
              value={target}
              onChange={(value) => (target = value)}
              controlled
              {disabled}
              emptyLabel="Choose target branch"
              searchLabel="Search target branches"
              groupLabel="Branches"
              scrollStrategy="styled"
              hasMore={Boolean(controller.snapshot?.branchesNextCursor)}
              loading={controller.branches.loading}
              onLoadMore={() => void controller.branches.load()}
            />
          </StoreWorkField>
          {#if dirty && problem}<p role="alert">{problem}</p>{/if}
          <Separator />
          <div class="flex justify-end gap-2 px-4 py-3">
            <Button
              variant="outline"
              type="button"
              disabled={disabled || !dirty}
              onclick={reset}
              ><X data-icon="inline-start" aria-hidden="true" />Cancel</Button
            >
            <Button
              type="submit"
              disabled={disabled || !dirty || Boolean(problem)}
              ><Save data-icon="inline-start" aria-hidden="true" />Save</Button
            >
          </div>
        </Field.Group>
      </form>
    {/snippet}
    {#snippet actions()}
      <section
        class="lc-channel-settings__danger-zone"
        aria-label="Destructive workspace actions"
      >
        <Button
          variant="ghost"
          class="lc-channel-settings__row lc-channel-settings__destructive"
          disabled={!controller.can("delete_workspace")}
          onclick={() =>
            void controller.prepare({
              kind: "delete_workspace",
              workspace: workspace!.id,
            })}
        >
          <Trash data-icon="inline-start" aria-hidden="true" /><span
            >Delete workspace</span
          >
        </Button>
      </section>
    {/snippet}
  </CommunityChannelSettings>
{/if}

<style>
  :global(.stack-workspace-settings__details .store-readonly-row) {
    padding: 0.75rem 1rem;
  }
</style>
