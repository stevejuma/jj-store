import type { Model } from "../api/client.js";
export type AgentConfiguration = Pick<
  Model<"AgentExecutionSetup">,
  "runner" | "preferences" | "options"
>;
export function canonicalProvider(value: string | null | undefined) {
  return value === "openai"
    ? "openai-compat"
    : value === "databricks-v2"
      ? "databricks_v2"
      : (value ?? "local");
}
export function changeHarness(draft: AgentConfiguration, runner: string) {
  if (draft.runner === runner) return;
  draft.runner = runner;
  draft.preferences = { model: null, effort: null, mode: null };
  draft.options.provider = null;
  draft.options.account = null;
  draft.options.endpoint = null;
}
export function changeProvider(
  draft: AgentConfiguration,
  provider: string | null,
) {
  if (draft.options.provider === provider) return;
  draft.options.provider = provider;
  draft.options.account = null;
  draft.options.endpoint = null;
  draft.preferences.model = null;
  draft.preferences.effort = null;
}
export function changeEndpoint(draft: AgentConfiguration, endpoint: string) {
  if ((draft.options.endpoint ?? "") === endpoint) return;
  draft.options.endpoint = endpoint || null;
  draft.preferences.model = null;
  draft.preferences.effort = null;
}
/** Only provider-identical session settings may be applied without restarting. */
export function sameLaunchOptions(
  a: AgentConfiguration,
  b: AgentConfiguration,
) {
  return (
    a.runner === b.runner &&
    JSON.stringify(a.options) === JSON.stringify(b.options)
  );
}

export function changeAccount(
  draft: AgentConfiguration,
  account: string | null,
) {
  if ((draft.options.account ?? null) === account) return;
  draft.options.account = account;
  draft.preferences.model = null;
  draft.preferences.effort = null;
}
