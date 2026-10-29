<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { tick, untrack } from "svelte";
  import Projects from "./ProjectsHubFixture.svelte";
  import Repository from "./StoreRepositoryApplicationFixture.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    repository = false,
    task = false,
    review = false,
    opLog = false,
  }: {
    repository?: boolean;
    task?: boolean;
    review?: boolean;
    opLog?: boolean;
  } = $props();
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
    <button onclick={remount}>Remount page</button>
  </div></FixtureControls
>
{#if mounted}
  {#if repository}<Repository
      initialPage={opLog ? "oplog" : review ? "reviews" : "agents"}
      initiallyCollapsed={false}
      {layoutStorage}
    />
  {:else}<Projects agents={!task} detail={task} {layoutStorage} />{/if}
{/if}
