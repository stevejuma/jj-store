<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import ArrowRight from "@lucide/svelte/icons/arrow-right";
</script>

<article class="workflow-card" aria-hidden="true">
  <div class="workflow-card__top">
    <div class="workflow-card__tiles">
      <Skeleton class="h-9 w-9 rounded-xl" /><ArrowRight class="h-4 w-4" />
      <Skeleton class="h-9 w-9 rounded-xl" /><Skeleton
        class="h-9 w-9 rounded-xl"
      />
    </div>
    <div class="workflow-card__controls">
      <Skeleton class="h-5 w-9 rounded-full" /><Skeleton
        class="h-4 w-4 rounded"
      />
    </div>
  </div>
  <div class="workflow-skeleton__copy">
    <Skeleton class="h-5 w-3/4 rounded" /><Skeleton
      class="h-3 w-full rounded"
    /><Skeleton class="h-3 w-2/3 rounded" />
  </div>
  <footer>
    <Skeleton class="h-3 w-24 rounded" /><Skeleton
      class="h-5 w-16 rounded-full"
    />
  </footer>
</article>

<style>
  .workflow-skeleton__copy {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 0.65rem;
  }
</style>
