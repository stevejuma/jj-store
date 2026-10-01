<script lang="ts" generics="T extends { actor: { created_by: string } }">
  import type { Component, Snippet } from "svelte";
  import StoreWorkCollection from "./StoreWorkCollection.svelte";
  let {
    rows,
    principal,
    isMine,
    kind,
    Icon,
    children,
  }: {
    rows: readonly T[];
    principal?: string;
    kind: "projects" | "tasks";
    isMine?: (row: T) => boolean;
    Icon: Component;
    children: Snippet<[readonly T[]]>;
  } = $props();
  const belongs = (row: T) =>
    isMine
      ? isMine(row)
      : Boolean(principal && row.actor.created_by === principal);
  const mine = $derived(rows.filter(belongs));
  const other = $derived(rows.filter((row) => !belongs(row)));
</script>

<div class="community-projects__collection-groups">
  {#if mine.length}<StoreWorkCollection title="Mine" count={mine.length} {Icon}
      >{@render children(mine)}</StoreWorkCollection
    >{/if}
  {#if other.length}<StoreWorkCollection
      title={`Other ${kind}`}
      count={other.length}
      {Icon}>{@render children(other)}</StoreWorkCollection
    >{/if}
</div>
