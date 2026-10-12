import { createCollabActions } from "./collab-actions.svelte.js";
import { createMessageReactions } from "./message-reactions.svelte.js";
import type { ThreadTransport } from "./thread-types.js";
import type { WorkTransport } from "./work-types.js";

export function workReactionSession(context: {
  source(): WorkTransport;
  repository?(): string;
  principal?(): string;
  changed(): void;
}) {
  let current: ReturnType<typeof createWorkReactions> | undefined;
  return {
    get current() {
      if (!context.repository || !context.principal) return undefined;
      try {
        if (!context.source().reactions) return undefined;
      } catch {
        return undefined;
      }
      return (current ??= createWorkReactions({
        source: () => context.source().reactions!,
        repository: context.repository,
        principal: context.principal,
        changed: context.changed,
      }));
    },
    invalidate() {
      current?.invalidate();
    },
    clear() {
      current?.clear();
      current = undefined;
    },
  };
}

/** One reaction session per metadata home; linked code never owns task reactions. */
export function createWorkReactions(options: {
  source(): ThreadTransport;
  repository(): string;
  principal(): string;
  changed(): void;
}) {
  let generation = 0;
  const valid = (ticket: number, transport: ThreadTransport) => {
    try {
      return ticket === generation && options.source() === transport;
    } catch {
      return false;
    }
  };
  const context = { ...options, generation: () => generation, valid };
  const actions = createCollabActions({
    ...context,
    async refresh(intent) {
      const command = intent.request.command;
      if (
        intent.kind === "reactions" &&
        (command.type === "set" ||
          command.type === "clear" ||
          command.type === "resolve_conflict")
      ) {
        await options.source().invalidate?.("reactions", command.target);
        await reactions.refresh(command.target);
      } else reactions.invalidate();
      options.changed();
    },
  });
  const reactions = createMessageReactions({
    ...context,
    execute: actions.execute,
  });
  return {
    setProfiles: reactions.setProfiles,
    get busy() {
      return actions.busy;
    },
    get pending() {
      return actions.pending;
    },
    get emojis() {
      return reactions.emojis;
    },
    get reactionRevision() {
      return reactions.revision;
    },
    reactionError: reactions.getError,
    getReaction: reactions.getReaction,
    reactionModels: reactions.reactionModels,
    loadEmojis: reactions.loadEmojis,
    loadReactions: reactions.loadReactions,
    toggle: reactions.toggle,
    checkPending: actions.checkPending,
    retryPending: actions.retryPending,
    invalidate: reactions.invalidate,
    clear() {
      generation++;
      actions.reset();
      reactions.clear();
    },
  };
}
