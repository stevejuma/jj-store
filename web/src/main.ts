import { mount } from "svelte";
import App from "./App.svelte";
import "./style.css";
import {
  createAppearanceStore,
  applyAppearance,
} from "./lib/appearance/store.svelte.js";
import { resolveHostEnvironment } from "./lib/host/environment.js";
async function startApplication() {
  const host = await resolveHostEnvironment({
    development: import.meta.env.DEV,
    desktopPreview: import.meta.env.VITE_JJ_STORE_DESKTOP_PREVIEW === "1",
  });
  let storage: Storage | undefined;
  try {
    storage = window.localStorage;
  } catch {
    /* Device storage is optional. */
  }
  const appearance = createAppearanceStore(
    storage,
    window.matchMedia("(prefers-color-scheme: dark)").matches,
  );
  applyAppearance(document.documentElement, appearance, host);
  mount(App, {
    target: document.getElementById("app")!,
    props: { appearance, host },
  });
}
void startApplication();
