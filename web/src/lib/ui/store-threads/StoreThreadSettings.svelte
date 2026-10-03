<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Label } from "@lapismd/design-core/shadcn/label";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import CommunityChannelSettings from "../community-channel-settings/CommunityChannelSettings.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityEntityChip from "../community-entity-chip/CommunityEntityChip.svelte";
  import StoreIdentityCopy from "../store-collaboration/StoreIdentityCopy.svelte";
  import { baselineLinks, activityLinks } from "./thread-context.js";
  import { threadWritable } from "./thread-view.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import StoreThreadLifecycleActions from "./StoreThreadLifecycleActions.svelte";
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
  const disabled = $derived(threads?.busy || !!threads?.pending);
  let renameOpen = $state(false),
    title = $state(""),
    error = $state("");
  async function run(action: () => Promise<void>) {
    error = "";
    try {
      await action();
      renameOpen = false;
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
    {#snippet actions()}<StoreThreadLifecycleActions {controller} />{/snippet}
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
      /><StoreNotice
        noticeKey="thread-settings"
        title="Could not update thread"
        message={error}
        pending={false}
      /><Dialog.Footer
        ><Button type="submit" disabled={disabled || !title.trim()}
          >Save name</Button
        ></Dialog.Footer
      >
    </form></Dialog.Content
  ></Dialog.Root
>
