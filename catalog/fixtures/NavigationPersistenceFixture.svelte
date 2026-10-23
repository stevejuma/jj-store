<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { tick, untrack } from "svelte";
  import ProjectsHubFixture from "./ProjectsHubFixture.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  const layoutStorage = untrack(createMemoryStoryStorage);
  let mounted = $state(true);
  async function remount() {
    mounted = false;
    await tick();
    mounted = true;
  }
</script>

<FixtureControls
  ><div data-story-controls>
    <button onclick={remount}>Remount navigation</button>
  </div></FixtureControls
>
{#if mounted}<ProjectsHubFixture {layoutStorage} />{/if}
