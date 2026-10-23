<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    lightThemes,
    themeNames,
  } from "../../web/src/lib/appearance/theme-catalog.js";
  import CommunityChatComposer from "../../web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let { pending = false, busy = false }: { pending?: boolean; busy?: boolean } =
    $props();
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
  <FixtureControls floating={false}
    ><button
      type="button"
      onclick={() => {
        index = (index + 1) % themeNames.length;
        select();
      }}>Next palette</button
    ></FixtureControls
  >
  <CommunityChatComposer
    bind:value
    disabled={pending}
    {busy}
    context={pending || busy
      ? {
          kind: "transcript-reply",
          target: { authorName: "You", content: "Retained reply context" },
        }
      : undefined}
    composerStatus={pending
      ? {
          type: "warning",
          message:
            "Check the original reply status before saving another reply.",
        }
      : undefined}
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
