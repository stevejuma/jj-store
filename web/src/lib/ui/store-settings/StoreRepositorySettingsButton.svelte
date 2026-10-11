<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  let { disabled = false, onOpen }: { disabled?: boolean; onOpen(): void } =
    $props();
</script>

<Tooltip.Provider
  ><Tooltip.Root
    ><Tooltip.Trigger>
      {#snippet child({ props })}<Button
          {...props}
          variant="ghost"
          size="icon-sm"
          aria-label="Repository settings"
          {disabled}
          onclick={onOpen}><SettingsIcon aria-hidden="true" /></Button
        >{/snippet}
    </Tooltip.Trigger><Tooltip.Content>Repository settings</Tooltip.Content
    ></Tooltip.Root
  ></Tooltip.Provider
>
