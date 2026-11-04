<script lang="ts">
  import { onDestroy } from "svelte";
  import { createServerConnections } from "./lib/connections/owner.svelte.js";
  import StoreApplicationRoot from "./lib/ui/store-application/StoreApplicationRoot.svelte";
  import type { AppearanceStore } from "./lib/appearance/store.svelte.js";
  import { browserHost, type HostEnvironment } from "./lib/host/contracts.js";
  import { provideHostEnvironment } from "./lib/host/context.js";
  import { untrack } from "svelte";
  let {
    appearance,
    host = browserHost,
  }: { appearance?: AppearanceStore; host?: HostEnvironment } = $props();
  provideHostEnvironment(untrack(() => host));
  const connections = createServerConnections({ host: untrack(() => host) });
  onDestroy(() => connections.destroy());
</script>

<StoreApplicationRoot {connections} {appearance} />
