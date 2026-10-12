<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import UnlinkIcon from "@lucide/svelte/icons/unlink";
  let {
    disabled,
    onReconnect,
    onRemove,
  }: {
    disabled: boolean;
    onReconnect: () => void;
    onRemove: () => void;
  } = $props();
</script>

<Button variant="outline" {disabled} onclick={onReconnect}
  ><GitBranchIcon aria-hidden="true" />Reconnect folder</Button
>
<Button variant="outline" {disabled} onclick={onRemove}
  ><UnlinkIcon aria-hidden="true" />Remove registration</Button
>
