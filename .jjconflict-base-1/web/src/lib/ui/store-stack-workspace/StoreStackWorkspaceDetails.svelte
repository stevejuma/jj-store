<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { selectedWorkspace } from "../../controller/stack-workspace/presentation.js";
  import {
    workspaceBranchSource,
    workspaceSourceStack,
  } from "../../controller/stack-workspace/files.js";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  let {
    controller,
    workspace,
  }: { controller: StoreController; workspace: StackWorkspaceController } =
    $props();
  const snapshot = $derived(workspace.snapshot);
  const active = $derived(snapshot ? selectedWorkspace(snapshot) : undefined);
  const route = $derived(controller.state.route);
  const targetHistory = $derived(
    workspace.page === "branches" && workspace.targetHistory.active,
  );
  const source = $derived(
    workspace.selectedPath
      ? workspace.source
      : (workspace.selectedCommit ??
          (workspace.selectedStack
            ? (workspace.stackSelections[workspace.selectedStack] ??
              `stack:${workspace.selectedStack}`)
            : "")),
  );
  const stack = $derived(
    snapshot ? workspaceSourceStack(snapshot, source) : undefined,
  );
  const branch = $derived(
    stack?.heads.find(
      (head) => source === workspaceBranchSource(stack.id, head.name),
    )?.name,
  );
  const commit = $derived(
    targetHistory
      ? workspace.targetHistory.commits.find(
          (item) => item.id === workspace.targetHistory.selectedId,
        )
      : stack?.commits.find((item) => item.id === workspace.selectedCommit),
  );
  const history = $derived(route.page === "log" || route.page === "oplog");
  const historySelection = $derived(
    route.page === "oplog" ? route.inspectedOperationId : route.commitId,
  );
  const revision = $derived(
    controller.state.graph.revisions.find(
      (item) =>
        item.eventId ===
        (route.page === "oplog" ? route.inspectedOperationId : route.commitId),
    ),
  );
</script>

<section class="stack-workspace-details" aria-label="Stack workspace details">
  <strong
    >{history
      ? route.page === "oplog"
        ? "Operation details"
        : "Commit details"
      : workspace.page === "branches"
        ? "Stack details"
        : "Workspace details"}</strong
  >
  <dl>
    <StoreReadOnlyRow
      stacked
      label="Workspace"
      value={active?.name}
      emptyLabel="No active workspace"
    />
    {#if active}<StoreReadOnlyRow
        stacked
        label="Workspace ID"
        value={active.id}
        copyable
      />{/if}
    <StoreReadOnlyRow
      stacked
      label="Target branch"
      value={snapshot?.target?.name}
      emptyLabel="Choose a target branch"
    />
    <StoreReadOnlyRow
      stacked
      label="Applied stacks"
      value={active?.applied.length ?? 0}
    />
    <StoreReadOnlyRow stacked label="Local folder" value={snapshot?.folder} />
    {#if history}
      {#if revision}<StoreReadOnlyRow
          stacked
          label={route.page === "oplog" ? "Operation" : "Commit"}
          value={revision.eventId}
          copyable
        />
        <StoreReadOnlyRow
          stacked
          label="Description"
          value={revision.message}
        />
        <StoreReadOnlyRow stacked label="Author" value={revision.author} />
        {#if revision.changeId}<StoreReadOnlyRow
            stacked
            label="Change ID"
            value={revision.changeId}
            copyable
          />{/if}
      {:else if historySelection}<StoreReadOnlyRow
          stacked
          label={route.page === "oplog" ? "Operation" : "Commit"}
          value={historySelection}
          copyable
        />
      {:else}<StoreReadOnlyRow
          stacked
          label="Selection"
          value={null}
          emptyLabel={route.page === "oplog"
            ? "Select an operation"
            : "Select a commit"}
        />{/if}
    {:else}
      {#if targetHistory}<StoreReadOnlyRow
          stacked
          label="Branch"
          value={workspace.targetHistory.branch?.name}
        />
      {:else if stack}<StoreReadOnlyRow
          stacked
          label="Stack"
          value={stack.name}
        />
        <StoreReadOnlyRow
          stacked
          label="Branch"
          value={branch ?? stack.name}
        />{/if}
      {#if commit}<StoreReadOnlyRow
          stacked
          label="Commit"
          value={commit.id}
          copyable
        />
        <StoreReadOnlyRow stacked label="Description" value={commit.message} />
        <StoreReadOnlyRow stacked label="Author" value={commit.author} />
        {#if commit.changeId}<StoreReadOnlyRow
            stacked
            label="Change ID"
            value={commit.changeId}
            copyable
          />{/if}
      {/if}
      {#if targetHistory ? workspace.targetHistory.diff?.path : workspace.selectedPath}<StoreReadOnlyRow
          stacked
          label="File"
          value={targetHistory
            ? workspace.targetHistory.diff?.path
            : workspace.selectedPath}
        />{/if}
    {/if}
  </dl>
</section>

<style>
  .stack-workspace-details {
    width: 100%;
    padding: 0.85rem 0.75rem;
    box-sizing: border-box;
  }
  strong {
    display: block;
    padding: 0 0.4rem 0.55rem;
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }
  dl {
    margin: 0;
  }
</style>
