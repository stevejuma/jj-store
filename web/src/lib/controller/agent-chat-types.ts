import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
export type ChatOrigin = Model<"AgentConversationOrigin">;
export type ChatSend = Exact<
  operations["local_agents_chat"]["requestBody"]["content"]["application/json"]
>;
export type ChatAttachment = ChatSend["attachments"][number];
export type ChatDraft = {
  id: string;
  origin: ChatOrigin;
  text: string;
  attachments: ChatAttachment[];
  threadCodebase?: ChatSend["thread_codebase"];
  allowedRepositories?: string[];
};
