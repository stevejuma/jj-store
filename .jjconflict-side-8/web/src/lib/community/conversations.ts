export type CommunityConversationReference =
  | { transport: "nip17"; participants: readonly string[] }
  | { transport: "buzz-dm"; relayUrl: string; groupId: string }
  | {
      transport: "nip29";
      relayUrl: string;
      groupId: string;
      threadId?: string;
    };
