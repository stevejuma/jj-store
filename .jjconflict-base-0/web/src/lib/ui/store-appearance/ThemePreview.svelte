<script lang="ts">
  import {
    themePairs,
    type ThemeName,
  } from "../../appearance/theme-catalog.js";
  let { theme, paired = false }: { theme: ThemeName; paired?: boolean } =
    $props();
</script>

<div class="theme-preview" data-store-theme={theme} aria-hidden="true">
  <div class="theme-preview__window">
    <div class="theme-preview__sidebar">
      <div class="theme-preview__lights"><i></i><i></i><i></i></div>
      {#each [1, 2, 3, 4] as row}<div class="theme-preview__nav">
          <i></i><b></b>
        </div>{/each}
      <div class="theme-preview__caption"></div>
    </div>
    <svg viewBox="0 0 61 80" preserveAspectRatio="xMinYMin slice"
      ><rect
        width="21"
        height="2"
        rx="0.5"
        x="3"
        y="4"
        fill="var(--foreground)"
      /><rect
        width="4"
        height="4"
        rx="1"
        x="48"
        y="3"
        fill="var(--primary)"
      /><rect
        width="4"
        height="4"
        rx="1"
        x="54"
        y="3"
        fill="var(--primary)"
        opacity="0.68"
      /><path d="M0 10.5H61" stroke="var(--border)" /></svg
    >
  </div>
  {#if paired}<div
      class="theme-preview__half"
      data-store-theme={themePairs.get(theme)}
    >
      <div class="theme-preview__window">
        <div class="theme-preview__sidebar">
          {#each [1, 2, 3, 4] as row}<div class="theme-preview__nav">
              <i></i><b></b>
            </div>{/each}
          <div class="theme-preview__caption"></div>
        </div>
      </div>
    </div>{/if}
</div>
