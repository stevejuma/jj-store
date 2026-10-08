import { getContext, setContext, tick } from "svelte";
import type { AppShellController } from "@lapismd/design-core/shell";
import type { ChatBodyTool } from "../ui/store-agent-chat-body/model.js";
import type { ChatBodyComments } from "../ui/store-agent-chat-body/comments.svelte.js";
const key = Symbol("conversation-inspection");
export function provideConversationInspection(shell: AppShellController) {
  let current = $state<
    { tool: ChatBodyTool; state: ChatBodyComments } | undefined
  >();
  let opener: HTMLElement | undefined,
    wasOpen = false;
  const panel = {
    get current() {
      return current;
    },
    async open(
      tool: ChatBodyTool,
      state: ChatBodyComments,
      element: HTMLElement,
    ) {
      if (!current) wasOpen = !shell.right.closed;
      current = { tool, state };
      opener = element;
      shell.right.open();
      await tick();
      if (shell.mobile.resolvedMode === "mobile")
        shell.mobile.show("right", "right", element);
    },
    async close() {
      current = undefined;
      if (!wasOpen) shell.right.close();
      if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
      await tick();
      if (opener?.isConnected) opener.focus({ preventScroll: true });
    },
  };
  setContext(key, panel);
  return panel;
}
export function useConversationInspection() {
  return getContext<
    ReturnType<typeof provideConversationInspection> | undefined
  >(key);
}
