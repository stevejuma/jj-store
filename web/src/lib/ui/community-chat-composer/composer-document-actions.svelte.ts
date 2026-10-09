import { untrack, type Snippet } from "svelte";

/** Register live editor actions in its host footer and release them on unmount. */
export function useDocumentComposerActions(host: {
  restore?: () => (submitted: string) => string;
  getDraft?: () => ((submitted: string) => string) | undefined;
  setDraft?: (restore: ((submitted: string) => string) | undefined) => void;
  active: () => boolean;
  actions: () => Snippet;
  get: () => Snippet | undefined;
  set: (actions: Snippet | undefined) => void;
}) {
  $effect(() => {
    const restore = host.restore?.();
    untrack(() => host.setDraft?.(restore));
    return () =>
      untrack(() => {
        if (host.getDraft?.() === restore) host.setDraft?.(undefined);
      });
  });
  $effect(() => {
    const current = host.active() ? host.actions() : undefined;
    untrack(() => host.set(current));
    return () =>
      untrack(() => {
        if (host.get() === current) host.set(undefined);
      });
  });
}
