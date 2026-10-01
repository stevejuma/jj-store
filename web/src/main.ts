import { mount } from "svelte";
import App from "./App.svelte";
import "./style.css";
import {
  createAppearanceStore,
  applyAppearance,
} from "./lib/appearance/store.svelte.js";
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
applyAppearance(document.documentElement, appearance);
mount(App, { target: document.getElementById("app")!, props: { appearance } });
