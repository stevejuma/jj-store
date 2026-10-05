<script lang="ts">
  import type { Snippet } from "svelte";
  let {
    title,
    description,
    children,
  }: { title: string; description: string; children: Snippet } = $props();
</script>

<div class="appearance-row">
  <div class="appearance-row__copy">
    <p>{title}</p>
    <p>{description}</p>
  </div>
  {@render children()}
</div>
