import { getContext, setContext, type Snippet } from "svelte";
export interface FocusDiscussion {
  title(): string;
  content(): Snippet;
  close(): void;
  returnFocus?: HTMLElement | null;
}
const key = Symbol("store-discussion-focus");
/** One structural sheet per application; open draft identities survive remounts. */
export function provideDiscussionFocus() {
  const entries = new Map<object, FocusDiscussion>();
  const opened = new WeakSet<object>();
  let preferred: object | null = null;
  let active = $state.raw<FocusDiscussion>();
  const focus = {
    get active() {
      return active;
    },
    observe(owner: object, open: boolean, entry: FocusDiscussion) {
      if (!open) {
        opened.delete(owner);
        entries.delete(owner);
        if (preferred === owner) {
          preferred = [...entries.keys()].at(-1) ?? null;
          active = preferred ? entries.get(preferred) : undefined;
        }
        return;
      }
      if (!opened.has(owner)) {
        opened.add(owner);
        preferred = owner;
        entry.returnFocus =
          document.activeElement instanceof HTMLElement
            ? document.activeElement
            : null;
      } else
        entry.returnFocus =
          entries.get(owner)?.returnFocus ?? entry.returnFocus;
      entries.set(owner, entry);
      if (preferred === null) preferred = owner;
      if (preferred === owner) active = entry;
    },
    unregister(owner: object) {
      const entry = entries.get(owner);
      entries.delete(owner);
      if (active === entry) active = undefined;
    },
  };
  setContext(key, focus);
  return focus;
}
export function useDiscussionFocus() {
  return getContext<ReturnType<typeof provideDiscussionFocus> | undefined>(key);
}
