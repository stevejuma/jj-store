import { untrack } from "svelte";
import {
  agentMentionToken,
  mentionedAgentIds,
} from "../../community/agent-mentions.js";
import {
  readAgentAudience,
  writeAgentAudience,
} from "../../community/agent-audience.js";
import type { CommunityMemberModel } from "../community-workspace/model.js";
/** Community-style opt-in pins, scoped to exact local agent identities. */
export function createComposerAgentAudience(input: {
  scope(): string | undefined;
  initial(): readonly string[];
  members(): readonly CommunityMemberModel[];
  value(): string;
  setValue(value: string): void;
}) {
  let enabled = $state(false),
    pinned = $state<string[]>([]);
  let loaded = "",
    previous: string[] = [];
  const storage = () => {
    try {
      return typeof localStorage === "undefined" ? undefined : localStorage;
    } catch {
      return undefined;
    }
  };
  const known = (ids: readonly string[]) =>
    ids
      .filter((id) => input.members().some((m) => m.acpAgentId === id))
      .slice(0, 4);
  const persist = () => {
    const scope = input.scope();
    if (scope) writeAgentAudience(scope, enabled, pinned, storage());
  };
  $effect(() => {
    const scope = input.scope();
    untrack(() => {
      if (scope === loaded) return;
      loaded = scope ?? "";
      const saved = scope
        ? readAgentAudience(scope, storage())
        : { enabled: false, ids: [] };
      enabled = saved.enabled;
      pinned = saved.ids;
      if (enabled) {
        const present = mentionedAgentIds(input.value());
        input.setValue(
          [
            ...pinned
              .filter((id) => !present.includes(id))
              .map(agentMentionToken),
            input.value(),
          ]
            .filter(Boolean)
            .join(" "),
        );
      }
      previous = mentionedAgentIds(input.value());
    });
  });
  $effect(() => {
    const next = mentionedAgentIds(input.value());
    untrack(() => {
      if (enabled) {
        const removed = previous.filter((id) => !next.includes(id));
        const updated = [
          ...new Set([
            ...pinned.filter((id) => !removed.includes(id)),
            ...known(next),
          ]),
        ].slice(0, 4);
        if (JSON.stringify(pinned) !== JSON.stringify(updated)) {
          pinned = updated;
          persist();
        }
      }
      previous = next;
    });
  });
  function unpin(id: string) {
    pinned = pinned.filter((p) => p !== id);
    input.setValue(
      input.value().replaceAll(agentMentionToken(id), "").trimStart(),
    );
    persist();
  }
  return {
    get available() {
      return !!input.scope() && input.members().some((m) => !!m.acpAgentId);
    },
    get lockedMemberIds() {
      return input
        .members()
        .filter((m) => m.acpAgentId && pinned.includes(m.acpAgentId))
        .map((m) => m.id);
    },
    get enabled() {
      return enabled;
    },
    get pinned() {
      return pinned;
    },
    get mentionButtonMember() {
      return enabled
        ? pinned.flatMap((id) =>
            input.members().filter((m) => m.acpAgentId === id),
          )[0]
        : undefined;
    },
    toggle(next: boolean) {
      enabled = next;
      pinned = next
        ? known([
            ...new Set([
              ...input.initial(),
              ...mentionedAgentIds(input.value()),
            ]),
          ])
        : [];
      if (next) {
        const present = mentionedAgentIds(input.value());
        input.setValue(
          [
            ...pinned
              .filter((id) => !present.includes(id))
              .map(agentMentionToken),
            input.value(),
          ]
            .filter(Boolean)
            .join(" "),
        );
        previous = mentionedAgentIds(input.value());
      }
      persist();
    },
    unpin,
    dismissMember(id: string) {
      const agentId = input.members().find((m) => m.id === id)?.acpAgentId;
      if (agentId) unpin(agentId);
    },
    toggleMember(id: string) {
      if (!input.scope()) return;
      const agentId = input.members().find((m) => m.id === id)?.acpAgentId;
      if (!agentId) return;
      if (pinned.includes(agentId)) return unpin(agentId);
      const updated = known([...pinned, agentId]);
      if (!updated.includes(agentId)) return;
      enabled = true;
      pinned = updated;
      if (!mentionedAgentIds(input.value()).includes(agentId))
        input.setValue(`${agentMentionToken(agentId)} ${input.value()}`);
      persist();
    },
    restore(submitted: string) {
      if (!enabled) {
        previous = [];
        return "";
      }
      pinned = known([
        ...new Set([...pinned, ...mentionedAgentIds(submitted)]),
      ]);
      persist();
      previous = [...pinned];
      return (
        pinned.map(agentMentionToken).join(" ") + (pinned.length ? " " : "")
      );
    },
  };
}
