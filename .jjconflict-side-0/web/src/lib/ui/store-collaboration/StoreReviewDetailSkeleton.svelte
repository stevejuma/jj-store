<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import "./community-native-review.css";

  let { writable = false }: { writable?: boolean } = $props();
</script>

<div
  class="community-projects__review-detail review-detail-skeleton"
  role="status"
  aria-label="Loading review details"
  aria-busy="true"
  data-layout="review-detail"
>
  <div class="community-projects__review-header" aria-hidden="true">
    <Skeleton class="review-detail-skeleton__back" />
    <Skeleton class="review-detail-skeleton__title" />
    <Skeleton class="review-detail-skeleton__subtitle" />
  </div>
  <ScrollArea
    class="review-detail-skeleton__scroll"
    type="hover"
    aria-hidden="true"
    inert
  >
    <div class="community-projects__review-body">
      <div class="community-projects__review-meta">
        {#each [0, 1, 2, 3] as row}
          <div>
            <Skeleton class="review-detail-skeleton__label" />
            <Skeleton class="review-detail-skeleton__value" />
          </div>
        {/each}
      </div>
      {#each ["description", "files", "commits", "activity"] as section}
        <div class="community-projects__review-section">
          <div class="community-projects__review-section-trigger">
            <Skeleton class="review-detail-skeleton__section-title" />
          </div>
          {#if section === "description"}
            <div
              class="community-projects__review-section-content review-detail-skeleton__description"
            >
              <Skeleton class="review-detail-skeleton__line" />
              <Skeleton class="review-detail-skeleton__short-line" />
            </div>
          {:else if section === "activity"}
            <div class="community-projects__review-section-content">
              {#each [0, 1] as row}
                <div class="review-detail-skeleton__activity">
                  <Skeleton class="review-detail-skeleton__avatar" />
                  <div class="review-detail-skeleton__copy">
                    <Skeleton class="review-detail-skeleton__value" />
                    <Skeleton class="review-detail-skeleton__line" />
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </ScrollArea>
  {#if writable}
    <div class="community-projects__review-footer" aria-hidden="true">
      <div class="review-detail-skeleton__composer">
        <Skeleton class="review-detail-skeleton__value" />
        <div class="review-detail-skeleton__toolbar">
          {#each [0, 1, 2] as action}
            <Skeleton class="review-detail-skeleton__action" />
          {/each}
          <Skeleton class="review-detail-skeleton__submit" />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  :global(.review-detail-skeleton__scroll) {
    flex: 1 1 auto;
    min-height: 0;
    min-width: 0;
  }
  :global(.review-detail-skeleton__back) {
    position: absolute;
    inset-inline-start: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 50%;
  }
  :global(.review-detail-skeleton__title) {
    width: 72%;
    height: 1.4rem;
  }
  :global(.review-detail-skeleton__subtitle) {
    width: min(40%, 12rem);
    height: 0.95rem;
    margin-top: 0.15rem;
  }
  :global(.review-detail-skeleton__label),
  :global(.review-detail-skeleton__section-title) {
    width: 4.5rem;
    height: 0.8rem;
  }
  :global(.review-detail-skeleton__value) {
    width: min(50%, 15rem);
    height: 0.8rem;
  }
  :global(.review-detail-skeleton__line) {
    width: 75%;
    height: 0.8rem;
  }
  :global(.review-detail-skeleton__short-line) {
    width: 45%;
    height: 0.8rem;
  }
  .review-detail-skeleton__description,
  .review-detail-skeleton__copy {
    display: grid;
    gap: 0.6rem;
  }
  .review-detail-skeleton__activity {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding-block: 0.75rem;
  }
  :global(.review-detail-skeleton__avatar) {
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 50%;
  }
  .review-detail-skeleton__copy {
    flex: 1;
    min-width: 0;
  }
  .review-detail-skeleton__composer {
    height: 7rem;
    box-sizing: border-box;
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 1rem;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  .review-detail-skeleton__toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  :global(.review-detail-skeleton__action),
  :global(.review-detail-skeleton__submit) {
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
  }
  :global(.review-detail-skeleton__submit) {
    width: 2rem;
    height: 2rem;
    margin-inline-start: auto;
  }
</style>
