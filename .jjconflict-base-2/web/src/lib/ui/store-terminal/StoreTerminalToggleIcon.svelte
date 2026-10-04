<script lang="ts">
  let { expanded }: { expanded: boolean } = $props();
</script>

<svg
  data-ui-part="terminal-toggle-icon"
  data-expanded={expanded}
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <rect x="1" y="2" width="22" height="20" rx="4" />
  <rect
    x="4"
    y={expanded ? 13 : 17}
    width="16"
    height={expanded ? 6 : 2}
    rx="1"
    fill="currentColor"
  />
</svg>
