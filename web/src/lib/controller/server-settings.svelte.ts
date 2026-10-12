import { ApiError } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import {
  copy,
  equal,
  validateSettings,
  validateConfiguration,
  type Configuration,
  type Section,
  type ServerSettings,
  type SaveServerSettings,
} from "./server-configuration.js";
export interface ConfigurationDraft {
  draft: Configuration[Section] | null;
  baseRevision: string;
  dirty: boolean;
  conflict: boolean;
  unknown: SaveServerSettings | null;
  unknownObserved: boolean;
  validation: string;
  error: string;
  message: string;
}
const empty = (): ConfigurationDraft => ({
  draft: null,
  baseRevision: "",
  dirty: false,
  conflict: false,
  unknown: null,
  unknownObserved: false,
  validation: "",
  error: "",
  message: "",
});
export function createServerSettings(options: {
  available(): boolean;
  client?: Pick<LocalClient, "request" | "close">;
}) {
  const client = options.client ?? new LocalClient("");
  const state = $state({
    value: null as ServerSettings | null,
    loading: false,
    saving: false,
    error: "",
    drafts: { service: empty(), protection: empty(), sync: empty() },
  });
  let epoch = 0;
  let observing: Promise<void> | null = null;
  let closed = false;
  const available = () => !closed && options.available();
  function accept(value: ServerSettings) {
    validateSettings(value);
    state.value = value;
    state.error = "";
    for (const section of ["service", "protection", "sync"] as const) {
      const edit = state.drafts[section];
      if (edit.unknown) {
        edit.unknownObserved = true;
        if (equal(value.desired[section], edit.unknown.update.configuration)) {
          edit.unknown = null;
          edit.dirty = false;
          edit.error = "";
          edit.message = "Saved settings match the original request.";
        }
      }
      edit.conflict = edit.dirty && edit.baseRevision !== value.revision;
      if (!edit.dirty) {
        edit.draft = copy(value.desired[section]);
        edit.baseRevision = value.revision;
      }
    }
  }
  async function observe() {
    if (!available() || state.saving) return;
    if (observing) return observing;
    const selected = epoch;
    state.loading = true;
    observing = (async () => {
      try {
        const value = await client.request<ServerSettings>("settings");
        if (available() && selected === epoch) accept(value);
      } catch {
        if (available() && selected === epoch)
          state.error =
            "Server configuration could not be loaded. Refresh settings to try again.";
      } finally {
        state.loading = false;
        observing = null;
      }
    })();
    return observing;
  }
  function configure<S extends Section>(section: S, value: Configuration[S]) {
    const edit = state.drafts[section];
    if (!available() || !edit.draft || state.saving || edit.unknown) return;
    edit.draft = copy(value);
    edit.dirty = true;
    edit.validation = "";
    edit.message = "";
  }
  function cancel(section: Section) {
    const edit = state.drafts[section];
    if (!state.value || state.saving || (edit.unknown && !edit.unknownObserved))
      return;
    Object.assign(edit, empty(), {
      draft: copy(state.value.desired[section]),
      baseRevision: state.value.revision,
    });
  }
  async function save(section: Section) {
    const edit = state.drafts[section];
    if (
      !available() ||
      !state.value ||
      !edit.draft ||
      !edit.dirty ||
      edit.conflict ||
      edit.unknown ||
      state.saving
    )
      return;
    let command: SaveServerSettings;
    try {
      validateConfiguration({
        ...copy(state.value.desired),
        [section]: copy(edit.draft),
      });
      command = {
        version: 1,
        expected_revision: edit.baseRevision,
        update: { section, configuration: copy(edit.draft) },
      } as SaveServerSettings;
    } catch (cause) {
      edit.validation =
        cause instanceof Error ? cause.message : "Check these settings.";
      return;
    }
    epoch++;
    state.saving = true;
    edit.error = edit.message = "";
    try {
      const value = await client.request<ServerSettings>("settings", command);
      validateSettings(value);
      if (!equal(value.desired[section], command.update.configuration))
        throw new Error("Changed acknowledgement");
      if (!available()) return;
      edit.dirty = false;
      accept(value);
      edit.error = "";
      edit.message = "Settings saved. Reload saved settings to apply changes.";
    } catch (cause) {
      if (!available()) return;
      if (
        cause instanceof ApiError &&
        [
          "server_settings_changed",
          "server_settings_denied",
          "server_settings_busy",
          "invalid_server_settings",
          "invalid_request",
          "limit_exceeded",
        ].includes(cause.code)
      ) {
        edit.conflict = cause.code === "server_settings_changed";
        edit.error = edit.conflict
          ? "Server configuration changed. Your draft is retained; refresh and review the saved values."
          : "Settings could not be saved. Check the fields and refresh settings.";
      } else {
        edit.unknown = command;
        edit.unknownObserved = false;
        edit.error =
          "Save outcome unknown. Your original draft is retained; refresh to inspect saved settings.";
      }
    } finally {
      state.saving = false;
    }
  }
  return {
    state,
    available,
    observe,
    configure,
    cancel,
    save,
    destroy() {
      closed = true;
      epoch++;
      client.close();
    },
  };
}
export type ServerSettingsManagement = ReturnType<typeof createServerSettings>;
