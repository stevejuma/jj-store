<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { MergeEditor } from "@lapismd/design-core/diff";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { WorkspaceConflict } from "../../controller/stack-workspace/types.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    open = $bindable(false),
  }: { controller: StackWorkspaceController; open?: boolean } = $props();
  let resolved = $state(""),
    complete = $state(false);
  const selected = $derived(controller.snapshot?.conflicts[0]);
  let nativeConflict = $state<WorkspaceConflict | null>(null),
    loading = $state(false),
    failure = $state("");
  const conflict = $derived(nativeConflict ?? selected);
  $effect(() => {
    const snapshot = controller.snapshot,
      path = selected?.path;
    nativeConflict = null;
    failure = "";
    if (!open || !path || !snapshot || !controller.adapter.readConflict) {
      loading = false;
      return;
    }
    let cancelled = false;
    loading = true;
    void controller.adapter
      .readConflict({
        repository: snapshot.repository,
        revision: snapshot.revision,
        readKey: snapshot.readKey,
        path,
      })
      .then((value) => {
        if (!cancelled) nativeConflict = value;
      })
      .catch((error) => {
        if (!cancelled)
          failure =
            error instanceof Error ? error.message : "Conflict unavailable.";
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });
    return () => {
      cancelled = true;
    };
  });
  $effect(() => {
    void conflict?.path;
    resolved = "";
    complete = false;
  });
  async function prepare() {
    if (!conflict || !complete) return;
    await controller.prepare({
      kind: "resolve_conflict",
      path: conflict.path,
      content: resolved,
    });
    if (controller.preview) open = false;
  }
</script>

<Dialog.Root bind:open
  ><DialogFrame
    title="Resolve workspace conflict"
    description={conflict?.path ?? "No unresolved conflicts"}
    preventScroll={false}
    size="wide"
  >
    <StoreNotice
      noticeKey={`stack-conflict:${selected?.path}`}
      title="Conflict unavailable"
      message={failure}
      lifetime="condition"
    />
    {#if loading}<p role="status">
        Loading conflict…
      </p>{:else if conflict && !failure}<MergeEditor
        mode="three-way"
        path={conflict.path}
        left={conflict.left}
        base={conflict.base}
        right={conflict.right}
        onResolvedChange={(value) => {
          resolved = value.content;
          complete = value.resolved;
        }}
      />{/if}
    {#snippet footer()}<Button variant="outline" onclick={() => (open = false)}
        >Close</Button
      ><Button
        disabled={loading ||
          Boolean(failure) ||
          !complete ||
          !controller.can("resolve_conflict")}
        onclick={prepare}>Preview resolution</Button
      >{/snippet}
  </DialogFrame></Dialog.Root
>
