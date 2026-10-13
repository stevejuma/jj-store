<script lang="ts">
  import { untrack } from "svelte";
  import Message from "@lucide/svelte/icons/message-square";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type {
    StoreController,
    StoreRepositorySummary,
  } from "../../controller/contracts.js";
  let {
    id,
    navigation,
    repositories,
    repository,
    value,
    disabled = false,
    onChange,
  }: {
    id: string;
    navigation?: StoreController["threadNavigation"];
    repositories: readonly StoreRepositorySummary[];
    repository: string;
    value?: string;
    disabled?: boolean;
    onChange(value: string | undefined, repository?: string): void;
  } = $props();
  const homes = $derived(
    repositories.filter(
      (r) =>
        r.profile !== "git_sha1_jj045" &&
        !!r.permission &&
        !["unavailable", "identity_changed"].includes(r.availability ?? ""),
    ),
  );
  const entries = $derived(
    homes.flatMap((home) => {
      const entry = navigation?.entries[home.id];
      return entry ? [{ home, entry }] : [];
    }),
  );
  const rows = $derived(
    entries.flatMap(({ home, entry }) =>
      entry.threads
        .filter(
          (t) =>
            t.value.status !== "tombstoned" &&
            !t.value.purpose &&
            !t.value.agent_message &&
            !t.value.comment_source,
        )
        .map((t) => ({
          id: `${home.id}:${t.value.thread_id}`,
          name: t.value.title || "Untitled thread",
          description: home.name,
        })),
    ),
  );
  const selected = $derived(value ? `${repository}:${value}` : "");
  const loading = $derived(entries.some((e) => e.entry.loading));
  const more = $derived(entries.filter((e) => !!e.entry.cursor));
  async function refresh() {
    if (navigation)
      await navigation.load(true, new Set(homes.map((h) => h.id)));
  }
  $effect(() => {
    const reader = navigation,
      allowed = homes.map((h) => h.id).join(",");
    if (!reader) return;
    untrack(() => void refresh());
    const timer = setInterval(() => void refresh(), 5000);
    window.addEventListener("focus", refresh);
    window.addEventListener("online", refresh);
    return () => {
      clearInterval(timer);
      window.removeEventListener("focus", refresh);
      window.removeEventListener("online", refresh);
    };
  });
</script>

{#each entries as { home, entry } (home.id)}
  <StoreNotice
    noticeKey={`workflow-threads-${id}-${home.id}`}
    title={`Threads could not be loaded from ${home.name}`}
    message={entry.error}
    pending={entry.loading}
    action={{ id: "retry-threads", label: "Retry", run: () => void refresh() }}
  />
{/each}
<StoreRepositoryPicker
  {id}
  label="Thread"
  repositories={rows}
  value={selected}
  controlled
  allowEmpty
  emptyLabel="Any thread"
  unavailableLabel="Thread"
  searchLabel="Find a thread"
  groupLabel="Threads"
  emptyResult={loading
    ? "Loading threads…"
    : "No matching threads in accessible metadata homes."}
  disabled={disabled || !navigation}
  notice={loading ? "Loading threads…" : undefined}
  {loading}
  hasMore={!!more.length}
  moreLabel="threads"
  onOpen={() => void refresh()}
  onLoadMore={() => {
    for (const { home } of more) void navigation?.more(home.id);
  }}
  onChange={(next) => {
    const row = rows.find((r) => r.id === next);
    onChange(
      row ? next.slice(33) : undefined,
      row ? next.slice(0, 32) : undefined,
    );
  }}
>
  {#snippet triggerIcon()}<Message aria-hidden="true" />{/snippet}
  {#snippet itemIcon()}<Message aria-hidden="true" />{/snippet}
</StoreRepositoryPicker>
