<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { terminalControls } from "../../terminal/controls.svelte.js";
  import StoreTerminalToggleIcon from "./StoreTerminalToggleIcon.svelte";
  let { disabled = false }: { disabled?: boolean } = $props();
  const controls = terminalControls();
</script>

{#if controls?.available}
  <span
    class="community-app-shell__sidebar-action store-terminal-header-toggle"
  >
    <Button
      bind:ref={controls.trigger}
      variant="default"
      size="icon-sm"
      aria-label={controls.open ? "Hide terminal" : "Show terminal"}
      aria-expanded={controls.open}
      aria-controls="store-terminal-dock"
      title="Terminal (⌘J)"
      {disabled}
      onclick={() => controls.toggle()}
      ><StoreTerminalToggleIcon expanded={controls.open} /></Button
    >
  </span>
{/if}
