import { defineMiraExtension } from "@lapismd/mira/extensions";
import { chatDraftCaret } from "../store-agent-chat-body/chat-caret.js";
export const documentCaret = defineMiraExtension({
  name: "conversation-document-caret",
  codeMirror: () => chatDraftCaret,
});
