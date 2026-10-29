<script lang="ts">
  import { untrack } from "svelte";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createCollaborationStateStoreController } from "../controller/collaboration-store-controller.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let { area = "reviews" }: { area?: "reviews" | "agents" } = $props();
  const controller = untrack(() => {
    const source = createInMemoryStoreController(area);
    if (area === "reviews")
      source.actions.selectReviewThread(
        source.state.reviewThreads[0]!.thread_id,
      );
    else
      source.actions.selectAgentSession(
        source.state.agentSessions[0]!.session_id,
      );
    return createCollaborationStateStoreController(source, "failed");
  });
</script>

<StoreRepositoryApplication
  {controller}
  initiallyCollapsed={false}
  layoutStorage={createMemoryStoryStorage()}
/>
