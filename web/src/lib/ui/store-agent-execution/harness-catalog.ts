import {
  object,
  type AgentSession,
} from "../../controller/agent-execution-types.js";
import type { AgentStatus } from "../../controller/agent-execution-types.js";
import type { EditorModelRow } from "../../community/agent-editor-model-rows.js";
import { runtimeChoices } from "./presentation.js";

/** Shared Add/Edit and installation defaults choices; saved unknowns stay visible. */
export function harnessModelRows(
  status: AgentStatus | null,
  discovered: (runner: string) => unknown,
  selectedRunner: string,
  selectedModel: string | null | undefined,
): EditorModelRow[] {
  return (status?.runners ?? []).flatMap((runner) => {
    const { facts } = harnessCatalog(
      status?.sessions ?? [],
      runner.id,
      discovered(runner.id),
    );
    const options = runtimeChoices(facts, "model");
    return [
      {
        runtimeAgent: runner.id,
        harnessLabel: runner.label,
        model: "",
        name: "Harness default",
        keywords: [runner.id],
      },
      ...options.map((option) => ({
        runtimeAgent: runner.id,
        harnessLabel: runner.label,
        model: option.id,
        name: option.name,
        keywords: [option.id],
      })),
      ...(runner.id === selectedRunner &&
      selectedModel &&
      !options.some((o) => o.id === selectedModel)
        ? [
            {
              runtimeAgent: runner.id,
              harnessLabel: runner.label,
              model: selectedModel,
              name: `${selectedModel} (saved preference)`,
              disabled: true,
              keywords: [selectedModel],
            },
          ]
        : []),
    ];
  });
}

/** Live negotiation takes precedence; a stopped session retains remembered choices. */
export function harnessCatalog(
  sessions: readonly AgentSession[],
  runner: string,
  discovered?: unknown,
) {
  const matching = sessions.filter((session) => session.runner === runner);
  const session =
    matching.find((session) => session.connected) ??
    matching
      .filter(
        (session) =>
          session.adapter_session_id &&
          Object.keys(object(object(session.capabilities).session)).length,
      )
      .sort(
        (a, b) =>
          Number(b.last_activity_at_millis ?? 0) -
          Number(a.last_activity_at_millis ?? 0),
      )[0];
  return {
    session,
    facts:
      session?.connected || !discovered
        ? object(object(session?.capabilities).session)
        : object(discovered),
    remembered: !!session && !session.connected && !discovered,
  };
}
