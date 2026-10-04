<script lang="ts">
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import InfoIcon from "@lucide/svelte/icons/info";
  import type { Snippet } from "svelte";
  let { children }: { children: Snippet } = $props();
</script>

<Alert.Root role="status">
  <InfoIcon aria-hidden="true" />
  <Alert.Description>{@render children()}</Alert.Description>
</Alert.Root>
