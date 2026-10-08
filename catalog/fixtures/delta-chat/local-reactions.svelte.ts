import type { CommunityReactionModel } from "../../../web/src/lib/community/workspace-model.js";
import type { CommunityCustomEmoji } from "../../../web/src/lib/community/contracts.js";
export class DemoReactions {
  values = $state<Record<string, CommunityReactionModel[]>>({});
  toggle = (id: string, emoji: string, custom?: CommunityCustomEmoji) => {
    const prior = this.values[id] ?? [];
    this.values[id] = prior.some((reaction) => reaction.emoji === emoji)
      ? prior.filter((reaction) => reaction.emoji !== emoji)
      : [
          ...prior,
          {
            emoji,
            count: 1,
            selected: true,
            ...(custom ? { customEmojiUrl: custom.url } : {}),
          },
        ];
  };
}
