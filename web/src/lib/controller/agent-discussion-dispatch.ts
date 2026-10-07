import type { StoreController } from "./contracts.js";
import { mentionedAgentIds } from "../community/agent-mentions.js";
/** Tagged Send becomes canonical agent conversation; ordinary replies keep ThreadStore authority. */
export async function sendDiscussionMessage(
  controller: StoreController,
  content: string,
  parent?: string,
) {
  const ids = mentionedAgentIds(content);
  if (!ids.length) return controller.threads?.send(content, parent);
  const manager = controller.execution;
  if (!manager?.available())
    throw new Error("Agent execution is unavailable on this device.");
  manager.chat.showContext(controller, false, parent);
  await manager.chat.send(content, ids);
}
