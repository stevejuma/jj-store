<script lang="ts">
  import StoreLazyView from "../../web/src/lib/ui/store-application/StoreLazyView.svelte";
  import { lazyView } from "../../web/src/lib/ui/store-application/lazy-view.js";
  import Content from "./LazyViewContent.svelte";
  let active = $state(false),
    present = $state(true),
    second = $state(false);
  let calls = $state(0);
  let firstResolve = () => {},
    secondResolve = () => {},
    firstReject = () => {};
  const first = lazyView(() => {
    calls += 1;
    return new Promise<{ default: typeof Content }>((resolve, reject) => {
      firstResolve = () => resolve({ default: Content });
      firstReject = () => reject(Error("simulated offline chunk"));
    });
  });
  const next = lazyView(() => {
    calls += 1;
    return new Promise<{ default: typeof Content }>((resolve) => {
      secondResolve = () => resolve({ default: Content });
    });
  });
</script>

<div style="padding:1rem">
  <button onclick={() => (active = true)}>Open view</button>
  <button onclick={() => (active = false)}>Close view</button>
  <button onclick={() => (second = true)}>Switch view</button>
  <button onclick={() => (present = false)}>Navigate away</button>
  <button onclick={() => firstResolve()}>Resolve first</button>
  <button onclick={() => secondResolve()}>Resolve second</button>
  <button onclick={() => firstReject()}>Reject first</button>
  <p role="status">Imports: {calls}</p>
  {#if present}
    <StoreLazyView {active} load={second ? next : first} label="Example view">
      {#snippet content(View)}<View
          label={second ? "Second" : "First"}
          open={active}
        />{/snippet}
    </StoreLazyView>
  {/if}
</div>
