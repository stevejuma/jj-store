<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import LinkIcon from "@lucide/svelte/icons/link";
  import { linkSite } from "../../links/site-links.js";
  let {
    href,
    label = "",
    title,
    children,
    ref = $bindable(null),
  }: {
    href: string;
    label?: string;
    title?: string;
    children?: Snippet;
    ref?: HTMLAnchorElement | null;
  } = $props();
  const site = $derived(linkSite(href));
</script>

<Tooltip.Provider delayDuration={200}>
  <Tooltip.Root disabled={label === href}>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        <a
          {...props}
          bind:this={ref}
          class="store-external-link"
          {href}
          {title}
          target="_blank"
          rel="noopener noreferrer"
        >
          {#if site}<svg
              class="store-external-link__icon"
              data-link-site={site.name}
              viewBox="0 0 24 24"
              fill="currentColor"
              aria-hidden="true"><path d={site.path} /></svg
            >{:else}<LinkIcon aria-hidden="true" />{/if}<span
            >{@render children?.()}{#if !children}{label || href}{/if}</span
          >
        </a>
      {/snippet}
    </Tooltip.Trigger>
    <Tooltip.Content role="tooltip" side="top" sideOffset={6}
      ><span class="store-external-link__destination">{href}</span
      ></Tooltip.Content
    >
  </Tooltip.Root>
</Tooltip.Provider>

<style>
  .store-external-link {
    color: var(--primary);
    text-decoration: underline;
    text-underline-offset: 0.2em;
    overflow-wrap: anywhere;
  }
  .store-external-link :global(svg) {
    display: inline-block;
    width: 0.95em;
    height: 0.95em;
    margin-inline-end: 0.25em;
    vertical-align: -0.12em;
  }
  .store-external-link:hover {
    color: var(--foreground);
  }
  .store-external-link:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
    border-radius: 2px;
  }
  .store-external-link__destination {
    overflow-wrap: anywhere;
  }
</style>
