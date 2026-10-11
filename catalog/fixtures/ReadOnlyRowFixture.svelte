<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreReadOnlyRow from "../../web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    value = "ab".repeat(64),
    onOpen,
    copyable = true,
    lifecycle = false,
    presentation = "default",
  }: {
    value?: string | number | null;
    onOpen?: () => void;
    copyable?: boolean;
    lifecycle?: boolean;
    presentation?: "default" | "repository";
  } = $props();
  let replaced = $state(false),
    shown = $state(true);
</script>

<main class="store-settings">
  <dl class="store-settings__rows">
    {#if shown}<StoreReadOnlyRow
        label="Operation ID"
        value={replaced ? "replacement" : value}
        {onOpen}
        {copyable}
        {presentation}
      />{/if}
  </dl>
  {#if lifecycle}<Button onclick={() => (replaced = true)}>Replace value</Button
    ><Button onclick={() => (shown = !shown)}>Toggle row</Button>{/if}
</main>
