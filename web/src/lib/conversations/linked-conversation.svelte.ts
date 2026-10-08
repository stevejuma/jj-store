import { untrack } from "svelte";
import { originReplyParent } from "../controller/agent-chat-context.js";
import type { StoreController } from "../controller/contracts.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
import { activityRows } from "../ui/store-agent-execution/presentation.js";
import { agentConversationProjection } from "./agent-adapter.js";

/** A body observes its own participation without changing the selected chat. */
export function useLinkedConversation(
  read: () => {
    controller?: StoreController;
    scope: string;
    home: string | null;
    parent?: string;
    user: CommunityMemberModel;
  },
) {
  const options = $derived(read());
  const ids = $derived(
    new Set(
      options.controller?.execution?.value?.conversations
        .filter(
          (c) =>
            c.origin.scope === options.scope &&
            c.origin.home === options.home &&
            originReplyParent(c.origin) === (options.parent ?? null),
        )
        .map((c) => c.id),
    ),
  );
  const sessions = $derived(
    options.controller?.execution?.value?.sessions.filter(
      (s) => s.conversation_id && ids.has(s.conversation_id),
    ) ?? [],
  );
  const rows = $derived(
    sessions
      .flatMap((session) =>
        activityRows(
          options.controller?.execution?.chat.events[session.id] ?? [],
        ).map((row) => ({ ...row, session, key: `${session.id}:${row.id}` })),
      )
      .sort(
        (a, b) =>
          (a.observedAtMillis ?? 0) - (b.observedAtMillis ?? 0) ||
          a.sequence - b.sequence,
      ),
  );
  const projection = $derived(
    agentConversationProjection(
      rows,
      options.controller?.execution?.discussions.values ?? {},
      options.user,
    ),
  );
  $effect(() => {
    const { controller, scope, home } = options;
    return untrack(() => controller?.execution?.chat.watch(scope, home));
  });
  return {
    get sessions() {
      return sessions;
    },
    get rows() {
      return rows;
    },
    get projection() {
      return projection;
    },
  };
}
