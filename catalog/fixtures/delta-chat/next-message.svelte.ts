import type { ChatBodyItem } from "../../../web/src/lib/ui/store-agent-chat-body/model.js";
export const demoSender = {
  id: "demo-user",
  name: "You",
  handle: "",
  initials: "YO",
};
export class DemoNextMessage {
  value = $state("");
  messages = $state<ChatBodyItem[]>([]);
  send() {
    if (!this.value.trim()) return;
    const id = `local-message-${this.messages.length + 1}`,
      text = this.value;
    this.messages.push({
      id,
      kind: "message",
      role: "user",
      label: "Local message from You",
      author: demoSender,
      turn: {
        id: `turn:${id}`,
        author: "You",
        prompt: text,
        summary: "Local demo message; no agent response.",
      },
      blocks: [
        {
          id: `${id}/text`,
          revision: id,
          kind: "markdown",
          label: "Local message",
          text,
        },
      ],
    });
    this.value = "";
  }
}
