<script lang="ts">
  import { onDestroy, type Snippet } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { ApiError } from "../../api/client.js";
  import { NativeRepositoryManager } from "../../controller/native-repository-manager.js";
  import { emptyRoute } from "../../controller/global-search-presentation.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    trigger,
  }: {
    controller: StoreController;
    trigger: Snippet<[create: () => void, disabled: boolean]>;
  } = $props();
  let manager = $state<NativeRepositoryManager>();
  let open = $state(false),
    busy = $state(false),
    pending = $state(false),
    name = $state(""),
    error = $state(""),
    fieldError = $state(""),
    progress = $state("");
  const id = $props.id();
  const enabled = $derived(
    controller.state.accessMode === "local_trusted" &&
      controller.state.nativeRepositoryCreation === true,
  );
  $effect(() => {
    if (enabled && !manager) {
      try {
        manager = new NativeRepositoryManager(undefined, localStorage);
        pending = Boolean(manager.pending);
        name = manager.pending?.name ?? "";
      } catch (cause) {
        error = storeErrorMessage(cause);
      }
    }
  });
  onDestroy(() => manager?.close());
  async function run(status = false) {
    if (busy || !manager) return;
    busy = true;
    error = "";
    fieldError = "";
    try {
      const result = status
        ? await manager.status()
        : await manager.create(name);
      if (result?.status === "committed") {
        open = false;
        await controller.projects?.refresh();
        await controller.actions.openRoute?.({
          ...emptyRoute(result.repository),
          page: "files",
        });
      } else
        progress =
          "Creation is awaiting confirmation. Check status or retry the same request.";
    } catch (cause) {
      if (cause instanceof ApiError && cause.code === "invalid_local_folder")
        fieldError =
          "Choose a name up to 256 bytes without control characters, backslashes or empty, dot or parent path segments.";
      else if (
        cause instanceof ApiError &&
        cause.code === "local_registration_conflict"
      )
        fieldError = "This name is already in use. Choose a different name.";
      else error = storeErrorMessage(cause);
    } finally {
      pending = Boolean(manager.pending);
      busy = false;
    }
  }
</script>

{@render trigger(
  () => {
    error = "";
    try {
      manager?.refresh();
      pending = Boolean(manager?.pending);
      if (pending) name = manager?.pending?.name ?? "";
      progress = pending
        ? "Creation is awaiting confirmation. Check status or retry the same request."
        : "";
      open = true;
    } catch (cause) {
      error = storeErrorMessage(cause);
    }
  },
  !enabled || busy || !manager,
)}
<StoreNotice
  noticeKey={`native-create:${manager?.pending?.request_id ?? id}`}
  title="Repository creation failed"
  message={error}
  lifetime="dismiss"
/>
<Dialog.Root bind:open>
  <DialogFrame
    preventScroll={false}
    title="Create server repository"
    description="Store code and collaboration in this installation. A local working folder is optional."
    {busy}
  >
    <div class="store-work__form-sections">
      <Field.Group class="community-settings__option-group">
        <StoreWorkField
          id={`${id}-name`}
          label="Name"
          description="Choose the name shown in this installation."
          disabled={busy || pending}
        >
          <Input
            id={`${id}-name`}
            bind:value={name}
            disabled={busy || pending}
            aria-invalid={Boolean(fieldError)}
            aria-describedby={fieldError ? `${id}-name-error` : undefined}
            oninput={() => (fieldError = "")}
            placeholder="Repository name"
          />
          {#if fieldError}<Field.Error id={`${id}-name-error`}
              >{fieldError}</Field.Error
            >{/if}
        </StoreWorkField>
      </Field.Group>
      {#if progress}<StoreSettingsNotice>{progress}</StoreSettingsNotice>{/if}
    </div>
    {#snippet footer()}
      <Button variant="outline" disabled={busy} onclick={() => (open = false)}
        >Close</Button
      >
      {#if pending}<Button
          variant="outline"
          disabled={busy}
          onclick={() => run(true)}>Check status</Button
        >{/if}
      <Button disabled={busy || !name.trim()} onclick={() => run()}
        >{pending ? "Retry creation" : "Create repository"}</Button
      >
    {/snippet}
  </DialogFrame>
</Dialog.Root>
