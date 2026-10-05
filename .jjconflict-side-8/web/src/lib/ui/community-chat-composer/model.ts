import type { CommunityCustomEmoji } from "../../community/contracts.js";

import type { CommunityUploadedMedia } from "../../community/composer-model.js";

export type {
  CommunityComposerContext,
  CommunityComposerSubmission,
  CommunityUploadedMedia,
} from "../../community/composer-model.js";

export function isFencedCodePosition(value: string, offset: number): boolean {
  const before = value.slice(0, Math.max(0, offset));
  const fences = before.match(/^\s*```/gmu)?.length ?? 0;
  return fences % 2 === 1;
}

export function isInlineCodePosition(value: string, offset: number): boolean {
  const lineStart = value.lastIndexOf("\n", Math.max(0, offset - 1)) + 1;
  const before = value.slice(lineStart, Math.max(lineStart, offset));
  return (before.match(/(?<!`)`(?!`)/gu)?.length ?? 0) % 2 === 1;
}

export function isSafeCommunityEmojiUrl(value: string): boolean {
  const trimmed = value.trim();
  if (/^data:image\/[a-z0-9.+-]+(?:;[^,]*)?,/iu.test(trimmed)) return true;
  try {
    const url = new URL(trimmed);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch {
    return false;
  }
}

export function communityEmojiToken(
  emoji: string,
  custom?: CommunityCustomEmoji,
): string {
  return custom === undefined ? emoji : `:${custom.shortcode}:`;
}
