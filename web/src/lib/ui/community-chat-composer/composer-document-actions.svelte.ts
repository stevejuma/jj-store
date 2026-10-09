import { untrack, type Snippet } from "svelte";

/** Register live editor actions in its host footer and release them on unmount. */
export function useDocumentComposerActions(host: {
  active: () => boolean;
  actions: () => Snippet;
  get: () => Snippet | undefined;
  set: (actions: Snippet | undefined) => void;
}) {
  $effect(() => {
    const current = host.active() ? host.actions() : undefined;
    untrack(() => host.set(current));
    return () =>
      untrack(() => {
        if (host.get() === current) host.set(undefined);
      });
  });
}
