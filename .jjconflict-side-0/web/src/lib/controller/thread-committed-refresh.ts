import type { CollabIntent } from "./collab-actions.svelte.js";
import type { ReactionTarget, ThreadTransport } from "./thread-types.js";
export async function refreshCommittedThreadAction(
  intent: CollabIntent,
  source: ThreadTransport,
  reactions: { refresh(target: ReactionTarget): Promise<void> },
  restore: () => Promise<void>,
) {
  const command = intent.request.command;
  if (
    intent.kind === "reactions" &&
    (command.type === "set" ||
      command.type === "clear" ||
      command.type === "resolve_conflict")
  ) {
    await source.invalidate?.("reactions", command.target);
    await reactions.refresh(command.target);
  } else {
    await source.invalidate?.("threads");
    await restore();
  }
}
