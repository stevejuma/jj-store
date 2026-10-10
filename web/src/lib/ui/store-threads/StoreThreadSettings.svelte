<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import CommunityChannelSettings from "../community-channel-settings/CommunityChannelSettings.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityEntityChip from "../community-entity-chip/CommunityEntityChip.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import { baselineLinks, activityLinks } from "./thread-context.js";
  import { threadWritable } from "./thread-view.js";
  import { useThreadPanel } from "./thread-panel.svelte.js";
  import StoreThreadLifecycleActions from "./StoreThreadLifecycleActions.svelte";
  let {
    controller,
    embedded = false,
    onClose,
  }: {
    controller: StoreController;
    embedded?: boolean;
    onClose?: () => void;
  } = $props();
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
    {embedded}
    onClose={onClose ?? panel.close}
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
    <StoreReadOnlyRow
      copyable
      label="Revision"
      copyLabel="Thread revision"
      value={detail.thread.reference.id}
      standalone
      stacked
    />
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
<Dialog.Root bind:open={renameOpen}>
  <DialogFrame
    size="wide"
    preventScroll={false}
    busy={disabled}
    title="Rename thread"
    description="Update this thread’s name."
  >
    <form
      id="rename-thread-form"
      onsubmit={(event) => {
        event.preventDefault();
        void run(() => threads!.rename(title));
      }}
    >
      <Field.Group class="store-work__fields community-settings__option-group">
        <StoreWorkField label="Thread name" id="rename-thread" {disabled}>
          <Input
            id="rename-thread"
            bind:value={title}
            {disabled}
            required
            maxlength={1024}
          />
        </StoreWorkField>
      </Field.Group>
    </form>
    <StoreNotice
      noticeKey="thread-settings"
      title="Could not update thread"
      message={error}
      pending={false}
    />
    {#snippet footer()}
      <Button
        type="button"
        variant="outline"
        {disabled}
        onclick={() => (renameOpen = false)}>Cancel</Button
      >
      <Button
        type="submit"
        form="rename-thread-form"
        disabled={disabled || !title.trim()}>Save name</Button
      >
    {/snippet}
  </DialogFrame>
</Dialog.Root>
