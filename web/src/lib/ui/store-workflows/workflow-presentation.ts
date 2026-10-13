import type { StoreController } from "../../controller/contracts.js";
import type { WorkflowDefinition } from "../../controller/workflow-types.js";
import { parseDocument, stringify } from "yaml";
export interface WorkflowDraft {
  definition: WorkflowDefinition;
  mode: string;
  yaml: string;
  unapplied: boolean;
}
export function initialDraft(definition: WorkflowDefinition): WorkflowDraft {
  return {
    definition,
    mode: "form",
    yaml: yamlDefinition(definition),
    unapplied: false,
  };
}
export const triggerLabel = (trigger: WorkflowDefinition["trigger"]) =>
  trigger.on === "manual"
    ? "Run manually"
    : trigger.on === "code_published"
      ? "Code published"
      : trigger.on === "bookmark_changed"
        ? `Bookmark ${trigger.bookmark}`
        : trigger.on === "message_posted"
          ? "Thread message posted"
          : "Reaction added";
export const executorLabel = (action: WorkflowDefinition["actions"][number]) =>
  action.executor.type === "command"
    ? action.executor.name
    : action.executor.agent;
export function initialDefinition(repository = ""): WorkflowDefinition {
  return {
    name: "",
    description: "",
    enabled: false,
    repository,
    trigger: { on: "manual" },
    labels: [],
    actions: [
      {
        id: "build",
        executor: { type: "command", name: "build" },
        inputs: {},
        timeout_seconds: 300,
      },
    ],
  };
}
export function yamlDefinition(definition: WorkflowDefinition) {
  return stringify(definition, { lineWidth: 0 });
}
export function readDefinition(
  text: string,
  draft = false,
): WorkflowDefinition {
  if (new TextEncoder().encode(text).length > 65536)
    throw Error("Workflow YAML exceeds 64 KiB");
  const doc = parseDocument(text, { uniqueKeys: true });
  if (doc.errors.length) throw Error(doc.errors[0]!.message);
  const value = doc.toJS({ maxAliasCount: 16 }) as WorkflowDefinition;
  if (
    !value ||
    typeof value !== "object" ||
    typeof value.name !== "string" ||
    (!draft && !value.name.trim()) ||
    typeof value.repository !== "string" ||
    (!draft && !/^[0-9a-f]{32}$/.test(value.repository)) ||
    (value.enabled !== undefined && typeof value.enabled !== "boolean") ||
    (value.description !== undefined &&
      typeof value.description !== "string") ||
    (value.labels !== undefined &&
      (!Array.isArray(value.labels) ||
        value.labels.some((label) => typeof label !== "string"))) ||
    !value.trigger ||
    ![
      "manual",
      "code_published",
      "bookmark_changed",
      "message_posted",
      "reaction_added",
    ].includes(value.trigger.on) ||
    !Array.isArray(value.actions) ||
    (!draft && !value.actions.length) ||
    value.actions.length > 32 ||
    value.actions.some(
      (a) =>
        !a ||
        typeof a.id !== "string" ||
        !a.executor ||
        (a.executor.type === "command"
          ? typeof a.executor.name !== "string"
          : a.executor.type === "agent"
            ? typeof a.executor.agent !== "string" ||
              typeof a.executor.prompt !== "string"
            : true),
    )
  )
    throw Error(
      "Choose a name, source repository, trigger and at least one action",
    );
  if (
    value.trigger.on === "bookmark_changed" &&
    typeof value.trigger.bookmark !== "string"
  )
    throw Error("A bookmark trigger needs a bookmark name");
  return value;
}

export function workflowAuthors(controller: StoreController) {
  return [
    ...controller.state.repositories.flatMap((r) =>
      r.principal ? [{ id: r.principal, name: r.principal }] : [],
    ),
    ...Object.values(controller.directory?.state.profiles ?? {}).map((p) => ({
      id: p.principal,
      name: p.display_name || p.principal,
    })),
    ...controller.state.graph.revisions.flatMap((r) =>
      r.authorEmail
        ? [{ id: r.authorEmail, name: r.author || r.authorEmail }]
        : [],
    ),
  ];
}
