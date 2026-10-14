import { tick } from "svelte";

export async function scrollToConversationParent(
  element: () => HTMLElement | undefined,
  parent: string,
) {
  await tick();
  const target = element()?.querySelector<HTMLElement>(
    `[data-chat-item="${CSS.escape(parent)}"]`,
  );
  target?.scrollIntoView({ block: "nearest" });
  target?.focus({ preventScroll: true });
}
