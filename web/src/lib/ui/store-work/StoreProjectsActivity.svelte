<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import { filterNativeActivity } from "../../controller/native-activity-filter.js";
  import type { NativeActivityCommit } from "../../controller/native-activity-types.js";
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
  let filtered = $state<NativeActivityCommit[]>([]),
    filtering = $state(false),
    unavailable = $state(0);
  const category = $derived(task ? "all" : (hub.route.workActivity ?? "all"));
  $effect(() => {
    const loaded = rows,
      choice = category;
    let active = true;
    filtering = choice !== "all" && loaded.length > 0;
    filtered = choice === "all" ? loaded : [];
    unavailable = 0;
    if (choice !== "all")
      void filterNativeActivity(loaded, choice, () => active).then((result) => {
        if (!active) return;
        filtered = result.rows;
        unavailable = result.unavailable;
        filtering = false;
      });
    return () => {
      active = false;
    };
  });
</script>

<section
  aria-label={task ? "Task repository activity" : "Project activity"}
  aria-busy={hub.activity.homes.some((home) => home.loading)}
>
  {#if task}<p class="store-projects__activity-scope">
      Repository activity from this task’s target or project links.
    </p>{/if}
  {#if filtering}<div class="store-projects__loading">
      <CommunitySkeleton
        layout="timeline"
        label="Checking recorded agent associations"
      />
    </div>{/if}
  <StoreNotice
    noticeKey="activity-agent-context"
    title="Activity is incomplete"
    severity="warning"
    message={unavailable
      ? `Agent context unavailable for ${unavailable} commits. Filtered results are incomplete.`
      : ""}
    pending={filtering || hub.loading}
    action={{
      id: "retry-activity-context",
      label: "Retry activity",
      run: () => hub.refresh(),
    }}
  />
  {#if rows.length && !filtering}<StoreRepositoryActivity
      commits={filtered}
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
    <StoreNotice
      scope="projects-reads"
      noticeKey={`project-activity:${home.repository.id}`}
      title={`Could not load activity in ${home.repository.name}`}
      message={home.error}
      pending={home.loading}
      action={{
        id: "retry-activity",
        label: "Retry activity",
        run: () => hub.refresh(),
      }}
    />
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
