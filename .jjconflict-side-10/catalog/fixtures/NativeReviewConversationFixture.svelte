<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreReviewConversation from "../../web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { reviewThreads } from "./collaboration-review.js";
  let {
    own = true,
    writable = true,
    unknown = false,
  }: { own?: boolean; writable?: boolean; unknown?: boolean } = $props();
  const base = untrack(() =>
    createInMemoryStoreController("reviews", {
      reviewOutcome: unknown ? "unknown" : "committed",
    }),
  );
  untrack(() => base.actions.selectReviewThread(reviewThreads[0]!.thread_id));
  const controller: StoreController = {
    ...base,
    get state() {
      return {
        ...base.state,
        repository: {
          ...base.state.repository,
          principal: own ? "e2".repeat(16) : "99".repeat(16),
          permission: writable ? ("write" as const) : ("read" as const),
        },
      };
    },
  };
</script>

<section
  aria-label="Native review discussion fixture"
  style="max-width: 52rem; padding: 2rem;"
>
  <StoreReviewConversation {controller} showDiff={false} />
  {#if controller.state.reviewActionRequestId}<Button
      onclick={() => controller.actions.checkReviewAction()}
      >Check review command</Button
    >{/if}
</section>
