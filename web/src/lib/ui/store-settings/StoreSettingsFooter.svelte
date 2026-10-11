<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Card from "@lapismd/design-core/shadcn/card";
  import { settingsPresentation } from "./settings-presentation.js";
  let {
    description,
    children,
    joined = false,
    standalone = false,
  }: {
    description?: Snippet;
    children: Snippet;
    joined?: boolean;
    /** One shared save footer for a form spanning several repository cards. */
    standalone?: boolean;
  } = $props();
  const presentation = settingsPresentation();
</script>

{#snippet content()}
  {#if description}<div class="store-settings-footer__description">
      {@render description()}
    </div>{/if}
  <div class="store-settings__actions">{@render children()}</div>
{/snippet}

{#if presentation.value !== "default"}
  {#if standalone}
    <Card.Root
      size="sm"
      style="--card-spacing: 0"
      class="store-settings__section store-settings-save-card"
    >
      <Card.Footer
        class="store-settings__action-row store-settings-footer store-settings-footer--standalone"
        >{@render content()}</Card.Footer
      >
    </Card.Root>
  {:else}<Card.Footer class="store-settings__action-row store-settings-footer"
      >{@render content()}</Card.Footer
    >{/if}
{:else}
  <div
    class="store-settings__action-row"
    class:store-settings__action-row--joined={joined}
  >
    {@render content()}
  </div>
{/if}
