<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Label } from "@lapismd/design-core/shadcn/label";
  import type { ThreadsController } from "../../controller/thread-types.js";
  let {
    threads,
    disabled = false,
  }: { threads: ThreadsController; disabled?: boolean } = $props();
  let open = $state(false),
    title = $state(""),
    error = $state("");
  async function create() {
    error = "";
    try {
      await threads.create(title);
      open = false;
      title = "";
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

<Dialog.Root bind:open>
  <Button
    variant="outline"
    disabled={disabled || threads.busy || !!threads.pending}
    onclick={() => (open = true)}>New thread</Button
  >
  <Dialog.Content>
    <Dialog.Header
      ><Dialog.Title>New thread</Dialog.Title><Dialog.Description
        >Create a channel for this repository’s conversation.</Dialog.Description
      ></Dialog.Header
    >
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void create();
      }}
    >
      <Label for="thread-title">Thread name</Label>
      <Input id="thread-title" bind:value={title} maxlength={1024} required />
      {#if error}<p role="alert">{error}</p>{/if}
      <Dialog.Footer
        ><Button
          type="submit"
          disabled={!title.trim() || threads.busy || !!threads.pending}
          >{threads.busy ? "Creating…" : "Create thread"}</Button
        ></Dialog.Footer
      >
    </form>
  </Dialog.Content>
</Dialog.Root>
