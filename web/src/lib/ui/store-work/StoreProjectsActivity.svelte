<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CommunitySkeleton } from "@lapismd/lapis-community/components";
  import StoreRepositoryActivity from "../store-collaboration/StoreRepositoryActivity.svelte";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    task = false,
  }: { controller: StoreController; task?: boolean } = $props();
  const hub = $derived(controller.projects!);
  const rows = $derived(
    hub.activity.homes
      .flatMap((home) => home.page?.commits ?? [])
      .sort((a, b) => b.revision.createdAt - a.revision.createdAt),
  );
</script>

<section
  aria-label={task ? "Task repository activity" : "Project activity"}
  aria-busy={hub.activity.homes.some((home) => home.loading)}
>
  {#if task}<p class="store-projects__activity-scope">
      Repository activity from this task’s target or project links.
    </p>{/if}
  {#if rows.length}<StoreRepositoryActivity
      commits={rows}
      {controller}
      query={hub.route.workQuery ?? ""}
      loading={hub.activity.homes.some((home) => home.loading)}
      hasMore={hub.activity.homes.some(
        (home) => !!home.page?.cursor && !home.error,
      )}
      onLoadMore={() => {
        for (const home of hub.activity.homes)
          if (!home.error) void hub.activity.more(home.repository.id);
      }}
    />{/if}
  {#each hub.activity.homes as home (home.repository.id)}
    {#if home.loading && !home.page}<div class="store-projects__loading">
        <CommunitySkeleton
          layout="timeline"
          label={`Loading ${home.repository.name} activity`}
        />
      </div>{/if}
    {#if home.error}<div class="store-projects__activity-notice">
        <p role="alert">{home.repository.name}: {home.error}</p>
        <Button variant="outline" onclick={() => hub.refresh()}
          >Retry activity</Button
        >
      </div>{/if}
    {#if home.page?.cursor}<Button
        variant="outline"
        disabled={home.loading}
        onclick={() => hub.activity.more(home.repository.id)}
        >More activity in {home.repository.name}</Button
      >{/if}
  {/each}
  {#if !hub.loading && !rows.length && hub.activity.homes.every((home) => !home.loading && !home.error)}<StoreWorkEmpty
      kind="activity"
      title="No repository activity yet"
      description="Commits from accessible linked repositories will appear here."
    />{/if}
</section>
