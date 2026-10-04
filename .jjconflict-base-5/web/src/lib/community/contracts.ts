import type { CommunityRoomKind } from "./workspace-model.js";

export interface CommunityReplyTarget {
  eventId: string;
  roomId: string;
  authorId: string;
  authorName: string;
  content: string;
  kind: CommunityRoomKind;
}

export interface CommunityCustomEmoji {
  shortcode: string;
  url: string;
  ownerPubkey: string;
  owned: boolean;
}

export type CommunityMessageReportCategory =
  | "spam"
  | "profanity"
  | "nudity"
  | "impersonation"
  | "malware"
  | "illegal"
  | "other";

export interface CommunityMessageReport {
  category: CommunityMessageReportCategory;
  context?: string;
}

export interface CommunityTimelinePosition {
  messageId: string;
  index?: number;
  offset: number;
  atLatest: boolean;
}

export interface CommunityTimelinePresentation {
  key: string;
  draft: string;
  position?: CommunityTimelinePosition;
}

export interface CommunityHistoryState {
  key: string;
  /** Eligible unread arrivals in this exact timeline; omitted for legacy adapters. */
  unreadEventIds?: readonly string[];
  loading: boolean;
  hasOlder: boolean;
  hasNewer: boolean;
  complete: boolean;
  error?: string;
}

export interface CommunityTopicDraft {
  title: string;
  content: string;
  tags: string[];
}
