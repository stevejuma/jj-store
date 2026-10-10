import type { Model } from "../../api/client.js";
import type { ChatBodyComments } from "./comments.svelte.js";
import type { Snippet } from "svelte";
import type { CommunityReactionModel } from "../../community/workspace-model.js";
import type { CommunityCustomEmoji } from "../../community/contracts.js";
import type { CommunityMessageModel } from "../../community/workspace-model.js";
import type { CommunityMemberModel } from "../community-workspace/model.js";

/** Immutable captured content; IDs and revisions belong to the caller. */
export interface ChatBodyBlock {
  id: string;
  revision: string;
  kind: "markdown" | "code" | "patch" | "custom" | "context";
  label: string;
  text: string;
  path?: string;
  /** Captured read output includes line-number/tab prefixes in its exact text. */
  lineNumbered?: boolean;
  /** UTF-16 start within the input supplied to markdownBlocks, before trimming. */
  sourceFrom?: number;
  /** False until an exact committed source is available. */
  commentable?: boolean;
  wholeSection?: boolean;
}
export interface ChatBodyTool {
  id: string;
  revision: string;
  title: string;
  status: "complete" | "error" | "running" | "unknown";
  blocks: readonly ChatBodyBlock[];
  raw: unknown;
  commentable?: boolean;
  detailPlacement?: "inline" | "sidebar";
}
export interface ChatBodyTurn {
  id: string;
  author: string;
  prompt: string;
  summary: string;
  agentName?: string;
  modelLabel?: string;
  createdAt?: number;
}
export interface ChatBodyItem {
  id: string;
  kind: "message" | "tools" | "context" | "reasoning" | "handoff";
  label: string;
  role?: "user" | "assistant";
  turn?: ChatBodyTurn;
  author?: CommunityMemberModel;
  blocks: readonly ChatBodyBlock[];
  tools?: readonly ChatBodyTool[];
  defaultExpanded?: boolean;
  external?: boolean;
  dayLabel?: string;
  message?: CommunityMessageModel;
  timestamp?: { timestamp: string; createdAt?: string };
  observedAt?: number;
  commentBatchId?: string;
  replyContext?: { threadId: string; commentIds: readonly string[] };
}
/** UTF-16 offsets into one exact block; custom blocks use the entire range. */
export interface ChatCommentAnchor {
  blockId: string;
  revision: string;
  from: number;
  to: number;
  quote: string;
}
export interface ChatComment {
  id: string;
  author: CommunityMemberModel;
  text: string;
  status: "draft" | "sending" | "sent" | "replied";
  /** Persisted identity is separate from this editor's stable presentation ID. */
  messageId?: string;
  historical?: boolean;
  batchIds?: string[];
  batchId?: string;
  replyItemId?: string;
  replyDestinations?: { id: string; label: string }[];
  revision?: Model<"CollabObjectRef">;
  replyTo?: string;
  createdAt?: number;
}
export interface ChatCommentThread {
  id: string;
  anchor: ChatCommentAnchor;
  label: string;
  sourceKind?: ChatBodyBlock["kind"];
  comments: ChatComment[];
  source?: Model<"ConversationSourceV1"> | null;
  revision?: Model<"CollabObjectRef">;
}
export interface ChatPromptComment {
  threadId: string;
  commentId: string;
  anchor: ChatCommentAnchor;
  author: CommunityMemberModel;
  text: string;
}
export interface ChatPromptSubmission {
  id: string;
  text: string;
  comments: readonly ChatPromptComment[];
}
export interface ChatSubmissionResult {
  publication: "published" | "unknown" | "refused";
  batchId?: string;
  delivery?: readonly {
    id: string;
    label: string;
    state: string;
    error?: string | null;
  }[];
  recoveryRequired?: boolean;
  error?: string;
}
export interface ChatCommentHost {
  externalBlocks?: readonly ChatBodyBlock[];
  restoreMessage?: (submitted: string) => string;
  canComment?: boolean;
  commentUnavailable?: string;
  items: readonly ChatBodyItem[];
  author?: CommunityMemberModel;
  onSend?: (
    submission: ChatPromptSubmission,
  ) => void | ChatSubmissionResult | Promise<void | ChatSubmissionResult>;
  onNavigateSource?: (thread: ChatCommentThread) => void | Promise<void>;
  onNavigateReply?: (itemId: string) => void | Promise<void>;
  onInspectTool?: (
    tool: ChatBodyTool,
    opener: HTMLElement,
  ) => void | Promise<void>;
}
export interface ChatBodyProps extends ChatCommentHost {
  view?: "document" | "classic";
  /** Reading history; retain existing items and suppress the empty state. */
  loading?: boolean;
  /** Host-specific completed empty state, replacing the default Empty. */
  emptyState?: Snippet;
  beforeItems?: Snippet;
  afterItems?: Snippet;
  supplement?: Snippet<[ChatBodyItem]>;
  metadataSupplement?: Snippet<[ChatBodyItem]>;
  messageActions?: Snippet<[string]>;
  logLabel?: string;
  onOpenThread?: (id: string) => void;
  communityEmojis?: readonly CommunityCustomEmoji[];
  members?: readonly CommunityMemberModel[];
  reactions?: Readonly<Record<string, readonly CommunityReactionModel[]>>;
  reactionOptions?: readonly string[];
  onReact?: (
    messageId: string,
    emoji: string,
    custom?: CommunityCustomEmoji,
  ) => void;
  composer?: Snippet<["document" | "classic"]>;
  footer?: Snippet;
  label?: string;
  customBlock?: Snippet<[ChatBodyBlock]>;
  commentState?: ChatBodyComments;
}
