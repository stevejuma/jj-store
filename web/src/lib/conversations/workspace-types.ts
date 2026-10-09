import type { ChatSend } from "../controller/agent-chat-types.js";
export type WorkspaceCommentDraft = NonNullable<
  NonNullable<ChatSend["comments"]>["workspace_comments"]
>[number];
export type WorkspaceFileSource = WorkspaceCommentDraft["source"];
