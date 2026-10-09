<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { MetadataQuery } from "../../controller/conversation-metadata.svelte.js";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Badge } from "@lapismd/design-core/shadcn/badge";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { manager, query }: { manager: AgentExecution; query: MetadataQuery } =
    $props();
  let entry = $state(untrack(() => manager.metadata.entry(query)));
  const id = $props.id();
  const busy = $derived(
    entry.submitting ||
      !!entry.pending ||
      (!!entry.job &&
        ["prepared", "generating", "publishing"].includes(entry.job.state)),
  );
  $effect(() => {
    let disposed = false;
    const current = query,
      revision = manager.value?.revision;
    untrack(() => {
      entry = manager.metadata.entry(current);
      void revision;
      void manager.metadata.load(current).then(() => {
        if (
          !disposed &&
          entry.job &&
          ["prepared", "generating", "publishing"].includes(entry.job.state)
        )
          void manager.metadata.poll(current);
      });
    });
    return () => {
      disposed = true;
      manager.metadata.leave(current);
    };
  });
  onDestroy(() => manager.metadata.leave(query));
  const generated = $derived(entry.value?.value.generated);
  function formatTime(value: unknown) {
    return typeof value === "number" && value > 0
      ? new Date(value).toLocaleString()
      : undefined;
  }
</script>

<section aria-label="Conversation details" class="lc-channel-settings">
  <h2>Overview</h2>
  {#if entry.loading && !entry.value}<p role="status">
      Loading conversation details…
    </p>{/if}
  {#if entry.value}
    <StoreReadOnlyRow
      standalone
      label="Conversation title"
      value={entry.value.title ?? "New conversation"}
    />
    <Field.Group class="store-settings__editable-rows"
      ><StoreWorkField label="Rename conversation" id={`${id}-title`}
        ><Input
          id={`${id}-title`}
          bind:value={entry.rename}
          disabled={busy || !entry.value.writable}
        /></StoreWorkField
      ></Field.Group
    >
    <div class="store-settings__action-row">
      <Button
        size="sm"
        disabled={busy || !entry.rename.trim() || !entry.value.writable}
        onclick={() =>
          manager.metadata.command(query, {
            action: "rename",
            title: entry.rename.trim(),
          })}>Rename</Button
      ><Button
        variant="ghost"
        size="sm"
        disabled={busy ||
          !entry.value.value.custom_title ||
          !entry.value.writable}
        onclick={() =>
          manager.metadata.command(query, { action: "reset_title" })}
        >Use suggested title</Button
      >
    </div>
    <StoreReadOnlyRow
      standalone
      label="Repository ID"
      value={entry.value.repository}
      copyable
    />
    <StoreReadOnlyRow
      standalone
      label="Conversation ID"
      value={entry.value.target.type === "conversation"
        ? entry.value.target.id
        : undefined}
      copyable
    />
    {#each Array.isArray(entry.value.details) ? entry.value.details : [] as capture}
      <StoreReadOnlyRow
        standalone
        label="Captured session ID"
        value={capture.session?.session_id}
        copyable
      />
      <StoreReadOnlyRow
        standalone
        label="Snapshot ID"
        value={capture.session?.snapshot_id}
        copyable
      />
      <StoreReadOnlyRow
        standalone
        label="Capture status"
        value={capture.session?.state}
      />
      <StoreReadOnlyRow
        standalone
        label="First captured activity"
        value={formatTime(capture.session?.first_activity_at_millis)}
      />
      <StoreReadOnlyRow
        standalone
        label="Last captured activity"
        value={formatTime(capture.session?.last_activity_at_millis)}
      />
      <StoreReadOnlyRow
        standalone
        label="Captured messages / tools"
        value={`${capture.counts?.user ?? 0} user · ${capture.counts?.assistant ?? 0} assistant · ${capture.counts?.tools ?? 0} tools${capture.partial ? " (partial)" : ""}`}
      />
    {/each}
    {#if entry.value.unavailable}<p role="status">
        {entry.value.unavailable}
      </p>{/if}
    <h3>Generated metadata</h3>
    {#if generated}<Badge variant="secondary"
        >{entry.value.fresh ? "Fresh" : "Stale"}{generated.truncated
          ? " · Partial capture"
          : ""}</Badge
      ><StoreReadOnlyRow
        standalone
        label="Generated title"
        value={generated.title}
      />
      <p>{generated.summary}</p>
      <StoreReadOnlyRow
        standalone
        label="Keywords"
        value={generated.keywords.join(", ")}
      /><StoreReadOnlyRow
        standalone
        label="Goals"
        value={generated.goals.join(" · ")}
      /><StoreReadOnlyRow
        standalone
        label="Generation model"
        value={`${generated.provider} · ${generated.model}`}
      /><StoreReadOnlyRow
        standalone
        label="Generated"
        value={formatTime(generated.generated_at_millis)}
      />{:else}<p>No generated metadata yet.</p>{/if}
    <div class="store-settings__action-row">
      <Button
        size="sm"
        variant="ghost"
        disabled={busy ||
          !entry.value.writable ||
          !manager.settings.value?.metadata}
        onclick={() => manager.metadata.command(query, { action: "refresh" })}
        ><RefreshIcon data-icon="inline-start" />Refresh metadata</Button
      ><Button
        size="sm"
        variant="ghost"
        disabled={busy ||
          !entry.value.writable ||
          !manager.settings.value?.metadata}
        onclick={() =>
          manager.metadata.command(query, { action: "regenerate_title" })}
        >Regenerate title</Button
      >
    </div>
    {#if !manager.settings.value?.metadata}<p>
        Choose a metadata model in Settings → Agents → Conversation metadata.
      </p>{/if}
    {#if entry.job}<p role="status">Metadata request: {entry.job.state}</p>
      {#if entry.job.state === "outcome_unknown" && !entry.job.result && ["refresh", "regenerate_title"].includes(entry.job.original_command?.action.action ?? "")}
        <Button
          variant="ghost"
          size="sm"
          onclick={() =>
            manager.metadata.command(query, {
              action: "cancel",
              job: entry.job!.request_id,
            })}>Stop waiting for generation</Button
        >
      {/if}
      {#if entry.job && ["prepared", "generating"].includes(entry.job.state)}<Button
          variant="ghost"
          size="sm"
          onclick={() =>
            manager.metadata.command(query, {
              action: "cancel",
              job: entry.job!.request_id,
            })}>Cancel generation</Button
        >{:else if entry.pending || entry.job.state === "outcome_unknown"}<Button
          variant="ghost"
          size="sm"
          onclick={() => manager.metadata.retry(query)}
          >Check retained request</Button
        >{/if}{#if entry.job.result && ["failed", "outcome_unknown"].includes(entry.job.state)}<Button
          variant="ghost"
          size="sm"
          onclick={() =>
            manager.metadata.command(query, {
              action: "publish",
              job: entry.job!.request_id,
            })}>Publish retained result</Button
        >{/if}{/if}
  {/if}
  <StoreNotice
    noticeKey={`conversation-metadata-${query.session}`}
    title="Conversation metadata needs attention"
    message={entry.error}
  />
</section>
