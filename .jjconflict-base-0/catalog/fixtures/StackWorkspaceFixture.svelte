<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreStackWorkspace from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceSnapshot } from "../../web/src/lib/controller/stack-workspace/types.js";
  import {
    MockStackWorkspaceAdapter,
    type FixtureFault,
  } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  let {
    showStackActions = true,
    profile = "git_sha1_jj045",
    availability = "ready",
    fault = "none",
    empty = false,
    immutable = false,
    page = "workspace",
    loading = false,
    narrow = false,
  }: {
    showStackActions?: boolean;
    profile?: WorkspaceSnapshot["profile"];
    availability?: WorkspaceSnapshot["availability"];
    fault?: FixtureFault;
    empty?: boolean;
    immutable?: boolean;
    page?: "branches" | "workspace";
    loading?: boolean;
    narrow?: boolean;
  } = $props();
  const controller = untrack(() => {
    const snapshot = workspaceStoryFixture(profile);
    snapshot.availability = availability;
    if (empty) {
      snapshot.stacks = [];
      snapshot.workspaces[0]!.applied = [];
    }
    if (immutable) snapshot.stacks[0]!.commits[0]!.immutable = true;
    const value = new StackWorkspaceController(
      new MockStackWorkspaceAdapter(snapshot, fault),
    );
    value.page = page;
    return value;
  });
  onMount(() => {
    if (!loading) void controller.refresh();
    return () => controller.close();
  });
</script>

<div
  class="stack-workspace-fixture"
  style:max-width={narrow ? "390px" : undefined}
>
  <StoreStackWorkspace {controller} {showStackActions} />
</div>

<style>
  .stack-workspace-fixture {
    height: 48rem;
    min-height: 30rem;
  }
</style>
