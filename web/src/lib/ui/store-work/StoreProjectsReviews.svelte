<script lang="ts">
  import StoreWorkCollection from "./StoreWorkCollection.svelte";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import StoreWorkReviewRow from "./StoreWorkReviewRow.svelte";
  import "./community-work-reviews.css";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreWorkEmpty from "./StoreWorkEmpty.svelte";
  let {
    controller,
    layout = "list",
  }: { controller: StoreController; layout?: "grid" | "list" } = $props();
  const hub = $derived(controller.projects!);
</script>

<section class="store-projects__review-directory" aria-label="Project reviews">
  {#if !hub.reviews.homes.length && !hub.loading}<StoreWorkEmpty
      kind="reviews"
      title="No repositories to review"
      description="Link an accessible repository to see its native review threads."
    />{/if}
  {#if hub.reviews.homes.length && !hub.loading && hub.reviews.homes.every((home) => home.snapshot && !home.threads.length && !home.error)}<StoreWorkEmpty
      kind="reviews"
      title="No reviews yet"
      description="Native code discussions in your accessible repositories will appear here."
    />{/if}
  {#each hub.reviews.homes.filter((home) => home.threads.length || home.loading || home.error || !home.snapshot) as home (home.repository.id)}
    <section
      class="store-projects__home"
      aria-label={`Reviews in ${home.repository.name}`}
      aria-busy={home.loading}
    >
      <StoreWorkCollection
        title={home.repository.name}
        count={home.threads.length}
        Icon={GitBranchIcon}
      >
        {#if home.error}<p role="alert">{home.error}</p>
          <Button variant="outline" onclick={() => hub.refresh()}
            >Retry reviews</Button
          >{/if}
        {#if home.loading && !home.snapshot}<div
            class="store-projects__loading"
          >
            <CommunitySkeleton layout="timeline" label="Loading reviews" />
          </div>
        {:else if !home.threads.length && !home.error}<StoreWorkEmpty
            kind="reviews"
            title="No reviews yet"
            description="Native code discussions in this repository will appear here."
          />{/if}
        <div
          class="community-projects__review-group-list community-projects__list"
          data-layout={layout}
          role="list"
          aria-label="Reviews"
        >
          {#each home.threads as thread (thread.thread_id)}
            <div role="listitem">
              <StoreWorkReviewRow
                {thread}
                onOpen={controller.actions.openReview
                  ? (id) =>
                      controller.actions.openReview?.(home.repository.id, id)
                  : undefined}
              />
            </div>
          {/each}
        </div>
        {#if home.cursor}<Button
            variant="outline"
            disabled={home.loading}
            onclick={() => hub.reviews.more(hub.route, home.repository.id)}
            >More reviews</Button
          >{/if}
      </StoreWorkCollection>
    </section>
  {/each}
</section>
