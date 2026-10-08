import {
  accents,
  lightThemes,
  themeNames,
  themePairs,
  type ThemeName,
} from "./theme-catalog.js";
export type ColorMode = "system" | "light" | "dark";
export type FontSize = "smaller" | "default" | "larger";
export type ConversationDensity = "compact" | "comfortable" | "spacious";
export type LinkStyle = "compact" | "rich";
export type ConversationView = "document" | "classic";
export type ThreadLayout = "focus" | "split";
export interface AppearancePreferences {
  version: 1;
  theme: ThemeName;
  mode: ColorMode;
  accent: string;
  glass: boolean;
  opacity: number;
  prominent: boolean;
  fontSize: FontSize;
  density: ConversationDensity;
  links: LinkStyle;
  threads: ThreadLayout;
  conversationView: ConversationView;
}
export const appearanceStorageKey = "jj-store.appearance.v1";
export const defaultAppearance: AppearancePreferences = {
  version: 1,
  theme: "buzz",
  mode: "system",
  accent: "#3b82f6",
  glass: false,
  opacity: 65,
  prominent: false,
  fontSize: "default",
  density: "comfortable",
  links: "compact",
  threads: "split",
  conversationView: "document",
};
export function parseAppearance(raw: string | null): AppearancePreferences {
  try {
    const value = JSON.parse(raw ?? "null");
    if (!value || value.version !== 1) return { ...defaultAppearance };
    const pick = <T extends string>(
      v: unknown,
      values: readonly T[],
      fallback: T,
    ): T => (values.includes(v as T) ? (v as T) : fallback);
    const mode = pick(value.mode, ["system", "light", "dark"], "system");
    const storedTheme = pick(value.theme, themeNames, "buzz");
    return {
      version: 1,
      theme:
        mode === "system" && !themePairs.has(storedTheme)
          ? "buzz"
          : storedTheme,
      mode,
      accent: pick(
        value.accent,
        accents.map((c) => c.value),
        "#3b82f6",
      ),
      glass: value.glass === true,
      prominent: value.prominent === true,
      opacity:
        typeof value.opacity === "number" && Number.isFinite(value.opacity)
          ? Math.round(Math.max(30, Math.min(90, value.opacity)))
          : 65,
      fontSize: pick(
        value.fontSize,
        ["smaller", "default", "larger"],
        "default",
      ),
      density: pick(
        value.density,
        ["compact", "comfortable", "spacious"],
        "comfortable",
      ),
      links: pick(value.links, ["compact", "rich"], "compact"),
      threads: pick(value.threads, ["focus", "split"], "split"),
      conversationView: pick(
        value.conversationView,
        ["document", "classic"],
        "document",
      ),
    };
  } catch {
    return { ...defaultAppearance };
  }
}
export function resolveTheme(
  prefs: AppearancePreferences,
  systemDark: boolean,
): ThemeName {
  const dark = prefs.mode === "dark" || (prefs.mode === "system" && systemDark);
  return lightThemes.has(prefs.theme) !== !dark
    ? (themePairs.get(prefs.theme) ?? prefs.theme)
    : prefs.theme;
}
export function switchMode(
  prefs: AppearancePreferences,
  mode: ColorMode,
): Partial<AppearancePreferences> {
  if (mode === "system")
    return { mode, theme: themePairs.has(prefs.theme) ? prefs.theme : "buzz" };
  const needsLight = mode === "light";
  return {
    mode,
    theme:
      lightThemes.has(prefs.theme) === needsLight
        ? prefs.theme
        : (themePairs.get(prefs.theme) ?? (needsLight ? "buzz" : "buzz-dark")),
  };
}
