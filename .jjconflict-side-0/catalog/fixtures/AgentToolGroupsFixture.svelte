<script lang="ts">
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { groupedAgentActivity } from "./agent-tool-groups.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  const base = createInMemoryStoreController("agents");
  const storage = createMemoryStoryStorage();
  const controller: StoreController = {
    ...base,
    get state() {
      const state = base.state;
      return !state.agentSession
        ? state
        : {
            ...state,
            agentSession: {
              ...state.agentSession,
              activity: [...groupedAgentActivity],
            },
          };
    },
  };
</script>

<StoreRepositoryApplication {controller} layoutStorage={storage} />
