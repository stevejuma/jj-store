<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import { onMount } from "svelte";
  import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
  import StoreStackWorkspace from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  const shell = createStoreShell(createMemoryStoryStorage(), true);
  const controller = new StackWorkspaceController(
    new MockStackWorkspaceAdapter(workspaceStoryFixture()),
  );
  onMount(() => {
    void controller.refresh();
    return () => {
      controller.close();
      void shell.dispose();
    };
  });
</script>

<AppShell.Root controller={shell} style="--ui-shell-height: 48rem">
  <AppShell.Main
    ><StoreStackWorkspace
      {controller}
      showStackActions={false}
    /></AppShell.Main
  >
</AppShell.Root>
