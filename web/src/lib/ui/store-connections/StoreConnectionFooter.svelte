<script lang="ts">
  import StoreServerSwitcher from "./StoreServerSwitcher.svelte";
  import StoreHelp from "../store-help/StoreHelp.svelte";
  import { version } from "../../../../../package.json";
  let {
    compact = false,
    showVersion = false,
  }: { compact?: boolean; showVersion?: boolean } = $props();
</script>

<StoreServerSwitcher {compact} /><StoreHelp {compact} sidebar />

{#if showVersion}
  <footer
    class="store-connection-footer__version"
    data-compact={compact || undefined}
    aria-label={`jj-store version ${version}`}
    title={`jj-store v${version}`}
  >
    v{version}
  </footer>
{/if}

<style>
  .store-connection-footer__version {
    padding: 0.5rem 1.5rem 1rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    white-space: nowrap;
  }
  .store-connection-footer__version[data-compact] {
    padding-inline: 0;
    text-align: center;
    font-size: 0.6rem;
  }
</style>
