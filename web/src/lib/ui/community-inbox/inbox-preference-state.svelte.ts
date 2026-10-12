import { tick, untrack, onMount } from "svelte";
import {
  readInboxPreferences,
  saveInboxPreferences,
  toggleInboxPreference,
  inboxPreferencesKey,
} from "./inbox-preferences.js";

export function createInboxPreferenceState(
  storage: Storage | undefined,
  kind: () => "session" | "thread",
  listElement: () => HTMLElement | null,
) {
  let preferences = $state(
    untrack(() => readInboxPreferences(storage, kind())),
  );
  function togglePreference(field: "pinned" | "archived", id: string) {
    const restoreFocus = document.activeElement?.closest(".inbox-row-actions");
    preferences = toggleInboxPreference(preferences, field, id);
    saveInboxPreferences(storage, kind(), preferences);
    if (restoreFocus)
      void tick().then(() => {
        const row = [
          ...(listElement()?.querySelectorAll<HTMLElement>(
            "[data-inbox-identity]",
          ) ?? []),
        ].find((row) => row.dataset.inboxIdentity === id);
        const next = row?.querySelector<HTMLButtonElement>(
          `button[aria-label="${field === "pinned" ? (preferences.pinned.includes(id) ? "Unpin conversation" : "Pin conversation") : preferences.archived.includes(id) ? "Archive on this device" : "Restore on this device"}"]`,
        );
        (
          next ??
          listElement()?.querySelector<HTMLButtonElement>(
            'button[aria-label="Search loaded conversations"],button[aria-label="Close search"]',
          )
        )?.focus({ preventScroll: true });
      });
  }
  onMount(() => {
    const update = (event: StorageEvent) => {
      if (
        event.storageArea === storage &&
        (event.key === inboxPreferencesKey(kind()) || event.key === null)
      )
        preferences = readInboxPreferences(storage, kind());
    };
    window.addEventListener("storage", update);
    return () => window.removeEventListener("storage", update);
  });
  return {
    get value() {
      return preferences;
    },
    toggle: togglePreference,
  };
}
