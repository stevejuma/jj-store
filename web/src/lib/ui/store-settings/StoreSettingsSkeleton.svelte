<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import type { RepositorySettingsSection } from "../../controller/repository-settings-navigation.js";
  import {
    settingsSkeletonCards,
    settingsSkeletonHeading,
  } from "./settings-skeleton.js";
  import "./settings-skeleton.css";
  import "./repository-settings.css";
  let {
    section = "general",
    tab,
    local = false,
    heading = true,
    rows,
    label = "Loading repository settings",
  }: {
    section?: RepositorySettingsSection;
    tab?: string;
    local?: boolean;
    heading?: boolean;
    /** Skeletonize a pending row group inside an already visible card. */
    rows?: number;
    label?: string;
  } = $props();
</script>

<div
  class="store-settings-skeleton"
  role="status"
  aria-label={label}
  aria-busy="true"
  data-section={section}
  data-tab={tab}
>
  <div aria-hidden="true">
    {#if rows !== undefined}
      {#each Array(rows) as _}{@render row("fact")}{/each}
    {:else}
      {#if heading && settingsSkeletonHeading(section, tab)}<div
          class="store-settings-skeleton__heading"
        >
          <Skeleton class="store-settings-skeleton__title" /><Skeleton
            class="store-settings-skeleton__description"
          />
        </div>{/if}
      {#each settingsSkeletonCards(section, tab, local) as card}
        <Card.Root
          size="sm"
          style="--card-spacing: 0"
          class="store-settings-card store-settings-skeleton__card"
        >
          <div
            class="store-settings-card__header store-settings-skeleton__header"
          >
            <Skeleton class="store-settings-skeleton__icon" /><Skeleton
              class="store-settings-skeleton__title"
            />
            {#if card.badge}<Skeleton
                class="store-settings-skeleton__badge"
              />{/if}
          </div>
          <Card.Content class="store-settings-card__content">
            {#each card.rows as kind}{@render row(kind)}{/each}
            {#if card.actions}<div class="store-settings-skeleton__footer">
                <Skeleton class="store-settings-skeleton__description" />
                <div>
                  {#each Array(card.actions) as _}<Skeleton
                      class="store-settings-skeleton__action"
                    />{/each}
                </div>
              </div>{/if}
          </Card.Content>
        </Card.Root>
      {/each}
    {/if}
  </div>
</div>

{#snippet row(kind: string)}
  <div class="store-settings-skeleton__row" data-kind={kind}>
    <div class="store-settings-skeleton__label">
      {#if kind === "fact"}<Skeleton
          class="store-settings-skeleton__row-icon"
        />{/if}
      <div>
        <Skeleton
          class="store-settings-skeleton__text"
        />{#if kind !== "fact"}<Skeleton
            class="store-settings-skeleton__help"
          />{/if}
      </div>
    </div>
    <div class="store-settings-skeleton__value" data-kind={kind}>
      <Skeleton class="store-settings-skeleton__control" />
      {#if kind === "numeric"}<Skeleton
          class="store-settings-skeleton__unit"
        />{/if}
    </div>
  </div>
{/snippet}
