import { untrack } from "svelte";
import type { StoreController } from "./contracts.js";
import { chatContext } from "./agent-chat-context.js";
export function observeAgentChats(read: () => StoreController) {
  observeAgentRuntime(() => read().execution);
  observeAgentChatNavigation(read);
}
function observeAgentRuntime(read: () => StoreController["execution"]) {
  $effect(() => {
    const execution = read();
    if (execution?.available()) untrack(() => void execution.ensure());
  });
}
/** Route changes reopen scoped drafts; uncertain Sends retain their displayed context. */
function observeAgentChatNavigation(read: () => StoreController) {
  let previous = "";
  $effect(() => {
    const controller = read(),
      { origin } = chatContext(controller);
    untrack(() => {
      const prior = previous;
      previous = origin.scope;
      if (
        prior &&
        prior !== origin.scope &&
        controller.execution?.chat.open &&
        !controller.execution.chat.pending &&
        !controller.execution.chat.busy
      )
        controller.execution.chat.showContext(controller);
    });
  });
}
