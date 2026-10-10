<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    createAppearanceStore,
    provideAppearance,
  } from "../../web/src/lib/appearance/store.svelte.js";
  import StoreAgentContext from "../../web/src/lib/ui/store-collaboration/StoreAgentContext.svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { agentDetail } from "./collaboration.js";
  const appearance = createAppearanceStore();
  provideAppearance(appearance);
  const base = createInMemoryStoreController("agents");
  let more = $state(true);
  const controller: StoreController = {
    ...base,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        agentSession: { ...agentDetail, activity: [] },
        agentTranscript: [
          ...(!more
            ? [
                {
                  sequence: 0,
                  media_type: "text" as const,
                  content: "Older captured answer",
                },
              ]
            : []),
          {
            sequence: 1,
            media_type: "text",
            content: "Latest captured answer",
          },
        ],
        canLoadMoreTranscript: more,
      };
    },
    actions: {
      ...base.actions,
      async loadMore(collection) {
        if (collection === "transcript") more = false;
      },
    },
  };
</script>

<div class="legacy-transcript-fixture">
  <Button onclick={() => appearance.update({ conversationView: "classic" })}
    >Use Classic view</Button
  >
  <StoreAgentContext {controller} showHeader={false} />
</div>

<style>
  .legacy-transcript-fixture {
    height: 28rem;
    overflow: auto;
  }
</style>
