<script lang="ts">
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    lightThemes,
    themeNames,
  } from "../../web/src/lib/appearance/theme-catalog.js";
  import CommunityChatComposer from "../../web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  const appearance = ownAppearance(
    createAppearanceStore(createMemoryStoryStorage()),
  );
  let index = $state(0);
  let value = $state("");
  function select() {
    const theme = themeNames[index]!;
    appearance.update({
      theme,
      mode: lightThemes.has(theme) ? "light" : "dark",
    });
  }
  select();
</script>

<div class="appearance-composer-fixture" data-theme={appearance.theme}>
  <h1>Composer palettes</h1>
  <p>Text and placeholders follow the selected app palette.</p>
  <button
    type="button"
    onclick={() => {
      index = (index + 1) % themeNames.length;
      select();
    }}>Next palette</button
  >
  <CommunityChatComposer
    bind:value
    placeholder="Write a message"
    onSubmit={async () => {}}
  />
</div>

<style>
  .appearance-composer-fixture {
    max-width: 44rem;
    margin: 2rem auto;
    padding: 1rem;
    background: var(--background);
    color: var(--foreground);
  }

  .appearance-composer-fixture p {
    color: var(--muted-foreground);
  }
</style>
