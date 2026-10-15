import {
  defineMiraExtension,
  type MiraExtension,
} from "@lapismd/mira/extensions";

import type { CommunityCustomEmoji } from "../../community/contracts.js";

import type {
  CommunityMemberModel,
  CommunityRoomModel,
} from "../community-workspace/model.js";

import CommunityChannelReference from "./CommunityChannelReference.svelte";

import CommunityEmojiReference from "./CommunityEmojiReference.svelte";

import CommunityEntityReference from "./CommunityEntityReference.svelte";

import CommunityMemberReference from "./CommunityMemberReference.svelte";

import CommunityExternalLink from "./CommunityExternalLink.svelte";
import CommunityLinkPreview from "./CommunityLinkPreview.svelte";
import CommunitySpoiler from "./CommunitySpoiler.svelte";
import { remarkNativeEntityLinks } from "./syntax-entity.js";
import { remarkStandaloneLinks } from "./syntax-external-links.js";

import {
  applyCommunityMarkdownSyntax,
  type CommunityMarkdownHastNode,
} from "./syntax-hast.js";

import {
  tokenizeCommunityMarkdownText,
  type CommunityMarkdownSegment,
} from "./syntax-tokens.js";

export type { CommunityMarkdownSegment };

export interface CommunityMarkdownSyntaxOptions {
  members(): readonly CommunityMemberModel[];
  rooms(): readonly CommunityRoomModel[];
  emojis(): readonly CommunityCustomEmoji[];
  mentionPubkeys(): readonly string[];
  source?(): string;
}

export function createCommunityMarkdownExtension(
  options: CommunityMarkdownSyntaxOptions,
): MiraExtension {
  const standaloneLinks = new Map<number, string>();
  const rehypeCommunitySyntax = () => (tree: CommunityMarkdownHastNode) => {
    applyCommunityMarkdownSyntax(
      tree,
      options,
      tokenizeCommunityMarkdownText,
      standaloneLinks,
    );
  };

  return defineMiraExtension({
    name: "community-message-markdown",
    components: {
      communitychannel: CommunityChannelReference,
      communityemoji: CommunityEmojiReference,
      communityentity: CommunityEntityReference,
      communitymember: CommunityMemberReference,
      communityexternallink: CommunityExternalLink,
      communitylinkpreview: CommunityLinkPreview,
      communityspoiler: CommunitySpoiler,
    },
    rehypePlugins: [rehypeCommunitySyntax],
    remarkPlugins: [
      remarkNativeEntityLinks,
      [
        remarkStandaloneLinks,
        {
          source: () => options.source?.() ?? "",
          paragraphs: standaloneLinks,
        },
      ],
    ],
  });
}

export { tokenizeCommunityMarkdownText };
