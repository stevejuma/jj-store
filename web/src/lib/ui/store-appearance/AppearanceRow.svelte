<script lang="ts">
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import { settingsRowIcons } from "../store-settings/settings-row-icons.js";
  import type { Snippet } from "svelte";
  let {
    title,
    description,
    children,
  }: { title: string; description: string; children: Snippet } = $props();
</script>

<div class="appearance-row">
  <div class="appearance-row__label">
    {#if settingsRowIcons[title]}<StoreSettingsIcon
        Icon={settingsRowIcons[title]}
      />{/if}
    <div class="appearance-row__copy">
      <p>{title}</p>
      <p>{description}</p>
    </div>
  </div>
  {@render children()}
</div>
