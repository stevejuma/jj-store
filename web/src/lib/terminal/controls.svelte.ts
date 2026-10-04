import { getContext, setContext } from "svelte";

const key = Symbol("store-terminal-controls");
export interface TerminalControls {
  available: boolean;
  open: boolean;
  toggle(): void;
  trigger: HTMLButtonElement | null;
}

/** One application owns the header control and its corresponding dock. */
export function provideTerminalControls() {
  const controls = $state<TerminalControls>({
    available: false,
    open: false,
    toggle: () => {},
    trigger: null,
  });
  setContext(key, controls);
  return controls;
}

export function terminalControls() {
  return getContext<TerminalControls | undefined>(key);
}
