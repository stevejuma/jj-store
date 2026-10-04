<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  const base = untrack(() => createInMemoryStoreController("files"));
  const storage = untrack(createMemoryStoryStorage);
  let pending = $state(false);
  const controller: StoreController = {
    get state(): StoreController["state"] {
      const view = base.state;
      return {
        ...view,
        status: { ...view.status, files: pending ? "loading" : "ready" },
      };
    },
    actions: {
      ...base.actions,
      selectDirectory(path) {
        pending = true;
        return base.actions.selectDirectory(path);
      },
    },
  };
</script>

<StoreRepositoryApplication {controller} layoutStorage={storage} />
<Button onclick={() => (pending = false)}>Finish folder load</Button>
