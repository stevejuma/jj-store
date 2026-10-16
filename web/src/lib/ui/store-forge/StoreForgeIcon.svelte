<script lang="ts">
  import GitPullRequest from "@lucide/svelte/icons/git-pull-request";
  import { forgeIcons } from "./forge-icons.js";
  let { provider }: { provider?: string } = $props();
  const icon = $derived(forgeIcons[provider as keyof typeof forgeIcons]);
</script>

{#if icon}
  <svg
    role="img"
    aria-label={icon.name}
    viewBox="0 0 24 24"
    width="16"
    height="16"
    fill="currentColor"
  >
    <title>{icon.name}</title>
    <path d={icon.path} />
  </svg>
{:else}
  <GitPullRequest size={16} aria-hidden="true" />
{/if}
