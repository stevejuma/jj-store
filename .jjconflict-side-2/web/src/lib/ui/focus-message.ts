import { tick } from "svelte";
/** Focus only an explicitly restored native message identity, after rendering. */
export function focusNativeMessage(node: HTMLElement, id?: string | null) {
  let generation = 0;
  async function focus(value?: string | null) {
    const ticket = ++generation;
    if (!value) return;
    await tick();
    if (ticket !== generation) return;
    const target = node.querySelector<HTMLElement>(
      `[data-message-id="${CSS.escape(value)}"]`,
    );
    if (!target) return;
    target.tabIndex = -1;
    target.scrollIntoView({ block: "center" });
    target.focus({ preventScroll: true });
  }
  void focus(id);
  return {
    update: (value?: string | null) => void focus(value),
    destroy: () => {
      generation++;
    },
  };
}
