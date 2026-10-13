<script lang="ts">
  import { untrack } from "svelte";
  import Message from "@lucide/svelte/icons/message-square";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import {
    threadSnapshotKey,
    type ThreadSnapshot,
  } from "../../controller/thread-types.js";
  import type { StoreDataSource } from "../../controller/native-data-source.js";
  let {
    id,
    source,
    value,
    disabled = false,
    onChange,
  }: {
    id: string;
    source?: StoreDataSource;
    value?: string;
    disabled?: boolean;
    onChange(value: string | undefined): void;
  } = $props();
  const { context } = ownNotifications();
  const notice = $derived(`workflow-threads-${id}`);
  let rows = $state<{ id: string; name: string; description: string }[]>([]),
    cursor = $state<string | null>(null),
    loading = $state(false);
  let generation = 0,
    snapshot: ThreadSnapshot | undefined;
  async function read(more = false) {
    const reader = source,
      ticket = generation;
    if (!reader?.threads || loading) return;
    loading = true;
    try {
      const page = await reader.threads.list(
        more ? (cursor ?? undefined) : undefined,
      );
      if (ticket !== generation || reader !== source) return;
      if (
        page.snapshot.repo_id !== reader.repository ||
        page.items.type !== "threads" ||
        (more &&
          snapshot &&
          (threadSnapshotKey(snapshot) !== threadSnapshotKey(page.snapshot) ||
            ("epoch" in snapshot &&
              (!("epoch" in page.snapshot) ||
                snapshot.epoch !== page.snapshot.epoch))))
      )
        throw Error("Thread list changed. Retry to refresh it.");
      snapshot = page.snapshot;
      rows = [
        ...new Map(
          [
            ...(more ? rows : []),
            ...page.items.threads
              .filter((t) => t.value.status !== "tombstoned")
              .map((t) => ({
                id: t.value.thread_id,
                name: t.value.title || "Untitled thread",
                description: t.value.status,
              })),
          ].map((t) => [t.id, t]),
        ).values(),
      ];
      cursor = page.next_cursor;
      context.service.resolve(context.scope, notice);
    } catch (cause) {
      if (ticket !== generation || reader !== source) return;
      context.service.publish({
        scope: context.scope,
        key: notice,
        title: "Threads could not be loaded",
        message:
          cause instanceof Error
            ? cause.message
            : "Try loading the thread list again.",
        severity: "error",
        lifetime: "condition",
        action: { id: "retry-threads", label: "Retry", run: () => void read() },
      });
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  $effect(() => {
    const reader = source;
    untrack(() => {
      generation++;
      rows = [];
      cursor = null;
      snapshot = undefined;
      loading = false;
      if (reader) void read();
    });
    return () => {
      generation++;
      context.service.resolve(context.scope, notice);
    };
  });
</script>

<StoreRepositoryPicker
  {id}
  label="Thread"
  repositories={rows}
  {value}
  controlled
  allowEmpty
  emptyLabel="Any thread"
  unavailableLabel="Thread"
  searchLabel="Find a thread"
  groupLabel="Threads"
  emptyResult={loading ? "Loading threads…" : "No matching threads."}
  disabled={disabled || !source?.threads}
  {loading}
  hasMore={!!cursor}
  moreLabel="threads"
  onLoadMore={() => void read(true)}
  onChange={(next) => onChange(next || undefined)}
  onSearch={(query) => {
    if (!query && !loading) void read();
  }}
>
  {#snippet triggerIcon()}<Message aria-hidden="true" />{/snippet}
  {#snippet itemIcon()}<Message aria-hidden="true" />{/snippet}
</StoreRepositoryPicker>
