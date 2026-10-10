<script lang="ts">
  import StoreLazyView from "./StoreLazyView.svelte";
  import { views } from "./application-views.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../controller/contracts.js";
  let {
    controller,
    current,
  }: { controller: StoreController; current: StoreRoute } = $props();
</script>

{#if controller.execution}<StoreLazyView load={views.agents} label="Agents"
    >{#snippet content(View)}<View
        {controller}
        manager={controller.execution!}
        repositories={controller.state.repositories}
        route={current}
        onReview={(repository, commit, operation) =>
          void controller.actions.openRoute?.({
            ...current,
            agentsHub: false,
            agentView: undefined,
            executionSessionId: null,
            setupId: null,
            repository,
            page: "log",
            commitId: commit,
            snapshotOperationId: operation,
            agentSessionId: null,
            projectsHub: false,
          })}
      />{/snippet}</StoreLazyView
  >{/if}
