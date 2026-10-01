import { getContext, setContext } from "svelte";
import {
  appearanceStorageKey,
  defaultAppearance,
  parseAppearance,
  resolveTheme,
  switchMode,
  type AppearancePreferences,
  type ColorMode,
} from "./preferences.js";
export function createAppearanceStore(storage?: Storage, systemDark = false) {
  let saved = $state<AppearancePreferences>({ ...defaultAppearance });
  try {
    saved = parseAppearance(storage?.getItem(appearanceStorageKey) ?? null);
  } catch {
    /* Memory-only mode. */
  }
  let preview = $state<Partial<AppearancePreferences>>({});
  let dark = $state(systemDark);
  const live = $derived({ ...saved, ...preview });
  const theme = $derived(resolveTheme(live, dark));
  return {
    get saved() {
      return saved;
    },
    get live() {
      return live;
    },
    get theme() {
      return theme;
    },
    get density() {
      return live.density === "comfortable"
        ? ("balanced" as const)
        : live.density;
    },
    update(patch: Partial<AppearancePreferences>) {
      saved = parseAppearance(JSON.stringify({ ...saved, ...patch }));
      preview = {};
      try {
        storage?.setItem(appearanceStorageKey, JSON.stringify(saved));
      } catch {
        /* Live state remains usable. */
      }
    },
    mode(mode: ColorMode) {
      this.update(switchMode(saved, mode));
    },
    preview(patch: Partial<AppearancePreferences>) {
      preview = patch;
    },
    resetPreview() {
      preview = {};
    },
    system(value: boolean) {
      dark = value;
    },
    restore(raw: string | null) {
      saved = parseAppearance(raw);
      preview = {};
    },
  };
}
export type AppearanceStore = ReturnType<typeof createAppearanceStore>;
const appearanceContext = Symbol("store-appearance");
export function provideAppearance(store: AppearanceStore) {
  setContext(appearanceContext, store);
}
export function useAppearance(): AppearanceStore | undefined {
  return getContext(appearanceContext);
}
export function applyAppearance(root: HTMLElement, store: AppearanceStore) {
  const live = store.live;
  root.dataset.storeTheme = store.theme;
  root.classList.toggle("dark", getIsDark(store.theme));
  root.dataset.storeAccent = live.accent;
  root.dataset.storeFont = live.fontSize;
  root.dataset.storeDensity = live.density;
  root.dataset.storeGlass = String(live.glass);
  root.dataset.storeProminent = String(live.prominent);
  root.style.setProperty("--store-glass-opacity", String(live.opacity / 100));
  root.style.colorScheme = getIsDark(store.theme) ? "dark" : "light";
}
import { lightThemes } from "./theme-catalog.js";
function getIsDark(theme: import("./theme-catalog.js").ThemeName) {
  return !lightThemes.has(theme);
}
export function attachAppearance(
  store: AppearanceStore,
  target: Window,
  root: HTMLElement,
) {
  const query = target.matchMedia("(prefers-color-scheme: dark)");
  store.system(query.matches);
  const system = () => store.system(query.matches);
  const storage = (event: StorageEvent) => {
    if (event.key === appearanceStorageKey || event.key === null)
      store.restore(event.newValue);
  };
  query.addEventListener("change", system);
  target.addEventListener("storage", storage);
  applyAppearance(root, store);
  return () => {
    query.removeEventListener("change", system);
    target.removeEventListener("storage", storage);
  };
}
