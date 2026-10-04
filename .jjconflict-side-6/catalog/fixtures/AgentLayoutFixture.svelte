<script lang="ts">
  import { untrack } from "svelte";
  import ProjectsHubFixture from "./ProjectsHubFixture.svelte";
  import RepositoryFixture from "./StoreRepositoryApplicationFixture.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    appearanceStorageKey,
    defaultAppearance,
  } from "../../web/src/lib/appearance/preferences.js";
  let {
    repository = false,
    project = false,
    mobile = false,
    focus = true,
    readOnly = false,
    reviewOutcome,
  }: {
    repository?: boolean;
    project?: boolean;
    mobile?: boolean;
    focus?: boolean;
    readOnly?: boolean;
    reviewOutcome?: "committed" | "unknown";
  } = $props();
  const storage = untrack(() => {
    const memory = createMemoryStoryStorage();
    memory.setItem(
      appearanceStorageKey,
      JSON.stringify({
        ...defaultAppearance,
        threads: focus ? "focus" : "split",
      }),
    );
    return memory;
  });
</script>

{#if repository}
  <RepositoryFixture
    initialPage="agents"
    {readOnly}
    {reviewOutcome}
    displayMode={mobile ? "mobile" : "desktop"}
    layoutStorage={storage}
  />
{:else}
  <ProjectsHubFixture
    agents
    projectDetail={project}
    {mobile}
    layoutStorage={storage}
  />
{/if}
