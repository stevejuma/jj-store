export interface CommunityWorkspaceSendHandlers {
  onSendMessage?: (
    roomId: string,
    content: string,
    mentionPubkeys?: readonly string[],
  ) => void | Promise<void>;
  onSendReply?: (
    rootMessageId: string,
    content: string,
    mentionPubkeys?: readonly string[],
  ) => void | Promise<void>;
}
