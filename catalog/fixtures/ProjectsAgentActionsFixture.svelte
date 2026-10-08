<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import ProjectsHubFixture from "./ProjectsHubFixture.svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type { AgentStatus } from "../../web/src/lib/controller/agent-execution-types.js";
  import { fixtureJsonResponse } from "./agent-execution-session.js";
  let {
    projectDetail = false,
    detail = false,
  }: { projectDetail?: boolean; detail?: boolean } = $props();
  const status: AgentStatus = {
    version: 1,
    runtime: "ready",
    revision: 0,
    sessions: [],
    setups: [],
    runners: [],
    conversations: [],
    live_agents: [],
  };
  const execution = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input) =>
          fixtureJsonResponse(String(input).includes("/targets") ? [] : status),
      ),
  );
  execution.value = status;
  onDestroy(() => execution.destroy());
</script>

<ProjectsHubFixture
  {execution}
  {projectDetail}
  {detail}
  agents={!projectDetail && !detail}
/>
<output aria-label="Launch context">{JSON.stringify(execution.context)}</output>
