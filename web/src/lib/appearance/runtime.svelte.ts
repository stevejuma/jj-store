import { onDestroy, untrack } from "svelte";
import { attachNativeGlass } from "./desktop-glass.js";
import {
  applyAppearance,
  attachAppearance,
  createAppearanceStore,
  provideAppearance,
  useAppearance,
  type AppearanceStore,
} from "./store.svelte.js";
/** One device store per app or isolated story, with disposable browser listeners. */
export function ownAppearance(provided?: AppearanceStore, storage?: Storage) {
  const inherited = useAppearance();
  if (inherited && !provided) return inherited;
  const store =
    provided ??
    untrack(() => {
      let local = storage;
      try {
        local ??= globalThis.localStorage;
      } catch {
        /* Storage may be unavailable. */
      }
      return createAppearanceStore(
        local,
        globalThis.matchMedia?.("(prefers-color-scheme: dark)").matches,
      );
    });
  provideAppearance(store);
  if (typeof document !== "undefined") {
    const root = document.documentElement;
    const attributes = [
      "data-ui-scroll-strategy",
      "data-store-theme",
      "data-store-accent",
      "data-store-font",
      "data-store-density",
      "data-store-glass",
      "data-store-native-glass",
      "data-store-prominent",
    ];
    const before = attributes.map(
      (name) => [name, root.getAttribute(name)] as const,
    );
    const wasDark = root.classList.contains("dark");
    const scheme = root.style.colorScheme;
    const opacity = root.style.getPropertyValue("--store-glass-opacity");
    root.dataset.uiScrollStrategy = "native";
    const dispose = attachAppearance(store, window, root);
    const nativeGlass = attachNativeGlass(root, () =>
      applyAppearance(root, store),
    );
    $effect(() => {
      applyAppearance(root, store);
      void nativeGlass?.set(store.live.glass);
    });
    onDestroy(() => {
      void nativeGlass?.dispose();
      dispose();
      for (const [name, value] of before)
        value === null
          ? root.removeAttribute(name)
          : root.setAttribute(name, value);
      root.classList.toggle("dark", wasDark);
      root.style.colorScheme = scheme;
      root.style.setProperty("--store-glass-opacity", opacity);
    });
  }
  return store;
}

import { provideDrafts } from "./drafts.svelte.js";
export function ownApplicationAppearance(
  repository: () => string,
  storage?: Storage,
) {
  provideDrafts(repository);
  return ownAppearance(undefined, storage);
}
