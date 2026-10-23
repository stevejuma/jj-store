<script lang="ts">
  import { onDestroy } from "svelte";
  import { createServerConnections } from "./lib/connections/owner.svelte.js";
  import StoreApplicationRoot from "./lib/ui/store-application/StoreApplicationRoot.svelte";
  import type { AppearanceStore } from "./lib/appearance/store.svelte.js";
  let { appearance }: { appearance?: AppearanceStore } = $props();
  const connections = createServerConnections();
  onDestroy(() => connections.destroy());
</script>

<StoreApplicationRoot {connections} {appearance} />
