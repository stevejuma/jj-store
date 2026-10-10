<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import "./community-work-layout.css";
  import "./community-work-rows.css";
  import "./community-work-collections.css";
  import "./community-work-tasks.css";
  import "./community-work-reviews.css";
  import "./community-work-cards.css";
  let {
    label,
    kind = "projects",
    layout = "list",
    grouped = true,
  }: {
    label: string;
    kind?: "projects" | "tasks" | "reviews" | "repositories";
    layout?: "list" | "grid";
    grouped?: boolean;
  } = $props();
  const rowsClass = $derived(
    kind === "projects"
      ? "community-projects__grid"
      : kind === "tasks"
        ? "community-projects__task-list"
        : kind === "reviews"
          ? "community-projects__review-group-list community-projects__list"
          : "community-projects__list",
  );
  const rowClass = $derived(
    kind === "projects"
      ? "community-projects__card"
      : kind === "tasks"
        ? "community-projects__task-row"
        : kind === "reviews"
          ? "community-projects__review-row store-projects__native-review-row"
          : "community-projects__row community-projects__repository-row",
  );
</script>

<div
  class="work-rows-skeleton"
  role="status"
  aria-label={label}
  aria-busy="true"
  data-layout={layout === "grid" ? "work-grid" : "work-list"}
  data-kind={kind}
>
  <div aria-hidden="true">
    {#if grouped}<div
        class="community-projects__list-group-heading community-projects__list-group-heading--inset"
      >
        <Skeleton class="work-rows-skeleton__heading" />
      </div>{/if}
    <div class={rowsClass} data-layout={layout}>
      {#each [0, 1, 2, 3] as row}
        <div class={rowClass}>
          <Skeleton class="work-rows-skeleton__icon" />
          <div class="work-rows-skeleton__copy">
            <Skeleton
              class="work-rows-skeleton__title"
            />{#if kind === "projects" || kind === "repositories"}<Skeleton
                class="work-rows-skeleton__line"
              />{/if}
          </div>
          <Skeleton class="work-rows-skeleton__meta" />
          {#if kind === "tasks" || kind === "reviews"}<Skeleton
              class="work-rows-skeleton__meta"
            /><Skeleton class="work-rows-skeleton__icon" /><Skeleton
              class="work-rows-skeleton__meta"
            />{/if}
          {#if kind === "reviews"}<Skeleton
              class="work-rows-skeleton__meta"
            />{/if}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .work-rows-skeleton {
    min-width: 0;
    width: 100%;
  }
  .work-rows-skeleton
    :is(
      .community-projects__grid,
      .community-projects__list,
      .community-projects__task-list
    ) {
    padding: 0.45rem 0;
  }
  .work-rows-skeleton__copy {
    display: grid;
    flex: 1;
    min-width: 0;
    gap: 0.25rem;
  }
  :global(.work-rows-skeleton__heading) {
    width: 6rem;
    height: 0.85rem;
  }
  :global(.work-rows-skeleton__icon) {
    width: 1rem;
    height: 1rem;
    flex: none;
  }
  :global(.work-rows-skeleton__title) {
    width: 60%;
    height: 0.85rem;
  }
  :global(.work-rows-skeleton__line) {
    width: 85%;
    height: 0.7rem;
  }
  :global(.work-rows-skeleton__meta) {
    width: 3rem;
    max-width: 100%;
    height: 0.7rem;
  }
</style>
