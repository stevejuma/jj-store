import { tick } from "svelte";

export async function scrollToConversationParent(
  element: () => HTMLElement | undefined,
  parent: string,
) {
  await tick();
  element()
    ?.querySelector<HTMLElement>(`[data-chat-item="${CSS.escape(parent)}"]`)
    ?.scrollIntoView({ block: "nearest" });
}
