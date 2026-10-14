import {
  runnerEnvironment,
  type HostSettings,
} from "./workflow-runner-environment.js";
import type { Model } from "../api/client.js";
type Host = Model<"WorkflowHostConfiguration">;
type Workflow = Model<"WorkflowConfiguration">;
const id = (v: unknown) => typeof v === "string" && /^[0-9a-f]{32}$/.test(v);
const name = (v: unknown) =>
  typeof v === "string" && /^[a-zA-Z0-9_.-]{1,128}$/.test(v);
const path = (v: unknown) =>
  typeof v === "string" &&
  v.startsWith("/") &&
  v.length <= 4096 &&
  !/\p{Cc}/u.test(v);
function object(v: unknown): Record<string, unknown> {
  if (!v || typeof v !== "object" || Array.isArray(v))
    throw Error("Host configuration must be an object.");
  return v as Record<string, unknown>;
}
function keys(v: Record<string, unknown>, allowed: string[]) {
  if (Object.keys(v).some((k) => !allowed.includes(k)))
    throw Error("Host configuration contains unsupported fields.");
}
function list(
  v: unknown,
  max: number,
  valid: (v: unknown) => boolean,
  required = false,
) {
  if (
    !Array.isArray(v) ||
    v.length > max ||
    (required && !v.length) ||
    v.some((n) => !valid(n)) ||
    new Set(v).size !== v.length
  )
    throw Error("Check the host's principal IDs, labels and configured lists.");
}
export function validateWorkflowHost(value: unknown): asserts value is Host {
  const h = object(value);
  keys(h, [
    "version",
    "host",
    "principal",
    "home",
    "state_dir",
    "jj_executable",
    "allowed_owners",
    "labels",
    "sources",
    "commands",
    "agents",
    "agent_endpoint",
    "poll_millis",
  ]);
  const poll = h.poll_millis ?? 1000;
  if (
    h.version !== 1 ||
    !name(h.host) ||
    !id(h.principal) ||
    typeof poll !== "number" ||
    !Number.isSafeInteger(poll) ||
    poll < 100 ||
    poll > 60000
  )
    throw Error(
      "Check host name, principal and polling interval (100–60000 ms).",
    );
  if (![h.home, h.state_dir, h.jj_executable].every(path))
    throw Error(
      "Host paths must be absolute and contain no control characters.",
    );
  list(h.allowed_owners, 64, id, true);
  list(h.labels ?? [], 32, name);
  list(h.agents ?? [], 32, name);
  if (!Array.isArray(h.sources) || !h.sources.length || h.sources.length > 32)
    throw Error("Configure between one and 32 source attachments.");
  const repos = new Set();
  for (const source of h.sources) {
    const s = object(source);
    keys(s, ["repository", "workspace", "target_id"]);
    if (
      !id(s.repository) ||
      !path(s.workspace) ||
      repos.has(s.repository) ||
      (s.target_id != null && typeof s.target_id !== "string")
    )
      throw Error("Use unique repository IDs and absolute source paths.");
    repos.add(s.repository);
  }
  const commands = object(h.commands ?? {});
  if (Object.keys(commands).length > 32)
    throw Error("Use at most 32 named commands.");
  for (const [key, definition] of Object.entries(commands)) {
    const c = object(definition);
    keys(c, ["program", "args", "environment", "secrets", "artifacts"]);
    if (!name(key) || !path(c.program))
      throw Error(
        "Named commands require a valid name and absolute program path.",
      );
    const args = c.args ?? [];
    if (
      !Array.isArray(args) ||
      args.length > 64 ||
      args.some(
        (v) => typeof v !== "string" || v.length > 4096 || v.includes("\0"),
      )
    )
      throw Error("Check command arguments.");
    const env = object(c.environment ?? {}),
      secrets = object(c.secrets ?? {});
    if (
      Object.keys(env).length + Object.keys(secrets).length > 64 ||
      Object.keys({ ...env, ...secrets }).some(
        (k) => !name(k) || k.startsWith("JJ_WORKFLOW_"),
      ) ||
      Object.values(env).some(
        (v) => typeof v !== "string" || v.length > 4096 || v.includes("\0"),
      ) ||
      Object.values(secrets).some((v) => !name(v))
    )
      throw Error(
        "Secret fields must contain environment names, and reserved workflow variables cannot be overridden.",
      );
    const artifacts = c.artifacts ?? [];
    if (
      !Array.isArray(artifacts) ||
      artifacts.length > 32 ||
      artifacts.some(
        (v) =>
          typeof v !== "string" ||
          v.startsWith("/") ||
          v.includes("\\") ||
          v.split("/").some((p) => ["", ".", "..", ".jj", ".git"].includes(p)),
      )
    )
      throw Error("Artifacts must use safe relative paths.");
  }
  if (h.agent_endpoint != null) {
    const a = object(h.agent_endpoint);
    keys(a, ["url", "local", "token_environment"]);
    if (typeof a.url !== "string" || typeof a.local !== "boolean")
      throw Error("Check the admitted agent endpoint.");
    const u = new URL(a.url);
    if (
      u.pathname !== "/" ||
      u.username ||
      u.password ||
      u.search ||
      u.hash ||
      (a.local
        ? u.protocol !== "http:" ||
          !["127.0.0.1", "[::1]"].includes(u.hostname) ||
          !u.port ||
          a.token_environment != null
        : u.protocol !== "https:" || !name(a.token_environment))
    )
      throw Error(
        "Use literal loopback HTTP for local agents, or HTTPS with a credential environment reference.",
      );
  } else if ((h.agents as unknown[] | undefined)?.length)
    throw Error("Configured agents require an admitted endpoint.");
}
export function parseWorkflowHost(text: string): HostSettings {
  const value: unknown = JSON.parse(text);
  if (object(value).version === 2) {
    const h = object(value);
    keys(h, [
      "version",
      "host",
      "labels",
      "commands",
      "agents",
      "agent_endpoint",
      "allowed_owners",
      "state_dir",
      "jj_executable",
      "poll_millis",
    ]);
    const owners = h.allowed_owners ?? [];
    const endpoint =
      h.agent_endpoint ??
      ((h.agents as unknown[] | undefined)?.length
        ? { url: "http://127.0.0.1:1/", local: true, token_environment: null }
        : null);
    list(owners, 64, id);
    const legacy = parseWorkflowHost(
      JSON.stringify({
        ...h,
        agent_endpoint: endpoint,
        version: 1,
        principal: "01".repeat(16),
        home: "/home",
        state_dir: h.state_dir ?? "/state",
        jj_executable: h.jj_executable ?? "/jj",
        allowed_owners: (owners as unknown[]).length
          ? owners
          : ["01".repeat(16)],
        sources: [
          {
            repository: "01".repeat(16),
            workspace: "/source",
            target_id: null,
          },
        ],
      }),
    );
    return {
      ...runnerEnvironment(legacy),
      agent_endpoint: (h.agent_endpoint ??
        null) as Model<"WorkflowHostAgentEndpoint"> | null,
      allowed_owners: [...(owners as string[])].sort(),
      state_dir: (h.state_dir as string | null) ?? null,
      jj_executable: (h.jj_executable as string | null) ?? null,
    };
  }
  validateWorkflowHost(value);
  return {
    ...value,
    allowed_owners: [...value.allowed_owners].sort(),
    labels: [...(value.labels ?? [])].sort(),
    agents: [...(value.agents ?? [])].sort(),
    poll_millis: value.poll_millis ?? 1000,
    sources: value.sources.map((source) => ({
      ...source,
      target_id: source.target_id ?? null,
    })),
    commands: Object.fromEntries(
      Object.entries(value.commands ?? {}).map(([name, command]) => [
        name,
        {
          ...command,
          args: command.args ?? [],
          environment: command.environment ?? {},
          secrets: command.secrets ?? {},
          artifacts: command.artifacts ?? [],
        },
      ]),
    ),
    agent_endpoint: value.agent_endpoint
      ? {
          ...value.agent_endpoint,
          token_environment: value.agent_endpoint.token_environment ?? null,
        }
      : null,
  };
}
export function validateWorkflowConfiguration(value: Workflow) {
  list(value.admitted, 64, id);
  if (value.local != null) {
    if (typeof value.local.enabled !== "boolean")
      throw Error("Check local host enablement.");
    parseWorkflowHost(JSON.stringify(value.local.configuration));
  }
}

/** Rust persists these lists as sets; match its acknowledgement order. */
export function normalizeWorkflowConfiguration(value: Workflow): Workflow {
  return {
    admitted: [...value.admitted].sort(),
    local: value.local
      ? {
          ...value.local,
          configuration: parseWorkflowHost(
            JSON.stringify(value.local.configuration),
          ),
        }
      : null,
  };
}
