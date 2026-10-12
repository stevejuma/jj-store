import { pretty } from "../api/client.js";
import type {
  AdministrationPending,
  ServerAgentReport,
  ServerAgentConfiguration,
} from "./server-agent-administration-types.js";
/** Principal allowlists are sets; transport ordering cannot block recovery. */
function configurationKey(value: ServerAgentConfiguration | null): string {
  return pretty(
    value && {
      ...value,
      agents: value.agents.map((a) => ({
        ...a,
        principals: [...a.principals].sort(),
      })),
    },
  );
}
export function rejectedAdministration(code: string): boolean {
  return [
    "invalid_agent_configuration",
    "agent_configuration_changed",
    "agent_configuration_busy",
    "agent_administration_denied",
    "invalid_directory_input",
    "directory_changed",
    "directory_denied",
    "invalid_request",
    "access_denied",
  ].includes(code);
}
/** Recovery reads the original intent; it never submits another mutation. */
export async function recoverAdministration(
  request: <T>(path: string, body?: unknown) => Promise<T>,
  intent: AdministrationPending,
): Promise<{ report?: ServerAgentReport; error?: string }> {
  if (intent.kind === "configuration") {
    const report = await request<ServerAgentReport>("operator/agents");
    return configurationKey(report.desired) ===
      configurationKey(intent.body.configuration)
      ? { report }
      : {
          report,
          error:
            "Save remains unresolved. The saved configuration differs from the original intent.",
        };
  }
  const result = await request<{
    request_id: string;
    published?: boolean;
  } | null>(
    intent.kind === "identity"
      ? `directory/commands/${intent.body.request_id}`
      : `repos/${intent.repository}/agent-access/status`,
    intent.kind === "grant" ? intent.body : undefined,
  );
  return !result ||
    result.request_id !== intent.body.request_id ||
    (intent.kind === "grant" && !result.published)
    ? { error: "Original outcome is still unknown. No new request was sent." }
    : {};
}

export async function administrationNames(
  request: <T>(path: string) => Promise<T>,
  ids: string[],
): Promise<Record<string, string>> {
  const names: Record<string, string> = {};
  for (let index = 0; index < ids.length; index += 100) {
    const values = await request<{ principal: string; display_name: string }[]>(
      `directory/profiles?ids=${ids.slice(index, index + 100).join(",")}`,
    );
    for (const profile of values)
      names[profile.principal] = profile.display_name;
  }
  return names;
}

export function administrationPrincipals(report: ServerAgentReport): string[] {
  return [
    ...new Set([
      ...report.principals,
      ...(report.desired?.agents.flatMap((agent) => agent.principals) ?? []),
    ]),
  ];
}
