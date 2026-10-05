<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
  import type { Snippet } from "svelte";

  let {
    title,
    description,
    tone = "warning",
    inline = false,
    details,
    children,
  }: {
    title?: string;
    description: string;
    tone?: "warning" | "destructive";
    inline?: boolean;
    details?: Snippet;
    children?: Snippet;
  } = $props();
</script>

<Alert.Root
  class="community-projects-column-alert"
  data-tone={tone}
  data-layout={inline ? "inline" : undefined}
  variant={tone === "destructive" ? "destructive" : "default"}
>
  {#if tone === "destructive"}
    <CircleAlertIcon aria-hidden="true" />
  {:else}
    <TriangleAlertIcon aria-hidden="true" />
  {/if}
  {#if title}<Alert.Title>{title}</Alert.Title>{/if}
  <Alert.Description class="community-projects-column-alert__description">
    <span>{description}</span>
    {@render details?.()}
    {#if children}
      <span class="community-projects-column-alert__actions">
        {@render children()}
      </span>
    {/if}
  </Alert.Description>
</Alert.Root>

<style>
  :global(
    [data-ui-component="alert"][data-ui-part="alert"].community-projects-column-alert
  ) {
    border-width: 0 0 1px;
    border-style: solid;
    border-color: var(--border);
    border-radius: 0;
  }

  :global(
    [data-ui-component="alert"][data-ui-part="alert"].community-projects-column-alert[data-tone="warning"]
  ) {
    background-color: var(--jj-store-warning-background);
    color: var(--foreground);
  }

  :global(
    [data-ui-component="alert"][data-ui-part="alert"].community-projects-column-alert[data-tone="destructive"]
  ) {
    background-color: color-mix(
      in srgb,
      var(--destructive) 10%,
      var(--background)
    );
    color: var(--foreground);
  }

  :global(
    .community-projects-column-alert[data-tone="warning"]
      [data-ui-part="alert-description"]
  ) {
    color: var(--foreground);
  }

  :global(
    .community-projects-column-alert[data-tone="destructive"]
      [data-ui-part="alert-description"]
  ) {
    color: var(--foreground);
  }

  :global(.community-projects-column-alert[data-tone="warning"] > svg) {
    color: var(--jj-store-warning-foreground);
  }

  :global(.community-projects-column-alert[data-tone="destructive"] > svg) {
    color: var(--destructive);
  }

  :global(.community-projects-column-alert__description) {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  :global(.community-projects-column-alert__actions) {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  :global(.community-projects-column-alert[data-layout="inline"]) {
    align-items: center;
    padding-block: 0.25rem;
    margin-block-end: 0.5rem;
  }
  :global(.community-projects-column-alert[data-layout="inline"] > svg) {
    grid-row: 1;
    align-self: center;
    translate: none;
  }
  :global(
    .community-projects-column-alert[data-layout="inline"]
      .community-projects-column-alert__description
  ) {
    flex-direction: row;
    align-items: center;
    min-width: 0;
  }
  :global(
    .community-projects-column-alert[data-layout="inline"]
      .community-projects-column-alert__description
      > span:first-child
  ) {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(
    .community-projects-column-alert[data-layout="inline"]
      .community-projects-column-alert__actions
  ) {
    flex: 0 0 auto;
    margin-inline-start: auto;
  }
</style>
