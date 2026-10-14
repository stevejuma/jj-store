import type { WorkflowDefinition } from "../../controller/workflow-types.js";
import GitBranch from "@lucide/svelte/icons/git-branch";
import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
import Message from "@lucide/svelte/icons/message-square";
import Smile from "@lucide/svelte/icons/smile";
import Play from "@lucide/svelte/icons/play";

export const workflowTriggerIcons = {
  manual: Play,
  code_published: GitCommit,
  bookmark_changed: GitBranch,
  message_posted: Message,
  reaction_added: Smile,
};
const quote = (value: string) => {
  const text = value.trim().replace(/\s+/g, " ");
  return `“${text.length > 36 ? text.slice(0, 33) + "…" : text}”`;
};
const textOperators = {
  contains: "contains",
  not_contains: "doesn’t contain",
  starts_with: "starts with",
  ends_with: "ends with",
  equals: "equals",
  not_equals: "doesn’t equal",
  is_empty: "is empty",
  is_not_empty: "is not empty",
};

/** Display recorded native conditions and executor names; never infer execution. */
export function workflowCardSummary(definition: WorkflowDefinition): string {
  const { trigger, conditions, actions } = definition;
  let when =
    trigger.on === "manual"
      ? "When run manually"
      : trigger.on === "bookmark_changed"
        ? `When bookmark ${quote(trigger.bookmark)} changes`
        : trigger.on === "code_published"
          ? "When code is published"
          : trigger.on === "message_posted"
            ? "When a thread message is posted"
            : conditions?.emoji
              ? `When reaction ${quote(conditions.emoji)} is added`
              : "When a reaction is added";
  if (
    conditions?.text &&
    ["message_posted", "code_published"].includes(trigger.on)
  ) {
    const text = conditions.text;
    const subject =
      trigger.on === "message_posted"
        ? "a thread message"
        : "a published commit description";
    when = `When ${subject} ${textOperators[text.operator]}${text.operator.startsWith("is_") ? "" : " " + quote(text.value)}`;
  }
  if (
    conditions?.revset &&
    ["code_published", "bookmark_changed"].includes(trigger.on)
  )
    when += ` matching ${quote(conditions.revset)}`;
  const first = actions[0];
  if (!first) return when;
  const action =
    first.executor.type === "command"
      ? `run ${quote(first.executor.name)}`
      : first.executor.type === "github_workflow"
        ? `run workflow ${quote(first.executor.path.split("/").at(-1) ?? first.executor.path)}`
        : `ask agent ${quote(first.executor.agent)}`;
  const remaining = actions.length - 1;
  return `${when}, ${action}${remaining ? `, then ${remaining} more ${remaining === 1 ? "step" : "steps"}` : ""}`;
}
