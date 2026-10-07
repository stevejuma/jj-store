<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import CommunityCommitDiff from "../community-commit-diff/CommunityCommitDiff.svelte";
  import type { AgentChangedFile } from "../../controller/agent-workspace-changes.js";
  let {
    files,
    repositoryName,
    open = $bindable(false),
    selectedPath = $bindable(""),
  }: {
    files: AgentChangedFile[];
    repositoryName: string;
    open?: boolean;
    selectedPath?: string;
  } = $props();
  const selectedFile = $derived(
    files.find((file) => file.path === selectedPath) ?? files[0],
  );
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="agent-changes-dialog">
    <Dialog.Header>
      <Dialog.Title>Changes in {repositoryName}</Dialog.Title>
      <Dialog.Description>
        Saved changes since this agent's starting revision.
      </Dialog.Description>
    </Dialog.Header>
    <label class="agent-changes-file-picker">
      Changed file
      <select
        aria-label="Changed file"
        value={selectedFile?.path}
        onchange={(e) => (selectedPath = e.currentTarget.value)}
      >
        {#each files as file (file.path)}
          <option value={file.path}>{file.path}</option>
        {/each}
      </select>
    </label>
    <div class="agent-changes-diff">
      {#if selectedFile?.patch !== undefined}
        <CommunityCommitDiff
          single
          files={[{ path: selectedFile.path, patch: selectedFile.patch }]}
          label="Saved agent change"
        />
      {:else}
        <p>{selectedFile?.unavailable ?? "Text diff unavailable."}</p>
      {/if}
    </div>
  </Dialog.Content>
</Dialog.Root>

<style>
  :global(.agent-changes-dialog) {
    width: min(95vw, 60rem);
    max-width: 60rem;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .agent-changes-file-picker {
    font-size: 0.8125rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
  }
  .agent-changes-file-picker select {
    min-width: 0;
    flex: 1;
    padding: 0.375rem;
    color: var(--foreground);
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .agent-changes-diff {
    overflow: auto;
    min-height: 0;
  }
</style>
