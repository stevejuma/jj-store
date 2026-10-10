<script lang="ts">
  import { untrack } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose: () => void } = $props();
  let title = $state("");
  let repositories = $state<string[]>(
    untrack(() =>
      controller.state.route.threadRepository
        ? [controller.state.route.threadRepository]
        : [],
    ),
  );
  const manager = $derived(controller.threadWorkspaces);
  const options = $derived(
    controller.state.repositories.filter(
      (r) =>
        !!r.permission &&
        !["unavailable", "identity_changed"].includes(r.availability ?? ""),
    ),
  );
  async function create() {
    const result = await manager
      ?.submit({ type: "create", title, repositories })
      .catch(() => undefined);
    if (result) onClose();
  }
</script>

<Dialog.Root
  open
  onOpenChange={(value) => {
    if (!value) onClose();
  }}
>
  <DialogFrame
    title="Create Thread"
    description="Start a conversation. Repositories are optional; adding one does not start an agent."
  >
    <Field.Group class="community-settings__option-group">
      <StoreWorkField label="Title" id="thread-create-title"
        ><Input
          id="thread-create-title"
          aria-label="Thread title"
          bind:value={title}
          disabled={manager?.busy || !!manager?.pending}
        /></StoreWorkField
      >
      <StoreWorkField
        label="Codebase"
        description="Optional. Choose up to eight repositories."
      >
        <StoreRepositoryPicker
          label="Thread Codebase"
          repositories={options}
          bind:values={repositories}
          multiple
          disabled={manager?.busy || !!manager?.pending}
          emptyLabel="Choose repositories"
        />
      </StoreWorkField>
      <div class="community-settings__option-row flex justify-end">
        {#if manager?.pending}<Button
            disabled={manager.busy}
            onclick={() =>
              void manager
                .retry()
                .then((result) => {
                  if (result) onClose();
                })
                .catch(() => {})}>Check same action</Button
          >{:else}<Button
            disabled={!manager || manager.busy || !title.trim()}
            onclick={() => void create()}>Create Thread</Button
          >{/if}
      </div>
    </Field.Group>
    <StoreNotice
      noticeKey="thread-create"
      title="Thread action needs attention"
      message={manager?.error ?? ""}
    />
  </DialogFrame>
</Dialog.Root>
