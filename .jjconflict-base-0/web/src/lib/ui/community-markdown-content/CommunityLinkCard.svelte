<script lang="ts">
  import { useAppearance } from "../../appearance/store.svelte.js";
  import { localLink } from "../../appearance/local-link.js";
  let { href, label = "" }: { href: string; label?: string } = $props();
  const appearance = useAppearance();
  const link = $derived(localLink(href, label));
</script>

{#if link}<a
    class="store-link-card"
    data-layout={appearance?.live.links ?? "compact"}
    href={link.href}
    aria-label={link.label}
    target="_blank"
    rel="noopener noreferrer"
    ><span class="store-link-card__mark" aria-hidden="true">↗</span><span
      ><small>{link.host}</small><strong>{link.label}</strong><span
        class="store-link-card__address">{link.href}</span
      ></span
    ></a
  >{/if}

<style>
  .store-link-card {
    display: inline-flex;
    align-items: center;
    vertical-align: middle;
    max-width: 100%;
    box-sizing: border-box;
    gap: 0.6rem;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: 0.65rem;
    background: var(--card, var(--background));
    color: var(--foreground);
    text-decoration: none;
    overflow-wrap: anywhere;
  }
  .store-link-card:hover {
    border-color: var(--primary);
  }
  .store-link-card:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
  }
  .store-link-card small {
    display: block;
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }
  .store-link-card strong {
    font-size: var(--community-text-body);
    font-weight: 500;
  }
  .store-link-card__mark {
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 0.5rem;
    background: var(--muted);
    color: var(--muted-foreground);
  }
  .store-link-card__address {
    display: none;
  }
  .store-link-card[data-layout="rich"] {
    display: inline-flex;
    flex-direction: column;
    align-items: stretch;
    width: min(100%, 26rem);
    padding: 0;
    overflow: hidden;
  }
  .store-link-card[data-layout="rich"] > span:last-child {
    padding: 0.75rem;
  }
  .store-link-card[data-layout="rich"] .store-link-card__mark {
    width: 100%;
    height: 6rem;
    border-radius: 0;
    background: var(--store-theme-gradient, var(--muted));
    font-size: 2rem;
  }
  .store-link-card[data-layout="rich"] .store-link-card__address {
    display: block;
    margin-top: 0.375rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
</style>
