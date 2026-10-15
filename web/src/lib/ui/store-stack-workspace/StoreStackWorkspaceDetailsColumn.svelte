<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import FileSearch from "@lucide/svelte/icons/file-search";
  import StoreForgeDetails from "../store-forge/StoreForgeDetails.svelte";
  import StoreStackWorkspaceDiff from "./StoreStackWorkspaceDiff.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceOpenDiff,
    WorkspaceCommand,
  } from "../../controller/stack-workspace/types.js";
  import "./stack-workspace-details.css";
  let {
    controller,
    origin,
    branch,
    diff,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    origin: string;
    branch?: string;
    diff?: WorkspaceOpenDiff;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  const forge = $derived(controller.forge);
  const pr = $derived(branch && forge?.pr(branch));
  function close() {
    controller.closeDiff(origin);
    if (forge?.surface === "inline" && forge.selected?.head === branch) {
      forge.retainDraft();
      forge.surface = "column";
    }
  }
</script>

{#if forge && branch && pr}
  <ColumnCanvas.Column
    resizable
    id={`diff-${origin}`}
    class="stack-workspace-details"
    aria-label={`Selected changes from ${origin}`}
  >
    <ColumnCanvas.Body scrollStrategy="styled">
      <div class="stack-workspace-details__sections">
        <StoreForgeDetails controller={forge} {branch} onClose={close} />
        {#if diff}
          <StoreStackWorkspaceDiff {controller} {diff} {onCommand} embedded />
        {:else}
          <section
            class="stack-workspace-details__file"
            aria-label="Selected file diff"
          >
            <Empty.Root class="stack-workspace-details__empty">
              <Empty.Header>
                <Empty.Media variant="icon"
                  ><FileSearch aria-hidden="true" /></Empty.Media
                >
                <Empty.Title>No file selected</Empty.Title>
                <Empty.Description
                  >Select a changed file to preview its diff below this pull
                  request.</Empty.Description
                >
              </Empty.Header>
            </Empty.Root>
          </section>
        {/if}
      </div>
    </ColumnCanvas.Body>
  </ColumnCanvas.Column>
{:else if diff}
  <StoreStackWorkspaceDiff {controller} {diff} {onCommand} />
{/if}
