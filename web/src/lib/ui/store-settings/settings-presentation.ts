import { getContext, setContext } from "svelte";

export type SettingsPresentation = "default" | "repository" | "global";
const key = Symbol("store-settings-presentation");
type PresentationOwner = { readonly value: SettingsPresentation };

export function provideSettingsPresentation(owner: PresentationOwner) {
  setContext(key, owner);
}

export function settingsPresentation(): PresentationOwner {
  return getContext<PresentationOwner>(key) ?? { value: "default" };
}
