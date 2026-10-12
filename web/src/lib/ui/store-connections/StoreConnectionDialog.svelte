<script lang="ts">
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import ServerIcon from "@lucide/svelte/icons/server";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsCard from "../store-settings/StoreSettingsCard.svelte";
  import StoreSettingsFooter from "../store-settings/StoreSettingsFooter.svelte";
  import { provideSettingsPresentation } from "../store-settings/settings-presentation.js";
  import type { ConnectionDialogOwner } from "./presentation.js";
  let { owner }: { owner: ConnectionDialogOwner } = $props();
  let name = $state(""),
    origin = $state(""),
    token = $state("");
  const field = $props.id();
  provideSettingsPresentation({ value: "global" });
  $effect(() => {
    if (owner.dialog) {
      name = owner.editing?.name ?? "";
      origin = owner.editing?.origin ?? "";
      token = "";
    } else token = "";
  });
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    const credential = token;
    token = "";
    await owner.connect({ name, origin, token: credential });
  }
</script>

<Dialog.Root bind:open={owner.dialog}>
  <DialogFrame
    title={owner.editing ? "Server connection" : "Add server"}
    description="Connect to an authenticated JJ Store server. Its token stays in this application session."
    busy={owner.busy}
    closeLabel="Close server connection"
  >
    <form onsubmit={submit} class="connection-dialog store-settings--global">
      <StoreSettingsCard
        id={`${field}-details`}
        title="Connection details"
        Icon={ServerIcon}
        description="Choose the server and authenticate with its access token."
      >
        <Field.Group class="store-settings__editable-rows">
          <StoreWorkField
            label="Name"
            id={`${field}-name`}
            description="A display name for this sidebar."
            disabled={owner.busy}
            class="store-settings__option-row"
            ><Input
              id={`${field}-name`}
              bind:value={name}
              required
              maxlength={128}
              disabled={owner.busy}
            /></StoreWorkField
          >
          <StoreWorkField
            label="Server address"
            id={`${field}-origin`}
            description="The server's HTTP or HTTPS origin."
            disabled={owner.busy || !!owner.editing}
            class="store-settings__option-row"
            ><Input
              id={`${field}-origin`}
              type="url"
              bind:value={origin}
              required
              placeholder="https://store.example"
              disabled={owner.busy || !!owner.editing}
            /></StoreWorkField
          >
          <StoreWorkField
            label="Access token"
            id={`${field}-token`}
            description="Use a token enrolled in the server's identity directory."
            disabled={owner.busy}
            class="store-settings__option-row"
            ><Input
              id={`${field}-token`}
              type="password"
              bind:value={token}
              autocomplete="off"
              disabled={owner.busy}
            /></StoreWorkField
          >
        </Field.Group>
        {#if owner.error}<Alert.Root variant="destructive">
            <Alert.Description>
              {owner.error.replaceAll("_", " ")}
            </Alert.Description>
          </Alert.Root>{/if}
        <StoreSettingsFooter>
          {#if owner.editing}
            <Button
              type="button"
              variant="ghost"
              size="sm"
              disabled={owner.busy}
              onclick={() => owner.signOut(owner.editing!.id)}>Sign out</Button
            ><Button
              type="button"
              variant="destructive"
              size="sm"
              disabled={owner.busy}
              onclick={() => owner.remove()}>Remove</Button
            >
          {/if}
          {#if owner.editing}<Button
              type="button"
              variant="outline"
              disabled={owner.busy || !name.trim()}
              onclick={() => owner.rename(name)}>Rename</Button
            >{/if}<Button
            type="submit"
            disabled={owner.busy ||
              !token.trim() ||
              !name.trim() ||
              !origin.trim()}
            >{#if owner.busy}<Spinner
                data-icon="inline-start"
              />{/if}Connect</Button
          >
        </StoreSettingsFooter>
      </StoreSettingsCard>
    </form>
  </DialogFrame>
</Dialog.Root>

<style>
  .connection-dialog {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .connection-dialog :global(.store-settings__section) {
    margin-block-start: 0;
  }
  .connection-dialog :global(.store-settings__actions) {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .connection-dialog :global(input) {
    width: min(20rem, 100%);
  }
</style>
