<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { MergeEditor } from "@lapismd/design-core/diff";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    open = $bindable(false),
  }: { controller: StackWorkspaceController; open?: boolean } = $props();
  let resolved = $state(""),
    complete = $state(false);
  const conflict = $derived(controller.snapshot?.conflicts[0]);
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
    {#if conflict}<MergeEditor
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
        disabled={!complete || !controller.can("resolve_conflict")}
        onclick={prepare}>Preview resolution</Button
      >{/snippet}
  </DialogFrame></Dialog.Root
>
