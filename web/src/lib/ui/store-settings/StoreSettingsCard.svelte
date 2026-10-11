<script lang="ts">
  import type { Component, Snippet } from "svelte";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import StoreSettingsHeading from "./StoreSettingsHeading.svelte";
  import { settingsPresentation } from "./settings-presentation.js";
  import "./repository-settings.css";
  let {
    id,
    title,
    description,
    Icon,
    badge,
    children,
    layout = "card",
    repositoryOnly = false,
  }: {
    id: string;
    title: string;
    description?: string;
    Icon?: Component;
    badge?: Snippet;
    children: Snippet;
    /** A page heading groups sibling cards without an outer card border. */
    layout?: "card" | "page";
    /** Preserve the original grouping for legacy consumers. */
    repositoryOnly?: boolean;
  } = $props();
  const presentation = settingsPresentation();
</script>

{#if presentation.value === "repository" && layout === "page"}
  <div class="store-settings__content-heading">
    <h1 {id}>{title}</h1>
    {#if description}<p>{description}</p>{/if}
  </div>
  {@render children()}
{:else if presentation.value === "repository"}
  <Card.Root
    size="sm"
    style="--card-spacing: 0"
    class="store-settings__section store-settings-card"
    role="region"
    aria-labelledby={id}
  >
    <div class="store-settings-card__header">
      <Card.Header>
        <Card.Title
          ><StoreSettingsHeading
            {id}
            {title}
            {description}
            {Icon}
            {badge}
          /></Card.Title
        >
      </Card.Header>
    </div>
    <Card.Content class="store-settings-card__content"
      >{@render children()}</Card.Content
    >
  </Card.Root>
{:else if repositoryOnly}
  {@render children()}
{:else}
  <section class="store-settings__section" aria-labelledby={id}>
    <header class="store-settings__section-heading">
      <div>
        <h2 {id}>{title}</h2>
        {#if description}<p>{description}</p>{/if}
      </div>
      {@render badge?.()}
    </header>
    {@render children()}
  </section>
{/if}
