<script lang="ts">
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import type { Component, Snippet } from "svelte";
  let {
    id,
    title,
    description,
    Icon,
    badge,
  }: {
    id: string;
    title: string;
    description?: string;
    Icon?: Component;
    badge?: Snippet;
  } = $props();
</script>

<div class="store-settings-heading">
  {#if Icon}<StoreSettingsIcon {Icon} size="section" />{/if}
  <div class="store-settings-heading__text">
    <h2 {id}>{title}</h2>
    {#if description}<Card.Description
        class="store-settings-heading__description"
        >{description}</Card.Description
      >{/if}
  </div>
  {@render badge?.()}
</div>

<style>
  .store-settings-heading {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
    flex-wrap: wrap;
  }
  .store-settings-heading__text {
    flex: 1;
    min-width: min(12rem, 100%);
  }
  h2 {
    margin: 0;
    font-size: var(--community-text-body);
    font-weight: 600;
  }
  :global(.store-settings-heading__description) {
    margin: 0.25rem 0 0;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    font-weight: 400;
  }
</style>
