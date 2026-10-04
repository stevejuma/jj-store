<script lang="ts">
  import type { Snippet } from "svelte";
  let {
    children,
    class: className = "",
    label = "Native data",
  }: {
    children: Snippet;
    class?: string;
    label?: string;
  } = $props();
</script>

<!-- Scrollable text needs keyboard focus in browsers without automatic scroll focus. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<pre
  tabindex="0"
  role="region"
  aria-label={label}
  class={className}>{@render children()}</pre>
