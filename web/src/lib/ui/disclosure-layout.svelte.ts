import { getContext, setContext, untrack } from "svelte";
import {
  disclosureKey,
  loadDisclosures,
  saveDisclosures,
} from "./disclosure-storage.js";
import type { StoreRoute } from "../controller/contracts.js";
const contextKey = Symbol("store-disclosure-layout");
export function createDisclosureLayout(
  storage: Storage | undefined,
  route: () => StoreRoute,
) {
  let flags = $state(loadDisclosures(storage));
  const key = (section: readonly string[]) => disclosureKey(route(), section);
  return {
    read(section: readonly string[], fallback: boolean) {
      return flags[key(section)] ?? fallback;
    },
    write(section: readonly string[], open: boolean) {
      flags = { ...flags, ...loadDisclosures(storage), [key(section)]: open };
      const entries = Object.entries(flags);
      if (entries.length > 1024)
        flags = Object.fromEntries(entries.slice(-1024));
      saveDisclosures(storage, flags);
    },
  };
}
export function provideDisclosureLayout(
  storage: Storage | undefined,
  route: () => StoreRoute,
) {
  setContext(contextKey, createDisclosureLayout(storage, route));
}
/** Section identities are stable keys, never display text or unsent content. */
export function sectionDisclosure(
  section: () => readonly string[],
  fallback = true,
) {
  const layout = getContext<
    ReturnType<typeof createDisclosureLayout> | undefined
  >(contextKey);
  let local = $state(fallback);
  return {
    get open() {
      return layout?.read(section(), fallback) ?? local;
    },
    set open(value: boolean) {
      untrack(() =>
        layout ? layout.write(section(), value) : (local = value),
      );
    },
  };
}
