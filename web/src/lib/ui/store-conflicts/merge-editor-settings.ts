import { resolveLayoutStorage } from "../layout-storage.js";

export const mergeEditorSettingsKey = "jj-store/merge-editor/v1";
export interface MergeEditorSettings {
  wrap: boolean;
  syncHorizontalScroll: boolean;
  ignoreWhitespace: boolean;
  ignoreCase: boolean;
}
export const defaultMergeEditorSettings: Readonly<MergeEditorSettings> = {
  wrap: false,
  syncHorizontalScroll: true,
  ignoreWhitespace: false,
  ignoreCase: false,
};

export function loadMergeEditorSettings(
  storage = resolveLayoutStorage(),
): MergeEditorSettings {
  try {
    const value = JSON.parse(
      storage?.getItem(mergeEditorSettingsKey) ?? "null",
    );
    if (
      value?.version === 1 &&
      typeof value.wrap === "boolean" &&
      typeof value.syncHorizontalScroll === "boolean" &&
      (value.ignoreWhitespace === undefined ||
        typeof value.ignoreWhitespace === "boolean") &&
      (value.ignoreCase === undefined || typeof value.ignoreCase === "boolean")
    ) {
      return {
        wrap: value.wrap,
        syncHorizontalScroll: value.syncHorizontalScroll,
        ignoreWhitespace: value.ignoreWhitespace ?? false,
        ignoreCase: value.ignoreCase ?? false,
      };
    }
  } catch {
    // Display remains usable when browser storage is unavailable or malformed.
  }
  return { ...defaultMergeEditorSettings };
}

export function saveMergeEditorSettings(
  settings: MergeEditorSettings,
  storage = resolveLayoutStorage(),
): void {
  try {
    storage?.setItem(
      mergeEditorSettingsKey,
      JSON.stringify({
        version: 1,
        wrap: settings.wrap,
        syncHorizontalScroll: settings.syncHorizontalScroll,
        ignoreWhitespace: settings.ignoreWhitespace,
        ignoreCase: settings.ignoreCase,
      }),
    );
  } catch {
    // A quota or privacy restriction must not interrupt conflict editing.
  }
}
