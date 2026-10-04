<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Label } from "@lapismd/design-core/shadcn/label";
  import type { ThreadsController } from "../../controller/thread-types.js";
  let {
    threads,
    disabled = false,
    open = $bindable(false),
    showTrigger = true,
  }: {
    threads: ThreadsController;
    disabled?: boolean;
    open?: boolean;
    showTrigger?: boolean;
  } = $props();
  let title = $state(""),
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
  {#if showTrigger}<Button
      variant="outline"
      disabled={disabled || threads.busy || !!threads.pending}
      onclick={() => (open = true)}>New thread</Button
    >{/if}
  <Dialog.Content preventScroll={false}>
    <Dialog.Header
      ><Dialog.Title>New thread</Dialog.Title><Dialog.Description
        >Create a thread for this repository’s conversation.</Dialog.Description
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
      <StoreNotice
        noticeKey="thread-create"
        title="Could not create thread"
        message={error}
        pending={threads.busy}
      />
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
