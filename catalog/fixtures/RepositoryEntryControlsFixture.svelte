<script lang="ts">
  import { untrack } from "svelte";
  import StoreApplication from "./ApplicationStory.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  const base = untrack(() => createInMemoryStoreController("files"));
  const controller = {
    ...base,
    get state() {
      return {
        ...base.state,
        accessMode: "local_trusted" as const,
        nativeRepositoryCreation: true,
        repositoryConversion: true,
      };
    },
  };
  const layoutStorage = untrack(createMemoryStoryStorage);
</script>

<StoreApplication {controller} {layoutStorage} initiallyCollapsed={false} />
