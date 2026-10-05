<script lang="ts">
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

<button onclick={remount}>Remount navigation</button>
{#if mounted}<ProjectsHubFixture {layoutStorage} />{/if}
