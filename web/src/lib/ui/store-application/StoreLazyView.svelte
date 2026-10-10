<script lang="ts" generics="Props extends Record<string, unknown>">
  import { untrack, type Component, type Snippet } from "svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let {
    load,
    label,
    active = true,
    loading,
    content,
  }: {
    load: () => Promise<{ default: Component<Props> }>;
    label: string;
    active?: boolean;
    loading?: Snippet;
    content: Snippet<[Component<Props>]>;
  } = $props();
  let requested = $state(false);
  let attempt = $state(0);
  let pending = $state.raw<Promise<{ default: Component<Props> }> | null>(null);
  // Once opened, retain closed overlays so their drafts and cleanup still run.
  $effect(() => {
    if (active) requested = true;
  });
  $effect(() => {
    attempt;
    const importer = load;
    pending = requested ? untrack(importer) : null;
  });
</script>

{#if pending}
  {#await pending}
    {#if active}
      {#if loading}{@render loading()}{:else}<CommunitySkeleton
          layout="details"
          count={3}
          padded
          label={`Loading ${label}`}
        />{/if}
    {/if}
  {:then module}
    {@render content(module.default)}
  {:catch}
    {#if active}
      <StoreNotice
        noticeKey={`view-load:${label}`}
        title={`${label} unavailable`}
        message="The view could not be loaded."
        action={{
          id: `retry-view:${label}`,
          label: "Retry",
          run: () => {
            attempt += 1;
          },
        }}
      />
    {/if}
  {/await}
{/if}
