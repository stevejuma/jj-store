<script lang="ts">
  // CommunityProjectsTaskList/RepositorySections collection-group (Apache-2.0).
  import { sectionDisclosure } from "../disclosure-layout.svelte.js";
  import type { Component, Snippet } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import "./community-work-collections.css";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  let {
    title,
    id,
    count,
    Icon,
    children,
  }: {
    id: string;
    title: string;
    count?: number;
    Icon: Component;
    children: Snippet;
  } = $props();
  const disclosure = sectionDisclosure(() => ["collection", id]);
</script>

<Collapsible.Root
  class="community-projects__collection-group"
  bind:open={disclosure.open}
>
  <Collapsible.Trigger class="community-projects__list-group-heading">
    <Icon aria-hidden="true" /><strong>{title}</strong>
    {#if !disclosure.open && count !== undefined}<span
        class="community-projects__list-group-count">{count}</span
      >{/if}
    {#if disclosure.open}<ChevronDownIcon
        class="community-projects__list-group-chevron"
        aria-hidden="true"
      />{:else}<ChevronRightIcon
        class="community-projects__list-group-chevron"
        aria-hidden="true"
      />{/if}
  </Collapsible.Trigger>
  <Collapsible.Content>{@render children()}</Collapsible.Content>
</Collapsible.Root>
