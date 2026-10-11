<script lang="ts">
  import { onDestroy } from "svelte";
  import ProjectsHubFixture from "./ProjectsHubFixture.svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type { AgentSession } from "../../web/src/lib/controller/agent-execution-types.js";
  import {
    fixtureExecutionSetup,
    fixtureExecutionStatus,
  } from "./agent-execution-status.js";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { homeA, homeB } from "./projects-hub-fixture.js";
  const status = fixtureExecutionStatus({
    repository: "aa".repeat(16),
    operation: "22".repeat(64),
    commit: "33".repeat(20),
    setup: fixtureExecutionSetup(null),
    status: "Online",
    preparing: false,
    alternativeAgent: false,
    permission: false,
    workspacePath: "/fixture/workspace",
    lastActivityAt: Date.now(),
    sessionPreferences: undefined,
    agentModeReady: true,
    advertisedModels: true,
  });
  const session = { ...status.sessions[0], avatar: "/agent-icons/codex.svg" };
  const binding: AgentSession["repository_bindings"][number] = {
    repository: homeA,
    target_id: "fixture",
    profile: "local_jj_git",
    workspace: "/fixture/workspace",
    workspace_name: "fixture",
    selected_operation: session.operation,
    base_commit: session.base_commit,
    operation: session.operation,
    commit: session.base_commit,
  };
  status.sessions = [
    {
      ...session,
      id: "a1".repeat(16),
      conversation_title: "Bound repository chat",
      repository_bindings: [binding],
    },
    {
      ...session,
      id: "a2".repeat(16),
      conversation_title: "Legacy repository chat",
      repository: homeB,
      repository_bindings: [],
    },
    {
      ...session,
      id: "a3".repeat(16),
      conversation_title: "Shared repository chat",
      repository_bindings: [binding, { ...binding, repository: homeB }],
    },
    {
      ...session,
      id: "a4".repeat(16),
      conversation_title: "Inaccessible repository chat",
      repository_bindings: [{ ...binding, repository: "ff".repeat(16) }],
    },
  ];
  const settings = new AgentSettingsFixture();
  const execution = new AgentExecution(
    () => true,
    async (input, init) =>
      settings.respond(String(input), init) ??
      Response.json(String(input).includes("/targets") ? [] : status),
  );
  execution.value = status;
  let selected = $state("");
  execution.onNavigate = (view, id) => (selected = `${view}:${id}`);
  onDestroy(() => execution.destroy());
</script>

<ProjectsHubFixture {execution} agents />
<output aria-label="Selected interactive session">{selected}</output>
