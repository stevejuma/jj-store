import { ApiError } from "../api/client.js";
import { captureReadBusy } from "./capture-errors.js";
import type { Model } from "../api/client.js";
import type {
  CaptureManagementState,
  CaptureTransport,
} from "./capture-types.js";
import {
  captureConfigDraft,
  captureConfigValue,
  sameCaptureConfiguration,
  validateCaptureSettings,
  type CaptureDraft,
  type CaptureSaveSettings,
} from "./capture-configuration.js";
export function createCaptureSettings(options: {
  state: CaptureManagementState;
  transport: CaptureTransport;
  available(): boolean;
}) {
  const { state, transport } = options;
  const settings = state.settings;
  let epoch = 0;
  function accept(value: Model<"LocalCaptureSettings">) {
    validateCaptureSettings(value);
    settings.value = value;
    if (settings.unknown) settings.unknownObserved = true;
    if (settings.dirty && settings.baseRevision !== value.revision)
      settings.conflict = true;
    if (
      settings.unknown &&
      sameCaptureConfiguration(value.desired, settings.unknown.configuration)
    ) {
      settings.unknown = null;
      settings.dirty = false;
      settings.conflict = false;
      settings.message =
        "Saved configuration matches the original request. Changes apply after restarting the local server.";
    }
    if (!settings.dirty) {
      settings.draft = captureConfigDraft(value.desired);
      settings.baseRevision = value.revision;
      settings.conflict = false;
    }
    settings.error = "";
  }
  return {
    async observe(signal?: AbortSignal) {
      if (!transport.settings || settings.saving) return;
      const selected = epoch;
      try {
        const value = await transport.settings(signal);
        if (selected === epoch && options.available() && !signal?.aborted)
          accept(value);
      } catch (cause) {
        if (
          selected === epoch &&
          options.available() &&
          !signal?.aborted &&
          !captureReadBusy(cause)
        )
          settings.error =
            "Agent configuration could not be loaded. Refresh settings to try again.";
      }
    },
    configure(patch: Partial<CaptureDraft>) {
      if (!settings.draft || settings.saving || settings.unknown) return;
      Object.assign(settings.draft, patch);
      settings.dirty = true;
      settings.validation = "";
      settings.message = "";
    },
    cancelConfiguration() {
      if (
        !settings.value ||
        settings.saving ||
        (settings.unknown && !settings.unknownObserved)
      )
        return;
      settings.unknown = null;
      settings.baseRevision = settings.value.revision;
      settings.conflict = false;
      settings.draft = captureConfigDraft(settings.value.desired);
      settings.dirty = false;
      settings.validation = "";
    },
    async saveConfiguration() {
      if (
        !options.available() ||
        !transport.saveSettings ||
        !settings.value ||
        !settings.draft ||
        settings.saving ||
        state.commandBusy ||
        state.pending ||
        settings.unknown ||
        !settings.dirty ||
        settings.conflict
      )
        return;
      let configuration: Model<"CaptureConfiguration">;
      try {
        configuration = captureConfigValue(
          settings.draft,
          settings.value.defaults,
        );
      } catch (cause) {
        settings.validation =
          cause instanceof Error ? cause.message : "Check configuration.";
        return;
      }
      const command = {
        version: 1,
        expected_revision: settings.baseRevision,
        configuration,
      };
      epoch++;
      settings.saving = true;
      settings.error = settings.message = settings.validation = "";
      try {
        const value = await transport.saveSettings(command);
        validateCaptureSettings(value);
        if (!sameCaptureConfiguration(value.desired, configuration))
          throw new Error("Configuration acknowledgement changed");
        if (!options.available()) return;
        settings.dirty = false;
        accept(value);
        settings.message = value.restart_required
          ? "Configuration saved. Restart the local server to apply these changes."
          : "Configuration saved. It matches the running configuration.";
      } catch (cause) {
        if (!options.available()) return;
        if (
          cause instanceof ApiError &&
          [
            "capture_settings_changed",
            "invalid_capture_settings",
            "capture_management_denied",
            "capture_control_busy",
            "invalid_request",
            "limit_exceeded",
          ].includes(cause.code)
        ) {
          if (cause.code === "capture_settings_changed")
            settings.conflict = true;
          settings.error =
            cause.code === "capture_settings_changed"
              ? "Server configuration changed. Your draft is retained. Refresh to review the latest saved settings before saving again."
              : "Configuration could not be saved. Check the fields and refresh settings.";
        } else {
          settings.unknown = command;
          settings.unknownObserved = false;
          settings.error =
            "Save outcome unknown. Your original draft is retained. Refresh settings to check the saved configuration.";
        }
      } finally {
        settings.saving = false;
      }
    },
  };
}
