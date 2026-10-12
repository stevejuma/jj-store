<script lang="ts">
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import type { ConnectionDialogOwner } from "./presentation.js";
  let { owner }: { owner: ConnectionDialogOwner } = $props();
  let name = $state(""),
    origin = $state(""),
    token = $state("");
  const field = $props.id();
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
    <form onsubmit={submit} class="connection-dialog">
      <Field.Group>
        <Field.Field
          ><Field.Label for={`${field}-name`}>Name</Field.Label><Input
            id={`${field}-name`}
            bind:value={name}
            required
            maxlength={128}
            disabled={owner.busy}
          /></Field.Field
        >
        <Field.Field
          ><Field.Label for={`${field}-origin`}>Server address</Field.Label
          ><Input
            id={`${field}-origin`}
            type="url"
            bind:value={origin}
            required
            placeholder="https://store.example"
            disabled={owner.busy || !!owner.editing}
          /></Field.Field
        >
        <Field.Field
          ><Field.Label for={`${field}-token`}>Access token</Field.Label><Input
            id={`${field}-token`}
            type="password"
            bind:value={token}
            autocomplete="off"
            disabled={owner.busy}
          /><Field.Description
            >Use a token enrolled in the server's identity directory.</Field.Description
          ></Field.Field
        >
      </Field.Group>
      {#if owner.error}<p role="alert">
          {owner.error.replaceAll("_", " ")}
        </p>{/if}
      <div class="connection-dialog__footer">
        {#if owner.editing}<div class="connection-dialog__management">
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
          </div>{/if}
        <div class="connection-dialog__actions">
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
        </div>
      </div>
    </form>
  </DialogFrame>
</Dialog.Root>

<style>
  .connection-dialog {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .connection-dialog__footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    border-block-start: 1px solid var(--border);
    padding-block-start: 1rem;
  }
  .connection-dialog__actions,
  .connection-dialog__management {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .connection-dialog__actions {
    margin-inline-start: auto;
  }
</style>
