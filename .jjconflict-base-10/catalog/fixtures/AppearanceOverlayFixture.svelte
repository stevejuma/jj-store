<script lang="ts">
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    buzzThemeNames,
    lightThemes,
  } from "../../web/src/lib/appearance/theme-catalog.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import CommunitySearchCommand from "../../web/src/lib/ui/community-search-command/CommunitySearchCommand.svelte";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let { initialTheme = buzzThemeNames[0]! } = $props<{
    initialTheme?: string;
  }>();
  const appearance = ownAppearance(
    createAppearanceStore(createMemoryStoryStorage()),
  );
  let index = $state(
    buzzThemeNames.findIndex((theme) => theme === initialTheme),
  );
  let query = $state("native");
  function select() {
    const theme = buzzThemeNames[index]!;
    appearance.update({
      theme,
      mode: lightThemes.has(theme) ? "light" : "dark",
    });
  }
  select();
</script>

<div class="appearance-overlay-fixture" data-theme={appearance.theme}>
  <span class="scope-rest-sample" aria-hidden="true"></span>
  <span class="scope-active-sample" aria-hidden="true"></span>
</div>
<CommunitySearchCommand
  open
  {query}
  status="ready"
  now={1000}
  scopeLabel="Projects"
  onScope={() => {}}
  results={[
    {
      id: "task",
      kind: "task",
      title: "Native task",
      source: "Repository",
      description: "Retained search result",
      timestamp: 1000,
    },
  ]}
  onQueryChange={(value) => (query = value)}
  onSelect={() => {}}
>
  {#snippet footer()}
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>Menu surface</DropdownMenu.Trigger>
      <DropdownMenu.Content>
        <DropdownMenu.Item>Example option</DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
    <Button
      variant="ghost"
      onclick={() => {
        index = (index + 1) % buzzThemeNames.length;
        select();
      }}>Next palette</Button
    >
  {/snippet}
</CommunitySearchCommand>

<style>
  .appearance-overlay-fixture {
    background: var(--background);
    color: var(--foreground);
  }
  .scope-rest-sample {
    background: color-mix(in srgb, var(--muted) 30%, var(--background));
  }
  .scope-active-sample {
    background: color-mix(in srgb, var(--muted) 55%, var(--background));
  }
</style>
