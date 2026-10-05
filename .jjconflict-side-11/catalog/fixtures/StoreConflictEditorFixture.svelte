<script lang="ts">
  import { untrack } from "svelte";
  import StoreApplication from "../../web/src/lib/ui/store-application/StoreApplication.svelte";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { conflictCommit } from "./conflicts.js";

  let {
    outcome = "committed",
    failure,
    displayMode = "desktop",
  }: {
    outcome?: "committed" | "unknown";
    failure?: string;
    displayMode?: AppShellDisplayMode;
  } = $props();
  const controller = untrack(() =>
    createInMemoryStoreController("log", {
      mutationOutcome: outcome,
      mutationFailure: failure,
    }),
  );
  untrack(() => controller.actions.selectRevision(conflictCommit));
</script>

<div class="store-conflict-editor-fixture">
  <StoreApplication {controller} {displayMode} initiallyCollapsed={false}>
    {#snippet children()}<p>Repository content</p>{/snippet}
  </StoreApplication>
</div>

<style>
  .store-conflict-editor-fixture {
    height: 44rem;
    background: var(--background);
  }
</style>
