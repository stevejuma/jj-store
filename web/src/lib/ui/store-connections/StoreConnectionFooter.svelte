<script lang="ts">
  import StoreServerSwitcher from "./StoreServerSwitcher.svelte";
  import StoreHelp from "../store-help/StoreHelp.svelte";
  let { compact = false }: { compact?: boolean } = $props();
</script>

<StoreServerSwitcher {compact} /><StoreHelp {compact} sidebar />
