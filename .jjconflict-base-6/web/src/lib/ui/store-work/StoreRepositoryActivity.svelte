<script lang="ts">
  // CommunityProjects.svelte repositoryActivityBar (Apache-2.0).
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  let {
    commits,
    reviews,
    tasks,
    partial = false,
  }: {
    commits: number;
    reviews: number;
    tasks: number;
    partial?: boolean;
  } = $props();
  const total = $derived(commits + reviews + tasks);
</script>

<Tooltip.Provider delayDuration={0}>
  <span
    class="community-projects__repository-activity"
    role="group"
    aria-label={`${partial ? "Loaded activity: " : ""}${commits} commits, ${reviews} reviews, ${tasks} tasks`}
    title={partial
      ? "Activity from loaded snapshot pages; more items may exist."
      : "Snapshot activity"}
  >
    {#each [["commits", commits], ["reviews", reviews], ["tasks", tasks]] as [kind, count]}
      {#if Number(count) > 0}<Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props })}<span
                {...props}
                class={`community-projects__repository-activity-segment community-projects__repository-activity-segment--${kind}`}
                style:width={`${(Number(count) / total) * 100}%`}
                role="img"
                aria-label={`${count} ${kind}`}
              ></span>{/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content
            >{count} {kind}{partial ? " in loaded pages" : ""}</Tooltip.Content
          >
        </Tooltip.Root>{/if}
    {/each}
  </span>
</Tooltip.Provider>
