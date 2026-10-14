import type { Model } from "../api/client.js";
export type RunnerEnvironment = Model<"WorkflowRunnerEnvironment">;
export type HostSettings = Model<"ManagedWorkflowHostConfiguration">;
/** Read legacy execution settings without changing saved attachment mappings. */
export function runnerEnvironment(value: HostSettings): RunnerEnvironment {
  return {
    version: 2,
    host: value.host,
    labels: [...(value.labels ?? [])],
    commands: value.commands ?? {},
    forgejo: value.forgejo ?? undefined,
    agents: [...(value.agents ?? [])],
    agent_endpoint: value.agent_endpoint ?? null,
    allowed_owners: [...(value.allowed_owners ?? [])],
    state_dir: value.state_dir || null,
    jj_executable: value.jj_executable || null,
    poll_millis: value.poll_millis ?? 1000,
  };
}
export function newRunnerEnvironment(): RunnerEnvironment {
  return {
    version: 2,
    host: "local",
    labels: [],
    commands: {},
    agents: [],
    agent_endpoint: null,
    allowed_owners: [],
    state_dir: null,
    jj_executable: null,
    poll_millis: 1000,
  };
}
