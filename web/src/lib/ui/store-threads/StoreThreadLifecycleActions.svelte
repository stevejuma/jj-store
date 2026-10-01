<!-- @community-copy src/components/community-channel-settings/CommunityChannelSettings.svelte -->
<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import Archive from "@lucide/svelte/icons/archive";
  import Pause from "@lucide/svelte/icons/pause";
  import Play from "@lucide/svelte/icons/play";
  import RotateCcw from "@lucide/svelte/icons/rotate-ccw";
  import Trash2 from "@lucide/svelte/icons/trash-2";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  import { threadWritable } from "./thread-view.js";
  let { controller }: { controller: StoreController } = $props();
  const threads = $derived(controller.threads);
  const detail = $derived(threads?.detail);
  const disabled = $derived(threads?.busy || !!threads?.pending);
  let removeOpen = $state(false),
    error = $state("");
  const transitions: Record<
    Model<"ThreadVersionV1">["status"],
    Model<"ThreadActionV1">[]
  > = {
    created: ["activate"],
    active: ["idle", "archive", "tombstone"],
    idle: ["activate", "archive", "tombstone"],
    archived: ["restore", "tombstone"],
    tombstoned: ["restore"],
  };
  const allowed = $derived(
    detail && threadWritable(controller)
      ? transitions[detail.thread.value.status].filter(
          (action) =>
            action !== "restore" ||
            controller.state.repository.permission === "admin",
        )
      : [],
  );
  const management = $derived(
    allowed.filter((action) => action !== "tombstone"),
  );
  async function run(action: Model<"ThreadActionV1">) {
    error = "";
    try {
      await threads?.lifecycle(action);
      removeOpen = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

{#if management.length}
  <section
    class="lc-channel-settings__management-zone"
    aria-label="Thread lifecycle"
  >
    {#each management as action}
      <Button
        variant="ghost"
        class="lc-channel-settings__row"
        {disabled}
        onclick={() => run(action)}
      >
        {#if action === "activate"}<Play
            data-icon="inline-start"
            aria-hidden="true"
          />
        {:else if action === "idle"}<Pause
            data-icon="inline-start"
            aria-hidden="true"
          />
        {:else if action === "archive"}<Archive
            data-icon="inline-start"
            aria-hidden="true"
          />
        {:else}<RotateCcw data-icon="inline-start" aria-hidden="true" />{/if}
        <span
          >{action === "idle"
            ? "Set idle"
            : `${action[0]!.toUpperCase()}${action.slice(1)} thread`}</span
        >
      </Button>
    {/each}
  </section>
{/if}
{#if allowed.includes("tombstone")}
  <section
    class="lc-channel-settings__danger-zone"
    aria-label="Destructive thread actions"
  >
    <Button
      variant="ghost"
      class="lc-channel-settings__row lc-channel-settings__destructive"
      {disabled}
      onclick={() => {
        error = "";
        removeOpen = true;
      }}
    >
      <Trash2 data-icon="inline-start" aria-hidden="true" /><span
        >Remove thread</span
      >
    </Button>
  </section>
{/if}
<Dialog.Root bind:open={removeOpen}>
  <Dialog.Content preventScroll={false}>
    <Dialog.Header
      ><Dialog.Title>Remove thread?</Dialog.Title><Dialog.Description>
        Hide this thread and make its conversation read only. A repository
        administrator can restore it.
      </Dialog.Description></Dialog.Header
    >
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (removeOpen = false)}
        >Cancel</Button
      >
      <Button variant="destructive" {disabled} onclick={() => run("tombstone")}
        >Remove thread</Button
      >
    </Dialog.Footer>
    {#if error}<p role="alert">{error}</p>{/if}
  </Dialog.Content>
</Dialog.Root>
