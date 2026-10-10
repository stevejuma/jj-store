<script lang="ts">
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    appearanceStorageKey,
    defaultAppearance,
  } from "../../web/src/lib/appearance/preferences.js";
  import { groupedAgentActivity } from "./agent-tool-groups.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  const base = createInMemoryStoreController("agents");
  const storage = createMemoryStoryStorage();
  // This family exercises Classic grouped-call rows; Document has its own stories.
  storage.setItem(
    appearanceStorageKey,
    JSON.stringify({ ...defaultAppearance, conversationView: "classic" }),
  );
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
