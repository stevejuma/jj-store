<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryPicker from "../../web/src/lib/ui/store-application/StoreRepositoryPicker.svelte";
  let { onChange }: { onChange: (value: string) => void } = $props();
  let disabled = $state(false);
</script>

<Button onclick={() => (disabled = !disabled)}>Toggle pending read</Button>
<StoreRepositoryPicker
  repositories={[{ id: "41".repeat(16), name: "Parser" }]}
  allowEmpty
  hasMore
  {disabled}
  {onChange}
/>
