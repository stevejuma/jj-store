<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Label } from "@lapismd/design-core/shadcn/label";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import CommunityChannelSettings from "../community-channel-settings/CommunityChannelSettings.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  import CommunityEntityChip from "../community-entity-chip/CommunityEntityChip.svelte";
  import StoreIdentityCopy from "../store-collaboration/StoreIdentityCopy.svelte";
  import { baselineLinks, activityLinks } from "./thread-context.js";
  import { threadWritable } from "./thread-view.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  let { controller }: { controller: StoreController } = $props();
  const threads = $derived(controller.threads);
  const detail = $derived(threads?.detail);
  const panel = useThreadPanel();
  const writable = $derived(threadWritable(controller));
  const baselines = $derived(
    baselineLinks(
      detail?.thread.value.baseline ?? null,
      controller.state.route,
    ),
  );
  const linked = $derived(
    activityLinks(threads?.events ?? [], controller.state.route),
  );
  let removeOpen = $state(false);
  const disabled = $derived(threads?.busy || !!threads?.pending);
  let renameOpen = $state(false),
    title = $state(""),
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
  async function run(action: () => Promise<void>) {
    error = "";
    try {
      await action();
      renameOpen = false;
      removeOpen = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

{#if detail && threads}<CommunityChannelSettings
    title={detail.thread.value.title}
    id={detail.thread.value.thread_id}
    onClose={panel.close}
    onMembers={() => panel.open("members")}
    onRename={writable && detail.thread.value.status !== "tombstoned"
      ? () => {
          title = detail.thread.value.title;
          renameOpen = true;
        }
      : undefined}
  >
    <div class="lc-channel-settings__row">
      <span
        ><strong>Lifecycle</strong><small>{detail.thread.value.status}</small
        ></span
      >
    </div>
    <div class="lc-channel-settings__row">
      <span
        ><strong>Revision</strong><StoreIdentityCopy
          value={detail.thread.reference.id}
          label="Thread revision"
        /></span
      >
    </div>
    <div class="lc-channel-settings__row">
      <span
        ><strong>Code baseline</strong
        >{#each baselines as row}<CommunityEntityChip
            link={row.link}
            label={row.label}
          />{:else}<small
            >{detail.thread.value.baseline?.profile === "portable"
              ? "Portable baseline recorded"
              : "No baseline recorded"}</small
          >{/each}</span
      >
    </div>
    <div class="lc-channel-settings__row">
      <span
        ><strong>Linked activity</strong
        >{#each linked as row}<CommunityEntityChip
            link={row.link}
            label={row.label}
          />{:else}<small>No active links in loaded history</small>{/each}</span
      >
    </div>
    {#snippet actions()}{#if writable}{#each transitions[detail.thread.value.status] as action}{#if action !== "restore" || controller.state.repository.permission === "admin"}<Button
              variant="ghost"
              class="lc-channel-settings__row"
              {disabled}
              onclick={() =>
                action === "tombstone"
                  ? (removeOpen = true)
                  : run(() => threads!.lifecycle(action))}
              >{action === "tombstone"
                ? "Remove thread"
                : action === "idle"
                  ? "Set idle"
                  : `${action[0]!.toUpperCase()}${action.slice(1)} thread`}</Button
            >{/if}{/each}{/if}{#if error}<p role="alert">
          {error}
        </p>{/if}{/snippet}
  </CommunityChannelSettings>{/if}
<Dialog.Root bind:open={renameOpen}
  ><Dialog.Content preventScroll={false}
    ><Dialog.Header
      ><Dialog.Title>Rename thread</Dialog.Title><Dialog.Description
        >Update this thread’s name.</Dialog.Description
      ></Dialog.Header
    >
    <form
      onsubmit={(e) => {
        e.preventDefault();
        void run(() => threads!.rename(title));
      }}
    >
      <Label for="rename-thread">Thread name</Label><Input
        id="rename-thread"
        bind:value={title}
        required
        maxlength={1024}
      />{#if error}<p role="alert">{error}</p>{/if}<Dialog.Footer
        ><Button type="submit" disabled={disabled || !title.trim()}
          >Save name</Button
        ></Dialog.Footer
      >
    </form></Dialog.Content
  ></Dialog.Root
>

<Dialog.Root bind:open={removeOpen}
  ><Dialog.Content preventScroll={false}
    ><Dialog.Header
      ><Dialog.Title>Remove thread?</Dialog.Title><Dialog.Description
        >Hide this thread and make its conversation read only. A repository
        administrator can restore it.</Dialog.Description
      ></Dialog.Header
    ><Dialog.Footer
      ><Button variant="outline" onclick={() => (removeOpen = false)}
        >Cancel</Button
      ><Button
        variant="destructive"
        {disabled}
        onclick={() => run(() => threads!.lifecycle("tombstone"))}
        >Remove thread</Button
      ></Dialog.Footer
    >{#if error}<p role="alert">{error}</p>{/if}</Dialog.Content
  ></Dialog.Root
>
