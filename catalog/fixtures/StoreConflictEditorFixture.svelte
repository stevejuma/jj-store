<script lang="ts">
  import { untrack } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import StoreConflictEditor from "../../web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { conflictCommit } from "./conflicts.js";

  let {
    outcome = "committed",
    failure,
  }: { outcome?: "committed" | "unknown"; failure?: string } = $props();
  const controller = untrack(() =>
    createInMemoryStoreController("log", {
      mutationOutcome: outcome,
      mutationFailure: failure,
    }),
  );
  untrack(() => controller.actions.selectRevision(conflictCommit));
  let open = $state(true);
</script>

<div class="store-conflict-editor-fixture">
  <Dialog.Root bind:open>
    <StoreConflictEditor {controller} {open} />
  </Dialog.Root>
  <p role="status" data-testid="conflict-state">
    {controller.state.mutation.status} · {controller.state.route.commitId}
  </p>
</div>

<style>
  .store-conflict-editor-fixture {
    min-height: 42rem;
    padding: 1rem;
    background: var(--background);
  }
</style>
