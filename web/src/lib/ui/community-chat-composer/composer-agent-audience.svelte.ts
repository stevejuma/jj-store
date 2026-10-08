import { useAgentExecution } from "../store-agent-execution/context.js";
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
/** Shared automatic-mention preference with conversation-scoped recipient pins. */
export function createComposerAgentAudience(input: {
  scope(): string | undefined;
  defaultEnabled?(): boolean;
  initial(): readonly string[];
  members(): readonly CommunityMemberModel[];
  value(): string;
  setValue(value: string): void;
}) {
  const settings = useAgentExecution()?.settings;
  let fallbackEnabled = $state(false),
    pinned = $state<string[]>([]);
  let error = $state("");
  let requested = false;
  const enabled = $derived(
    settings
      ? (settings.value?.automatically_mention ?? false)
      : fallbackEnabled,
  );
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
    if (scope)
      writeAgentAudience(scope, settings ? true : enabled, pinned, storage());
  };
  $effect(() => {
    const scope = input.scope();
    const defaultEnabled =
      input.defaultEnabled?.() ??
      settings?.value?.automatically_mention ??
      false;
    const marker = `${scope}:${defaultEnabled}`;
    untrack(() => {
      if (marker === loaded) return;
      loaded = marker;
      const saved = scope
        ? readAgentAudience(scope, storage(), defaultEnabled)
        : { enabled: false, ids: [] };
      fallbackEnabled = saved.enabled;
      pinned = saved.ids;
      if (settings && enabled)
        pinned = [
          ...new Set([
            ...pinned,
            ...known([...input.initial(), ...mentionedAgentIds(input.value())]),
          ]),
        ].slice(0, 4);
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
      if (settings && enabled) persist();
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
  async function toggle(next: boolean) {
    requested = next;
    error = "";
    if (settings) {
      const accepted = settings.pending
        ? await settings.retry()
        : await settings.setAutomaticallyMention(next);
      if (!accepted) {
        error =
          settings.error ||
          "Agent preference could not be saved. Try again in Settings → Agents.";
        return false;
      }
      return true;
    }
    fallbackEnabled = next;
    pinned = next
      ? known([
          ...new Set([...input.initial(), ...mentionedAgentIds(input.value())]),
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
    return true;
  }
  return {
    get error() {
      return error;
    },
    get busy() {
      return (
        !!settings && (!settings.value || settings.busy || !!settings.pending)
      );
    },
    retry: () => toggle(requested),
    clearError: () => {
      error = "";
    },
    get available() {
      return !!input.scope() && input.members().some((m) => !!m.acpAgentId);
    },
    get lockedMemberIds() {
      if (!enabled) return [];
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
    toggle,
    unpin,
    dismissMember(id: string) {
      const agentId = input.members().find((m) => m.id === id)?.acpAgentId;
      if (agentId) unpin(agentId);
    },
    toggleMember(id: string) {
      if (!input.scope()) return;
      const agentId = input.members().find((m) => m.id === id)?.acpAgentId;
      if (!agentId) return;
      if (enabled && pinned.includes(agentId)) return unpin(agentId);
      const updated = known([...new Set([...pinned, agentId])]);
      if (!updated.includes(agentId)) return;
      pinned = updated;
      // Selecting a recipient edits the draft even if saving the preference fails.
      if (!mentionedAgentIds(input.value()).includes(agentId))
        input.setValue(`${agentMentionToken(agentId)} ${input.value()}`);
      persist();
      void toggle(true);
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
